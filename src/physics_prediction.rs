use crate::{
    physics::{get_gravity_acceleration, GravitySource, Mass, Velocity},
    ship::Ship,
};
use bevy::{prelude::*, sprite::Mesh2dHandle};

pub struct PhysicsPredictionPlugin;

impl Plugin for PhysicsPredictionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics_prediction)
            .add_systems(Update, physics_prediction_system);
    }
}

#[derive(Component)]
struct PhysicsPrediction;

#[derive(Bundle)]
struct PhysicsPredictionBundle {
    physics_prediction: PhysicsPrediction,
    mesh: ColorMesh2dBundle,
}

fn setup_physics_prediction(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PhysicsPredictionBundle {
        physics_prediction: PhysicsPrediction,
        mesh: ColorMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(1.0))).into(),
            material: materials.add(Color::rgb(0.5, 0.1, 0.1).into()),
            ..default()
        },
    });
}

fn physics_prediction_system(
    mut mesh_query: Query<(&Mesh2dHandle, &mut Transform), With<PhysicsPrediction>>,
    ship_query: Query<(&Transform, &Velocity), (With<Ship>, Without<PhysicsPrediction>)>,
    affectors: Query<(&Transform, &Mass), (With<GravitySource>, Without<PhysicsPrediction>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (ship_tr, ship_vel) = ship_query.single();
    let path = generate_path(ship_tr.translation.truncate(), ship_vel.0, &affectors);

    let (mesh_handle, mut transform) = mesh_query.single_mut();
    let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
    *mesh = generate_mesh_from_path(&path);
    transform.translation = path[0].extend(0.0);
}

fn generate_path<'a>(
    ship_pos: Vec2,
    ship_vel: Vec2,
    affectors: impl IntoIterator<Item = (&'a Transform, &'a Mass)> + Copy,
) -> Vec<Vec2> {
    let mut path = Vec::new();
    let mut pos = ship_pos;
    let mut vel = ship_vel;
    let mut distance_travelled = 0.0;

    for _ in 0..1000 {
        path.push(pos);
        generate_next_path_point(&mut pos, &mut vel, &mut distance_travelled, affectors);
    }

    path
}

fn generate_next_path_point<'a>(
    pos: &mut Vec2,
    vel: &mut Vec2,
    distance_travelled: &mut f32,
    affectors: impl IntoIterator<Item = (&'a Transform, &'a Mass)> + Copy,
) {
    const DELTA: f32 = 1.0 / 60.0;
    const STEPS: i32 = 10;

    for _ in 0..STEPS {
        // Update the velocity
        for (affector_transform, affector_mass) in affectors {
            let relative_position = affector_transform.translation.truncate() - *pos;
            if let Ok(change_in_vel) =
                get_gravity_acceleration(relative_position, affector_mass.0)
            {
                *vel += change_in_vel * DELTA;
            }
        }

        // Update the position
        *pos += *vel * DELTA;
        *distance_travelled += vel.length() * DELTA;
    }
}

fn generate_mesh_from_path(path: &Vec<Vec2>) -> Mesh {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineStrip);
    let mut vertices = Vec::new();
    for pos in path {
        let pos_relative_to_first = *pos - path[0];
        vertices.push(pos_relative_to_first.extend(0.0));
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh
}
