use spacetimedb::SpacetimeType;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum DirectionV1 {
    North,
    East,
    #[default]
    South,
    West,
}

impl From<MovementV1> for DirectionV1 {
    fn from(movement: MovementV1) -> Self {
        match movement {
            MovementV1::North | MovementV1::NorthEast => DirectionV1::North,
            MovementV1::East | MovementV1::SouthEast => DirectionV1::East,
            MovementV1::South | MovementV1::SouthWest => DirectionV1::South,
            MovementV1::West | MovementV1::NorthWest => DirectionV1::West,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum MovementV1 {
    North,
    NorthEast,
    East,
    SouthEast,
    #[default]
    South,
    SouthWest,
    West,
    NorthWest,
}

impl MovementV1 {
    pub fn translate(&self, x: u16, y: u16) -> (u16, u16) {
        match self {
            MovementV1::North => (x, y.saturating_sub(1)),
            MovementV1::NorthEast => (x.saturating_add(1), y.saturating_sub(1)),
            MovementV1::East => (x.saturating_add(1), y),
            MovementV1::SouthEast => (x.saturating_add(1), y.saturating_add(1)),
            MovementV1::South => (x, y.saturating_add(1)),
            MovementV1::SouthWest => (x.saturating_sub(1), y.saturating_add(1)),
            MovementV1::West => (x.saturating_sub(1), y),
            MovementV1::NorthWest => (x.saturating_sub(1), y.saturating_sub(1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SpacetimeType)]
pub enum MapTileV1 {
    Water,
    Grass,
}

impl MapTileV1 {
    pub fn is_walkable(&self) -> bool {
        matches!(self, MapTileV1::Grass)
    }
}
