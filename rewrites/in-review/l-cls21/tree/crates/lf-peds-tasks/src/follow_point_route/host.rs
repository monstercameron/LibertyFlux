//! What the follow-point task asks of the outside world.
//!
//! Every call a verified method makes that is not itself lifted is one
//! method here. The host performs the call and gives back its answer; the
//! lifted method makes the same calls, with the same arguments, in the same
//! order as the verified rewrite. Opaque handles (`u32` words the host
//! assigns) name the task, the sub-task and the ped; the lift never reads
//! through them.

use super::route::RoutePoint;
use super::task::FollowPointRouteTask;

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

    /// The move speed for a selector: the helper the selector form of the
    /// constructor calls first.
    fn select_speed(&mut self, selector: u32) -> f32;

    /// The move base's constructor, run on the new task before the task's own
    /// fields are set. It may write any part of the task; the lifted
    /// constructor reads the task's flag word after this call.
    fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32;

    /// Releases the route table word the task owns, through the class's
    /// shared release context (the host reads the context itself).
    fn release_route(&mut self, table: u32) -> u32;

    /// The move base's destructor, run after the task's own release.
    /// Returns its answer, which the destructor returns in turn.
    fn base_destroy(&mut self, task: u32) -> u32;

    /// Allocates the storage for a new task of this class. Returns the new
    /// task's handle, or 0 when the allocator has none to give.
    fn allocate_task(&mut self) -> u32;

    /// The route constructor: builds the route from its arguments.
    fn route_ctor(
        &mut self,
        task: u32,
        target: u32,
        route_start: u32,
        route_end: u32,
        final_value: u32,
        route_id: u32,
    ) -> u32;
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


/// The calls and memory reads of the route-advance poll that reach outside
/// the task: the owner object, the current sub-task, the two sub-task slots
/// and the slot's helpers. Each call is one method; each memory read of
/// another object is one method that returns the word read.
pub trait RoutePollHost {
    /// The type query of the current sub-task.
    fn subtask_query(&mut self, subtask: u32) -> u32;

    /// The sub-task's first slot object (read, not called).
    fn subtask_slot_a(&mut self, subtask: u32) -> u32;

    /// The sub-task's second slot object (read, not called).
    fn subtask_slot_b(&mut self, subtask: u32) -> u32;

    /// The owner's gate query.
    fn owner_gate(&mut self, owner: u32) -> u32;

    /// The owner's next-slot query: the object whose float and int answers
    /// feed the helper.
    fn owner_next(&mut self, owner: u32) -> u32;

    /// The owner's poll, given the poll's three words.
    fn owner_poll(&mut self, owner: u32, a0: u32, a1: u32, a2: u32) -> u32;

    /// The owner's flag word (read, not called).
    fn owner_flags(&mut self, owner: u32) -> u32;

    /// Writes the owner's flag word (not a call).
    fn set_owner_flags(&mut self, owner: u32, flags: u32);

    /// The owner's finish call. The third argument is always zero on this path.
    fn owner_finish(&mut self, owner: u32, a0: u32, zero: u32, a2: u32) -> u32;

    /// The slot's vector query: the point it names.
    fn slot_point(&mut self, slot: u32) -> RoutePoint;

    /// The float answer of the next-slot object.
    fn slotb_float(&mut self, slotb: u32) -> f32;

    /// The int answer of the next-slot object.
    fn slotb_int(&mut self, slotb: u32) -> u32;

    /// The helper that decides whether the segment is taken: returns its word.
    fn helper_ok(&mut self, slot: u32, int_answer: u32, float_bits: u32) -> u32;
}
