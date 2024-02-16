use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};


#[derive(Component, Deref, DerefMut)]
/// Represents a collider used for collision detection. Set the
/// size to the size of the sprite. Must be rectangular. The
/// origin of the sprite is assumed to be in the center.
pub struct Collider {
    pub size: Vec2,
}

use std::ops::DerefMut;
fn check_for_collisions<
Player: Component,
PlayerMovement: DerefMut< Target = Vec2 > + Resource,
>(
    player_controlled_query: Query<(&Transform, Option<&Collider>), With<Player>>,
    colliders: Query<(&Transform, &Collider), Without<Player>>,
    mut player_movement: ResMut<PlayerMovement>,
) {
    // let mut collided = vec![];
    let Ok(
        (&Transform{translation: player_translation,..}, maybe_player_collider)
    ) = player_controlled_query.get_single() else { return;};
    let Some(&Collider {size: player_size}) = maybe_player_collider else {return;};
    let Vec2{x: dx, y: dy} = **player_movement;
    let next_position = player_translation + Vec3::new(dx, dy, 0.);
    colliders
        .iter()
        .map(|(tf, collider)| {
            collide(
                next_position,
                player_size,
                tf.translation,
                collider.size,
            )
        })
        .flatten()
        .for_each(|collision| deal_with_collision(collision, &mut player_movement));

}

fn deal_with_collision
(
    collided: Collision,
    player_movement: &mut Vec2,
) {
    match collided {
        Collision::Right => {
            if player_movement.x < 0. {
                player_movement.x = 0.;
            }
        },
        Collision::Left => {
            if player_movement.x > 0. {
                player_movement.x = 0.;

            }
        },
        Collision::Bottom => {
            if player_movement.y > 0. {
                player_movement.y = 0.;
            }
        },
        Collision::Top => {
            if player_movement.y < 0. {
                player_movement.y = 0.;
            }
        },
        _ => {}

    }
}

fn move_player<
PlayerMovement: DerefMut< Target = Vec2 > + Resource,
Player: Component,
>(
    mut player_controlled_query: Query<&mut Transform, With<Player>>,
    mut player_movement: ResMut<PlayerMovement>,
) {
    let Ok(mut player) = player_controlled_query.get_single_mut() else {return;};
    player.translation += Vec3::new(player_movement.x, player_movement.y, 0.);
    // if !player_movement.has_moved {
    //     player_movement.has_moved = true;
    // }
}

#[derive(Default)]
pub struct CollisionPlugin<
PlayerMovement: DerefMut< Target = Vec2 > + Resource,
Player: Component,
> {
    _marker: std::marker::PhantomData<(PlayerMovement, Player)>,
}

impl<
PlayerMovement: DerefMut< Target = Vec2 > + Resource,
Player: Component,
> CollisionPlugin<PlayerMovement, Player> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<PlayerMovement, Player> Plugin for CollisionPlugin<PlayerMovement, Player>
where
    PlayerMovement: DerefMut< Target = Vec2 > + Resource,
    Player: Component,
{

    fn build(&self, app: &mut App) {
        app
            // .insert_resource(PlayerMovement::default())
            // .insert_resource(Speed::default())
            .add_systems(
                Update,
                (
                    check_for_collisions::<Player, PlayerMovement>,
                    move_player::<PlayerMovement, Player>,
                ).chain()
            )
            ;
    }
}

pub mod prelude {
    pub use super::{CollisionPlugin, Collider};
}