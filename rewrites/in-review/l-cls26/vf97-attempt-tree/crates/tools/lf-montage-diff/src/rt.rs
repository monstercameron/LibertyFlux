//! The 32-bit differential runtime: checker macros and recording callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). A numbered callee dispatches through the installed script:
//! each stub records its raw argument words and returns the scripted
//! return word. Virtual-slot calls land on the named stubs below, which
//! the cases plant in test-owned vtables; they record and answer the same
//! way. One float stub serves the blend callee, answering scripted bits
//! through the floating-point return slot.
//!
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through the stubs. Every access
// stays inside memory the running case built.
#![allow(unsafe_code)]

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

use lf_input_frontend::montage_container::Viewport;

use crate::fake::{CallSpec, Script};

/// Which stub shape serves a callee number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Thiscall with the object word only.
    T1,
    /// Thiscall with the object word and one argument.
    T2,
    /// Thiscall with the object word and one argument, float result.
    T2f32,
    /// Thiscall with the object word and two arguments.
    T3,
    /// Thiscall with the object word and three arguments.
    T4,
    /// Cdecl with one argument.
    C1,
    /// Cdecl with two arguments.
    C2,
}

/// The installed script: stub shapes and answer queues by callee number
/// or virtual id, plus the recorded calls.
#[derive(Default)]
struct State {
    kinds: HashMap<u32, Kind>,
    answers: HashMap<u32, std::collections::VecDeque<u32>>,
    log: Vec<Recorded>,
}

/// A recorded call: the callee number or virtual id, and the words.
pub type Recorded = (u32, Vec<u32>);

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// Serialises the cases: one rewrite runs at a time.
pub fn case_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    f(guard.get_or_insert_with(State::default))
}

/// Installs the script: each call of the numbered table gets its stub
/// kind and its answers by name, and each virtual id gets its answers by
/// name. Clears the log.
pub fn install(table: &[(CallSpec, Kind)], virtuals: &[CallSpec], script: &Script) {
    with_state(|state| {
        let mut fresh = State::default();
        for (spec, kind) in table {
            fresh.kinds.insert(spec.id, *kind);
            let queue = script.get(spec.name).cloned().unwrap_or_default();
            fresh.answers.insert(spec.id, queue);
        }
        for spec in virtuals {
            let queue = script.get(spec.name).cloned().unwrap_or_default();
            fresh.answers.insert(spec.id, queue);
        }
        *state = fresh;
    });
}

/// Takes the recorded calls, clearing the log.
pub fn take_log() -> Vec<Recorded> {
    with_state(|state| std::mem::take(&mut state.log))
}

thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
}

fn current_id() -> u32 {
    CURRENT_ID.with(Cell::get)
}

/// Raw stub address for callee number `id`.
///
/// # Panics
///
/// When no script entry covers `id`: a case bug, never a guess.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let kind = with_state(|state| {
        *state
            .kinds
            .get(&id)
            .unwrap_or_else(|| panic!("no stub installed for callee {id}"))
    });
    CURRENT_ID.with(|c| c.set(id));
    let addr = match kind {
        Kind::T1 => stub_t1 as *const () as usize,
        Kind::T2 => stub_t2 as *const () as usize,
        Kind::T2f32 => stub_t2f32 as *const () as usize,
        Kind::T3 => stub_t3 as *const () as usize,
        Kind::T4 => stub_t4 as *const () as usize,
        Kind::C1 => stub_c1 as *const () as usize,
        Kind::C2 => stub_c2 as *const () as usize,
    };
    addr as u32
}

/// Records a call and returns its scripted return word (exhausted queues
/// answer 0).
fn dispatch(id: u32, args: Vec<u32>) -> u32 {
    with_state(|state| {
        state.log.push((id, args));
        state
            .answers
            .get_mut(&id)
            .and_then(std::collections::VecDeque::pop_front)
            .unwrap_or(0)
    })
}

extern "thiscall" fn stub_t1(a: u32) -> u32 {
    dispatch(current_id(), vec![a])
}

extern "thiscall" fn stub_t2(a: u32, b: u32) -> u32 {
    dispatch(current_id(), vec![a, b])
}

extern "thiscall" fn stub_t2f32(a: u32, b: u32) -> f32 {
    f32::from_bits(dispatch(current_id(), vec![a, b]))
}

extern "thiscall" fn stub_t3(a: u32, b: u32, c: u32) -> u32 {
    dispatch(current_id(), vec![a, b, c])
}

extern "thiscall" fn stub_t4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    dispatch(current_id(), vec![a, b, c, d])
}

extern "cdecl" fn stub_c1(a: u32) -> u32 {
    dispatch(current_id(), vec![a])
}

extern "cdecl" fn stub_c2(a: u32, b: u32) -> u32 {
    dispatch(current_id(), vec![a, b])
}

/// Virtual ids live apart from callee numbers. Each names one
/// (object, slot) call site of the rewrite under test.
pub const VID_OWN_OPEN: u32 = 0x1001;
/// The container's own re-arm entry.
pub const VID_OWN_REARM: u32 = 0x1002;
/// The session tag word.
pub const VID_SESSION_TAG: u32 = 0x1003;
/// The worker session's first word.
pub const VID_SESSION_FIRST: u32 = 0x1004;
/// The worker session's mode set.
pub const VID_SESSION_MODE: u32 = 0x1005;
/// The clip list's element count.
pub const VID_LIST_COUNT: u32 = 0x1006;
/// The clip element at an index.
pub const VID_LIST_AT: u32 = 0x1007;
/// The list's mode set.
pub const VID_LIST_MODE: u32 = 0x1008;
/// The list rank of a key.
pub const VID_LIST_RANK: u32 = 0x1009;
/// The auxiliary object's mode set.
pub const VID_AUX_SET: u32 = 0x100A;
/// The parameter sub-object's mode set.
pub const VID_TAIL_SET: u32 = 0x100B;
/// A probe opened on the inner sub-object.
pub const VID_PROBE_OPEN: u32 = 0x100C;
/// The property word of the inner sub-object.
pub const VID_PROP_READ: u32 = 0x100D;
/// A probe's prime step.
pub const VID_PROBE_PRIME: u32 = 0x100E;
/// A probe's mode set.
pub const VID_PROBE_SET: u32 = 0x100F;
/// The view's mode set.
pub const VID_VIEW_SET: u32 = 0x1010;
/// The configuration sub-object's mode set.
pub const VID_CFG_SUB_SET: u32 = 0x1011;

macro_rules! virt0 {
    ($name:ident, $vid:expr) => {
        /// A virtual-slot stub with no stack arguments.
        pub extern "thiscall" fn $name(this: u32) -> u32 {
            dispatch($vid, vec![this])
        }
    };
}

macro_rules! virt1 {
    ($name:ident, $vid:expr) => {
        /// A virtual-slot stub with one stack argument.
        pub extern "thiscall" fn $name(this: u32, a: u32) -> u32 {
            dispatch($vid, vec![this, a])
        }
    };
}

virt0!(v_own_open, VID_OWN_OPEN);
virt1!(v_own_rearm, VID_OWN_REARM);
virt0!(v_session_tag, VID_SESSION_TAG);
virt0!(v_session_first, VID_SESSION_FIRST);
virt1!(v_session_mode, VID_SESSION_MODE);
virt0!(v_list_count, VID_LIST_COUNT);
virt1!(v_list_at, VID_LIST_AT);
virt1!(v_list_mode, VID_LIST_MODE);
virt1!(v_list_rank, VID_LIST_RANK);
virt1!(v_aux_set, VID_AUX_SET);
virt1!(v_tail_set, VID_TAIL_SET);
virt0!(v_probe_open, VID_PROBE_OPEN);
virt0!(v_prop_read, VID_PROP_READ);
virt0!(v_probe_prime, VID_PROBE_PRIME);
virt1!(v_probe_set, VID_PROBE_SET);
virt1!(v_view_set, VID_VIEW_SET);
virt1!(v_cfg_sub_set, VID_CFG_SUB_SET);

macro_rules! virt_f32 {
    ($name:ident, $id:expr) => {
        extern "thiscall" fn $name(this: u32) -> f32 {
            f32::from_bits(dispatch($id, vec![this]))
        }
    };
}

macro_rules! virt_item0 {
    ($name:ident, $id:expr) => {
        extern "thiscall" fn $name(this: u32) -> u32 {
            dispatch($id, vec![this])
        }
    };
}

const ITEM_VID_SOURCE_HALF_WIDTH: u32 = 0x2001;
const ITEM_VID_SOURCE_CENTER_X: u32 = 0x2002;
const ITEM_VID_SOURCE_HALF_HEIGHT: u32 = 0x2003;
const ITEM_VID_SOURCE_CENTER_Y: u32 = 0x2004;
const ITEM_VID_CONTAINER_HALF_WIDTH: u32 = 0x2005;
const ITEM_VID_CONTAINER_CENTER_X: u32 = 0x2006;
const ITEM_VID_CONTAINER_HALF_HEIGHT: u32 = 0x2007;
const ITEM_VID_CONTAINER_CENTER_Y: u32 = 0x2008;
const ITEM_VID_SOURCE_TAG: u32 = 0x2009;
const ITEM_VID_SOURCE_SUBFLAG: u32 = 0x200A;
const ITEM_VID_OWN_FAIL: u32 = 0x200B;
const ITEM_VID_SOURCE_INDEX: u32 = 0x200C;

virt_f32!(v_item_source_half_width, ITEM_VID_SOURCE_HALF_WIDTH);
virt_f32!(v_item_source_center_x, ITEM_VID_SOURCE_CENTER_X);
virt_f32!(v_item_source_half_height, ITEM_VID_SOURCE_HALF_HEIGHT);
virt_f32!(v_item_source_center_y, ITEM_VID_SOURCE_CENTER_Y);
virt_f32!(v_item_container_half_width, ITEM_VID_CONTAINER_HALF_WIDTH);
virt_f32!(v_item_container_center_x, ITEM_VID_CONTAINER_CENTER_X);
virt_f32!(v_item_container_half_height, ITEM_VID_CONTAINER_HALF_HEIGHT);
virt_f32!(v_item_container_center_y, ITEM_VID_CONTAINER_CENTER_Y);
virt_item0!(v_item_source_tag, ITEM_VID_SOURCE_TAG);
virt_item0!(v_item_source_subflag, ITEM_VID_SOURCE_SUBFLAG);
virt_item0!(v_item_own_fail, ITEM_VID_OWN_FAIL);
virt_item0!(v_item_source_index, ITEM_VID_SOURCE_INDEX);

/// The item-commit method's virtual slots, for its isolated differential fixture.
#[derive(Debug, Clone, Copy)]
pub struct ItemVStubs {
    /// Source horizontal half-extent getter.
    pub source_half_width: u32,
    /// Source horizontal center getter.
    pub source_center_x: u32,
    /// Source vertical half-extent getter.
    pub source_half_height: u32,
    /// Source vertical center getter.
    pub source_center_y: u32,
    /// Container horizontal half-extent getter.
    pub container_half_width: u32,
    /// Container horizontal center getter.
    pub container_center_x: u32,
    /// Container vertical half-extent getter.
    pub container_half_height: u32,
    /// Container vertical center getter.
    pub container_center_y: u32,
    /// Source identity getter.
    pub source_tag: u32,
    /// Source notification guard getter.
    pub source_subflag: u32,
    /// Container failure entry.
    pub own_fail: u32,
    /// Source list-index getter.
    pub source_index: u32,
    /// Container dismiss entry.
    pub own_dismiss: u32,
    /// List sink getter.
    pub list_sink: u32,
    /// Inner-object completion getter.
    pub inner_done: u32,
}

impl ItemVStubs {
    /// Collects the item-commit slot stub addresses.
    #[must_use]
    pub fn new() -> Self {
        Self {
            source_half_width: v_item_source_half_width as *const () as usize as u32,
            source_center_x: v_item_source_center_x as *const () as usize as u32,
            source_half_height: v_item_source_half_height as *const () as usize as u32,
            source_center_y: v_item_source_center_y as *const () as usize as u32,
            container_half_width: v_item_container_half_width as *const () as usize as u32,
            container_center_x: v_item_container_center_x as *const () as usize as u32,
            container_half_height: v_item_container_half_height as *const () as usize as u32,
            container_center_y: v_item_container_center_y as *const () as usize as u32,
            source_tag: v_item_source_tag as *const () as usize as u32,
            source_subflag: v_item_source_subflag as *const () as usize as u32,
            own_fail: v_item_own_fail as *const () as usize as u32,
            source_index: v_item_source_index as *const () as usize as u32,
            own_dismiss: v_own_open as *const () as usize as u32,
            list_sink: v_list_rank as *const () as usize as u32,
            inner_done: v_prop_read as *const () as usize as u32,
        }
    }
}

impl Default for ItemVStubs {
    fn default() -> Self {
        Self::new()
    }
}

/// The item method's call mapping, kept separate from the dispatcher IDs.
#[cfg(target_arch = "x86")]
pub const ITEM_CALLS: &[CallSpec] = &[
    CallSpec { id: 1, name: "lookup_source" },
    CallSpec { id: 11, name: "item_tag_id" },
    CallSpec { id: 12, name: "notify" },
    CallSpec { id: 14, name: "gatekeeper" },
    CallSpec { id: 18, name: "post_index" },
    CallSpec { id: 19, name: "tail_notify" },
    CallSpec { id: 20, name: "measure" },
    CallSpec { id: 21, name: "build_configuration" },
    CallSpec { id: 22, name: "release_configuration" },
    CallSpec { id: 24, name: "listener_primary" },
    CallSpec { id: 25, name: "listener_secondary" },
    CallSpec { id: 27, name: "tail_forward" },
];

/// The item method's virtual slots, paired with the recorded operation names.
#[cfg(target_arch = "x86")]
pub const ITEM_VIRTUALS: &[CallSpec] = &[
    CallSpec { id: ITEM_VID_SOURCE_HALF_WIDTH, name: "geometry_source_half_width" },
    CallSpec { id: ITEM_VID_SOURCE_CENTER_X, name: "geometry_source_center_x" },
    CallSpec { id: ITEM_VID_SOURCE_HALF_HEIGHT, name: "geometry_source_half_height" },
    CallSpec { id: ITEM_VID_SOURCE_CENTER_Y, name: "geometry_source_center_y" },
    CallSpec { id: ITEM_VID_CONTAINER_HALF_WIDTH, name: "geometry_container_half_width" },
    CallSpec { id: ITEM_VID_CONTAINER_CENTER_X, name: "geometry_container_center_x" },
    CallSpec { id: ITEM_VID_CONTAINER_HALF_HEIGHT, name: "geometry_container_half_height" },
    CallSpec { id: ITEM_VID_CONTAINER_CENTER_Y, name: "geometry_container_center_y" },
    CallSpec { id: ITEM_VID_SOURCE_TAG, name: "source_tag" },
    CallSpec { id: ITEM_VID_SOURCE_SUBFLAG, name: "source_subflag" },
    CallSpec { id: ITEM_VID_OWN_FAIL, name: "own_fail" },
    CallSpec { id: ITEM_VID_SOURCE_INDEX, name: "source_index" },
    CallSpec { id: VID_OWN_OPEN, name: "own_dismiss" },
    CallSpec { id: VID_LIST_RANK, name: "list_sink" },
    CallSpec { id: VID_PROP_READ, name: "inner_done" },
];

/// The addresses of the virtual-slot stubs, for planting vtables.
#[derive(Debug, Clone, Copy)]
pub struct VStubs {
    /// The container's own open entry.
    pub own_open: u32,
    /// The container's own re-arm entry.
    pub own_rearm: u32,
    /// The session tag word.
    pub session_tag: u32,
    /// The worker session's first word.
    pub session_first: u32,
    /// The worker session's mode set.
    pub session_mode: u32,
    /// The clip list's element count.
    pub list_count: u32,
    /// The clip element at an index.
    pub list_at: u32,
    /// The list's mode set.
    pub list_mode: u32,
    /// The list rank of a key.
    pub list_rank: u32,
    /// The auxiliary object's mode set.
    pub aux_set: u32,
    /// The parameter sub-object's mode set.
    pub tail_set: u32,
    /// A probe opened on the inner sub-object.
    pub probe_open: u32,
    /// The property word of the inner sub-object.
    pub prop_read: u32,
    /// A probe's prime step.
    pub probe_prime: u32,
    /// A probe's mode set.
    pub probe_set: u32,
    /// The view's mode set.
    pub view_set: u32,
    /// The configuration sub-object's mode set.
    pub cfg_sub_set: u32,
}

impl VStubs {
    /// Collects the stub addresses.
    #[must_use]
    pub fn new() -> Self {
        Self {
            own_open: v_own_open as *const () as usize as u32,
            own_rearm: v_own_rearm as *const () as usize as u32,
            session_tag: v_session_tag as *const () as usize as u32,
            session_first: v_session_first as *const () as usize as u32,
            session_mode: v_session_mode as *const () as usize as u32,
            list_count: v_list_count as *const () as usize as u32,
            list_at: v_list_at as *const () as usize as u32,
            list_mode: v_list_mode as *const () as usize as u32,
            list_rank: v_list_rank as *const () as usize as u32,
            aux_set: v_aux_set as *const () as usize as u32,
            tail_set: v_tail_set as *const () as usize as u32,
            probe_open: v_probe_open as *const () as usize as u32,
            prop_read: v_prop_read as *const () as usize as u32,
            probe_prime: v_probe_prime as *const () as usize as u32,
            probe_set: v_probe_set as *const () as usize as u32,
            view_set: v_view_set as *const () as usize as u32,
            cfg_sub_set: v_cfg_sub_set as *const () as usize as u32,
        }
    }
}

impl Default for VStubs {
    fn default() -> Self {
        Self::new()
    }
}

/// The state pointer cell: the case points it at the state object before
/// the rewrite reads the flag word through it.
static STATE_PTR: AtomicU32 = AtomicU32::new(0);
static VIEWPORT_WIDTH: AtomicU32 = AtomicU32::new(0);
static VIEWPORT_HEIGHT: AtomicU32 = AtomicU32::new(0);
static VIEWPORT_WIDTH_SCALE: AtomicU32 = AtomicU32::new(0);
static VIEWPORT_HEIGHT_SCALE: AtomicU32 = AtomicU32::new(0);

/// Points the state cell at the case's state object.
pub fn set_state_ptr(ptr: u32) {
    STATE_PTR.store(ptr, Ordering::SeqCst);
}

/// Installs the viewport cells read by the item method's rewrite.
pub fn set_item_viewport(viewport: Viewport) {
    VIEWPORT_WIDTH.store(viewport.width as u32, Ordering::SeqCst);
    VIEWPORT_HEIGHT.store(viewport.height as u32, Ordering::SeqCst);
    VIEWPORT_WIDTH_SCALE.store(viewport.width_scale_bits, Ordering::SeqCst);
    VIEWPORT_HEIGHT_SCALE.store(viewport.height_scale_bits, Ordering::SeqCst);
}

/// File VA to relocated address. Only the constants the proof set reads
/// are mapped; they answer as themselves, since the cases compare them by
/// value.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x0198_1A4C | 0x0117_6888 | 0x019D_2F18 | 0x00EF_AD68 | 0x00EF_AD78 | 0x00EF_AD90
        | 0x00EF_AF04 | 0x00EF_AF14 | 0x00EF_AF24 | 0x00EF_AF44
        | 0x00EF_AF50 | 0x00EF_AF60 | 0x00EF_AF80 | 0x00EF_AF90 | 0x00EF_AF9C | 0x00EF_AFAC => {
            file_va
        }
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Pointer to the shared global at a file VA: the state pointer cell.
///
/// # Panics
///
/// For any VA the proof set does not map: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    match file_va {
        0x018B_6C8C => STATE_PTR.as_ptr().cast::<T>(),
        0x018B_7A80 => VIEWPORT_WIDTH.as_ptr().cast::<T>(),
        0x018B_7A8C => VIEWPORT_HEIGHT.as_ptr().cast::<T>(),
        0x017A_CCE8 => VIEWPORT_WIDTH_SCALE.as_ptr().cast::<T>(),
        0x017A_CCF0 => VIEWPORT_HEIGHT_SCALE.as_ptr().cast::<T>(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
