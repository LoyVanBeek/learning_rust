extern crate geometry_msgs;

trait Arm {
    // fn new(side: String) -> Arm;
    fn operational(& mut self) -> bool;
    fn send_goal(& mut self, goal: geometry_msgs::PoseStamped) -> bool;
    fn send_joint_goal(& mut self, goal: Vec<f64>) -> bool;
    fn send_joint_trajectory(& mut self, trajectory: Vec<Vec<f64>>) -> bool;
    fn reset(& mut self);
    fn occupied_by(& mut self) -> bool;
    fn send_gripper_goal(& mut self, open_close: bool) -> bool;
}

#[derive(Debug)]
struct AmigoArm
{
    side: String,
    end_effector_pose: geometry_msgs::PoseStamped,
    joints: Vec<f64>,
    operational: bool,
}

impl Arm for AmigoArm {
    // fn new(side: String) -> Arm{
    //     Arm{side: side, 
    //         operational: true}
    // }
    fn operational(& mut self) -> bool{
        return self.operational;
    }
        
    fn send_goal(& mut self, goal: geometry_msgs::PoseStamped) -> bool{
        self.end_effector_pose = goal;
        return true;
    }

    fn send_joint_goal(& mut self, goal: Vec<f64>) -> bool{
        assert_eq!(goal.len(), self.joints.len());

        self.joints = goal;
        return true
    }

    fn send_joint_trajectory(& mut self, trajectory: Vec<Vec<f64>>) -> bool{
        for pose in trajectory {
            if(!self.send_joint_goal(pose)){
                return false;
            }
        }
        return true;
    }

    fn reset(& mut self){

    }

    fn occupied_by(& mut self) -> bool{
        return true;
    }

    fn send_gripper_goal(& mut self, open_close: bool) -> bool{
        return true;
    }
}