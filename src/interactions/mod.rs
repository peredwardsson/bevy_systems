use bevy::prelude::*;
use bevy::math::prelude::*;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy::utils::tracing;

/// A component which marks an entity as interactable.
#[derive(Component)]
pub struct Interactable;

/// A rectangle which searches for entities with the `Interactable` component.
#[derive(Component)]
pub struct InteractionZone {
    pub size: Vec2, // width and height
    pub origin: Vec2, // top left corner
}

#[derive(Event)]
pub struct InteractionEvent {
    pub entity: Entity,
    pub zone: InteractionZone,
}

impl InteractionZone {
    pub fn new(size: Vec2, origin: Vec2) -> Self {
        Self {
            size,
            origin,
        }
    }

    pub fn update_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }

    pub fn update_size(&mut self, size: Vec2) {
        self.size = size;
    }


    fn intersects_with_zone(&self, other: &InteractionZone) -> bool {
        let self_as_rect = Rect::from_center_size(self.origin, self.size);
        let other_as_rect = Rect::from_center_size(other.origin, other.size);
        !self_as_rect.intersect(other_as_rect).is_empty()
    }
}

impl Default for InteractionZone {
    fn default() -> Self {
        Self {
            size: Vec2::new(32., 32.),
            origin: Vec2::new(0., 0.),
        }
    }
}

fn interact(
    mut interaction_events: EventReader<InteractionEvent>,
    interactables: Query<(&Sprite, &Transform), With<Interactable>>,
) {
    for event in interaction_events.read() {
        screen_print!(sec:1.0, "zone: origin: {:?}, size: {:?}", event.zone.origin, event.zone.size);
        let interactables_count = interactables.iter().count();
        screen_print!(sec:1.0, "Interactables count: {:?}", interactables_count);
        for (sprite, tf) in interactables.iter() {
            let size = sprite.custom_size;
            if let Some(size) = size {
                let position = tf.translation.xy();
                let zone = InteractionZone::new(size, position);
                let intersects = zone.intersects_with_zone(&event.zone);
                let span = tracing::span!(tracing::Level::INFO, "interact", "intersects",);
                let _enter = span.enter();


                if intersects {
                    screen_print!(sec:1.0, "Interactable IS IN ZONE!");
                } else {
                    screen_print!(sec:1.0, "Interactable is not in zone!");
                }
            }
        }
    }
}

fn input_handler<Player: Component>(
    mut interaction_events: EventWriter<InteractionEvent>,
    mut player_query: Query<(Entity, &Transform, &InteractionZone), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::E) {
        screen_print!("Player pressed E!");
        for (entity, transform, zone) in player_query.iter_mut() {
            interaction_events.send(InteractionEvent {
                entity,
                zone: InteractionZone::new(zone.size, transform.translation.xy() + zone.origin),
            });
        }
    }
}

#[derive(Component)]
struct DebugSprite;

fn draw_interaction_zones(
    mut commands: Commands,
    interaction_zone: Query<(&InteractionZone, &Transform)>,
    debug_sprites: Query<(Entity, Option<&DebugSprite>)>,
) {
    let red = Color::rgb(1., 0., 0.);
    for (zone, transform) in interaction_zone.iter() {
        screen_print!("Drawing interaction zone at {:?}", transform.translation.xy());
        for (entity, sprite) in debug_sprites.iter() {
            if sprite.is_some() {
                commands.entity(entity).despawn_recursive();
            }
        }
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(zone.size),
                color: red,
                ..Default::default()
            },
            transform: Transform::from_xyz(
                transform.translation.x + zone.origin.x,
                transform.translation.y + zone.origin.y,
                0.,
            ),
            ..Default::default()
        }, DebugSprite));
    }
}


pub struct InteractionPlugin<Player: Component>{
    _marker: std::marker::PhantomData<Player>,
}

impl<Player> Default for InteractionPlugin<Player>
where Player: Component{
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<Player> Plugin for InteractionPlugin<Player>
where Player: Component{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(OverlayPlugin::default())
            .add_event::<InteractionEvent>()
            .add_systems(
                Update,
                (
                    draw_interaction_zones,
                    interact,
                    input_handler::<Player>,));
    }
}

pub mod prelude {
    pub use super::{InteractionPlugin, Interactable, InteractionZone, InteractionEvent};
}