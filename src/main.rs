use bevy::prelude::*;
use bevy_mod_picking::*;

mod board;
mod pieces;
mod ui;
use board::*;
use pieces::*;
use ui::UIPlugin;

fn main() {
    App::build()
        // set AA to MSAA with 4 samples
        .add_resource(Msaa { samples: 4 })
        // Window 1600x1600
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.system())
        .run()
}

fn setup(commands: &mut Commands) {
    commands
        // Create a 3D camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                    Vec3::new(-7.0, 20.0, 4.0),
                )
            ),
            ..Default::default()
        })
        .with(PickSource::default())
        // Spawn our lighting
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}
