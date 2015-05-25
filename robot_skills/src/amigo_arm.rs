extern crate geometry_msgs;

trait Arm {
    fn __init__(& mut self) -> Arm;
    fn operational(& mut self) -> bool;
    fn send_goal(& mut self, goal: geometry_msgs::PoseStamped) -> bool;
    fn send_joint_goal(& mut self, goal: Vec<f64>);
    fn send_joint_trajectory(& mut self, trajectory: Vec<Vec<f64>>) -> bool;
    fn reset(& mut self);
    fn occupied_by(& mut self) -> bool;
    fn send_gripper_goal(& mut self, open_close: bool) -> bool;
}