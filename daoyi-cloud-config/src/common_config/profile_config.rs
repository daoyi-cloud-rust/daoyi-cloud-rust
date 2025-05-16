use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProfileConfig {
    #[serde(default = "default_active_profile")]
    pub active: String,
}

fn default_active_profile() -> String {
    String::from("dev")
}
