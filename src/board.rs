use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}

// A square on the board (at position (x,y)).
struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Mesh used for each square, and color defs for white/black squares
    let mesh: Handle<Mesh> = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    for i in 0..8 {
        for j in 0..8 {
            // Create mesh for a square on the board
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: match (i + j) % 2 {
                        0 => materials.add(Color::rgb(1., 0.9, 0.9).into()),
                        _ => materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    // Squares are of size 1; simply lay them out with loop vars as coords
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j, });
        }
    };
}

// Track which square is currently selected.
#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

// Changes colors of selected squares.
fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>
) {
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        let material = materials.get_mut(material_handle).unwrap();

        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    selected_square.entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };
}
