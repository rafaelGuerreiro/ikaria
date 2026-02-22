use spacetimedb::{Identity, Timestamp, table};

pub mod types;

#[table(name = user_v1, private)]
pub struct UserV1 {
    #[primary_key]
    pub user_id: Identity,
    #[index(btree)]
    pub username: String,
    pub is_online: bool,
    pub created_at: Timestamp,
    pub last_active_at: Timestamp,
}

#[table(name = character_v1, private)]
pub struct CharacterV1 {
    #[auto_inc]
    #[primary_key]
    pub character_id: u64,
    #[index(btree)]
    pub user_id: Identity,
    #[index(btree)]
    pub name: String,
    pub vocation: String,
    pub level: u16,
    pub experience: u64,
    pub health: u32,
    pub mana: u32,
    pub capacity: u16,
    pub created_at: Timestamp,
    pub last_login_at: Timestamp,
}
