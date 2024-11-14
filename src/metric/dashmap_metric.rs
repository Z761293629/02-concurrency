use core::fmt;
use std::sync::Arc;

use dashmap::DashMap;

use super::Inc;

#[derive(Debug, Clone)]
pub struct DashMetric {
    data: Arc<DashMap<String, i64>>,
}

impl Default for DashMetric {
    fn default() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
}

impl Inc for DashMetric {
    fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut value = self.data.entry(key.into()).or_insert(0);
        (*value) += 1;
        Ok(())
    }
}

impl fmt::Display for DashMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{} : {}", entry.key(), entry.value())?;
        }
        writeln!(f)?;
        Ok(())
    }
}
