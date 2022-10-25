use super::jwt::Claims;


#[derive(Debug)]
#[allow(dead_code)]
pub struct AuthUser {
    id: String,
    permissions: Vec<String>,
}

impl AuthUser {
    pub fn new(id: String, permissions: Vec<String>) -> Self {
        Self {
            id,
            permissions,
        }
    }

    pub fn default() -> Self {
        Self {
            id: "0".to_owned(),
            permissions: vec!["ROLE_ANY".to_owned(), ],
        }
    }
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        Self {
            id: claims.id,
            permissions: claims.permissions,
        }
    }
}
