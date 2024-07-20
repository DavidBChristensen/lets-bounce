use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

// TODO(DavidC)
// balls react to impact on ground
// make a sound on impact
// bring in music

const GRAVITY: Vec3 = Vec3::new(0.0, -9.8, 0.0);

fn main() {
    println!("Let's Bounce - start");
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(Startup, (lets_bounce_start, setup))
        .add_systems(Update, (apply_physics, spawn_ball, check_out_of_bounds))
        .run();
    println!("Let's Bounce - end");
}

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

#[derive(Component)]
struct BounceBall;

#[derive(Component)]
struct Velocity {
    vector: Vec3,
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Spawner;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let point_light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };

    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    );

    let ground = (
        PbrBundle {
            mesh: meshes.add(Circle::new(100.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        Ground {},
    );

    let cube = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Spawner {},
    );

    commands.spawn(point_light);
    commands.spawn(camera);
    commands.spawn(ground);
    commands.spawn(cube);
}

fn lets_bounce_start() {
    info!("Let's Bounce - start system.");
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut ball_timer: ResMut<SpawnTimer>,
    spawners: Query<&Transform, With<Spawner>>,
) {
    if !ball_timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    for spawner in &spawners {
        info!("Spawning ball @ position -> {0}", spawner.translation);

        let ball = (
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.5)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: spawner.clone(),
                ..default()
            },
            BounceBall {},
            Velocity {
                vector: Vec3::new(5.0, 20.0, 3.0),
            },
        );

        commands.spawn(ball);
    }
}

fn check_out_of_bounds(
    mut commands: Commands,
    balls: Query<(Entity, &Transform), With<BounceBall>>,
) {
    for (entity, &transform) in balls.iter() {
        if transform.translation.y < -100.0 {
            info!("Despawning a ball, because it went out of range.");
            commands.entity(entity).despawn();
        }
    }
}

fn apply_physics(
    time: Res<Time>,
    mut balls: Query<(&mut Transform, &mut Velocity), With<BounceBall>>,
) {
    for (mut transform, mut velocity) in balls.iter_mut() {
        // apply gravity
        let delta_seconds = time.delta_seconds();
        velocity.vector += GRAVITY * delta_seconds;
        transform.translation += velocity.vector * delta_seconds;
    }
}
