use crate::OAuthConfig;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FtUser {
    pub id: i64,
    pub email: String,
    pub login: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Provider {
    Ft,
}

#[derive(Debug, Clone)]
pub struct OAuth {
    config: OAuthConfig,
    clients: HashMap<Provider, BasicClient>,
}

impl OAuth {
    pub fn new(config: OAuthConfig) -> Self {
        let ft_client_id = ClientId::new(config.ft_client_id.clone());
        let ft_client_secret = ClientSecret::new(config.ft_client_secret.clone());
        let ft_auth_url =
            AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string()).expect("Failed to parse auth URL");
        let ft_token_url =
            TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string()).expect("Failed to parse token URL");
        let ft_redirect_url = RedirectUrl::new(config.ft_redirect_uri.clone()).expect("Failed to parse redirect URL");

        let ft_client = BasicClient::new(ft_client_id, Some(ft_client_secret), ft_auth_url, Some(ft_token_url))
            .set_redirect_uri(ft_redirect_url);

        let oauth_clients: HashMap<Provider, BasicClient> = vec![(Provider::Ft, ft_client)].into_iter().collect();

        Self {
            config,
            clients: oauth_clients,
        }
    }

    pub fn get_client(&self, provider: &Provider) -> Option<&BasicClient> {
        self.clients.get(provider)
    }
}
