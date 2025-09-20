pub mod user_server {
    #![allow(clippy::all)]
    tonic::include_proto!("fractal.user");
}
use user_server::{user_server::User, GetUserReq, UserRes};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub struct UserService {
    pub db: PgPool,
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        req: Request<GetUserReq>,
    ) -> Result<Response<UserRes>, Status> {
        let uid = uuid::Uuid::parse_str(&req.into_inner().user_id)
            .map_err(|_| Status::invalid_argument("bad uuid"))?;
        let row = sqlx::query!(
            "SELECT username FROM users WHERE id = $1",
            uid
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|_| Status::internal("db"))?
        .ok_or(Status::not_found("user"))?;
        Ok(Response::new(UserRes {
            user_id: uid.to_string(),
            username: row.username,
        }))
    }
}
