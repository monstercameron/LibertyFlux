//! The collaborators of the game camera class.
//!
//! Every call a lifted camera method makes that is not lifted itself is one
//! method here. Numbered callees keep the order the verified rewrites call
//! them in; the virtual calls are the object's own dispatched slot and the
//! worker's release slot. Global reads are methods too, because a lifted
//! method owns no global state.

use lf_core::Handle32;

use super::game::CCamGame;

/// Tag: the opaque identity of a game camera object.
pub struct CamTag;
/// Tag: the opaque identity of the owner a camera is attached to.
pub struct CamOwnerTag;
/// Tag: the opaque identity of a camera worker object.
pub struct WorkerTag;
/// Tag: the opaque identity of the object pool manager.
pub struct PoolTag;

/// Everything a game camera method reaches outside its own fields.
///
/// A `this` argument is the camera's identity as the original passes it.
/// A `cam` argument is the camera's own state, which the callee may change;
/// the lifted method reads its fields again after such a call.
pub trait CamGameWorld {
    /// Callee 1 of the deleting destructor: the class's destructor body.
    /// Not lifted yet, so it is a call on the world.
    fn destroy_body(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// The pool manager, read from a shared global after the destructor
    /// body has run. `None` stands for the original's zero.
    fn pool_manager(&mut self) -> Option<Handle32<PoolTag>>;

    /// Callee 2 of the deleting destructor: returns the camera's storage to
    /// the pool held by `manager`.
    fn pool_release(&mut self, manager: Option<Handle32<PoolTag>>, this: Handle32<CamTag>);

    /// The object's own dispatched teardown step (its table's slot 6).
    /// The lift does not read the table: a host whose camera class is
    /// overridden in a derived class dispatches here.
    fn teardown(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 2 of the restart: builds a worker for `owner`. The original
    /// passes two constant words (mode 1, extra 0) between the owner and
    /// the camera; they are fixed by the restart and are passed in the
    /// same order here.
    fn create_worker(
        &mut self,
        owner: Option<Handle32<CamOwnerTag>>,
        mode: u32,
        extra: u32,
        this: Handle32<CamTag>,
    ) -> Option<Handle32<WorkerTag>>;

    /// Callee 3 of the restart: chains the worker's flags.
    fn chain_set_flags(&mut self, worker: Option<Handle32<WorkerTag>>);

    /// The worker's own release step (its table's slot 1). The restart
    /// never reads the worker's table itself.
    fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>);

    /// Callee 1 of the idle reset: the shared reset step, which may change
    /// any field of the camera.
    fn reset_state_block(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// The shared default word, read from a global and copied into the
    /// camera by the idle reset.
    fn shared_default_word(&mut self) -> u32;
}
