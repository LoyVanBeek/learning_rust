use std::ops::Add;

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

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self: Self, _rhs: Vector3) -> Vector3 {
        Vector3{x: self.x + _rhs.x, 
                y: self.y + _rhs.y, 
                z: self.z + _rhs.z}
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self: Self, _rhs: Vector3) -> Point {
        Point{x: self.x + _rhs.x, 
                y: self.y + _rhs.y, 
                z: self.z + _rhs.z}
    }
}

impl Add for Twist {
    type Output = Twist;

    fn add(self: Self, _rhs: Twist) -> Twist {
        Twist{  linear: self.linear + _rhs.linear, 
                angular: self.angular + _rhs.angular}
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    //TODO: This is NOT actual quaternion adding but only a stub!
    fn add(self: Self, _rhs: Quaternion) -> Quaternion {
        Quaternion{ x: self.x + _rhs.x,
                    y: self.y + _rhs.y,
                    z: self.z + _rhs.z,
                    w: self.w + _rhs.w }
    }
}

impl Add<Vector3> for Quaternion {
    type Output = Quaternion;

    //TODO: This is NOT actual quaternion adding but only a stub!
    //You need a conversion with Euler angles etc. 
    fn add(self: Self, _rhs: Vector3) -> Quaternion {
        Quaternion{ x: self.x + _rhs.x,
                    y: self.y + _rhs.y,
                    z: self.z + _rhs.z,
                    .. self}
    }
}

impl Add<Twist> for Pose {
    type Output = Pose;

    fn add(self: Self, _rhs: Twist) -> Pose {
        Pose{   position: self.position + _rhs.linear, 
                orientation: self.orientation + _rhs.angular,
                .. self}
    }
}