use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct JWT {
    access_token_secret: String,
    refresh_token_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl JWT {
    fn dispatch_jwt_access_refresh_pair(&self) -> String {
        let my_claims = Claims {
            aud: "foo".to_string(),
            exp: 0,
            iat: 0,
            iss: "bar".to_string(),
            nbf: 0,
            sub: "baz".to_string(),
        };

        encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )
        .unwrap()
    }
}
