//! The collaborators of the montage container, as one trait.
//!
//! Every call the lifted dispatcher makes that is not itself lifted is one
//! method here: the session open, the three identity gates and their wanted
//! tags, the list walk, the refresh sequence with its float blend tail, and
//! the reconciliation. The production game implements the trait over its own
//! subsystems; tests implement it with a recorder. Arguments are in the
//! lifted form: handles instead of object addresses, tag enums instead of
//! the game's string constants, words instead of fixed cells (a fixed
//! context object the production world supplies itself is not passed), and
//! the calling convention is not visible here.

use lf_core::Handle32;

/// Tag for the handle to the container object itself.
pub struct OwnTag;
/// Tag for the opaque helper context supplied by the UI subsystem.
pub struct HelperContextTag;
/// Tag for the opaque lookup context supplied by the registry subsystem.
pub struct LookupContextTag;
/// Tag for the shared editor state object.
pub struct StateTag;
/// Tag for the helper object returned by an activation call.
pub struct ActiveTag;
/// Tag for the handle to the clip list object.
pub struct ListTag;
/// Tag for the handle to the auxiliary object the refresh toggles.
pub struct AuxTag;
/// Tag for the handle to the configuration object.
pub struct CfgTag;
/// Tag for the handle to the output object the refresh signals.
pub struct OutTag;
/// Tag for the handle to the view object the reconcile step drives.
pub struct ViewTag;
/// Tag for the handle to the source object the regions read and notify through.
pub struct SrcTag;
/// Tag for the handle to an opened session object.
pub struct SessionTag;
/// Tag for the handle to one clip element of the list.
pub struct ElemTag;
/// Tag for the handle to the list's parameter sub-object.
pub struct TailTag;
/// Tag for the handle to the list's inner sub-object.
pub struct InnerTag;
/// Tag for the handle to a probe opened on the inner sub-object.
pub struct ProbeTag;
/// Tag for the handle to a resolved blend target.
pub struct OutputTag;
/// Tag for the object used by the pick method's optional epilogue.
pub struct EpilogueTag;
/// Tag for the handle to the configuration's sub-object.
pub struct CfgSubTag;

/// Which tag constant a lookup call passes.
///
/// The original passes the address of a game string; the lift names the
/// role instead. Each variant is one constant the dispatcher uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// The first gate's wanted tag.
    A,
    /// The activation tag of the list region.
    B,
    /// The lookup tag of the list region.
    C,
    /// The second gate's wanted tag.
    D,
    /// The activation tag of the refresh region.
    E,
    /// The lookup tag of the refresh region.
    F,
    /// The preparation tag of the refresh region.
    G,
    /// The handle tag of the refresh region.
    H,
    /// The blend target tag of the refresh region.
    I,
    /// The third gate's wanted tag.
    J,
}

/// The calls the montage container's lifted dispatcher makes on its collaborators.
///
/// Names describe the role each call plays in the region that makes it.
/// Where a call's result is consumed, the result is returned; where the
/// original ignores the answer, nothing is returned (see the registry for
/// the per-method narrowings). Fixed contexts and constant arguments are
/// explicit so the differential recorder can compare every argument. The
/// two raw byte stores are represented as semantic effects on collaborator
/// objects; their full blocks are compared by the differential test.
pub trait MontageWorld {
    /// Opens a session for the state flag; answers the session object.
    fn open_session(
        &mut self,
        context: Handle32<HelperContextTag>,
        flag: u32,
    ) -> Option<Handle32<SessionTag>>;
    /// The container's own open entry, run after the session opens.
    fn own_open(&mut self, own: Handle32<OwnTag>);
    /// The session's tag word, compared against the wanted tag at each gate.
    fn session_tag(&mut self, session: Option<Handle32<SessionTag>>) -> u32;
    /// The wanted tag word for an identity gate, or the refresh handle.
    fn tag_id(&mut self, tag: Tag) -> u32;
    /// Activates the helper for a region.
    fn activate(
        &mut self,
        context: Handle32<HelperContextTag>,
        tag: Tag,
    ) -> Handle32<ActiveTag>;
    /// Sets the helper object's active flag, a byte store made after activation.
    fn mark_active(&mut self, active: Handle32<ActiveTag>, value: bool);
    /// The shared lookup a region runs after activating.
    fn lookup(&mut self, context: Handle32<LookupContextTag>, tag: Tag);
    /// The clip list's element count.
    fn list_count(&mut self, list: Handle32<ListTag>) -> u32;
    /// The clip element at an index.
    fn list_at(&mut self, list: Handle32<ListTag>, index: u32) -> Option<Handle32<ElemTag>>;
    /// The first notify word of a clip element (read, not called).
    fn elem_word_a(&mut self, elem: Option<Handle32<ElemTag>>) -> u32;
    /// The second notify word of a clip element (read, not called).
    fn elem_word_b(&mut self, elem: Option<Handle32<ElemTag>>) -> u32;
    /// Notifies the first helper for one clip element.
    fn emit_a(&mut self, src: Handle32<SrcTag>, index: u32, word: u32);
    /// Notifies the second helper for one clip element.
    fn emit_b(&mut self, src: Handle32<SrcTag>, index: u32, word: u32);
    /// The freshness value the refresh region thresholds.
    fn freshness(&mut self, src: Handle32<SrcTag>) -> u32;
    /// The teardown mark, run when the freshness threshold trips.
    fn note_teardown_mark(&mut self, marker: u32);
    /// The shared state refresh, run on the refresh path.
    fn state_refresh(&mut self, state: Handle32<StateTag>);
    /// The shared state teardown; answers the dispatcher's result on the
    /// threshold path.
    fn state_teardown(&mut self, state: Handle32<StateTag>) -> u32;
    /// Sets the auxiliary object's mode on the toggled refresh path.
    fn aux_set(&mut self, aux: Handle32<AuxTag>, mode: u32);
    /// The list's parameter sub-object (read, not called).
    fn list_tail(&mut self, list: Handle32<ListTag>) -> Option<Handle32<TailTag>>;
    /// Sets the parameter sub-object's mode on the toggled refresh path.
    fn tail_set(&mut self, tail: Option<Handle32<TailTag>>, mode: u32);
    /// Signals the output object on the toggled refresh path.
    fn signal_out(&mut self, out: Handle32<OutTag>, flag: u32);
    /// The list's inner sub-object (read, not called).
    fn list_inner(&mut self, list: Handle32<ListTag>) -> Option<Handle32<InnerTag>>;
    /// Opens a probe on the inner sub-object.
    fn probe_open(&mut self, inner: Option<Handle32<InnerTag>>) -> Option<Handle32<ProbeTag>>;
    /// Primes an opened probe.
    fn probe_prime(&mut self, probe: Option<Handle32<ProbeTag>>);
    /// Sets a second probe's mode.
    fn probe_set(&mut self, probe: Option<Handle32<ProbeTag>>, mode: u32);
    /// The preparation step of the refresh sequence.
    fn prepare(&mut self, tag: Tag, argument: u32);
    /// Opens the refresh worker session for the container and the first
    /// session; answers the worker session.
    fn open_worker(
        &mut self,
        own: Handle32<OwnTag>,
        first: Option<Handle32<SessionTag>>,
        handle: u32,
        trailing: u32,
    ) -> Option<Handle32<SessionTag>>;
    /// The worker session's first word.
    fn session_first(&mut self, session: Option<Handle32<SessionTag>>) -> u32;
    /// The list rank of a key word.
    fn list_rank(&mut self, list: Handle32<ListTag>, key: u32) -> u32;
    /// Notes a rank on the container.
    fn note_rank(&mut self, own: Handle32<OwnTag>, key: u32);
    /// Notes the worker session on the container.
    fn note_session(&mut self, own: Handle32<OwnTag>, session: Option<Handle32<SessionTag>>);
    /// Touches the list at a rank with a count.
    fn list_touch(&mut self, list: Handle32<ListTag>, key: u32, count: u32);
    /// Sets the worker session's mode.
    fn session_mode(&mut self, session: Option<Handle32<SessionTag>>, mode: u32);
    /// Sets the list's mode.
    fn list_mode(&mut self, list: Handle32<ListTag>, mode: u32);
    /// The container sync step of the refresh sequence.
    fn note_sync(&mut self, own: Handle32<OwnTag>);
    /// The property word of the inner sub-object.
    fn prop_read(&mut self, inner: Option<Handle32<InnerTag>>) -> u32;
    /// Sets the configuration from a property word.
    fn cfg_set(&mut self, cfg: Handle32<CfgTag>, prop: u32);
    /// Clears the configuration object's status byte on the toggled path.
    fn clear_cfg_status(&mut self, cfg: Handle32<CfgTag>);
    /// The blend value for a property word, as bits.
    fn blend_level(&mut self, src: Handle32<SrcTag>, prop: u32) -> u32;
    /// Resolves the blend target for a tag and a blend value's bits.
    fn resolve_output(
        &mut self,
        context: Handle32<HelperContextTag>,
        tag: Tag,
        blend: u32,
    ) -> Option<Handle32<OutputTag>>;
    /// Releases the blend target; answers the dispatcher's result on the
    /// refresh path.
    fn release_output(&mut self, target: Option<Handle32<OutputTag>>) -> u32;
    /// Sets the view's mode on the reconcile path.
    fn view_set(&mut self, view: Handle32<ViewTag>, mode: u32);
    /// The configuration's sub-object (read, not called).
    fn cfg_sub(&mut self, cfg: Handle32<CfgTag>) -> Option<Handle32<CfgSubTag>>;
    /// Sets the configuration sub-object's mode on the reconcile path.
    fn cfg_sub_set(&mut self, sub: Option<Handle32<CfgSubTag>>, mode: u32);
    /// The container's own re-arm entry; answers the dispatcher's result
    /// on the reconcile path.
    fn own_rearm(&mut self, own: Handle32<OwnTag>, mode: u32) -> u32;
    /// Drops the pending index on the reconcile path.
    fn drop_pending(&mut self, own: Handle32<OwnTag>, pending: u32);
}
