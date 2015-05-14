pub struct Time {
    time: u64,
}

pub struct Header {
    seq: u32,
    stamp: Time,
    frame_id: String
}

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}


pub struct PointStamped {
    header: Header,
    point: Point,
}

pub struct Pose {
    position: Point,
    orientation: Quaternion,
}

pub struct PoseStamped {
    header: Header,
    pose: Pose
}

pub struct Twist {
    linear: Vector3,
    angular: Vector3,
}

pub struct TwistStamped {
    header: Header,
    twist: Twist
}