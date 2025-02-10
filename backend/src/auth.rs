use jsonwebtoken::{encode, decode, errors::Error, Algorithm, EncodingKey, Header, DecodingKey, Validation};
use serde::{de::value::UsizeDeserializer, Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct Claims {
    username: String,
    password: String 
}

const JWT_SECRET: &[u8] = b"secret";

pub fn create_jwt(username: String, password: String) -> Result<String,Error> {
    let claims = Claims {
        username : username,
        password : password
    };

    let header = Header::new(Algorithm::HS256);

    return encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET));
}