use crate::error::ApiError;
use crate::infrastructure::models::user::User;
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::http::Error;
use actix_web::{FromRequest, HttpRequest};
use apistos::ApiSecurity;
use futures::future::{ready, Ready};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(name = "session", api_key_in = "cookie"))))]
pub struct Security {
    pub session: actix_session::Session,
}

impl FromRequest for Security {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        ready(Ok(Security { session }))
    }
}

pub trait GetUserFromSession {
    fn get_user(&self, pool: &SqlitePool) -> impl std::future::Future<Output = Result<User, ApiError>>;
    fn is_authenticated(&self) -> bool;
}

impl GetUserFromSession for Security {
    async fn get_user(&self, pool: &SqlitePool) -> Result<User, ApiError> {
        if let Some(user_id) = self.session.get::<Uuid>("user_id")? {
            tracing::info!("User ID: {:?}", user_id);
            let user = User::get_by_id(pool, user_id).await?;

            return Ok(user);
        };

        Err(ApiError::Unauthorized)
    }

    fn is_authenticated(&self) -> bool {
        self.session.get::<Uuid>("user_id").is_ok()
    }
}
