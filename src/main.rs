use bevy::{
    prelude::{shape::Circle, *},
    transform::{self, commands},
};

use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

// For each point spawn a shape bundle, color, and stroke maybe

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_sequence)
        .add_system(point_movement)
        .add_system(line_movement)
        .add_system(find_center_point)
        .add_system(camera_follow_system)
        .add_system(confine_movement)
        .run();
}

pub const POINT_SPEED: f32 = 200.0;
pub const GRAVITY: f32 = 9.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Point;

#[derive(Component)]
struct Direction(Vec2);

// TODO Derive a speed compontent and use it to make the speed of eack poit independat
#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct ObjectName(String);

#[derive(Component)]
struct Mass(i32);

#[derive(Component)]
struct Force(Vec2);

#[derive(Component)]
struct DampingFactor(i32);

#[derive(Component)]
struct Car(Vec2);

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

#[derive(Bundle)]
struct Spring {
    // Two points
    // Stiffness
    // Rest length
    // Dampang Factor
}
#[derive(Component)]
struct Square {
    points: Vec<Vec2>,
}

// We have a BoundingBox that has its own shape
// position
// And stroke color
#[derive(Bundle)]
struct BoundingBoxBundle {
    shepe: ShapeBundle,
}

impl Default for Square {
    fn default() -> Self {
        Square {
            points: vec![
                Vec2::new(0., 0.),
                Vec2::new(0., 1.),
                Vec2::new(1., 1.),
                Vec2::new(1., 0.),
                Vec2::new(0., 0.),
            ],
        }
    }
}

#[derive(Bundle)]
struct utility {}

#[derive(Component)]
struct Group;

#[derive(Bundle)]
struct PointMassBundle {
    // These are the properties of a point mass
    mass: Mass,
    position: Position,
    direction: Direction,
    // Later replace speed with force
    speed: Speed,

    // Superflous data
    shape: ShapeBundle,
    color: Fill,
}

impl utility {
    fn new_group(list_of_points: &Vec<Vec2>) -> Vec<PointMassBundle> {
        let mut point_masses = Vec::new();

        for point in list_of_points {
            // giving the point a center makes the transform.translation glitchy
            let circle = shapes::Circle {
                radius: 6.,
                ..default()
            };

            point_masses.push(PointMassBundle {
                mass: Mass(1),
                position: Position(point.clone()),
                // random::<f32>(),random::<f32>()
                direction: Direction(Vec2::new(0., -1.)),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&circle),
                    transform: Transform::from_xyz(point.clone().x, point.clone().y, 0.),
                    ..default()
                },
                // in the future get the name from MassPointgroup
                color: Fill::color(Color::WHITE),
                speed: Speed(0.),
            })
        }

        point_masses
    }

    fn draw_paths(list_of_points: &Vec<Vec2>) -> ShapeBundle {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(list_of_points[0]);

        for point in list_of_points {
            path_builder.line_to(*point);
        }

        path_builder.close();
        let path = path_builder.build();

        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 0., 4.),
            ..default()
        }
    }
    fn new_bounnding_box() -> ShapeBundle {
        ShapeBundle { ..default() }
    }
}

// The line is the parent and the points are the children
// Query children in the line query

fn minimum_bounding_box(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Path, &Children), With<Group>>,
    time: Res<Time>,
) {
    let mut path_builder = PathBuilder::new();
    let mut maxX: f32 = 0.0;
    let mut minX: f32 = 100.0;
    let mut maxY: f32 = 0.0;
    let mut minY: f32 = 100.0;
    for transform in point_query.iter() {
        let position = transform.translation;
        // Update minimum and maximum X and Y values
        if position.x < minX {
            minX = position.x;
        }
        if position.y < minY {
            minY = position.y;
        }
        if position.x > maxX {
            maxX = position.x;
        }
        if position.y > maxY {
            maxY = position.y;
        }

        // ...
    }

    // Create new Path component with four points representing the bounding box corners
    // ...
}

// Bounding Box needs to be calculated every frame for all non moving entitys

fn line_movement(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Path, &Children), With<Group>>,
    time: Res<Time>,
) {
    for (mut path, children) in line_query.iter_mut() {
        let mut path_builder = PathBuilder::new();
        for &child in children.iter() {
            let point = point_query.get(child);
            if let Ok(transform) = point {
                path_builder.line_to(transform.translation.truncate());
            }
        }
        path_builder.close();
        *path = path_builder.build();
    }
}

fn point_movement(
    mut point_query: Query<(&mut Transform, &Point, &Direction, &mut Speed)>,
    time: Res<Time>,
) {
    for (mut transform, point, velocity, mut speed) in point_query.iter_mut() {
        let direction = Vec3::new(velocity.0.x, velocity.0.y, 0.);
        speed.0 += GRAVITY;
        transform.translation += direction.normalize() * speed.0 * time.delta_seconds();
    }
}

// Give MassPointgroup a list of 2d vectors for an object
fn startup_sequence(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let trapezoid = vec![
        Vec2::new(-200., 20.),
        Vec2::new(-100., 20.),
        Vec2::new(-100., 120.),
        Vec2::new(-200., 220.),
        Vec2::new(-200., 20.),
    ];

    let car = vec![
        Vec2::new(0., 0.),
        Vec2::new(200., 0.),
        Vec2::new(200., 30.),
        Vec2::new(170., 40.),
        Vec2::new(140., 90.),
        Vec2::new(60., 90.),
        Vec2::new(30., 45.),
        Vec2::new(0., 40.),
        Vec2::new(0., 0.),
    ];

    let points = utility::new_group(&car);
    let paths = utility::draw_paths(&car);
    let bounding_box = utility::new_bounnding_box();

    let square_points = utility::new_group(&trapezoid);
    let square_lines = utility::draw_paths(&trapezoid);

    commands
        .spawn((
            paths,
            Stroke::new(Color::WHITE, 4.0),
            Group,
            Car(Vec2::new(0.0, 0.0)),
        ))
        .with_children(|parent| {
            for point in points {
                parent.spawn((point, Point));
            }
            // Make a bounding box here
        });

    // Parent is the lines, child is the bounding box, and children are all the points
    commands
        .spawn((square_lines, Stroke::new(Color::WHITE, 4.0), Group))
        .with_children(|parent| {
            for point in square_points {
                parent.spawn((point, Point));
            }
        });
}

fn camera_follow_system(
    car_query: Query<&Car>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(car_transform) = car_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = car_transform.0.x;
            camera_transform.translation.y = car_transform.0.y;
        }
    }
}

fn find_center_point(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&Children), With<Car>>,
    mut center_query: Query<&mut Car>,
) {
    let mut count: f32 = 0.0;
    let mut sum_x: f32 = 0.0;
    let mut sum_y: f32 = 0.0;
    for (children) in line_query.iter_mut() {
        for &child in children.iter() {
            if let Ok(transform) = point_query.get(child) {
                count = count + 1.0;
                sum_x += transform.translation.x;
                sum_y += transform.translation.y;
            }
        }
    }
    let centerpoint_x: f32 = sum_x / count;
    let centerpoint_y: f32 = sum_y / count;
    if let Ok(mut centerpoint) = center_query.get_single_mut() {
        centerpoint.0 = Vec2::new(centerpoint_x, centerpoint_y);
        //println!("centerpoint: ({},{})", centerpoint_x, centerpoint_y);
    }
}

fn confine_movement(
    mut point_query: Query<(&mut Speed, &mut Transform), With<Point>>,
    mut line_query: Query<(&Children), With<Car>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (children) in line_query.iter_mut() {
        for &child in children.iter() {
            let mut point = point_query.get_mut(child);
            if let Ok((mut speed_transform, mut point_transform)) = point {
                let window = window_query.get_single().unwrap();
                let min_y: f32 = -200.0;
                let mut translation = point_transform.translation;
                if translation.y < min_y {
                    translation.y = min_y;
                    speed_transform.0 = 0.0;
                }
                point_transform.translation = translation;
            }
        }
    }
}
