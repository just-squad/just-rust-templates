// public modules
pub mod cfg;
pub mod error;

// private modules
mod system;

use axum::Router;

use cfg::ApiConfiguration;

pub struct ApiProvider {
    api_configuration: ApiConfiguration,
}

impl ApiProvider {
    pub fn new(api_cfg: &ApiConfiguration) -> Self {
        ApiProvider {
            api_configuration: api_cfg.clone(),
        }
    }

    pub async fn start_server(&self) -> anyhow::Result<()> {
        // let my_router = my_controller::create_router();
        let debug_router = system::create_debug_router();

        // Main application router
        let app_router = Router::new()
            // .nest("/api/my_api", my_router)
            ;

        let app_listener = tokio::net::TcpListener::bind(format!(
            "0.0.0.0:{}",
            self.api_configuration.http_port
        ))
        .await?;
        tracing::info!("🚀 Main server listening on {}", app_listener.local_addr()?);
        let app_server = axum::serve(app_listener, app_router);

        // Debug and system router
        let debug_listener = tokio::net::TcpListener::bind(format!(
            "0.g.0.0:{}",
            self.api_configuration.debug_port
        ))
        .await?;
        tracing::info!("🩺 Debug server listening on {}", debug_listener.local_addr()?);
        let debug_server = axum::serve(debug_listener, debug_router);

        // Run both servers concurrently
        tokio::try_join!(app_server, debug_server)?;

        Ok(())
    }
}
