use apistos::ApiComponent;
use garde::Validate;
use oauth2::AuthorizationCode;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}
