use crate::error::Result;
use crate::{configuration::Settings, handler::user_handler::DatabaseState, model};

#[derive(Clone)]
pub struct Application {
    // hmac_secret: Secret<String>,
    pub address: String,
    pub port: u16,
    pub settings: Settings,
    pub database_state: DatabaseState,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let database_state = DatabaseState {
            model: model::ModelManager::lazy_connect(&configuration.database)
                .await
                .expect("connect database occur error!"),
        };
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        tracing::info!("starting application, listening address is {:?}", address);
        Ok(Application {
            address,
            port: configuration.application.port,
            settings: configuration,
            database_state: database_state,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
