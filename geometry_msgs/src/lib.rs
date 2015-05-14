struct Time {
    time: u64,
}

struct Header {
    seq: u32,
    stamp: Time,
    frame_id: String
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}


struct PointStamped {
    header: Header,
    point: Point,
}

struct Pose {
    position: Point,
    orientation: Quaternion,
}

struct PoseStamped {
    header: Header,
    pose: Pose
}

struct Twist {
    linear: Vector3,
    angular: Vector3,
}

struct TwistStamped {
    header: Header,
    twist: Twist
}