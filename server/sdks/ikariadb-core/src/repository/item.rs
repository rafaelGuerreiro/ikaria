use spacetimedb::table;

pub mod types;

#[table(accessor = item_definition_v1, private)]
pub struct ItemDefinitionV1 {
    #[auto_inc]
    #[primary_key]
    pub item_id: u64,
    pub name: String,
    pub kind: String,
    pub stackable: bool,
    pub weight: u32,
    pub base_value: u32,
}
