use bevy::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Editor".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup);
    }
}

fn setup(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();
    registry.iter().for_each(|ty| {
        let typeinfo = ty.type_info().ty().path();
        if typeinfo.contains("Cube") {
            println!("Type: {typeinfo:?}");
        }
    });
}
