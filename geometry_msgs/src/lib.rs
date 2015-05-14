#[derive(Debug)]
pub struct Time {
    time: u64,
}

#[derive(Debug)]
pub struct Header {
    seq: u32,
    stamp: Time,
    frame_id: String
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug)]
pub struct PointStamped {
    header: Header,
    point: Point,
}

#[derive(Debug)]
pub struct Pose {
    position: Point,
    orientation: Quaternion,
}

#[derive(Debug)]
pub struct PoseStamped {
    header: Header,
    pose: Pose
}

#[derive(Debug)]
pub struct Twist {
    linear: Vector3,
    angular: Vector3,
}

#[derive(Debug)]
pub struct TwistStamped {
    header: Header,
    twist: Twist
}