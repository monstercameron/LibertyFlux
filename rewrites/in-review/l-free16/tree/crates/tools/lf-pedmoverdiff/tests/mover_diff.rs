//! Differential tests: the lifted mover pose against its verified 32-bit
//! rewrites, on the same generated images.
//!
//! Each case loads the same images into four buffers and runs the rewrite
//! on their addresses, recording every stub call it makes. The buffers are
//! then reloaded and the lifted method runs on the same addresses, with a
//! recording stand-in for the family's callees and virtual methods. The
//! two sides must agree on the images byte for byte, on the calls in order
//! with their arguments, and on the return value.
//!
//! Each program holds one lock for its whole test, so the stubs and their
//! recorder never interleave. The wrong versions are run through the same
//! harness and must be caught.
#![cfg(target_arch = "x86")]

use std::sync::{Mutex, MutexGuard};

use lf_peds_tasks::ped_task::{BlendState, MoverCallees, MoverPose, NOT_READY, SNAP_FLAG};
use lf_pedmoverdiff::{rewrites, set_callee};

/// Image sizes. The pose sits at offset `0x70` (through `0x82`); the state
/// block's highest field is at `0x1f68` (a float, four bytes).
const TASK_LEN: usize = 0x200;
const STATE_LEN: usize = 0x2000;
const VTABLE_LEN: usize = 0x200;

// Offsets of the fields the family touches, as in the 32-bit layouts.
const POS: usize = 0x70;
const HEADING: usize = 0x7c;
const MODE: usize = 0x80;
const FLAG: usize = 0x81;
const LEVEL: usize = 0x82;
const ST_FLAGS: usize = 0x24;
const ST_X: usize = 0x1ed4;
const ST_Y: usize = 0x1ec0;
const ST_H: usize = 0x1ed8;
const ST_H_MIRROR: usize = 0x1edc;
const ST_F: [usize; 3] = [0x1ef4, 0x1ef8, 0x1efc];
const ST_TOP: usize = 0xd14;
const ST_FLAG: usize = 0xd4f;
const ST_LEVEL: usize = 0x1f68;
/// The state block's virtual-table slots: the height setter and getter.
const SLOT_SET_HEIGHT: usize = 0xf4;
const SLOT_HEIGHT: usize = 0xfc;

/// Event kinds, shared by the stubs and the lifted-side recorder.
const SETUP: u8 = 1;
const READY: u8 = 2;
const BLEND: u8 = 3;
const ENCODE: u8 = 4;
const FLAG_NOTIFY: u8 = 5;
const STATE_NOTIFY: u8 = 6;
const DECODE: u8 = 7;
const SET_HEIGHT: u8 = 8;
const HEIGHT: u8 = 9;

/// One recorded call: its kind and its argument words, in order.
type Event = (u8, Vec<u32>);

/// Answers the stand-ins give, planted per case.
#[derive(Clone, Copy)]
struct Script {
    /// The readiness probe's answer.
    ready: u32,
    /// The height getter's answer.
    height: f32,
}

/// The encode stand-in's answers, indexed by a two-bit code.
const ENCODE_ANSWERS: [f32; 4] = [0.25, 1.0, -2.0, 4.5];

static TEST_LOCK: Mutex<()> = Mutex::new(());
static RECORD: Mutex<Vec<Event>> = Mutex::new(Vec::new());
static SCRIPT: Mutex<Script> = Mutex::new(Script { ready: 0, height: 0.0 });

fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn script() -> Script {
    *SCRIPT.lock().unwrap()
}

fn set_script(script: Script) {
    *SCRIPT.lock().unwrap() = script;
}

fn push(kind: u8, words: &[u32]) {
    RECORD.lock().unwrap().push((kind, words.to_vec()));
}

fn encode_answer(code: u32) -> f32 {
    ENCODE_ANSWERS[(code & 3) as usize]
}

fn decode_answer(bits: u32) -> u32 {
    bits.rotate_left(5) ^ 0x9E37_79B9
}

// The stubs the rewrites call. Each records its kind and arguments, then
// answers as the script or the fixed tables say.

extern "thiscall" fn stub_setup(this: u32, a: u32, b: u32, c: u32) -> u32 {
    push(SETUP, &[this, a, b, c]);
    0
}

extern "thiscall" fn stub_ready(this: u32) -> u32 {
    push(READY, &[this]);
    script().ready
}

extern "thiscall" fn stub_blend(this: u32, a: u32, b: u32, c: u32) -> u32 {
    push(BLEND, &[this, a, b, c]);
    0
}

extern "thiscall" fn stub_encode(this: u32, code: u32) -> f32 {
    push(ENCODE, &[this, code]);
    encode_answer(code)
}

extern "thiscall" fn stub_flag(this: u32, arg: u32) -> u32 {
    push(FLAG_NOTIFY, &[this, arg]);
    0
}

extern "thiscall" fn stub_state(this: u32, state: u32) -> u32 {
    push(STATE_NOTIFY, &[this, state]);
    0
}

extern "thiscall" fn stub_decode(this: u32, bits: u32) -> u32 {
    push(DECODE, &[this, bits]);
    decode_answer(bits)
}

extern "thiscall" fn stub_set_height(this: u32, z: u32, pad: u32) -> u32 {
    push(SET_HEIGHT, &[this, z, pad]);
    0
}

extern "thiscall" fn stub_height(this: u32) -> f32 {
    push(HEIGHT, &[this]);
    script().height
}

/// The lifted side's stand-in: records the same events the stubs record.
struct Recorder {
    events: Vec<Event>,
}

impl Recorder {
    fn log(&mut self, kind: u8, words: &[u32]) {
        self.events.push((kind, words.to_vec()));
    }
}

impl MoverCallees for Recorder {
    type Handle = u32;

    fn setup(&mut self, task: u32, state: u32, target: Option<u32>, t: f32) {
        self.log(SETUP, &[task, state, target.unwrap_or(0), t.to_bits()]);
    }

    fn ready(&mut self, task: u32) -> u32 {
        self.log(READY, &[task]);
        script().ready
    }

    fn blend_into(&mut self, task: u32, state: u32, target: u32, t: f32) {
        self.log(BLEND, &[task, state, target, t.to_bits()]);
    }

    fn encode(&mut self, task: u32, code: u32) -> f32 {
        self.log(ENCODE, &[task, code]);
        encode_answer(code)
    }

    fn notify_flag(&mut self, task: u32, arg: u32) {
        self.log(FLAG_NOTIFY, &[task, arg]);
    }

    fn notify_state(&mut self, task: u32, state: u32) {
        self.log(STATE_NOTIFY, &[task, state]);
    }

    fn decode(&mut self, task: u32, bits: u32) -> u32 {
        self.log(DECODE, &[task, bits]);
        decode_answer(bits)
    }

    fn set_height(&mut self, state: u32, z: f32) {
        // The rewrite's trailing padding word is zero; the proof checks it on
        // the rewrite side, so the lifted side records zero.
        self.log(SET_HEIGHT, &[state, z.to_bits(), 0]);
    }

    fn height(&mut self, state: u32) -> f32 {
        self.log(HEIGHT, &[state]);
        script().height
    }
}

// Images.

fn rd32(img: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([img[at], img[at + 1], img[at + 2], img[at + 3]])
}

fn wr32(img: &mut [u8], at: usize, value: u32) {
    img[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn rdf(img: &[u8], at: usize) -> f32 {
    f32::from_bits(rd32(img, at))
}

fn wrf(img: &mut [u8], at: usize, value: f32) {
    wr32(img, at, value.to_bits());
}

fn read_pose(img: &[u8]) -> MoverPose {
    MoverPose {
        pos: [rdf(img, POS), rdf(img, POS + 4), rdf(img, POS + 8)],
        heading: rdf(img, HEADING),
        mode: img[MODE],
        flag: img[FLAG],
        level: img[LEVEL],
    }
}

fn write_pose(img: &mut [u8], pose: &MoverPose) {
    wrf(img, POS, pose.pos[0]);
    wrf(img, POS + 4, pose.pos[1]);
    wrf(img, POS + 8, pose.pos[2]);
    wrf(img, HEADING, pose.heading);
    img[MODE] = pose.mode;
    img[FLAG] = pose.flag;
    img[LEVEL] = pose.level;
}

fn read_state(img: &[u8]) -> BlendState {
    BlendState {
        x: rdf(img, ST_X),
        y: rdf(img, ST_Y),
        heading: rdf(img, ST_H),
        heading_mirror: rdf(img, ST_H_MIRROR),
        factors: [rdf(img, ST_F[0]), rdf(img, ST_F[1]), rdf(img, ST_F[2])],
        top: img[ST_TOP],
        flag: img[ST_FLAG],
        level: rdf(img, ST_LEVEL),
        flags: rd32(img, ST_FLAGS),
    }
}

fn write_state(img: &mut [u8], state: &BlendState) {
    wrf(img, ST_X, state.x);
    wrf(img, ST_Y, state.y);
    wrf(img, ST_H, state.heading);
    wrf(img, ST_H_MIRROR, state.heading_mirror);
    for (i, factor) in state.factors.iter().enumerate() {
        wrf(img, ST_F[i], *factor);
    }
    img[ST_TOP] = state.top;
    img[ST_FLAG] = state.flag;
    wrf(img, ST_LEVEL, state.level);
    wr32(img, ST_FLAGS, state.flags);
}

/// The buffers one case runs in. Their addresses are fixed for the whole
/// program, so both sides see the same handles.
struct Bufs {
    task: Vec<u8>,
    state: Vec<u8>,
    src: Vec<u8>,
    vtable: Vec<u8>,
}

/// One generated case: the starting image of each buffer and the inputs.
#[derive(Clone)]
struct Case {
    task: Vec<u8>,
    state: Vec<u8>,
    src: Vec<u8>,
    t: u32,
    target_null: bool,
    script: Script,
}

/// The addresses and scalar inputs a case hands to either side.
struct Addrs {
    task: u32,
    state: u32,
    src: u32,
    target: u32,
    t: u32,
}

impl Bufs {
    fn new() -> Self {
        let mut vtable = vec![0u8; VTABLE_LEN];
        wr32(&mut vtable, SLOT_SET_HEIGHT, stub_set_height as usize as u32);
        wr32(&mut vtable, SLOT_HEIGHT, stub_height as usize as u32);
        Bufs {
            task: vec![0u8; TASK_LEN],
            state: vec![0u8; STATE_LEN],
            src: vec![0u8; TASK_LEN],
            vtable,
        }
    }

    /// Loads a case's images. The state block's first word is its virtual
    /// table, which is this program's table.
    fn load(&mut self, case: &Case) {
        self.task.copy_from_slice(&case.task);
        self.src.copy_from_slice(&case.src);
        self.state.copy_from_slice(&case.state);
        let table = self.vtable.as_ptr() as usize as u32;
        wr32(&mut self.state, 0, table);
    }

    fn addrs(&self, case: &Case) -> Addrs {
        let src = self.src.as_ptr() as usize as u32;
        Addrs {
            task: self.task.as_ptr() as usize as u32,
            state: self.state.as_ptr() as usize as u32,
            src,
            target: if case.target_null { 0 } else { src },
            t: case.t,
        }
    }
}

/// What one side produced for a case.
struct Outcome {
    task: Vec<u8>,
    state: Vec<u8>,
    src: Vec<u8>,
    events: Vec<Event>,
    ret: u32,
}

impl Outcome {
    fn of(bufs: &Bufs, events: Vec<Event>, ret: u32) -> Self {
        Outcome {
            task: bufs.task.clone(),
            state: bufs.state.clone(),
            src: bufs.src.clone(),
            events,
            ret,
        }
    }
}

// Generation. Every float the generator makes is a quiet NaN or a number,
// so a value returned through the x87 stack is not altered by the convention.

/// Small deterministic generator (xorshift32).
struct Rng(u32);

impl Rng {
    fn u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn pick<T: Copy>(&mut self, options: &[T]) -> T {
        options[(self.u32() as usize) % options.len()]
    }
}

/// Any float bit pattern, with signalling NaNs made quiet.
fn any_float(rng: &mut Rng) -> f32 {
    let mut bits = rng.u32();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits |= 0x0040_0000;
    }
    f32::from_bits(bits)
}

/// Edge values for pose and target floats: zeros, the half and full turns
/// (the heading wrap), infinities and quiet NaNs.
const EDGE_FLOATS: [f32; 13] = [
    0.0,
    -0.0,
    1.0,
    -1.0,
    0.5,
    core::f32::consts::PI,
    -core::f32::consts::PI,
    core::f32::consts::TAU,
    -core::f32::consts::TAU,
    3.0,
    -3.0,
    f32::INFINITY,
    f32::NAN,
];

fn pose_float(rng: &mut Rng) -> f32 {
    match rng.u32() % 4 {
        0 => any_float(rng),
        1 => (rng.u32() % 64) as f32 * 0.25 - 8.0,
        2 => rng.pick(&EDGE_FLOATS),
        _ => (rng.u32() % 200_000) as f32 / 1000.0 - 100.0,
    }
}

/// The factor a drive step sees as `t`: zero, one, a half, negative zero,
/// a quiet NaN, an infinity, or any bits.
const EDGE_T: [u32; 9] = [
    0x0000_0000,
    0x8000_0000,
    0x3f80_0000,
    0x3f00_0000,
    0x4000_0000,
    0xbf80_0000,
    0x7fc0_0000,
    0x7f80_0000,
    0x3e4c_cccd,
];

/// A readiness answer: the snap answer, a blend answer, or any word.
const READY_ANSWERS: [u32; 5] = [0, 1, 2, NOT_READY, 0xffff_ffff];

fn fill_pose(rng: &mut Rng, img: &mut [u8]) {
    for byte in img.iter_mut() {
        *byte = rng.u32() as u8;
    }
    let pose = MoverPose {
        pos: [pose_float(rng), pose_float(rng), pose_float(rng)],
        heading: pose_float(rng),
        mode: rng.u32() as u8,
        flag: rng.u32() as u8,
        level: rng.u32() as u8,
    };
    write_pose(img, &pose);
}

fn gen_case(rng: &mut Rng) -> Case {
    let mut task = vec![0u8; TASK_LEN];
    fill_pose(rng, &mut task);
    let mut src = vec![0u8; TASK_LEN];
    fill_pose(rng, &mut src);

    let mut state = vec![0u8; STATE_LEN];
    for byte in state.iter_mut() {
        *byte = rng.u32() as u8;
    }
    // The snap bit is set in about a third of the cases.
    let flags = if rng.u32() % 3 == 0 {
        rng.u32() | SNAP_FLAG
    } else {
        rng.u32() & !SNAP_FLAG
    };
    wr32(&mut state, ST_FLAGS, flags);
    let block = BlendState {
        x: pose_float(rng),
        y: pose_float(rng),
        heading: pose_float(rng),
        heading_mirror: pose_float(rng),
        factors: [any_float(rng), any_float(rng), any_float(rng)],
        top: rng.u32() as u8,
        flag: rng.u32() as u8,
        level: any_float(rng),
        flags,
    };
    write_state(&mut state, &block);

    Case {
        task,
        state,
        src,
        t: if rng.u32() % 2 == 0 {
            rng.pick(&EDGE_T)
        } else {
            any_float(rng).to_bits()
        },
        target_null: rng.u32() % 4 == 0,
        script: Script {
            ready: rng.pick(&READY_ANSWERS),
            height: any_float(rng),
        },
    }
}

/// Result of one differential run.
struct Report {
    trials: u32,
    mismatches: u32,
    first: Option<String>,
}

/// Runs `trials` generated cases through the rewrite and the lifted side.
fn diff_run(
    seed: u32,
    trials: u32,
    rewrite: impl Fn(&Addrs) -> u32,
    lifted: impl Fn(&mut Recorder, &mut Bufs, &Addrs),
) -> Report {
    let mut rng = Rng(seed);
    let mut bufs = Bufs::new();
    let mut report = Report {
        trials,
        mismatches: 0,
        first: None,
    };
    for trial in 0..trials {
        let case = gen_case(&mut rng);
        set_script(case.script);

        bufs.load(&case);
        let addrs = bufs.addrs(&case);
        RECORD.lock().unwrap().clear();
        let ret = rewrite(&addrs);
        let rewritten_events = std::mem::take(&mut *RECORD.lock().unwrap());
        let rewritten = Outcome::of(&bufs, rewritten_events, ret);

        bufs.load(&case);
        let mut recorder = Recorder { events: Vec::new() };
        lifted(&mut recorder, &mut bufs, &addrs);
        let lifted_outcome = Outcome::of(&bufs, recorder.events, 0);

        // The rewrite's padding word on the height setter is zero.
        for (kind, words) in &rewritten.events {
            if *kind == SET_HEIGHT && words[2] != 0 {
                panic!("trial {trial}: rewrite passes a non-zero padding word");
            }
        }

        let verdict = if rewritten.task != lifted_outcome.task {
            Some("task image differs")
        } else if rewritten.state != lifted_outcome.state {
            Some("state image differs")
        } else if rewritten.src != lifted_outcome.src {
            Some("target image differs")
        } else if rewritten.events != lifted_outcome.events {
            Some("calls differ")
        } else if rewritten.ret != lifted_outcome.ret {
            Some("return differs")
        } else {
            None
        };
        if let Some(what) = verdict {
            report.mismatches += 1;
            if report.first.is_none() {
                report.first = Some(format!(
                    "trial {trial}: {what}; rewrite calls {:?}, lifted calls {:?}",
                    rewritten.events, lifted_outcome.events
                ));
            }
        }
    }
    report
}

// The lifted methods, each adapted to the runner.

fn lerp_lifted(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let pose = read_pose(&bufs.task);
    let src = read_pose(&bufs.src);
    let mut state = read_state(&bufs.state);
    pose.lerp_into(rec, a.state, &mut state, &src, f32::from_bits(a.t));
    write_state(&mut bufs.state, &state);
}

fn drive_lifted(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let pose = read_pose(&bufs.task);
    let mut state = read_state(&bufs.state);
    let target = if a.target == 0 { None } else { Some(a.target) };
    pose.drive_blend(rec, a.task, a.state, &mut state, target, f32::from_bits(a.t));
    write_state(&mut bufs.state, &state);
}

fn commit_lifted(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let state = read_state(&bufs.state);
    let mut pose = read_pose(&bufs.task);
    pose.commit_blend(rec, a.task, a.state, &state);
    write_pose(&mut bufs.task, &pose);
}

// The wrong versions. Each reads or passes one field wrongly; the harness
// must catch every one.

/// Wrong: the heading mirror keeps its old value.
fn lerp_wrong(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let pose = read_pose(&bufs.task);
    let src = read_pose(&bufs.src);
    let mut state = read_state(&bufs.state);
    let kept = state.heading_mirror;
    pose.lerp_into(rec, a.state, &mut state, &src, f32::from_bits(a.t));
    state.heading_mirror = kept;
    write_state(&mut bufs.state, &state);
}

/// Wrong: the snap bit is read inverted.
fn drive_wrong(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let pose = read_pose(&bufs.task);
    let mut state = read_state(&bufs.state);
    let target = if a.target == 0 { None } else { Some(a.target) };
    state.flags ^= SNAP_FLAG;
    pose.drive_blend(rec, a.task, a.state, &mut state, target, f32::from_bits(a.t));
    state.flags ^= SNAP_FLAG;
    write_state(&mut bufs.state, &state);
}

/// Wrong: the top field of the packed mode is read from the wrong bit.
fn commit_wrong(rec: &mut Recorder, bufs: &mut Bufs, a: &Addrs) {
    let mut state = read_state(&bufs.state);
    let mut pose = read_pose(&bufs.task);
    state.top ^= 1;
    pose.commit_blend(rec, a.task, a.state, &state);
    write_pose(&mut bufs.task, &pose);
}

// The rewrites, adapted to the runner.

fn lerp_rewrite(a: &Addrs) -> u32 {
    rewrites::fn_00BEE530::rw_00bee530(a.task, a.state, a.src, a.t)
}

fn drive_rewrite(a: &Addrs) -> u32 {
    rewrites::fn_00BEE3F0::rw_00bee3f0(a.task, a.state, a.target, a.t)
}

fn commit_rewrite(a: &Addrs) -> u32 {
    rewrites::fn_00BEE6A0::rw_00bee6a0(a.task, a.state)
}

const TRIALS: u32 = 4000;

fn plant_drive_callees() {
    set_callee(1, stub_setup as usize as u32);
    set_callee(2, stub_ready as usize as u32);
    set_callee(4, stub_blend as usize as u32);
    set_callee(5, stub_encode as usize as u32);
}

fn plant_commit_callees() {
    set_callee(1, stub_flag as usize as u32);
    set_callee(2, stub_state as usize as u32);
    set_callee(4, stub_decode as usize as u32);
}

#[test]
fn lerp_position_heading_matches_rewrite() {
    let _guard = lock();
    let report = diff_run(0x0A11_CE01, TRIALS, lerp_rewrite, lerp_lifted);
    assert_eq!(report.mismatches, 0, "{:?}", report.first);
    assert_eq!(report.trials, TRIALS);
}

#[test]
fn lerp_position_heading_wrong_version_is_caught() {
    let _guard = lock();
    let report = diff_run(0x0A11_CE01, TRIALS, lerp_rewrite, lerp_wrong);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}

#[test]
fn drive_blend_matches_rewrite() {
    let _guard = lock();
    plant_drive_callees();
    let report = diff_run(0x0D41_7E02, TRIALS, drive_rewrite, drive_lifted);
    assert_eq!(report.mismatches, 0, "{:?}", report.first);
    assert_eq!(report.trials, TRIALS);
}

#[test]
fn drive_blend_wrong_version_is_caught() {
    let _guard = lock();
    plant_drive_callees();
    let report = diff_run(0x0D41_7E02, TRIALS, drive_rewrite, drive_wrong);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}

#[test]
fn commit_blend_matches_rewrite() {
    let _guard = lock();
    plant_commit_callees();
    let report = diff_run(0x0C01_4417, TRIALS, commit_rewrite, commit_lifted);
    assert_eq!(report.mismatches, 0, "{:?}", report.first);
    assert_eq!(report.trials, TRIALS);
}

#[test]
fn commit_blend_wrong_version_is_caught() {
    let _guard = lock();
    plant_commit_callees();
    let report = diff_run(0x0C01_4417, TRIALS, commit_rewrite, commit_wrong);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}
