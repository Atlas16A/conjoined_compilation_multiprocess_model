use bevy::prelude::*;
use bevy::remote::RemotePlugin;

mod game;
use game::GamePlugin;
mod editor;
use editor::{EditorApp, EditorAppExt, EditorPlugin};

/*Standard bevy app main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
*/

fn main() {
    EditorApp::new()
        .add_plugins(GamePlugin)
        .add_editor_plugins(EditorPlugin)
        .add_plugins(RemotePlugin::default())
        .run();
}
