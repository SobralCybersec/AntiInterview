use crate::domain::Configuration;
use anyhow::Result;

pub trait ConfigurationRepository {
    fn load(&self) -> Result<Configuration>;
    fn save(&self, config: &Configuration) -> Result<()>;
}
