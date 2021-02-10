use bevy::prelude::*;

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
        // Run `fn setup` at start
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
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
        // Spawn our lighting
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Mesh used for each square, and color defs for white/black squares
    let mesh: Handle<Mesh> = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_sq: Handle<StandardMaterial> = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_sq: Handle<StandardMaterial> = materials.add(Color::rgb(0., 0.1, 0.1).into());

    for i in 0..8 {
        for j in 0..8 {
            // Create mesh for a square on the board
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: match (i + j) % 2 {
                    0 => white_sq.clone(),
                    _ => black_sq.clone()
                },
                // Squares are of size 1; simply lay them out with loop vars as coords
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    };
}

fn create_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes using the AssetServer.
    // Note: the king and the knight are both separated into 2 meshes.
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Defining colors for white and black pieces.
    let white_material: Handle<StandardMaterial> =
        materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material: Handle<StandardMaterial> =
        materials.add(Color::rgb(0., 0.2, 0.2).into());

    // Creating the pieces from the meshes using the colors defined above.
    // Uses a parent entity with children to eliminate any translation
    // present in the meshes (and combine split meshes); parent tracks
    // actual position, and the child tracks the mesh.
    commands
        // Spawn parent entity (tracks actual position).
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        })
        // Spawn children (combines and holds meshes).
        .with_children(|parent| {
            // King body mesh
            parent.spawn(PbrBundle {
                mesh: king_handle.clone(),
                material: white_material.clone(),
                // Compensate for offset
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            // King cross mesh
            parent.spawn(PbrBundle {
                mesh: king_cross_handle.clone(),
                material: white_material.clone(),
                // Compensate for offset
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}
