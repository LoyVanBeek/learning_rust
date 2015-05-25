extern crate geometry_msgs;

pub mod amigo_base;

// trait Base {
//     fn base_move(& mut self, goal: geometry_msgs::PoseStamped);
//     fn force_drive(& mut self, direction: geometry_msgs::Twist);
//     fn get_location(& mut self) -> geometry_msgs::PoseStamped;
//     fn set_initial_pose(& mut self, pose: geometry_msgs::PoseStamped);
//     fn go(& mut self, goal: geometry_msgs::PoseStamped, timeout: i32);
//     fn reset_costmap(& mut self);
//     fn cancel_goal(& mut self);
// }