use crate::error::ApiError;
use crate::users::error::UserError;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub permissions: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub permissions: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<DbUser> for User {
    type Error = UserError;

    fn try_from(user: DbUser) -> Result<Self, Self::Error> {
        let user = User {
            id: Uuid::parse_str(&user.id).map_err(|e| {
                tracing::error!("Error parsing UUID: {}", e);
                UserError::DatabaseError
            })?,
            username: user.username,
            email: user.email,
            password: user.password,
            permissions: user.permissions,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        Ok(user)
    }
}

#[derive(Debug, PartialEq)]
pub struct UserInsert {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub permissions: u32,
}

#[derive(Debug, Default, PartialEq)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub permissions: Option<u32>,
}

impl User {
    pub async fn create(pool: &SqlitePool, user: &UserInsert) -> Result<User, UserError> {
        let uuid = Uuid::new_v4().to_string();

        let result = sqlx::query_as!(
            DbUser,
            r#"
            INSERT INTO user (id, username, email, password, permissions)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING *
            "#,
            uuid,
            user.username,
            user.email,
            user.password,
            user.permissions
        )
        .fetch_one(pool)
        .await?;

        Ok(result.try_into()?)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<User, UserError> {
        let id = id.to_string();

        let result = sqlx::query_as!(
            DbUser,
            r#"
            SELECT *
            FROM user
            WHERE id = ?1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.try_into()?)
    }

    pub async fn get_by_username(pool: &SqlitePool, username: &str) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            DbUser,
            r#"
            SELECT *
            FROM user
            WHERE username = ?1
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(result.try_into()?)
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            DbUser,
            r#"
            SELECT *
            FROM user
            WHERE email = ?1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(result.try_into()?)
    }

    pub async fn update(pool: &SqlitePool, id: Uuid, user: &UserUpdate) -> Result<User, UserError> {
        let id = id.to_string();

        let result = sqlx::query_as!(
            DbUser,
            r#"
            UPDATE user
            SET username = COALESCE(?2, username),
                email = COALESCE(?3, email),
                password = COALESCE(?4, password),
                permissions = COALESCE(?5, permissions),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            RETURNING *
            "#,
            id,
            user.username,
            user.email,
            user.password,
            user.permissions
        )
        .fetch_one(pool)
        .await?;

        Ok(result.try_into()?)
    }

    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<(), UserError> {
        let id = id.to_string();

        let result = sqlx::query_as!(
            DbUser,
            r#"
            DELETE FROM user
            WHERE id = ?1
            RETURNING *
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
