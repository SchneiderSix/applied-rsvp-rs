#[derive(Default)]
pub struct PathConfig {
    config: String,
    data: String,
    cache: String
}

impl PathConfig {
    pub fn get_config(&self) -> &str {
        &self.config
    }
    pub fn set_config(&mut self, path: String) {
        self.config = path;
    }
    pub fn get_data(&self) -> &str {
        &self.data
    }
    pub fn set_data(&mut self, path: String) {
        self.data = path;
    }
    pub fn get_cache(&self) -> &str {
        &self.cache
    }
    pub fn set_cache(&mut self, path: String) {
        self.cache = path;
    }
}
