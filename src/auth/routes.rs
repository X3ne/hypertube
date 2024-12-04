use crate::auth::error::AuthError;
use crate::auth::requests::OAuthCallback;
use crate::auth::responses::OAuthResponse;
use crate::error::ApiError;
use crate::infrastructure::authorization::security::Security;
use crate::infrastructure::authorization::AuthorizationService;
use crate::infrastructure::models::user::{User, UserInsert};
use crate::infrastructure::oauth::{FtUser, OAuth, Provider};
use crate::users::error::UserError;
use actix_web::{web, HttpRequest};
use apistos::actix::NoContent;
use apistos::api_operation;
use apistos::web::{get, resource, scope, ServiceConfig};
use oauth2::reqwest::async_http_client;
use oauth2::{http, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use reqwest::Url;
use std::sync::Arc;
use tracing::instrument;
// **
// * TODO:
// * - Implement pkce
// * - Implement csrf
// **

pub fn config_auth(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(
                scope("/oauth").service(
                    scope("/42")
                        .service(resource("/login").route(get().to(login_42)))
                        .service(resource("/callback").route(get().to(callback_42))),
                ),
            )
            .service(
                scope("") // .service(resource("/login").route(post().to(login)))
                    .service(resource("/logout").route(get().to(logout))),
            ),
    );
}

#[api_operation(tag = "auth", operation_id = "login_42", summary = "Login with 42 account")]
#[instrument(skip(oauth))]
pub async fn login_42(oauth: web::Data<Arc<OAuth>>) -> Result<web::Json<OAuthResponse>, ApiError> {
    let (auth_url, csrf_state) = oauth
        .get_client(&Provider::Ft)
        .ok_or(AuthError::InvalidProvider)?
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .url();

    Ok(web::Json(OAuthResponse {
        url: auth_url.to_string(),
    }))
}

#[api_operation(tag = "auth", operation_id = "callback_42", summary = "Callback for 42 OAuth")]
#[instrument(skip(req, oauth, security))]
pub async fn callback_42(
    req: HttpRequest,
    query: web::Query<OAuthCallback>,
    oauth: web::Data<Arc<OAuth>>,
    pool: web::Data<sqlx::SqlitePool>,
    security: Security,
) -> Result<NoContent, ApiError> {
    let oauth_callback = query.into_inner();

    let token = oauth
        .get_client(&Provider::Ft)
        .ok_or(AuthError::InvalidProvider)?
        .exchange_code(AuthorizationCode::new(oauth_callback.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("Failed to exchange code: {:?}", e);
            AuthError::FailedToRequestToken
        })?;

    let mut headers = http::HeaderMap::new();
    headers.insert(
        http::header::AUTHORIZATION,
        http::HeaderValue::from_str(&format!("Bearer {}", token.access_token().secret())).unwrap(),
    );

    let res = async_http_client(oauth2::HttpRequest {
        url: Url::parse("https://api.intra.42.fr/v2/me").unwrap(),
        method: http::method::Method::GET,
        headers,
        body: vec![],
    })
    .await
    .map_err(|e| {
        tracing::error!("Failed to request users info: {:?}", e);
        AuthError::FailedToRequestUserInfo
    })?;

    if !res.status_code.is_success() {
        tracing::error!("Failed to request users info: {:?}", res);
        return Err(AuthError::FailedToRequestUserInfo.into());
    }

    let ft_user = serde_json::from_slice::<FtUser>(&res.body).map_err(|e| {
        tracing::error!("Failed to parse users info: {:?}", e);
        AuthError::FailedToRequestUserInfo
    })?;

    let user = match User::get_by_email(&pool, &ft_user.email).await {
        Ok(user) => user,
        Err(err) => match err {
            UserError::UserNotFound => {
                User::create(
                    &pool,
                    &UserInsert {
                        email: ft_user.email,
                        username: ft_user.login,
                        password: None,
                        permissions: AuthorizationService::default_permissions(),
                    },
                )
                .await?
            }
            _ => return Err(err.into()),
        },
    };

    security.session.insert("user_id", user.id)?;
    security.session.renew();

    Ok(NoContent)
}

#[api_operation(tag = "auth", operation_id = "logout", summary = "Logout the current user")]
pub async fn logout(security: Security) -> Result<NoContent, ApiError> {
    security.session.clear();

    Ok(NoContent)
}
