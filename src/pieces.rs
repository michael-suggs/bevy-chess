use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}

fn spawn_single_mesh(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    piece_type: PieceType,
    mesh: Handle<Mesh>,
    position: (u8, u8),
    transform: Transform,
) {
    // Creating the pieces from the meshes using the colors defined above.
    // Uses a parent entity with children to eliminate any translation
    // present in the meshes (and combine split meshes); parent tracks
    // actual position, and the child tracks the mesh.
    commands
        // Spawn parent entity (tracks actual position).
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: piece_type,
            x: position.0,
            y: position.1,
        })
        // Spawn children (holds mesh).
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh, material: material.clone(), transform, ..Default::default()
            });
        });
}

fn spawn_dual_mesh(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    piece_type: PieceType,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (u8, u8),
    transform: Transform,
) {
    // Creating the pieces from the meshes using the colors defined above.
    // Uses a parent entity with children to eliminate any translation
    // present in the meshes (and combine split meshes); parent tracks
    // actual position, and the child tracks the mesh.
    commands
        // Spawn parent entity (tracks actual position).
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .with(Piece {
            color: piece_color,
            piece_type: piece_type,
            x: position.0,
            y: position.1,
        })
        // Spawn children (combines and holds meshes).
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_1, material: material.clone(), transform, ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_2, material: material.clone(), transform, ..Default::default()
            });

        });
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_dual_mesh(
        commands, material, color, PieceType::King, mesh, mesh_cross, position, transform
    );
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_dual_mesh(
        commands, material, color, PieceType::Knight, mesh_1, mesh_2, position, transform
    );
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(
        commands, material, color, PieceType::Queen, mesh, position, transform
    );
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(
        commands, material, color, PieceType::Bishop, mesh, position, transform
    );
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(
        commands, material, color, PieceType::Rook, mesh, position, transform
    );
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(
        commands, material, color, PieceType::Pawn, mesh, position, transform
    );
}

pub fn create_pieces(
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

    // White pieces.
    spawn_rook(
        commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 0),
    );
    spawn_knight(
        commands,
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 1),
    );
    spawn_bishop(
        commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 2),
    );
    spawn_queen(
        commands,
        white_material.clone(),
        PieceColor::White,
        queen_handle.clone(),
        (0, 3),
    );
    spawn_king(
        commands,
        white_material.clone(),
        PieceColor::White,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0, 4),
    );
    spawn_bishop(
        commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 5),
    );
    spawn_knight(
        commands,
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 6),
    );
    spawn_rook(
        commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 7),
    );
    for i in 0..8 {
        spawn_pawn(
            commands,
            white_material.clone(),
            PieceColor::White,
            pawn_handle.clone(),
            (1, i),
        );
    }

    // Black pieces.
    spawn_rook(
        commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 0),
    );
    spawn_knight(
        commands,
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 1),
    );
    spawn_bishop(
        commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 2),
    );
    spawn_queen(
        commands,
        black_material.clone(),
        PieceColor::Black,
        queen_handle.clone(),
        (7, 3),
    );
    spawn_king(
        commands,
        black_material.clone(),
        PieceColor::Black,
        king_handle.clone(),
        king_cross_handle.clone(),
        (7, 4),
    );
    spawn_bishop(
        commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 5),
    );
    spawn_knight(
        commands,
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 6),
    );
    spawn_rook(
        commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 7),
    );
    for i in 0..8 {
        spawn_pawn(
            commands,
            black_material.clone(),
            PieceColor::Black,
            pawn_handle.clone(),
            (6, i),
        );
    }
}
