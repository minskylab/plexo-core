use chrono::Utc;
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::graphql::resources::member::Member;

#[derive(Clone)]
pub struct JWT {
    access_token_secret: String,
    refresh_token_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    sub: String,
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
    ) -> Result<(String, String), Error> {
        let access_claims = Claims {
            iss: "Plexo".to_string(),
            aud: "access.plexo.app".to_string(),
            sub: member.id.to_string(),
            exp: (Utc::now() + chrono::Duration::minutes(10)).timestamp() as usize,
        };

        let refresh_claims = Claims {
            iss: "Plexo".to_string(),
            aud: "refresh.plexo.app".to_string(),
            sub: member.id.to_string(),
            exp: (Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.refresh_token_secret.as_ref()),
        )?;

        Ok((access_token, refresh_token))
    }

    fn decode_access_token(&self, token: &str) -> Result<Claims, Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_token_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    fn decode_refresh_token(&self, token: &str) -> Result<Claims, Error> {
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
