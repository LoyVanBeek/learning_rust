pub mod math;
use math::Addable;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    time: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    seq: u32,
    stamp: Time,
    // frame_id: String //TODO: Copy not implemented for String
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct PointStamped {
    header: Header,
    point: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Pose {
    position: Point,
    orientation: Quaternion,
}

#[derive(Debug, Clone, Copy)]
pub struct PoseStamped {
    header: Header,
    pose: Pose
}

#[derive(Debug, Clone, Copy)]
pub struct Twist {
    linear: Vector3,
    angular: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct TwistStamped {
    header: Header,
    twist: Twist
}


impl Addable for Vector3 {
    fn add(a: Vector3, b: Vector3) -> Vector3{
        Vector3{x: a.x + b.x, 
                y: a.y + b.y,
                z: a.z + b.z}
    }
}
