mod dashmap_metric;
mod mutex_metric;
mod rwlock_metric;
pub use dashmap_metric::DashMetric;
pub use mutex_metric::MutexMetric;
pub use rwlock_metric::RwMetric;

use core::fmt;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Metric<T> {
    inner: T,
}

pub trait Inc {
    fn inc(&self, key: impl Into<String>) -> Result<()>;
}

impl<T> Metric<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self {
            inner: T::default(),
        }
    }
}

impl<T> Default for Metric<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Inc for Metric<T>
where
    T: Inc,
{
    fn inc(&self, key: impl Into<String>) -> Result<()> {
        self.inner.inc(key)
    }
}

impl<T> fmt::Display for Metric<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
