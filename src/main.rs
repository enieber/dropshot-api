use dropshot::ApiDescription;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::ServerBuilder;

use crate::projects::project_api_mod;
use crate::server::ServerImpl;

mod models;
mod projects;
mod server;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Set up a logger.
    let log =
        ConfigLogging::StderrTerminal {
            level: ConfigLoggingLevel::Info,
        }
        .to_logger("minimal-example")
        .map_err(|e| e.to_string())?;

    // Describe the API.
    let api: ApiDescription<()> =
        project_api_mod::api_description::<ServerImpl>().unwrap();

    // Start the server.
    let server = ServerBuilder::new(api, (), log)
        .start()
        .map_err(|error| format!("failed to start server: {}", error))?;

    server.await
}
