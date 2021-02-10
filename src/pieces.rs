use bevy::prelude::*;

fn spawn_single_mesh(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
    transform: Transform,
) {
    // Creating the pieces from the meshes using the colors defined above.
    // Uses a parent entity with children to eliminate any translation
    // present in the meshes (and combine split meshes); parent tracks
    // actual position, and the child tracks the mesh.
    commands
        // Spawn parent entity (tracks actual position).
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3,
    transform: Transform,
) {
    // Creating the pieces from the meshes using the colors defined above.
    // Uses a parent entity with children to eliminate any translation
    // present in the meshes (and combine split meshes); parent tracks
    // actual position, and the child tracks the mesh.
    commands
        // Spawn parent entity (tracks actual position).
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
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

pub fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_dual_mesh(commands, material, mesh, mesh_cross, position, transform)
    // commands
    //     // Spawn parent entity (tracks actual position).
    //     .spawn(PbrBundle {
    //         transform: Transform::from_translation(position),
    //         ..Default::default()
    //     })
    //     // Spawn children (combines and holds meshes).
    //     .with_children(|parent| {
    //         // King body mesh
    //         parent.spawn(PbrBundle {
    //             mesh: mesh,
    //             material: material.clone(),
    //             // Compensate for offset
    //             transform: {
    //                 let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
    //                 transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    //                 transform
    //             },
    //             ..Default::default()
    //         });
    //         // King cross mesh
    //         parent.spawn(PbrBundle {
    //             mesh: mesh_cross,
    //             material: material.clone(),
    //             // Compensate for offset
    //             transform: {
    //                 let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
    //                 transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    //                 transform
    //             },
    //             ..Default::default()
    //         });
    //     });
}

pub fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_dual_mesh(commands, material, mesh_1, mesh_2, position, transform);
    // commands
    //     // Spawn parent entity (tracks actual position).
    //     .spawn(PbrBundle {
    //         transform: Transform::from_translation(position),
    //         ..Default::default()
    //     })
    //     // Spawn children (combines and holds meshes).
    //     .with_children(|parent| {
    //         // King body mesh
    //         parent.spawn(PbrBundle {
    //             mesh: mesh_1,
    //             material: material.clone(),
    //             // Compensate for offset
    //             transform: {
    //                 let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
    //                 transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    //                 transform
    //             },
    //             ..Default::default()
    //         });
    //         // King cross mesh
    //         parent.spawn(PbrBundle {
    //             mesh: mesh_2,
    //             material: material.clone(),
    //             // Compensate for offset
    //             transform: {
    //                 let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
    //                 transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    //                 transform
    //             },
    //             ..Default::default()
    //         });
    //     });
}

pub fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(commands, material, mesh, position, transform);
}

pub fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(commands, material, mesh, position, transform);
}

pub fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(commands, material, mesh, position, transform);
}

pub fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
    spawn_single_mesh(commands, material, mesh, position, transform);
}
