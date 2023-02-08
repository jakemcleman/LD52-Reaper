use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

pub struct PickupPlugin;

#[derive(Debug, Clone, PartialEq)]
pub enum PickupType {
    Soul,
}

#[derive(Component, Clone, Default)]
pub struct Pickup {
    pub pickup_type: Option<PickupType>,
}

#[derive(Component, Default, Clone)]
pub struct PickupCollector;

pub struct PickupEvent {
    pub pickup_entity: Entity,
    pub pickup_type: PickupType,
    pub collector_entity: Entity,
}

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickupEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(check_for_pickups),
        );
    }
}

pub fn check_for_pickups(
    mut pickup_writer: EventWriter<PickupEvent>,
    rapier_context: Res<RapierContext>,
    collector_query: Query<(Entity, &Transform), With<PickupCollector>>,
    pickup_query: Query<&Pickup>,
    mut commands: Commands,
) {
    for (collector_entity, transform) in collector_query.iter() {
        let shape = Collider::capsule_y(5.5, 5.5);
        let filter = QueryFilter::new();
        let shape_pos = transform.translation.truncate();

        rapier_context.intersections_with_shape(shape_pos, 0., &shape, filter, |entity| -> bool {
            if let Ok(pickup) = pickup_query.get(entity) {
                if let Some(pickup_type) = &pickup.pickup_type {
                    pickup_writer.send(PickupEvent {
                        pickup_entity: entity,
                        pickup_type: pickup_type.clone(),
                        collector_entity,
                    });
                }

                commands.entity(entity).despawn_recursive();
            }

            true
        });
    }
}
