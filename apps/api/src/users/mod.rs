use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum UserRole {
    Admin,
    Operator,
    Customer,
    Viewer,
}

impl UserRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Operator => "operator",
            Self::Customer => "customer",
            Self::Viewer => "viewer",
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub role: UserRole,
    pub account_id: Option<Uuid>,
}

impl From<&UserRow> for UserPublic {
    fn from(value: &UserRow) -> Self {
        Self {
            id: value.id,
            username: value.username.clone(),
            email: value.email.clone(),
            full_name: value.full_name.clone(),
            role: value.role,
            account_id: value.account_id,
        }
    }
}

pub async fn find_by_username_or_email(
    pool: &sqlx::PgPool,
    identifier: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, account_id, username, email, password_hash, full_name, phone,
               role, is_active, last_login_at, created_at, updated_at
        FROM users
        WHERE lower(username) = lower($1) OR lower(email) = lower($1)
        LIMIT 1
        "#,
    )
    .bind(identifier)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, account_id, username, email, password_hash, full_name, phone,
               role, is_active, last_login_at, created_at, updated_at
        FROM users
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_username(
    pool: &sqlx::PgPool,
    username: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, account_id, username, email, password_hash, full_name, phone,
               role, is_active, last_login_at, created_at, updated_at
        FROM users
        WHERE lower(username) = lower($1)
        LIMIT 1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_user<'e, E>(
    executor: E,
    id: Uuid,
    username: &str,
    email: &str,
    password_hash: &str,
    full_name: &str,
    role: UserRole,
    account_id: Option<Uuid>,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    sqlx::query(
        r#"
        INSERT INTO users (
            id, account_id, username, email, password_hash, full_name, phone, role,
            is_active, last_login_at, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, NULL, $7, TRUE, NULL, now(), now())
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(role)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn list_users(pool: &sqlx::PgPool) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, account_id, username, email, password_hash, full_name, phone,
               role, is_active, last_login_at, created_at, updated_at
        FROM users
        ORDER BY created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn touch_last_login(pool: &sqlx::PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET last_login_at = now(), updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
