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
        }));
    }
}

/// In this challenge/exploration, please use the bevy remote protocol to implement all the builtin fns of the brp.
/// The builtin fns are:
/// bevy/get
/// bevy/query
/// bevy/spawn
/// bevy/destroy
/// bevy/remove
/// bevy/insert
/// bevy/mutate_component
/// bevy/reparent
/// bevy/list
/// bevy/get+watch
/// bevy/list+watch
/// bevy/get_resource
/// bevy/insert_resource
/// bevy/remove_resource
/// bevy/mutate_resource
/// bevy/list_resources
/// 
/// Using these methods, you should be able to interact with the game app tied with this project.
/// You must be able to interact with both built in components/events/resources and custom components/events/resources.
/// Take as much advantage as possible of the fact the game and editor are the same compiled binary.
/// What traditional safety issues would you have to deal with if the game and editor were separate binaries?
/// What checks do you no longer have to make?
/// What data could you have the editor pull from the game that you would not be able to if they were separate binaries without forcing code changes on the game?
/// Please make sure to document your thought process and the code you write.
/// Please explore these questions as you work. 
/// Good luck!
