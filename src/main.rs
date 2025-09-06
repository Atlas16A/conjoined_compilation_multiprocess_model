use argh::FromArgs;
use bevy::prelude::*;
use bevy::remote::RemotePlugin;

mod game;
use game::GamePlugin;
mod editor;
use editor::EditorPlugin;

fn main() {
    let args = argh::from_env::<Args>();
    let mut app = App::new();
    if args.editor_mode {
        println!("Running in editor mode");
        app.add_plugins(EditorPlugin);
    } else {
        println!("Running in game mode");
        app.add_plugins(GamePlugin)
            .add_plugins(RemotePlugin::default());
    }
    app.run();
}

#[derive(FromArgs)]
/// Main command line arguments
struct Args {
    #[argh(switch, short = 'e')]
    /// whether to run as the editor or the game
    editor_mode: bool,
}
