use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};

use super::Inc;

#[derive(Debug, Clone)]
pub struct MutexMetric {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for MutexMetric {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
impl Inc for MutexMetric {
    fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut mutex = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let value = mutex.entry(key.into()).or_insert(0);
        *value += 1;
        Ok(())
    }
}

impl fmt::Display for MutexMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mutex = self.data.lock().unwrap();
        for (key, value) in mutex.iter() {
            writeln!(f, "{} : {}", key, value)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
