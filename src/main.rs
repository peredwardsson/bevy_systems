use bevy::prelude::*;

mod collisions;
mod interactions;
mod player_input;

use collisions::prelude::*;
// imports Collider and CollisionPlugin
use interactions::prelude::*;

use player_input::prelude::*;
// imports PlayerControlled, Speed, PlayerMovement and PlayerInputPlugin

const PLAYER_SIZE: Vec2 = Vec2::new(50., 100.);

fn add_sphere(
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-250., 0., 0.)),
            ..default()
        },
        Collider {
            size: PLAYER_SIZE,
        },
        PlayerControlled,
        InteractionZone::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(250., 0., 0.)),
            ..default()
        }, Collider {
            size: Vec2::new(50., 100.),
        },
        Interactable
    ));

    // add a third rectangle which intersects with the last one creating
    // a reverse L shape
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.25),
                custom_size: Some(Vec2::new(100.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 50., 0.)),
            ..default()
        }, Collider {
            size: Vec2::new(100., 50.),
        },
        Interactable));
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        }
    );
}

fn update_interaction_zone_based_on_input
(
    input: Res<Input<KeyCode>>,
    mvmt_keys: Res<MovementKeys>,
    mut player_interaction_query: Query<(&mut InteractionZone), With<PlayerControlled>>,
) {
    // if the player moves upwards, place the zone just above the player.
    for mut zone in player_interaction_query.iter_mut() {
        // zone.origin = Vec2::new(0., offset);
        if input.just_pressed(mvmt_keys.up) {
            let offset = (PLAYER_SIZE.y + zone.size.y)/2.;
            zone.origin = Vec2::new(0., offset);
        } else if input.just_pressed(mvmt_keys.down) {
            let offset = (PLAYER_SIZE.y + zone.size.y)/2.;
            zone.origin = Vec2::new(0., -offset);
        } else if input.just_pressed(mvmt_keys.left) {
            let offset = (PLAYER_SIZE.x + zone.size.x)/2.;
            zone.origin = Vec2::new(-offset, 0.);
        } else if input.just_pressed(mvmt_keys.right) {
            let offset = (PLAYER_SIZE.x + zone.size.x)/2.;
            zone.origin = Vec2::new(offset, 0.);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CollisionPlugin::<PlayerMovement, PlayerControlled>::new())
        .add_plugins(InteractionPlugin::<PlayerControlled>::default())
        .add_plugins(PlayerInputPlugin::<PlayerControlled>::default())
        .add_systems(
            Startup,
            (add_sphere, setup)
        )
        .add_systems(
            Update,
            (
                update_interaction_zone_based_on_input,
            )
        )
        .insert_resource(Speed(300.))
        .run();
}
