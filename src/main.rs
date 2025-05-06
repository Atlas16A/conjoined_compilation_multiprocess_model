use argh::FromArgs;
use bevy::prelude::*;
use bevy::remote::RemotePlugin;

mod game;
use game::GamePlugin;
mod editor;
use editor::EditorPlugin;

fn main() {
    let args = argh::from_env::<Args>();
    if args.editor_mode {
        println!("Running in editor mode");
        App::new().add_plugins(EditorPlugin).run();
    } else {
        println!("Running in game mode");
        App::new()
            .add_plugins(GamePlugin)
            .add_plugins(RemotePlugin::default())
            .run();
    }
}

#[derive(FromArgs)]
/// Main command line arguments
struct Args {
    #[argh(switch, short = 'e')]
    /// whether to run as the editor or the game
    editor_mode: bool,
}
