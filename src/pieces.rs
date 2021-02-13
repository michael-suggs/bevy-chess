use bevy::prelude::*;
use crate::board::Square;

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system())
            .add_system(move_pieces.system());
    }
}

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

impl PartialEq<Square> for Piece {
    fn eq(&self, other: &Square) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Piece {
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        if square_color(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y == new_position.1))
                // Vertical
                || ((self.y as i8 - new_position.1 as i8).abs() == 1
                    && (self.x == new_position.0))
                // Diagonal
                || ((self.x as i8 - new_position.0 as i8).abs() == 1
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
                        || ((self.x == new_position.0 && self.y != new_position.1)
                            || (self.y == new_position.1 && self.x != new_position.0)))
            }
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && (self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
            }
            PieceType::Knight => {
                ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == 1 && (self.y == new_position.1) {
                        if square_color(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 squares
                    if self.x == 1
                        && new_position.0 as i8 - self.x as i8 == 2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if square_color(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if square_color(new_position, &pieces) == Some(PieceColor::Black) {
                            return true;
                        }
                    }
                } else {
                    // Normal move
                    if new_position.0 as i8 - self.x as i8 == -1 && (self.y == new_position.1) {
                        if square_color(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Move 2 squares
                    if self.x == 6
                        && new_position.0 as i8 - self.x as i8 == -2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                    {
                        if square_color(new_position, &pieces).is_none() {
                            return true;
                        }
                    }

                    // Take piece
                    if new_position.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                    {
                        if square_color(new_position, &pieces) == Some(PieceColor::White) {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn square_color(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color)
        }
    }
    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    // Check in the same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0 && (
                (piece.y > begin.1 && piece.y < end.1)
                || (piece.y > end.1 && piece.y < begin.1)
            ) {
                return false;
            }
        }
    }
    // Check in the same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1 && (
                (piece.x > begin.0 && piece.x < end.0)
                || (piece.x > end.0 && piece.x < begin.0)
            ) {
                return false;
            }
        }
    }
    // Check diagonals
    let dx: i8 = (begin.0 as i8 - end.0 as i8).abs();
    let dy: i8 = (begin.1 as i8 - end.1 as i8).abs();

    if dx == dy {
        for i in 1..dx {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if square_color(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
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
