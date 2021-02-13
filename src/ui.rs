use crate::{board::*, pieces::*};
use bevy::prelude::*;

struct NextMoveText;

fn init_next_move_text(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
}
