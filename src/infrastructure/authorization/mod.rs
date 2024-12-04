pub mod security;

#[derive(Debug, Clone, Copy)]
pub enum Permission {
    PostComment = 0b00000001,
    UpdateComment = 0b00000010,
    DeleteComment = 0b00000100,
    UpdateProfiles = 0b00001000,
    DeleteProfiles = 0b00010000,
    ServerAdmin = 0b10000000,
}

impl Permission {
    pub fn as_bitmask(&self) -> u32 {
        *self as u32
    }
}

pub struct AuthorizationService;

impl AuthorizationService {
    pub fn combine_permissions(permissions: &[Permission]) -> u32 {
        permissions
            .iter()
            .fold(0, |acc, permission| acc | permission.as_bitmask())
    }

    pub fn default_permissions() -> u32 {
        AuthorizationService::combine_permissions(&[Permission::PostComment])
    }

    pub fn has_permission(permissions: u32, permission: Permission) -> bool {
        permissions & permission.as_bitmask() != 0
    }

    pub fn has_permissions(permissions: u32, required_permissions: &[Permission]) -> bool {
        required_permissions
            .iter()
            .all(|permission| permissions & permission.as_bitmask() != 0)
    }
}
