use bevy::prelude::*;
use hexx::Hex;

use crate::arena::Arena;

const HIT_PLANE_HEIGHT: f32 = 0.5;
const HIT_PLANE_SIZE: f32 = 1000.0;

pub struct PointerTrackingPlugin;

impl Plugin for PointerTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointerPosition>()
            .add_event::<PointerMovedEvent>()
            .add_event::<PointerChangedHexEvent>()
            .add_systems(Startup, setup_hit_plane);
    }
}

#[derive(Resource, Default)]
pub struct PointerPosition {
    pub pos: Vec3,
    pub hex: Hex,
}

#[derive(Event)]
pub struct PointerMovedEvent {
    pub old_pos: Vec3,
    pub new_pos: Vec3,
}

#[derive(Event)]
pub struct PointerChangedHexEvent {
    pub old_hex: Hex,
    pub new_hex: Hex,
}

/// Spawns a plane as the only mesh picker enabled in the game.
/// The pointer hit position is tracked with `PointerPosition`.
fn setup_hit_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn((
            Visibility::Hidden,
            Pickable {
                is_hoverable: true,
                should_block_lower: true,
            },
            Mesh3d(meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::new(HIT_PLANE_SIZE, HIT_PLANE_SIZE),
            ))),
            Transform::from_xyz(0.0, HIT_PLANE_HEIGHT, 0.0),
        ))
        .observe(pointer_move_observer);
}

fn pointer_move_observer(
    trigger: Trigger<Pointer<Move>>,
    arena: Res<Arena>,
    mut pointer_pos: ResMut<PointerPosition>,
    mut evw_pointer_moved: EventWriter<PointerMovedEvent>,
    mut evw_pointer_changed_hex: EventWriter<PointerChangedHexEvent>,
) {
    if let Some(pos) = trigger.hit.position {
        evw_pointer_moved.write(PointerMovedEvent {
            old_pos: pointer_pos.pos,
            new_pos: pos,
        });
        pointer_pos.pos = pos;

        // Check if the hex has changed
        let hex = arena.layout.world_pos_to_hex(pos.xz());
        if hex != pointer_pos.hex {
            evw_pointer_changed_hex.write(PointerChangedHexEvent {
                old_hex: pointer_pos.hex,
                new_hex: hex,
            });
            pointer_pos.hex = hex;
        }
    }
}
