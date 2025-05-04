// src/api.rs
use dropshot::{ApiDescription, endpoint};
use crate::handlers;

pub fn api() -> ApiDescription<()> {
    let mut api = ApiDescription::new();
    api.handle::<handlers::get_project>();
    api
}

