use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_resource::<GameUniqueResource>()
        .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.5,
            -0.5,
            0.5,
        )),
        Cube,
    ));
    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 10.5,
        ..default()
    });
}

#[derive(Component, Reflect)]
pub struct Cube;

#[derive(Resource)]
struct GameUniqueResource {
    name: String,
    age: u32,
    stuff: Vec<String>,
}

impl Default for GameUniqueResource {
    fn default() -> Self {
        Self {
            name: "Game".to_string(),
            age: 15,
            stuff: vec!["stuff".to_string(), "other stuff".to_string()],
        }
    }
}

#[derive(Event)]
struct GameEvent;
