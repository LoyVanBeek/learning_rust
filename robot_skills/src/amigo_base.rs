extern crate geometry_msgs;

trait Base {
    fn base_move(& mut self, goal: geometry_msgs::PoseStamped);
    fn force_drive(& mut self, direction: geometry_msgs::Twist);
    fn get_location(& mut self) -> geometry_msgs::PoseStamped;
    fn set_initial_pose(& mut self, pose: geometry_msgs::PoseStamped);
    fn go(& mut self, goal: geometry_msgs::PoseStamped, timeout: i32);
    fn reset_costmap(& mut self);
    fn cancel_goal(& mut self);
}

#[derive(Debug)]
struct AmigoBase
{
    current_position: geometry_msgs::PoseStamped,
}

impl Base for AmigoBase {
    fn base_move(& mut self, goal: geometry_msgs::PoseStamped){
        self.current_position = goal;
    }

    fn force_drive(& mut self, direction: geometry_msgs::Twist){
    }

    fn get_location(& mut self) -> geometry_msgs::PoseStamped {
        self.current_position
    }

    fn set_initial_pose(& mut self, pose: geometry_msgs::PoseStamped){
        self.current_position = pose;
    }

    fn go(& mut self, goal: geometry_msgs::PoseStamped, timeout: i32){
        self.base_move(goal);
    }

    fn reset_costmap(& mut self){}

    fn cancel_goal(& mut self){}
}