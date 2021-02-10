use bevy::prelude::*;

fn main() {
    App::build()
        // set AA to MSAA with 4 samples
        .add_resource(Msaa { samples: 4 })
        // Window 1600x1600
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run()
}
