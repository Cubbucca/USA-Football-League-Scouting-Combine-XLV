use crate::{AppState, game_state, collision, assets::GameAssets, player::Player, ingame,
LEFT_END, RIGHT_END, LEFT_GOAL, RIGHT_GOAL};
use bevy::prelude::*;
use bevy::gltf::Gltf;
use std::f32::consts::{FRAC_PI_2};

pub struct FootballPlugin;
impl Plugin for FootballPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchFootballEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(check_for_football_pickup)
                .with_system(handle_launch_football_event)
                .with_system(move_football),
        );
    }
}

#[derive(Component)]
pub struct Football {
    has_landed: bool,
    target: Vec3,
    starting_position: Vec3,
    current_movement_time: f32,
}
#[derive(Component)]
pub struct CarriedFootball;
pub struct LaunchFootballEvent;

fn handle_launch_football_event(
    mut commands: Commands,
    mut launch_football_event_reader: EventReader<LaunchFootballEvent>,
    game_assets: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    game_state: Res<game_state::GameState>,
) {
    for event in launch_football_event_reader.iter() {
        println!("launch event read");
        if let Some(gltf) = assets_gltf.get(&game_assets.football.clone()) {
            let left_side = Vec3::new(0.0, 0.0, ((LEFT_GOAL - LEFT_END) / 2.0) + LEFT_END);
            let right_side = Vec3::new(0.0, 0.0, ((RIGHT_GOAL - RIGHT_END) / 2.0) + RIGHT_END);

            let position = if game_state.touchdown_on_leftside { right_side } else { left_side };
            let target = Vec3::new(0.0, 0.0, 0.0);
            commands.spawn_bundle(SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: {
                            let mut t = Transform::from_scale(Vec3::splat(3.0));
                            t.translation = position;
                            t
                        },
                        ..default()
                    })
                    .insert(Football {
                        has_landed: false,
                        target: target,
                        starting_position: position,
                        current_movement_time: 0.0,
                    })
                    .insert(ingame::CleanupMarker);
        }
    }
}

const FOOTBALL_PICKUP_DISTANCE: f32 = 0.7;
fn check_for_football_pickup(
    mut commands: Commands,
    footballs: Query<(Entity, &Football, &Transform)>,
    mut player: Query<(Entity, &mut Player, &Transform)>,
    mut carried_footballs: Query<(&CarriedFootball, &mut Visibility, &Parent)>,
) {
    for (football_entity, football, football_transform) in &footballs {
        let (player_entity, mut player, player_transform) = player.single_mut();

        if football_transform.translation.distance(player_transform.translation) < FOOTBALL_PICKUP_DISTANCE {
            player.has_football = true;

            for (_, mut visibility, parent) in &mut carried_footballs {
                if player_entity == parent.get() {
                    visibility.is_visible = true;
                }
            }

            commands.entity(football_entity).despawn_recursive();
        }
    }
}

fn move_football(
    mut footballs: Query<(&mut Football, &mut Transform)>,
    time: Res<Time>,
) {
    let flight_time = 2.0;
    let flight_height = 20.0;

    for (mut football, mut transform) in &mut footballs {
        if !football.has_landed {
            let (target_with_height, start_with_height) 
                = if football.current_movement_time / flight_time <= 0.5 {
                     (Vec3::new(football.target.x, flight_height, football.target.z),
                     football.starting_position)
                  } else {
                     (football.target,
                     (Vec3::new(football.starting_position.x, flight_height, football.starting_position.z)))
                  };
            transform.translation = 
                start_with_height.lerp(target_with_height, football.current_movement_time / flight_time);
            transform.rotate_x(time.delta_seconds());
            transform.rotate_y(time.delta_seconds() / 2.0);
            transform.rotate_z(time.delta_seconds() / 3.0);
            football.current_movement_time += time.delta_seconds();

            if football.current_movement_time >= flight_time {
                football.current_movement_time = 0.0;
                football.has_landed = true;
            }
        }
    }
}