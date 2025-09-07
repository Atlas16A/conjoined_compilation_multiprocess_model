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

use argh::FromArgs;
use bevy::{
    app::{Plugins, PluginsState},
    prelude::*,
};

#[derive(FromArgs)]
/// Main command line arguments
struct Args {
    #[argh(switch, short = 'e')]
    /// whether to run as the editor or the game
    editor_mode: bool,
}

pub struct EditorApp(App);

pub trait EditorAppExt {
    fn new() -> Self;

    fn add_editor_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self;

    fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self;

    fn run(&mut self) -> AppExit;
}

impl EditorAppExt for EditorApp {
    fn new() -> Self {
        Self(App::new())
    }

    fn add_editor_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        let args = argh::from_env::<Args>();
        if !args.editor_mode {
            return self;
        }
        if matches!(
            self.0.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }

        plugins.add_to_app(&mut self.0);
        self
    }

    fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        let args = argh::from_env::<Args>();
        if args.editor_mode {
            return self;
        }
        if matches!(
            self.0.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }

        plugins.add_to_app(&mut self.0);
        self
    }

    fn run(&mut self) -> AppExit {
        self.0.run()
    }
}
