use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ChessPlugin)
        .run();
}

/***
 * Introduction
 * - Hello World!
 * - Update Schedule
 */
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

/***
 * MovingCircle
 * - Marker Components
 * - Input Handling
 * - Movement
 */
#[derive(Component)]
struct Player;

pub struct MovingCirclePlugin;

impl Plugin for MovingCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, move_player);
    }
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle { radius: 1.0 })),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec2::new(50.0, 50.0).extend(1.0),
            ..Default::default()
        },
    ));
}

const MOVE_SPEED: f32 = 6.0;
fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }
    }
}

/***
 * Chess
 * - Render Grid
 * - Unit Selection
 * - Win/Lose Condition
 */

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (chess_setup, create_chessboard));
    }
}

fn chess_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7., 20., 4.),
        )),
    ));
    commands.spawn( (
        PointLight { intensity: 3_000_000., ..Default::default() },
        Transform::from_translation(Vec3::new(4.,8.,4.))
    ));
}

fn create_chessboard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {    
    for i in 0..8 {
        for j in 0..8 {
            let mut color = Color::BLACK;

            if (i+j+1) % 2 == 0 {
                color = Color::WHITE;
            }

            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(1., 1.))),
                MeshMaterial3d(materials.add(color)),
                Transform::from_translation(Vec3::new(i as f32, 0., j as f32))
            ));
        }
    }
}
