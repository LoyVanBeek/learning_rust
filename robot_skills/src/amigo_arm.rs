extern crate geometry_msgs;

#[derive(Debug)]
enum ArmError {
    JointLimitHit,
    HardwareFail,
    EnvironmentCollision,
    SelfCollision,
}

pub type ArmResult = Result<bool, ArmError>;

pub trait Arm {
    fn new(side: String) -> Self;
    fn operational(& mut self) -> bool;
    fn send_goal(& mut self, goal: geometry_msgs::PoseStamped) -> ArmResult;
    fn send_joint_goal(& mut self, goal: Vec<f64>) -> ArmResult;
    fn send_joint_trajectory(& mut self, trajectory: Vec<Vec<f64>>) -> ArmResult;
    fn reset(& mut self);
    fn occupied_by(& mut self) -> bool;
    fn send_gripper_goal(& mut self, open_close: bool) -> ArmResult;
}

#[derive(Debug)]
pub struct AmigoArm
{
    side: String,
    end_effector_pose: geometry_msgs::PoseStamped,
    joints: Vec<f64>,
    operational: bool,
}

impl Arm for AmigoArm {
    fn new(side: String) -> AmigoArm{
        AmigoArm{   side:           side, 
                    operational:    true,
                    end_effector_pose: Default::default(),
                    joints: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]}
    }
    
    fn operational(& mut self) -> bool{
        return self.operational;
    }
        
    fn send_goal(& mut self, goal: geometry_msgs::PoseStamped) -> ArmResult{
        self.end_effector_pose = goal;
        return Ok(true);
    }

    fn send_joint_goal(& mut self, goal: Vec<f64>) -> ArmResult{
        assert_eq!(goal.len(), self.joints.len());

        self.joints = goal;
        return Ok(true)
    }

    fn send_joint_trajectory(& mut self, trajectory: Vec<Vec<f64>>) -> ArmResult{
        for pose in trajectory {
            match self.send_joint_goal(pose){
                Ok(result) => (),
                Err(why) => return Err(why)
            }
        }
        return Ok(true);
    }

    fn reset(& mut self){

    }

    fn occupied_by(& mut self) -> bool{
        return true;
    }

    fn send_gripper_goal(& mut self, open_close: bool) -> ArmResult{
        return Ok(true);
    }
}