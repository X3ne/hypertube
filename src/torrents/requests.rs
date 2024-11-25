use apistos::ApiComponent;
use garde::Validate;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
pub struct AddTorrentWithMagnet {
    #[garde(pattern("magnet:\\?xt=urn:btih:[a-zA-Z0-9]*"))]
    pub magnet: String,
}
