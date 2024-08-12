use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::web::res::Res;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,

    exp: Option<usize>,
    iss: Option<String>,
    aud: String,
    iat: Option<usize>,
    nbf: Option<usize>,
    jti: Option<String>,

    username: String,
    role: Vec<String>,
    domain: String,
    org: Option<String>,
}

impl Claims {
    pub fn new(
        sub: String,
        aud: String,
        username: String,
        role: Vec<String>,
        domain: String,
        org: Option<String>,
    ) -> Self {
        Self {
            sub,
            exp: None,
            iss: None,
            aud,
            iat: None,
            nbf: None,
            jti: None,
            username,
            role,
            domain,
            org,
        }
    }

    pub fn set_exp(&mut self, exp: usize) {
        self.exp = Some(exp);
    }

    pub fn set_iss(&mut self, iss: String) {
        self.iss = Some(iss);
    }

    pub fn set_iat(&mut self, iat: usize) {
        self.iat = Some(iat);
    }

    pub fn set_nbf(&mut self, nbf: usize) {
        self.nbf = Some(nbf);
    }

    pub fn set_jti(&mut self, jti: String) {
        self.jti = Some(jti);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    user_id: String,
    username: String,
    role: Vec<String>,
    domain: String,
    org: Option<String>,
}

impl User {
    pub fn subject(&self) -> Vec<String> {
        self.role.clone()
    }

    pub fn domain(&self) -> String {
        self.domain.to_string()
    }
}

impl From<Claims> for User {
    fn from(claims: Claims) -> Self {
        User {
            user_id: claims.sub,
            username: claims.username,
            role: claims.role,
            domain: claims.domain,
            org: claims.org,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, _state: &B) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<User>()
            .cloned()
            .ok_or_else(|| Res::new_error(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized"))
    }
}
