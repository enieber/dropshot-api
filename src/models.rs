use serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct Project {
    pub name: String,
}
