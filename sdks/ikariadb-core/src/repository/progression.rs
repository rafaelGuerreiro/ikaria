use self::types::SkillV1;
use spacetimedb::table;

pub mod types;

#[table(accessor = character_skill_v1, private)]
pub struct CharacterSkillV1 {
    #[auto_inc]
    #[primary_key]
    pub skill_entry_id: u64,
    #[index(btree)]
    pub character_id: u64,
    pub skill: SkillV1,
    pub level: u16,
    pub progress_percent: u8,
}
