use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::Inc;

#[derive(Debug, Clone)]
pub struct RwMetric {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for RwMetric {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Inc for RwMetric {
    fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut guard = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let value = guard.entry(key.into()).or_insert(0);
        *value += 1;
        Ok(())
    }
}

impl fmt::Display for RwMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.data.read().unwrap();
        for (key, value) in guard.iter() {
            writeln!(f, "{} : {}", key, value)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
