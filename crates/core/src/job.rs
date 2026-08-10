use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Processing => write!(f, "processing"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(JobStatus::Pending),
            "processing" => Ok(JobStatus::Processing),
            "completed" => Ok(JobStatus::Completed),
            "failed" => Ok(JobStatus::Failed),
            _ => Err(anyhow::anyhow!("Invalid job status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobType {
    ProcessGpx { activity_id: String },
    ProcessFit { activity_id: String },
    ProcessTcx { activity_id: String },
    ComputeStats { activity_id: String },
    GenerateGeoJson { activity_id: String },
}

impl std::fmt::Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::ProcessGpx { activity_id } => write!(f, "process_gpx:{}", activity_id),
            JobType::ProcessFit { activity_id } => write!(f, "process_fit:{}", activity_id),
            JobType::ProcessTcx { activity_id } => write!(f, "process_tcx:{}", activity_id),
            JobType::ComputeStats { activity_id } => write!(f, "compute_stats:{}", activity_id),
            JobType::GenerateGeoJson { activity_id } => write!(f, "generate_geojson:{}", activity_id),
        }
    }
}

impl std::str::FromStr for JobType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid job type format: {}", s));
        }

        let activity_id = parts[1].to_string();
        match parts[0] {
            "process_gpx" => Ok(JobType::ProcessGpx { activity_id }),
            "process_fit" => Ok(JobType::ProcessFit { activity_id }),
            "process_tcx" => Ok(JobType::ProcessTcx { activity_id }),
            "compute_stats" => Ok(JobType::ComputeStats { activity_id }),
            "generate_geojson" => Ok(JobType::GenerateGeoJson { activity_id }),
            _ => Err(anyhow::anyhow!("Unknown job type: {}", parts[0])),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: String,
    pub job_type: String,
    pub payload: String,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub priority: i32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub next_retry_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JobRepository {
    pool: SqlitePool,
}

impl JobRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn enqueue(&self, job_type: JobType, priority: i32) -> Result<Job, anyhow::Error> {
        let job_type_str = job_type.to_string();
        let payload = serde_json::to_string(&job_type)?;

        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO jobs (id, job_type, payload, status, priority, created_at)
             VALUES (?, ?, ?, 'pending', ?, ?)"
        )
        .bind(&id)
        .bind(&job_type_str)
        .bind(&payload)
        .bind(priority)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(&id)
            .fetch_one(&self.pool)
            .await?;

        Ok(job)
    }

    pub async fn dequeue(&self) -> Result<Option<Job>, anyhow::Error> {
        let now = Utc::now().to_rfc3339();

        // Get next pending job that's ready to process
        let job = sqlx::query_as::<_, Job>(
            "SELECT * FROM jobs
             WHERE status = 'pending'
             AND (next_retry_at IS NULL OR next_retry_at <= ?)
             ORDER BY priority DESC, created_at ASC
             LIMIT 1"
        )
        .bind(&now)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(job) = &job {
            // Mark as processing
            let processing_at = Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE jobs SET status = 'processing', started_at = ?, attempts = attempts + 1
                 WHERE id = ?"
            )
            .bind(&processing_at)
            .bind(&job.id)
            .execute(&self.pool)
            .await?;

            // Fetch the updated job
            let updated_job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
                .bind(&job.id)
                .fetch_one(&self.pool)
                .await?;

            return Ok(Some(updated_job));
        }

        Ok(None)
    }

    pub async fn complete(&self, job_id: &str) -> Result<(), anyhow::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE jobs SET status = 'completed', completed_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fail(&self, job_id: &str, error: &str) -> Result<(), anyhow::Error> {
        let now = Utc::now();

        // Get job to check attempts
        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(job_id)
            .fetch_one(&self.pool)
            .await?;

        if job.attempts >= job.max_attempts {
            // Max attempts reached, mark as failed
            sqlx::query(
                "UPDATE jobs SET status = 'failed', error = ?, completed_at = ? WHERE id = ?"
            )
            .bind(error)
            .bind(now.to_rfc3339())
            .bind(job_id)
            .execute(&self.pool)
            .await?;
        } else {
            // Calculate next retry with exponential backoff
            let backoff_seconds = 2_i64.pow(job.attempts as u32) * 60; // 1min, 2min, 4min, etc.
            let next_retry = (now + chrono::Duration::seconds(backoff_seconds)).to_rfc3339();

            sqlx::query(
                "UPDATE jobs SET status = 'pending', error = ?, next_retry_at = ? WHERE id = ?"
            )
            .bind(error)
            .bind(next_retry)
            .bind(job_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_pending_count(&self) -> Result<i64, anyhow::Error> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM jobs WHERE status = 'pending'"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.0)
    }

    pub async fn cleanup_old_jobs(&self, days: i64) -> Result<u64, anyhow::Error> {
        let cutoff = (Utc::now() - chrono::Duration::days(days)).to_rfc3339();
        let result = sqlx::query(
            "DELETE FROM jobs WHERE status IN ('completed', 'failed') AND completed_at < ?"
        )
        .bind(cutoff)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                job_type TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                attempts INTEGER NOT NULL DEFAULT 0,
                max_attempts INTEGER NOT NULL DEFAULT 3,
                priority INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                started_at TEXT,
                completed_at TEXT,
                next_retry_at TEXT,
                error TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_enqueue_job() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        let job = repo.enqueue(JobType::ProcessGpx { activity_id: "test-123".to_string() }, 0).await.unwrap();

        assert_eq!(job.status, "pending");
        assert_eq!(job.job_type, "process_gpx:test-123");
        assert_eq!(job.attempts, 0);
    }

    #[tokio::test]
    async fn test_dequeue_job() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        repo.enqueue(JobType::ProcessGpx { activity_id: "test-123".to_string() }, 0).await.unwrap();

        let job = repo.dequeue().await.unwrap();
        assert!(job.is_some());

        let job = job.unwrap();
        assert_eq!(job.status, "processing");
        assert_eq!(job.attempts, 1);
    }

    #[tokio::test]
    async fn test_complete_job() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        let job = repo.enqueue(JobType::ProcessGpx { activity_id: "test-123".to_string() }, 0).await.unwrap();
        repo.dequeue().await.unwrap();
        repo.complete(&job.id).await.unwrap();

        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(&job.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(job.status, "completed");
        assert!(job.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_fail_job_retry() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        let job = repo.enqueue(JobType::ProcessGpx { activity_id: "test-123".to_string() }, 0).await.unwrap();
        repo.dequeue().await.unwrap();
        repo.fail(&job.id, "Parse error").await.unwrap();

        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(&job.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(job.status, "pending"); // Should retry
        assert!(job.next_retry_at.is_some());
        assert_eq!(job.error, Some("Parse error".to_string()));
    }

    #[tokio::test]
    async fn test_fail_job_max_attempts() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        let job = repo.enqueue(JobType::ProcessGpx { activity_id: "test-123".to_string() }, 0).await.unwrap();

        // Simulate max attempts: dequeue, fail (retry), dequeue, fail (retry), dequeue, fail (permanent)
        for i in 0..3 {
            let dequeued = repo.dequeue().await.unwrap().unwrap();
            assert_eq!(dequeued.attempts, i + 1);

            // Fail the job (will retry if attempts < max_attempts)
            repo.fail(&dequeued.id, &format!("Error attempt {}", i + 1)).await.unwrap();

            // Reset next_retry_at to make job immediately available (for test purposes)
            if i < 2 {
                sqlx::query("UPDATE jobs SET next_retry_at = NULL WHERE id = ?")
                    .bind(&dequeued.id)
                    .execute(&pool)
                    .await
                    .unwrap();
            }
        }

        // After 3 attempts and 3 failures, job should be permanently failed
        let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = ?")
            .bind(&job.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(job.status, "failed");
        assert_eq!(job.attempts, 3);
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let pool = setup_db().await;
        let repo = JobRepository::new(pool.clone());

        repo.enqueue(JobType::ProcessGpx { activity_id: "low".to_string() }, 0).await.unwrap();
        repo.enqueue(JobType::ProcessGpx { activity_id: "high".to_string() }, 10).await.unwrap();

        let job = repo.dequeue().await.unwrap().unwrap();
        // High priority should come first
        assert!(job.payload.contains("high"));
    }

    #[tokio::test]
    async fn test_job_type_serialization() {
        let job_type = JobType::ComputeStats { activity_id: "test-123".to_string() };
        let serialized = job_type.to_string();
        let parsed: JobType = serialized.parse().unwrap();
        assert_eq!(job_type, parsed);
    }
}
