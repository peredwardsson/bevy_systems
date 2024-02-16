use bevy::prelude::*;

pub trait SetSpeed {
    fn set_speed(&mut self, speed: f32);
}
// const struct for the keys used for movement
pub enum MOVEMENT {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}


#[derive(Resource)]
pub struct MovementKeys {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}

impl Default for MovementKeys {
    fn default() -> Self {
        Self {
            left: KeyCode::A,
            right: KeyCode::D,
            up: KeyCode::W,
            down: KeyCode::S,
        }
    }
}

/// Add this component to the player entity. This is used to
/// identify the player entity in the query and apply controls
/// to it.
#[derive(Component)]
pub struct PlayerControlled;

#[derive(Resource, Default, Debug, Clone, DerefMut, Deref)]
pub struct PlayerMovement {
    // wished movement of the player. This is not the actual movement
    // but will be used to check for collisions. If there is no collision
    // the player will be moved. we assume a 2d movement for now.
    #[deref]
    pub player_movement: Vec2,
}

impl PlayerMovement {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            player_movement: Vec2::new(x, y),
        }
    }
}

/// The speed of the player in some kind of arbitrary units. If you add
/// this resource to your app, it will override the default speed of 100.
#[derive(Resource)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(100.)
    }

}

fn take_input(
    mut player_movement: ResMut<PlayerMovement>,
    speed: Res<Speed>,
    input: Res<Input<KeyCode>>,
    movement_keys: Res<MovementKeys>,
    time: Res<Time>,
) {
    let x_sign = {
        if input.pressed(movement_keys.left) {
            -1.
        } else if input.pressed(movement_keys.right) {
            1.
        } else {
            0.
        }
    };
    let x_movement = x_sign * speed.0 * time.delta_seconds();
    let y_sign = {
        if input.pressed(movement_keys.up) {
            1.
        } else if input.pressed(movement_keys.down) {
            -1.
        } else {
            0.
        }
    };

    let y_movement = y_sign * speed.0 * time.delta_seconds();

    *player_movement = PlayerMovement {
        player_movement: Vec2::new(
            x_movement,
            y_movement,
        ),
    };
}

pub struct PlayerInputPlugin<
    Player: Component,
> {
    _marker: std::marker::PhantomData<(Player)>,
}

impl <Player>Plugin for PlayerInputPlugin<Player>
where
    Player: Component,
{
    fn build(&self, app: &mut App) {
        app
            .add_systems(First, take_input)
            .insert_resource(PlayerMovement::new(0., 0.))
            .insert_resource(MovementKeys::default())
            ;
    }
}

impl<Player> Default for PlayerInputPlugin<Player>
where
    Player: Component,
{
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}


pub mod prelude {
    pub use super::{
        PlayerControlled,
        PlayerMovement,
        Speed,
        PlayerInputPlugin,
        MovementKeys
    };
}