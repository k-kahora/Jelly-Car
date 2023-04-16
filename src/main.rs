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
        .add_system(minimum_bounding_box)
        .add_system(find_center_point)
        .add_system(camera_follow_system)
        .add_system(confine_movement)
        .add_system(update_springs)
        .run();
}

pub const POINT_SPEED: f32 = 200.0;
pub const GRAVITY: Vec2 = Vec2::new(0., -9.8);
pub const STIFFNESS: f32 = 9.;
pub const DAMPING_FACTOR: f32 = 9.;

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
struct MiniBox(Vec<Vec2>);

#[derive(Component)]
struct ObjectName(String);

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Force(Vec2);

#[derive(Component)]
struct Car(Vec2);

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

// A spring is a entity
// With A, and B as th start and end-point between
// It also has a 
#[derive(Component)]
struct Stiffness(f32);

#[derive(Component)]
struct DampingFactor(f32);

#[derive(Component)]
struct RestLength(f32);

#[derive(Component)]
struct Once(bool);

#[derive(Component)]
struct PointAandB(Vec<Entity>);

#[derive(Bundle)]
struct SpringBundle {
    restLength: RestLength,
    dampingFactor: DampingFactor,
    stiffness: Stiffness,
    once: Once,
    point_a_and_b: PointAandB,
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

#[derive(Component)]
struct Anchored;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct PointMassBundle {
    // These are the properties of a point mass
    mass: Mass,
    force: Force,
    position: Position,
    direction: Direction,
    // Later replace speed with force
    velocity: Velocity,
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
                mass: Mass(1.),
                force: Force(Vec2::new(0., 0.)),
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
                velocity: Velocity(Vec2::new(0., 0.)),
            })
        }

        point_masses
    }

    // generate a springs based on the points
    fn make_springs(list_of_points: &Vec<Entity>) -> Vec<SpringBundle>{
	let mut springs: Vec<SpringBundle> = Vec::new();
	for i in 0..list_of_points.len() - 1 {
	    let current = list_of_points[i];
	    let next_val = list_of_points[i + 1];
	    let spring = SpringBundle {
		restLength: RestLength(1.),
		once: Once(true),
		dampingFactor: DampingFactor(DAMPING_FACTOR),
		stiffness: Stiffness(STIFFNESS),
		point_a_and_b: PointAandB(vec![current, next_val]),
	    };
	    springs.push(spring);
	}
	springs
    }

    fn spawn_shape( commands: &mut Commands, list_of_points: &Vec<Vec2>, anchor: bool ) {
	
	let points = utility::new_group(&list_of_points);
	let paths = utility::draw_paths(&list_of_points);
	let bounding_box = utility::new_bounnding_box();
	let default_minibox = vec![Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.)];
	let mut entitys = Vec::new();

	let spawner = commands
	    .spawn((
		paths,
		Stroke::new(Color::WHITE, 4.0),
		Group,
		MiniBox(default_minibox.clone()),
		Car(Vec2::new(0.0, 0.0)),
	    ))
	    .with_children(|parent| {
		for point in points {
		    let mut spawn = parent.spawn((point, Point));
		    if anchor {
			spawn.insert(Anchored);
		    }
		    let id = spawn.id();
		    entitys.push(id);
		}
		// Make a bounding box here
	    });
	// if anchor {
	//     spawner.insert(Anchored);
	// }

	let springs = utility::make_springs(&entitys);
	for spring in springs {
	    commands.spawn(spring);
	}




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

// Calculates the force on each spring
fn update_springs(
    // Need to query for the position of aech point on the spring
    // Step 1: Query all sprinps
    // Step 2: Get a query for the two points on the spring, we need theri position
    mut spring_query: Query<(&mut RestLength, &mut Once, &Stiffness, &DampingFactor, &PointAandB)>,
    point_query: Query<(&Transform, &mut Velocity), With<Point>>
) {
    
    for (mut rest_length, mut once, stiff, damp, a_b) in spring_query.iter_mut() {
	let a = point_query.get(a_b.0[0]).unwrap();
	let b = point_query.get(a_b.0[1]).unwrap();
	let a_translation = a.0.translation;
	let a_velocity = a.1.0;
	let b_translation = b.0.translation;
	let b_velocity = b.1.0;

	if once.0 {
	    once.0 = false;

	    rest_length.0 = a_translation.distance(b_translation);
	    println!("Rest Length is {}", rest_length.0);
	}
	// Hooks law 
	let b_minus_a_norm = (b_translation - a_translation).normalize();
	// normalized direction vector form A to B
	let spring_force = ( b_minus_a_norm - rest_length.0) * stiff.0;
	// Veclocity diffrence
	let vel_diff = b_velocity - a_velocity;
	let vel_diff_three = Vec3::new(vel_diff.x, vel_diff.y, 0.);
	let vel_diff_b_minus_a_norm_dot = b_minus_a_norm.dot(vel_diff_three);
	let vel_diff_b_minus_a_norm_dot_damp = b_minus_a_norm.dot(vel_diff_three) * damp.0;
	let total_spring_force = spring_force + vel_diff_b_minus_a_norm_dot_damp;

	// take total spring force and multiply it by the normalized dircetioin vector of the other point

	// println!("Spring Force {}", total_spring_force);
    }
    

}

fn minimum_bounding_box(
    point_query: Query<&Transform, With<Point>>,
    mut group_query: Query<(&mut Children, &mut MiniBox), With<Group>>,    time: Res<Time>
)
{

    // I want to query all groups
    // Gathre point children
    // Gather bounding box children
    // Use point children to update the 
    
    for (mut children, mut minibox) in group_query.iter_mut() {
	let mut maxX = f32::MIN;
	let mut maxY = f32::MIN;
	let mut minX = f32::MAX;
	let mut minY = f32::MAX; 
	for mut child  in children.iter() {
	    let point = point_query.get(*child);
	    // let mut box = bounding_box_query.get(child)
	    if let Ok(transform) = point {
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
		    //   calulate all four points to get minimum bounding box
	    }
	}
	let minibox_test = vec![
	   Vec2::new(minX, minY),
	   Vec2::new(minX, maxY),
	   Vec2::new(maxX, maxY),
	   Vec2::new(maxX, minY),
	];

	minibox.0 = minibox_test;
	println!("{:?}", minibox.0);
    }
	
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
    mut point_query: Query<(&mut Transform, &mut Force, &Mass, &Point, &Direction, &mut Velocity), Without<Anchored>>,
    time: Res<Time>,
) {
    for (mut transform,mut force, mass, point, direction, mut velocity) in point_query.iter_mut() {
        let direction = Vec3::new(velocity.0.x, velocity.0.y, 0.);
	force.0 = Vec2::new(0.,0.);
	force.0 += GRAVITY * mass.0;
	velocity.0 += (force.0 / mass.0) * time.delta_seconds();
	let displacement = velocity.0 * time.delta_seconds();
	let vec2to3 = Vec3::new(displacement.x, displacement.y, 0.);
	transform.translation += vec2to3;
        // transform.translation += direction.normalize() * time.delta_seconds();
    }
}

// Give MassPointgroup a list of 2d vectors for an object
fn startup_sequence(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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

    utility::spawn_shape(&mut commands, &car, false);

    let rect = vec![
        Vec2::new(0., -100.),
        Vec2::new(50., -100.),
        Vec2::new(50., -4000.),
        Vec2::new(0., -4000.),
    ];

    utility::spawn_shape(&mut commands, &rect, true);

    // utility::spawn_shape(commands, &car, true);

    // let pointsk = utility::new_group(&car);
    // let paths = utility::draw_paths(&car);
    // let bounding_box = utility::new_bounnding_box();
    // let default_minibox = vec![Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.)];
    // let mut entitys = Vec::new();

    // commands
    //     .spawn((
    //         paths,
    //         Stroke::new(Color::WHITE, 4.0),
    //         Group,
    // 	    MiniBox(default_minibox.clone()),
    //         Car(Vec2::new(0.0, 0.0)),
    //     ))
    //     .with_children(|parent| {
    //         for point in points {
    //             let id = parent.spawn((point, Point)).id();
    // 		entitys.push(id);
    //         }
    //         // Make a bounding box here
    //     });

    // let springs = utility::make_springs(&entitys);
    // for spring in springs {
    // 	commands.spawn(spring);
    // }


    // Parent is the lines, child is the bounding box, and children are all the points
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
    mut point_query: Query<(&mut Velocity, &mut Transform), With<Point>>,
    mut line_query: Query<(&Children), With<Car>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (children) in line_query.iter_mut() {
        for &child in children.iter() {
            let mut point = point_query.get_mut(child);
            if let Ok((mut velocity_transform, mut point_transform)) = point {
                let window = window_query.get_single().unwrap();
                let min_y: f32 = -200.0;
                let mut translation = point_transform.translation;
                if translation.y < min_y {
                    translation.y = min_y;
                    velocity_transform.0.y = 0.0;
                }
                point_transform.translation = translation;
            }
        }
    }
}
