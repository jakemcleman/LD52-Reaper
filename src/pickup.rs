use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum PickupType {
    Soul,
}

#[derive(Component, Clone)]
struct Pickup {
    pickup_type: Option<PickupType>,
}