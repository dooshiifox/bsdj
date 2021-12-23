use bevy::{prelude::*, window::WindowMode};
// use bevy_inspector_egui::WorldInspectorPlugin;

mod events;
mod resources;
mod scenes;
mod states;
mod tilerender;
mod utils;

const SCALE: f32 = 1.;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

/// Create the app and open the program.
fn main() {
    let mut app = App::new();

    // Set the properties of the window itself
    app.insert_resource(WindowDescriptor {
        title: "LSDj".to_string(),
        width: 160. * SCALE,
        height: 144. * SCALE,
        vsync: true,
        // resizable: false,
        mode: WindowMode::Windowed,
        ..Default::default()
    });

    app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    // region:      ADD THE PLUGINS
    app.add_plugins(DefaultPlugins);
    // app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(events::EventsPlugin);
    app.add_plugin(resources::ResourcePlugin);
    app.add_plugin(scenes::ScenePlugin);
    app.add_plugin(tilerender::TileRenderPlugin);
    // endregion:   ADD THE PLUGINS

    // Add the setup for the app.
    app.add_startup_system(setup);

    // Set the current game state to be the song screen.
    app.add_state(states::States::Song);

    // RUN THE PROGRAM
    app.run();
}

fn setup(mut commands: Commands) {
    // Add a camera to the scene.
    // commands.spawn().insert_bundle(CameraBundle {
    //     camera: Camera {
    //         // Set our camera to have a fixed height and width
    //         size: CameraSize::LetterBoxed {
    //             width: 160,
    //             height: 144,
    //         },
    //         centered: false,
    //         background_color: Color::Rgba(0.2, 0.2, 0.2, 1.),
    //         letterbox_color: Color::Rgba(0., 0., 0., 1.),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
