extern crate geometry_msgs;

#[derive(Debug)]
enum BaseError {
    HardwareFail,
    EnvironmentCollision,
    GoalUnreachable,
}

pub type BaseResult = Result<geometry_msgs::PoseStamped, BaseError>;

pub trait Base {
    fn new() -> Self;
    fn base_move(& mut self, goal: geometry_msgs::PoseStamped) -> BaseResult;
    fn force_drive(& mut self, direction: geometry_msgs::Twist) -> BaseResult;
    fn get_location(& mut self) -> geometry_msgs::PoseStamped;
    fn set_initial_pose(& mut self, pose: geometry_msgs::PoseStamped);
    fn go(& mut self, goal: geometry_msgs::PoseStamped, timeout: i32) -> BaseResult;
    fn reset_costmap(& mut self);
    fn cancel_goal(& mut self);
}

#[derive(Debug)]
pub struct AmigoBase
{
    current_position: geometry_msgs::PoseStamped,
}

impl Base for AmigoBase {

    fn new() -> AmigoBase{
        AmigoBase{  current_position: Default::default()}
    }
    
    fn base_move(& mut self, goal: geometry_msgs::PoseStamped) -> BaseResult{
        self.current_position = goal;
        return Ok(self.current_position);
    }

    fn force_drive(& mut self, direction: geometry_msgs::Twist) -> BaseResult{
        self.current_position.pose = self.current_position.pose + direction;
        return Ok(self.current_position);
    }

    fn get_location(& mut self) -> geometry_msgs::PoseStamped {
        self.current_position
    }

    fn set_initial_pose(& mut self, pose: geometry_msgs::PoseStamped){
        self.current_position = pose;
    }

    fn go(& mut self, goal: geometry_msgs::PoseStamped, timeout: i32) -> BaseResult{
        return self.base_move(goal);
    }

    fn reset_costmap(& mut self){}

    fn cancel_goal(& mut self){}
}