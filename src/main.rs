use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Player {
    move_up: KeyCode,
    move_down: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0., 0., 0.),
                custom_size: Some(Vec2::new(640.0, 640.0)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        }
    );
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0), // This creates a blue color
                custom_size: Some(Vec2::new(40.0, 40.0)), // Size of the square
                ..default()
            },
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        }, 
        Player {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            move_left: KeyCode::ArrowLeft,
            move_right: KeyCode::ArrowRight
        }
    ));
}

fn sprite_movement(
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut player = player.single_mut();

    if input.pressed(KeyCode::ArrowUp) {
        player.translation.y += 2.;
    };
    if input.pressed(KeyCode::ArrowDown) {
        player.translation.y -= 2.;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        player.translation.x -= 2.;
    }
    if input.pressed(KeyCode::ArrowRight) {
        player.translation.x += 2.;
    }
    
}
























//fn keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
//    if keys.just_pressed(KeyCode::ArrowLeft) {
//
//    }
//}
//
//fn sprite_movement(time: Res<Time>, mut sprite_playerition: Query<(&mut Direction, &mut Transform)>) {
//    //for (mut logo, mut transform) in &mut sprite_playerition {
//    //    match *logo {
//    //        Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
//    //        Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
//    //    }
//
//    //    if transform.translation.y > 200. {
//    //        *logo = Direction::Down;
//    //    } else if transform.translation.y < -200. {
//    //        *logo = Direction::Up;
//    //    }
//    //}
//}
