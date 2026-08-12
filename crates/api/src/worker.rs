use lievre_core::{JobRepository, JobType, RouteRepository, StatsComputer};
use sqlx::SqlitePool;

/// Background worker that polls for pending jobs and processes them.
pub async fn run_worker(pool: SqlitePool, interval_ms: u64) {
    let job_repo = JobRepository::new(pool.clone());
    let route_repo = RouteRepository::new(pool.clone());

    tracing::info!("Worker started (poll every {}ms)", interval_ms);

    loop {
        match job_repo.dequeue().await {
            Ok(Some(job)) => {
                let job_type = job.job_type.parse::<JobType>();
                match job_type {
                    Ok(jt) => {
                        tracing::info!("Processing job {}: {}", job.id, jt);
                        match process_job(&jt, &pool, &route_repo).await {
                            Ok(()) => {
                                if let Err(e) = job_repo.complete(&job.id).await {
                                    tracing::error!(
                                        "Failed to mark job {} complete: {}",
                                        job.id,
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                tracing::error!("Job {} failed: {}", job.id, e);
                                if let Err(e2) = job_repo.fail(&job.id, &e.to_string()).await {
                                    tracing::error!(
                                        "Failed to record failure for job {}: {}",
                                        job.id,
                                        e2
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "Unknown job type '{}' in job {}: {}",
                            job.job_type,
                            job.id,
                            e
                        );
                        let _ = job_repo.fail(&job.id, &e.to_string()).await;
                    }
                }
            }
            Ok(None) => {
                // No pending jobs
            }
            Err(e) => {
                tracing::error!("Worker dequeue error: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    }
}

async fn process_job(
    job_type: &JobType,
    pool: &SqlitePool,
    _route_repo: &RouteRepository,
) -> Result<(), anyhow::Error> {
    match job_type {
        JobType::ComputeStats { activity_id } => {
            compute_stats(activity_id, pool).await?;
        }
        JobType::GenerateGeoJson { activity_id } => {
            // GeoJSON is generated on-the-fly from route coordinates
            // No pre-computation needed — just verify the route exists
            let route = sqlx::query_as::<_, lievre_core::Route>(
                "SELECT * FROM routes WHERE activity_id = ?",
            )
            .bind(activity_id)
            .fetch_optional(pool)
            .await?;

            if route.is_none() {
                return Err(anyhow::anyhow!(
                    "No route found for activity {}",
                    activity_id
                ));
            }
        }
        JobType::FederationDeliver => {
            // Federation delivery is handled by the delivery service
            // This job type is a placeholder — actual delivery happens inline
            tracing::debug!("FederationDeliver job — handled by delivery service");
        }
        _ => {
            tracing::warn!("Unhandled job type: {:?}", job_type);
        }
    }
    Ok(())
}

async fn compute_stats(activity_id: &str, pool: &SqlitePool) -> Result<(), anyhow::Error> {
    // Get the route
    let route =
        sqlx::query_as::<_, lievre_core::Route>("SELECT * FROM routes WHERE activity_id = ?")
            .bind(activity_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No route found for activity {}", activity_id))?;

    // Get the activity for timestamps
    let activity =
        sqlx::query_as::<_, lievre_core::Activity>("SELECT * FROM activities WHERE id = ?")
            .bind(activity_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Activity {} not found", activity_id))?;

    let coordinates: Vec<[f64; 2]> = serde_json::from_str(&route.coordinates)?;
    let elevations: Vec<f64> = route
        .elevation_data
        .as_ref()
        .and_then(|e| serde_json::from_str(e).ok())
        .unwrap_or_default();

    let computer = StatsComputer::new();
    let stats = computer.compute_stats(
        activity_id,
        &coordinates,
        &elevations,
        &[], // timestamps not stored separately yet
        &activity.started_at.to_rfc3339(),
        &activity.created_at.to_rfc3339(),
    );

    // Upsert stats
    sqlx::query(
        "INSERT OR REPLACE INTO activity_stats
         (activity_id, total_distance_meters, total_duration_seconds,
          total_elevation_gain_meters, total_elevation_loss_meters,
          avg_speed_ms, max_speed_ms, computed_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))",
    )
    .bind(&stats.activity_id)
    .bind(stats.total_distance_meters)
    .bind(stats.total_duration_seconds)
    .bind(stats.total_elevation_gain_meters)
    .bind(stats.total_elevation_loss_meters)
    .bind(stats.avg_speed_ms)
    .bind(stats.max_speed_ms)
    .execute(pool)
    .await?;

    tracing::info!(
        "Computed stats for {}: distance={:.0}m, duration={:?}s",
        activity_id,
        stats.total_distance_meters.unwrap_or(0.0),
        stats.total_duration_seconds
    );

    Ok(())
}
