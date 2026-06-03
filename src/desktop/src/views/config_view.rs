use bakunin_config::Value;
use dioxus::prelude::*;
use dsot_lib::DsotState;

#[component]
pub fn ConfigView() -> Element {
    let state = use_context::<DsotState>();

    let config_store = use_store(|| ConfigStore::from(&state));

    let on_save = move || match config_store.read().save(&state) {
        Ok(()) => log::info!("Config saved"),
        Err(e) => log::error!("Failed to save config: {}", e),
    };

    rsx! {
        h1 { "Config" }

        div {
            class: "form-group",
            label { "Token" }
            input {
                r#type: "text",
                value: config_store.token(),
                oninput: move |e| config_store.token().set(e.value()),
            }
        }

        div {
            class: "form-group",
            label { "User" }
            input {
                r#type: "text",
                value: config_store.user(),
                oninput: move |e| config_store.user().set(e.value()),
            }
        }

        button {
            onclick: move |_| on_save(),
            "Save"
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Store)]
struct ConfigStore {
    token: String,
    user: String,
}

impl From<&DsotState> for ConfigStore {
    fn from(state: &DsotState) -> Self {
        Self {
            token: state.config.value.token.clone(),
            user: state.config.value.user.clone(),
        }
    }
}

impl TryInto<Value> for ConfigStore {
    type Error = bakunin_config::model::ModelError;

    fn try_into(self) -> std::prelude::v1::Result<Value, Self::Error> {
        Value::serialize::<ConfigStore>(self)
    }
}

impl ConfigStore {
    pub fn save(&self, state: &DsotState) -> anyhow::Result<()> {
        if let Some(layer) = state.config.get_config_layer() {
            let mut to_write = layer.read_value()?;
            let store: Value = self.clone().try_into()?;

            to_write.merge(&store);

            layer.write_value(&to_write)?
        }

        Ok(())
    }
}
