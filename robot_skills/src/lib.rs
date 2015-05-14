extern crate geometry_msgs;

trait Base {
    fn base_move(&self, goal: geometry_msgs::PoseStamped);
    fn force_drive(&self, direction: geometry_msgs::Twist);
    fn get_location(&self) -> geometry_msgs::PoseStamped;

    fn set_initial_pose(&self, pose: geometry_msgs::PoseStamped);
    fn go(&self, goal: geometry_msgs::PoseStamped, timeout: i32);
    fn reset_costmap(&self);
    fn cancel_goal(&self);
}

#[derive(Debug)]
struct AmigoBase
{
    current_position: geometry_msgs::PoseStamped,
}