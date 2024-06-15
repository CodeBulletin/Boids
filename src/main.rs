use bevy::math::vec2;
use bevy::{prelude::*, transform};
use bevy::window::{WindowMode, WindowResolution};
use iyes_perf_ui::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rand::Rng;
use bevy::window::PrimaryWindow;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct FishCount {
    #[inspector(min = 200, max = 1000)]
    count: i32,
}

impl Default for FishCount {
    fn default() -> Self {
        FishCount { count: 200 }
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Physics {
    #[inspector(min = 0.0, max = 100.0)]
    align_radius: f32,
    #[inspector(min = 0.0, max = 100.0)]
    cohesion_radius: f32,
    #[inspector(min = 0.0, max = 100.0)]
    separation_radius: f32,

    #[inspector(min = 0.0, max = 10.0)]
    align_factor: f32,
    #[inspector(min = 0.0, max = 10.0)]
    cohesion_factor: f32,
    #[inspector(min = 0.0, max = 10.0)]
    separation_factor: f32,

    #[inspector(min = 0.0, max = 10.0)]
    velocity_mag: f32,
    #[inspector(min = 0.0, max = 1.0)]
    max_force: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Physics { 
            align_radius: 50.0,
            cohesion_radius: 75.0,
            separation_radius: 25.0,

            align_factor: 0.1,
            cohesion_factor: 0.1,
            separation_factor: 0.8,

            velocity_mag: 1.0,
            max_force: 0.1,
        }
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Simulation {
    #[inspector(min = 0.0, max = 100.0)]
    velocity_multiplier: f32,
    #[inspector(min = 0.0, max = 100.0)]
    acceleration_multiplier: f32,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation {
            velocity_multiplier: 60.0,
            acceleration_multiplier: 60.0,
        }
    }
}

#[derive(Resource)]
struct Bounds {
    a: Vec2,
    b: Vec2,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            a: Vec2::new(-500.0, -500.0),
            b: Vec2::new(500.0, 500.0),
        }
    }
}


#[derive(Component)]
struct Fish {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub force: Vec2,
}

#[derive(Component)]
struct FishData {
    pub f_sep: Vec2,
    pub f_sep_count: i32,
    pub f_avoid: Vec2,
    pub f_avoid_count: i32,
    pub f_align: Vec2,
    pub f_align_count: i32,
    pub f_cohesion: Vec2,
    pub f_cohesion_count: i32,
}

impl Default for FishData {
    fn default() -> Self {
        FishData {
            f_sep: Vec2::new(0.0, 0.0),
            f_avoid: Vec2::new(0.0, 0.0),
            f_align: Vec2::new(0.0, 0.0),
            f_cohesion: Vec2::new(0.0, 0.0),
            f_sep_count: 0,
            f_avoid_count: 0,
            f_align_count: 0,
            f_cohesion_count: 0,
        }
    }
}

#[derive(Component)]
struct Obstacle {
    pub position: Vec2,
}

fn set_bounds (mut bounds: ResMut<Bounds>, window: Query<&Window>) {
    let window = window.single();
    bounds.a = Vec2::new(-window.width() / 2.0, -window.height() / 2.0);
    bounds.b = Vec2::new(window.width() / 2.0, window.height() / 2.0);
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, fish_count: Res<FishCount>, bounds: Res<Bounds>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        }, ..default()
    });

    commands.spawn((
        PerfUiRoot::default(),
        (
            PerfUiEntryFPS::default(),
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFrameTime::default(),
            PerfUiEntryFrameTimeWorst::default(),
            PerfUiEntryEntityCount::default(),
        )
    ));

    let texture_handle = assets.load("Fish1.png");

    let mut rng = rand::thread_rng();

    for i in 0..fish_count.count {
        let x_pos: f32 = rng.gen::<f32>() * (bounds.b.x - bounds.a.x) + bounds.a.x;
        let y_pos: f32 = rng.gen::<f32>() * (bounds.b.y - bounds.a.y) + bounds.a.y;

        let x_vel: f32 = rng.gen::<f32>() * 8.0 - 4.0;
        let y_vel: f32 = rng.gen::<f32>() * 8.0 - 4.0;

        commands.spawn((
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, y_pos, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new(format!("Fish {}", i)),
            Fish {
                velocity: Vec2::new(x_vel, y_vel),
                acceleration: Vec2::new(0.0, 0.0),
                force: Vec2::new(0.0, 0.0),
            },
            FishData::default()
        ));
    }

}

fn on_click_spawn_obstical(mut commands: Commands, mouse_button_input: Res<ButtonInput<MouseButton>>, bounds: Res<Bounds>, q_windows: Query<&Window, With<PrimaryWindow>>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let position = Vec2::new(position.x + bounds.a.x, -(position.y + bounds.a.y));
            println!("Mouse clicked at: {:?}", position);
            if position.x > bounds.a.x && position.x < bounds.b.x && position.y > bounds.a.y && position.y < bounds.b.y {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(position.x, position.y, 0.0),
                            scale: Vec3::new(10.0, 10.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Obstacle {
                        position: Vec2::new(position.x, position.y),
                    }
                ));
            }
        }
        else {
            println!("Mouse out of bounds");
        }
    }
}

fn on_fish_count_change(mut commands: Commands, fish_count: Res<FishCount>, query: Query<(Entity, &Fish), With<Fish>>, assets: Res<AssetServer>, bounds: Res<Bounds>) {
    let fish_count = fish_count.count;
    let current_fish_count = query.iter().count() as i32;

    
    if current_fish_count < fish_count {
        let texture_handle = assets.load("Fish1.png");    
        let mut rng = rand::thread_rng();
        for i in current_fish_count..fish_count {
            let x_pos: f32 = rng.gen::<f32>() * (bounds.b.x - bounds.a.x) + bounds.a.x;
            let y_pos: f32 = rng.gen::<f32>() * (bounds.b.y - bounds.a.y) + bounds.a.y;

            let x_vel: f32 = rng.gen::<f32>() * 8.0 - 4.0;
            let y_vel: f32 = rng.gen::<f32>() * 8.0 - 4.0;

            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x_pos, y_pos, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Name::new(format!("Fish {}", i)),
                Fish {
                    velocity: Vec2::new(x_vel, y_vel),
                    acceleration: Vec2::new(0.0, 0.0),
                    force: Vec2::new(0.0, 0.0),
                },
                FishData::default()
            ));
        }
    } else if current_fish_count > fish_count {
        for (id, entity) in query.iter().enumerate() {
            if id as i32 >= fish_count {
                commands.entity(entity.0).despawn();
            }
        }
    }
}

fn fish_calculate_force(mut query: Query<(Entity, &Fish, &mut FishData, &Transform), With<Fish>>, game_physics: Res<Physics>) {
    let mut combinations = query.iter_combinations_mut();


    while let Some([mut f1, mut f2]) = combinations.fetch_next() {
        let velocity_f1 = Vec2::new(f1.1.velocity.x, f1.1.velocity.y);
        let velocity_f2 = Vec2::new(f2.1.velocity.x, f2.1.velocity.y);

        let position_f1 = Vec2::new(f1.3.translation.x, f1.3.translation.y);
        let position_f2 = Vec2::new(f2.3.translation.x, f2.3.translation.y);
        let distance = position_f1.distance(position_f2);

        if distance == 0.0 {
            continue;
        }

        // Separation
        if distance < game_physics.separation_radius {
            let mut separation = position_f1 - position_f2;
            separation /= distance;
            f1.2.f_sep += separation;
            f1.2.f_sep_count += 1;
            f2.2.f_sep -= separation;
            f2.2.f_sep_count += 1;
        }

        // Alignment
        if distance < game_physics.align_radius {
            f1.2.f_align += velocity_f2;
            f1.2.f_align_count += 1;
            f2.2.f_align += velocity_f1;
            f2.2.f_align_count += 1;
        }

        // Cohesion
        if distance < game_physics.cohesion_radius {
            f1.2.f_cohesion += position_f2;
            f1.2.f_cohesion_count += 1;
            f2.2.f_cohesion += position_f1;
            f2.2.f_cohesion_count += 1;
        }
    }
}

fn fish_avoid_obstacles(mut query: Query<(&Fish, &mut FishData, &Transform), With<Fish>>, obstacles: Query<&Obstacle>) {
    for (fish, mut fish_data, transform) in query.iter_mut() {
        for obstacle in obstacles.iter() {
            let position_fish = Vec2::new(transform.translation.x, transform.translation.y);
            let position_obstacle = obstacle.position;
            let distance = position_fish.distance(position_obstacle);

            if distance == 0.0 {
                continue;
            }

            if distance < 50.0 {
                let separation = position_fish - position_obstacle;
                let separation = separation / distance;
                fish_data.f_avoid += separation;
                fish_data.f_avoid_count += 1;
            }
        }
    }
}

fn fish_normalize_force(mut query: Query<(&mut Fish, &mut FishData, &Transform), With<Fish>>, game_physics: Res<Physics>) {
    for (mut fish, mut fish_data, transform) in query.iter_mut() {
        if fish_data.f_sep_count > 0 {
            let avg_sepration = fish_data.f_sep / fish_data.f_sep_count as f32;
            let steering = (avg_sepration - fish.velocity).normalize();
            fish_data.f_sep = steering * game_physics.separation_factor;
        }

        if fish_data.f_align_count > 0 {
            let avg_velocity = fish_data.f_align / fish_data.f_align_count as f32;
            let steering = (avg_velocity - fish.velocity).normalize();
            fish_data.f_align = steering * game_physics.align_factor;
        }

        if fish_data.f_cohesion_count > 0 {
            let avg_location = fish_data.f_cohesion / fish_data.f_cohesion_count as f32;
            let desired = avg_location - Vec2::new(transform.translation.x, transform.translation.y);
            let steering = (desired - fish.velocity).normalize();
            fish_data.f_cohesion = steering * game_physics.cohesion_factor;
        }

        if fish_data.f_avoid_count > 0 {
            let avg_avoid = fish_data.f_avoid / fish_data.f_avoid_count as f32;
            let steering = (avg_avoid - fish.velocity).normalize();
            fish_data.f_avoid = steering * 10.0;
        }

        fish.force = fish_data.f_sep + fish_data.f_cohesion + fish_data.f_align + fish_data.f_avoid;
        if fish.force.length() > 0.0 {
            fish.force = fish.force.normalize() * game_physics.max_force;
        }
        fish.acceleration = fish.force;
        fish.force = Vec2::new(0.0, 0.0);
        fish_data.f_cohesion = Vec2::new(0.0, 0.0);
        fish_data.f_cohesion_count = 0;
        fish_data.f_sep = Vec2::new(0.0, 0.0);
        fish_data.f_sep_count = 0;
        fish_data.f_avoid = Vec2::new(0.0, 0.0);
        fish_data.f_avoid_count = 0;
        fish_data.f_align = Vec2::new(0.0, 0.0);
        fish_data.f_align_count = 0;
    }
}

fn fish_update(mut query: Query<(&mut Fish, &mut Transform), With<Fish>>, game_physics: Res<Physics>, time: Res<Time>, simulation_settings: Res<Simulation>, bounds: Res<Bounds>) {
    for (mut fish, mut transform) in query.iter_mut() {
        let acc = fish.acceleration;
        fish.velocity += acc * time.delta_seconds() * simulation_settings.acceleration_multiplier;

        // Limit velocity to max velocity
        fish.velocity = fish.velocity.normalize() * game_physics.velocity_mag;

        transform.translation.x += fish.velocity.x * time.delta_seconds() * simulation_settings.velocity_multiplier;
        transform.translation.y += fish.velocity.y * time.delta_seconds() * simulation_settings.velocity_multiplier;

        // Rotate fish to face direction of velocity
        transform.rotation = Quat::from_rotation_z(fish.velocity.y.atan2(fish.velocity.x));

        if transform.translation.x < bounds.a.x {
            transform.translation.x = bounds.b.x;
        } else if transform.translation.x > bounds.b.x {
            transform.translation.x = bounds.a.x;
        }

        if transform.translation.y < bounds.a.y {
            transform.translation.y = bounds.b.y;
        } else if transform.translation.y > bounds.b.y {
            transform.translation.y = bounds.a.y;
        }

        fish.acceleration = Vec2::new(0.0, 0.0);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window : Some(Window {
                resizable: false,
                mode:  WindowMode::Windowed,
                resolution: WindowResolution::new(1920.0, 1080.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // .add_plugins(PerfUiPlugin::default())
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .init_resource::<FishCount>()
        .register_type::<FishCount>()
        .init_resource::<Physics>()
        .register_type::<Physics>()
        .init_resource::<Simulation>()
        .register_type::<Simulation>()
        .init_resource::<Bounds>()
        // .add_plugins(ResourceInspectorPlugin::<FishCount>::default())
        // .add_plugins(ResourceInspectorPlugin::<Physics>::default())
        // .add_plugins(ResourceInspectorPlugin::<Simulation>::default())
        .add_systems(Startup, (set_bounds, setup).chain())
        .add_systems(Update, on_click_spawn_obstical)
        .add_systems(Update, (on_fish_count_change, (fish_calculate_force, fish_avoid_obstacles, fish_normalize_force, fish_update).chain()).chain())
        .run();
}