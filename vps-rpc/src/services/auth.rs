use crate::utils::TonicResult;
use database::utils::time::now;
use proto_types::service::{LoginRequest, LoginResponse, auth_service_server::AuthService};
use sqlx::{Pool, Postgres};
use tonic::Request;
use tracing::instrument;

#[derive(Debug)]
pub struct AuthRpc {
    pub conn: Pool<Postgres>,
}

#[tonic::async_trait]
impl AuthService for AuthRpc {
    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn login(&self, request: Request<LoginRequest>) -> TonicResult<LoginResponse> {
        let conn = &self.conn;
        let req = request.into_inner();
        let timestamp = now();

        // add to session table
        sqlx::query("INSERT INTO oauth_user (email, created_at, last_updated) VALUES ($1, $2, $3) ON CONFLICT (email) DO NOTHING")
            .bind(req.user_email.to_owned())
            // TODO: this is supposed to be set only once
            .bind(timestamp)
            .bind(timestamp)
            .execute(conn)
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO session (user_id, session_id, expires_at)
            VALUES (
                (SELECT ID FROM oauth_user WHERE email = $1 LIMIT 1),
                $2, $3)
                ON CONFLICT (user_id) DO UPDATE SET
                session_id = excluded.session_id,
                expires_at = excluded.expires_at",
        )
        .bind(req.user_email)
        .bind(req.access_token)
        .bind(timestamp + req.max_age)
        .execute(conn)
        .await
        .unwrap();

        Ok(LoginResponse { success: true }.into())
    }
}
