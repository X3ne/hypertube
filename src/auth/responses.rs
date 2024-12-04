use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, Debug, ApiComponent, JsonSchema)]
pub struct OAuthResponse {
    pub url: String,
}
