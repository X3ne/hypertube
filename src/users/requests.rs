use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, garde::Validate)]
pub struct RegisterUser {
    #[garde(length(min = 3, max = 50))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, garde::Validate)]
pub struct PatchUser {
    #[garde(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[garde(email)]
    pub email: Option<String>,
    #[garde(length(min = 8))]
    pub password: Option<String>,
}
