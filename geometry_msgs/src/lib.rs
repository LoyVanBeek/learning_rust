use std::ops::Add;

#[derive(Debug, Clone, Copy, Default)]
pub struct Time {
    pub time: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Header {
    pub seq: u32,
    pub stamp: Time,
    // frame_id: String //TODO: Copy not implemented for String
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointStamped {
    pub header: Header,
    pub point: Point,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PoseStamped {
    pub header: Header,
    pub pose: Pose
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TwistStamped {
    pub header: Header,
    pub twist: Twist
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