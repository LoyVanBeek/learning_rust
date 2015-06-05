extern crate geometry_msgs;

pub mod amigo_base;
pub mod amigo_arm;

use amigo_arm::Arm;
use amigo_base::Base;

pub struct Amigo{
    name: String,
    leftArm: amigo_arm::AmigoArm,
    rightArm: amigo_arm::AmigoArm,
    base: amigo_base::AmigoBase,
}

pub trait Robot {
    fn new(name: String) -> Self;
}

impl Robot for Amigo {
    fn new(name: String) -> Amigo {
        Amigo{name: name,
              leftArm: amigo_arm::AmigoArm::new("left".to_string()),
              rightArm: amigo_arm::AmigoArm::new("right".to_string()),
              base: amigo_base::AmigoBase::new()}
    }
}