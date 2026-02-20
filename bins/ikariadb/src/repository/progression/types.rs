use spacetimedb::SpacetimeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum SkillV1 {
    Melee,
    Magic,
    Shield,
    Distance,
}
