use {{ project_name }}_core::settings::CoreSettings;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub core: CoreSettings,
}

impl Settings {
    pub fn load(config_file: Option<&str>) -> anyhow::Result<Self> {
        let mut figment = Figment::from(Serialized::defaults(Self::default())).merge(Toml::file("config/default.toml"));

        if let Some(path) = config_file {
            figment = figment.merge(Toml::file(path));
        }

        figment = figment.merge(Env::prefixed("APP_").split("__"));

        // Platform environment contract (S3): the deployment manifests inject the server ports as
        // bare variables (SERVER_PORT / MANAGEMENT_PORT). Layer them onto the settings tree so the
        // platform's names are honored; APP_-prefixed vars keep working for local overrides.
        // Without this the service reads only its figment prefix and silently binds its compiled-in
        // defaults, ignoring the ports the platform actually gave it.
        figment = figment.merge(
            Env::raw()
                .filter(|key| key == "SERVER_PORT" || key == "MANAGEMENT_PORT")
                .map(|key| {
                    if key == "SERVER_PORT" {
                        "server.port".into()
                    } else {
                        "server.management_port".into()
                    }
                })
                .split("."),
        );

        Ok(figment.extract()?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub management_port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            management_port: 8081,
        }
    }
}
