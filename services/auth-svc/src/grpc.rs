pub mod auth_server {
    #![allow(clippy::all)]
    tonic::include_proto!("fractal.auth");
}
use auth_server::{auth_server::Auth, AuthRes, LoginReq, RegisterReq};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct AuthService {
    pub db: PgPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(&self, req: Request<RegisterReq>) -> Result<Response<AuthRes>, Status> {
        let body = req.into_inner();
        let hash = crate::jwt::hash_pwd(&body.password).map_err(|_| Status::internal("hash"))?;
        let id = uuid::Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)",
            id,
            body.username,
            hash
        )
        .execute(&self.db)
        .await
        .map_err(|_| Status::already_exists("user"))?;
        let token = crate::jwt::make_token(&id).map_err(|_| Status::internal("jwt"))?;
        Ok(Response::new(AuthRes { token }))
    }

    async fn login(&self, req: Request<LoginReq>) -> Result<Response<AuthRes>, Status> {
        let body = req.into_inner();
        let row = sqlx::query!(
            "SELECT id, password_hash FROM users WHERE username = $1",
            body.username
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|_| Status::internal("db"))?
        .ok_or(Status::unauthenticated("bad creds"))?;
        let ok = crate::jwt::verify_pwd(&row.password_hash, &body.password)
            .map_err(|_| Status::internal("verify"))?;
        if !ok {
            return Err(Status::unauthenticated("bad creds"));
        }
        let token = crate::jwt::make_token(&row.id).map_err(|_| Status::internal("jwt"))?;
        Ok(Response::new(AuthRes { token }))
    }
}
