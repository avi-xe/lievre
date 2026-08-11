use lievre_core::job::{Job, JobRepository, JobType};
use sqlx::SqlitePool;
use url::Url;

/// Federation delivery service
#[derive(Clone)]
pub struct DeliveryService {
    pool: SqlitePool,
}

impl DeliveryService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Queue an activity for delivery to a remote inbox
    pub async fn deliver(
        &self,
        activity: &serde_json::Value,
        inbox_url: &str,
    ) -> anyhow::Result<()> {
        let _payload = serde_json::json!({
            "activity": activity,
            "inbox_url": inbox_url,
        });

        let job_repo = JobRepository::new(self.pool.clone());
        job_repo.enqueue(JobType::FederationDeliver, 0).await?;

        tracing::info!("Queued delivery to {}", inbox_url);
        Ok(())
    }

    /// Deliver activity to all followers of a user
    pub async fn deliver_to_followers(
        &self,
        activity: &serde_json::Value,
        user_id: &str,
    ) -> anyhow::Result<()> {
        // Get all accepted followers
        let followers = sqlx::query_as::<_, (String,)>(
            "SELECT follower_actor_url FROM actor_follows WHERE following_actor_url = ? AND status = 'accepted'",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        for (follower_url,) in followers {
            // Extract inbox URL from actor URL (simplified - real impl would fetch actor)
            if let Ok(actor_url) = Url::parse(&follower_url) {
                let inbox_url = format!(
                    "{}://{}/inbox",
                    actor_url.scheme(),
                    actor_url.host_str().unwrap_or("")
                );
                self.deliver(activity, &inbox_url).await?;
            }
        }

        Ok(())
    }

    /// Process pending deliveries
    pub async fn process_pending(&self) -> anyhow::Result<()> {
        let job_repo = JobRepository::new(self.pool.clone());

        // Process up to 10 jobs
        for _ in 0..10 {
            let job = job_repo.dequeue().await?;
            match job {
                Some(job) => match self.process_job(&job).await {
                    Ok(()) => {
                        job_repo.complete(&job.id).await?;
                    }
                    Err(e) => {
                        tracing::error!("Delivery failed for job {}: {}", job.id, e);
                        job_repo.fail(&job.id, &e.to_string()).await?;
                    }
                },
                None => break,
            }
        }

        Ok(())
    }

    async fn process_job(&self, job: &Job) -> anyhow::Result<()> {
        let payload: serde_json::Value = serde_json::from_str(&job.payload)?;
        let activity = payload
            .get("activity")
            .ok_or_else(|| anyhow::anyhow!("Missing activity"))?;
        let inbox_url = payload
            .get("inbox_url")
            .and_then(|u| u.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing inbox_url"))?;

        // In a real implementation, this would:
        // 1. Sign the activity with HTTP Signatures
        // 2. POST to the inbox URL
        // 3. Handle responses

        tracing::info!("Would deliver to {}: {}", inbox_url, activity);
        Ok(())
    }
}
