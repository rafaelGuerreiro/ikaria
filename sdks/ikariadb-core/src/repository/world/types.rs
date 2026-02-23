use spacetimedb::SpacetimeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum DirectionV1 {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum MapTileV1 {
    Water,
    Grass,
}
