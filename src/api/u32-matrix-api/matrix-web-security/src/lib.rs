use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Secret(String);

impl Secret {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for Secret {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for Secret {
    fn default() -> Self {
        Secret(Uuid::new_v4().to_string())
    }
}
