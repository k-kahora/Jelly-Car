use bevy::{
    prelude::{shape::Circle, *},
    transform::{commands, self},
};

use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;

// For each point spawn a shape bundle, color, and stroke maybe

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_sequence)
        .add_system(point_movement)
        .add_system(minimum_bounding_box)
        .add_system(update_bounding_box)
        .add_system(line_movement)
        .run();
}

pub const POINT_SPEED: f32 = 200.0;
pub const GRAVITY: f32 = 9.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct BoundingBox;

// This is a marker
#[derive(Component)]
struct Point;

// Bounding Box marker
#[derive(Component)]
struct MiniBox(Vec<Vec2>);


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

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

#[derive(Bundle)]
struct Spring {
    // Two points
    // Stiffness
    // Rest length
    // Dampang Factor
}

// We have a BoundingBox that has its own shape
// position
// And stroke color

#[derive(Bundle)]
struct BoundingBoxBundle {
    shape: ShapeBundle,
    stroke: Stroke,
}

impl Default for BoundingBoxBundle {
    fn default() -> Self {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(Vec2::new(0., 0.));
        path_builder.line_to(Vec2::new(0., 20.));
        path_builder.line_to(Vec2::new(10., 10.));
	path_builder.close();
	let path = path_builder.build();
	BoundingBoxBundle {
	    stroke: Stroke::new(Color::YELLOW, 3.0),
	    shape: ShapeBundle {
		path,
		..default()
	    }
	}
    }
}

#[derive(Bundle)]
struct utility {
}

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
    fn new_bounnding_box() -> BoundingBoxBundle {
	BoundingBoxBundle {
	    ..default()
	}
    }
}

// The line is the parent and the points are the children
// Query children in the line query
fn update_bounding_box(
    mut bounding_box_query: Query<&mut Path, With<BoundingBox>>,
    mut group_query: Query<(&mut Children, &mut MiniBox), With<Group>>,    time: Res<Time>
) {
    
    for (mut children, mut minibox) in group_query.iter_mut() {
	for mut child  in children.iter() {
	    let mut path_builder = PathBuilder::new();
	    let box_bounding = bounding_box_query.get_mut(*child);
	    println!("{:?}", minibox.0);
	    path_builder.move_to(minibox.0[0]);
	    path_builder.line_to(minibox.0[1]);
	    path_builder.line_to(minibox.0[2]);
	    path_builder.line_to(minibox.0[3]);
	    if let Ok(mut path) = box_bounding {
		path_builder.close();
		*path = path_builder.build();
	    }
	    
	}

    }

}

// Check that the boxes collide if they collide then we need to send an event to do the raycast
// Query for the group and grab it's minibox
// Query for anoth group and grab it's minbox

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
    }
	
}
  

// Bounding Box needs to be calculated every frame for all non moving entitys

fn line_movement(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Path, &Children), With<Group>>,
    time: Res<Time>
)
{
    for (mut path, children) in line_query.iter_mut() {
	let mut path_builder = PathBuilder::new();
	for &child  in children.iter() {
	    let point = point_query.get(child);
	    if let Ok(transform) = point {
		path_builder.line_to(transform.translation.truncate());
	    }
	}
	path_builder.close();
	*path = path_builder.build();
    }
}

fn point_movement(mut point_query: Query<(&mut Transform, &Point, &Direction, &mut Speed)>, time: Res<Time>) {
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
    let bounding_box_trap = utility::new_bounnding_box();

    let square_points = utility::new_group(&trapezoid);
    let square_lines  = utility::draw_paths(&trapezoid);

    let default_minibox = vec![Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.),Vec2::new(0., 0.)];

    commands.spawn((
        paths,
        Stroke::new(Color::WHITE, 4.0),
	Group,
	MiniBox(default_minibox.clone())
    )).with_children(|parent| {
		     for point in points {
			 parent.spawn((point, Point));
		     }
	parent.spawn((bounding_box, BoundingBox));
	// Make a bounding box here
    });

    // Parent is the lines, child is the bounding box, and children are all the points
    commands.spawn((
        square_lines,
        Stroke::new(Color::WHITE, 4.0),
	Group,
	MiniBox(default_minibox.clone())
    )).with_children(|parent| {
		     for point in square_points {
			 parent.spawn((point, Point));
		     }
	parent.spawn((bounding_box_trap, BoundingBox));

    });
}
