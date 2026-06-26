pub mod error;
pub mod handlers;
pub mod routes;
pub mod settings;

use anyhow::Result;
use axum::Router;
use settings::CoreSettings;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    settings: CoreSettings,
}

pub struct {{ PrefixName }}{{ SuffixName }}Core {
    state: AppState,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn router(self) -> Router {
        routes::router(self.state)
    }

    pub fn management_router() -> Router {
        routes::management_router()
    }
}

pub struct Builder {
    settings: CoreSettings,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            settings: CoreSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        Ok({{ PrefixName }}{{ SuffixName }}Core {
            state: AppState {
                settings: self.settings,
            },
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
