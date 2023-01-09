use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::sdk::member::Member;

#[derive(Clone)]
pub struct JWT {
    access_token_secret: String,
    refresh_token_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    // company: String,
    exp: usize,
}

impl JWT {
    pub fn new(access_token_secret: String, refresh_token_secret: String) -> Self {
        Self {
            access_token_secret,
            refresh_token_secret,
        }
    }

    pub fn dispatch_jwt_access_refresh_pair(
        &self,
        member: &Member,
    ) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let my_claims = Claims {
            aud: "foo".to_string(),
            exp: (Utc::now() + chrono::Duration::minutes(10)).timestamp() as usize,
            // iat: 0,
            // iss: "bar".to_string(),
            // nbf: 0,
            sub: member.id.to_string(),
        };

        let access_token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )?;

        let refresh_token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(self.refresh_token_secret.as_ref()),
        )?;

        Ok((access_token, refresh_token))
    }

    fn decode_access_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    fn decode_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let mut claims_access_token = self.decode_access_token(access_token)?;
        let claims_refresh_token = self.decode_refresh_token(refresh_token)?;

        claims_access_token.exp += 1000; // TODO

        let token = encode(
            &Header::default(),
            &claims_access_token,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )?;

        Ok(token)
    }
}
