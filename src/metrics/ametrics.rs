use anyhow::Result;
use core::str;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}
impl AMetrics {
    pub fn new(names: &[&'static str]) -> Self {
        let metrics_name = names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AMetrics {
            data: Arc::new(metrics_name),
        }
    }
    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let count = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key not found in key: {}", key))?;
        count.fetch_add(1, std::sync::atomic::Ordering::Release);
        Ok(())
    }
}
impl Display for AMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
impl Clone for AMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}
