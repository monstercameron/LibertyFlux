//! What the follow-point task asks of the outside world.
//!
//! Every call a verified method makes that is not itself lifted is one
//! method here. The host performs the call and gives back its answer; the
//! lifted method makes the same calls, with the same arguments, in the same
//! order as the verified rewrite. Opaque handles (`u32` words the host
//! assigns) name the task, the sub-task and the ped; the lift never reads
//! through them.

use super::route::RoutePoint;

/// The calls the follow-point task makes on the task system and on its
/// sub-tasks.
pub trait RouteTaskHost {
    /// Asks the task for the id of the next order: the route's own
    /// planning call used by the waypoint pick when the ped is not moving.
    fn next_order_id(&mut self, task: u32) -> u32;

    /// Gives the ped an order: the task system's order call, with the
    /// order id and the ped handle.
    fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32;

    /// The type id of a sub-task, read through the sub-task's own virtual
    /// slot. The verified method may ask more than once per call; the
    /// answer does not change between asks.
    fn subtask_type(&mut self, subtask: u32) -> u32;

    /// Creates a sub-task of type `id` for the ped, through the task's
    /// sub-task factory.
    fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32;

    /// Names the next route sub-task type, from the route planner.
    fn plan_next_subtask(&mut self, task: u32) -> u32;
}

/// The part of a ped the waypoint pick reads: its handle for the calls it
/// makes, its position, and the state byte whose bit 2 blocks route
/// following.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PedView {
    /// Opaque handle passed to the task system with the order.
    pub handle: u32,
    /// The ped's position, read once per pick.
    pub position: RoutePoint,
    /// The ped's state byte; bit 2 set means the ped may not follow a route.
    pub state_byte: u8,
}
