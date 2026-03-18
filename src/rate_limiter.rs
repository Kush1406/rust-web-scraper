use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn execute<F, Fut, T>(&self, task: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Acquire a permit before executing the task
        let _permit = self.semaphore.acquire().await?;

        // Execute the task
        let result = task().await?;

        // Permit automatically released when _permit goes out of scope
        Ok(result)
    }
}
