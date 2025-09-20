use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SECRET: &[u8] = b"change-me-in-prod";

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub fn hash_pwd(pw: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(pw, bcrypt::DEFAULT_COST)
}

pub fn verify_pwd(hash: &str, pw: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(pw, hash)
}

pub fn make_token(uid: &Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (std::time::SystemTime::now() + std::time::Duration::from_secs(86400))
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims {
        sub: uid.to_string(),
        exp,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

pub fn validate_token(token: &str) -> Result<Uuid, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Uuid::parse_str(&token_data.claims.sub).map_err(|_| jsonwebtoken::errors::Error::InvalidToken)
}
