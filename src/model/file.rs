use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileResponse {
    pub name: String,
    pub content: String,
}
