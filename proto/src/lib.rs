pub mod auth_server {
    tonic::include_proto!("fractal.auth");
}
pub mod user_server {
    tonic::include_proto!("fractal.user");
}
pub mod ledger_server {
    tonic::include_proto!("fractal.ledger");
}
