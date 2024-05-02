use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn inc(&self, key: String) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let count = data.entry(key).or_insert(0);
        *count += 1;
        Ok(())
    }
}
impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.lock().unwrap().iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
