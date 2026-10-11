//! Differential tests: the lifted seek record against its verified 32-bit
//! rewrite, on generated position blocks.
//!
//! The rewrite makes no outgoing calls. Its inputs are an owner object
//! whose sub-object holds the mover's position block, a target object
//! whose position block holds the goal, and the output record and mode
//! word. Both sides run on the same images; the record, the mode word and
//! every image must agree byte for byte, and the return value (the mover's
//! position block) too. Wrong versions run through the same harness and
//! must be caught.
#![cfg(target_arch = "x86")]

use std::sync::{Mutex, MutexGuard};

use lf_peds_tasks::ped_task::{SEEK_MODE, SeekGoal, SeekMover, SeekRecord};
use lf_pedmoverdiff::rewrites;

/// Image sizes. The owner's sub-object pointer is at `+0x18`; the sub-object's
/// position pointer is at `+0x20`; a position block holds x, y, z at
/// `+0x30/+0x34/+0x38` and a word at `+0x3c`. The record is 0x40 bytes.
const OBJ_LEN: usize = 0x40;
const SUB_LEN: usize = 0x40;
const BLOCK_LEN: usize = 0x40;
const OUT_LEN: usize = 0x40;
const SUB_PTR: usize = 0x18;
const POS_PTR: usize = 0x20;
const PX: usize = 0x30;
const PY: usize = 0x34;
const PZ: usize = 0x38;
const PW: usize = 0x3c;

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

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

/// The buffers one run uses. Their addresses are fixed for the program.
struct Images {
    obj: Vec<u8>,
    sub: Vec<u8>,
    mover: Vec<u8>,
    target: Vec<u8>,
    goal: Vec<u8>,
    out: Vec<u8>,
    flag: Vec<u8>,
}

impl Images {
    /// Allocates the images and links the owner, sub-object, position
    /// blocks and target: the pointers the rewrite follows.
    fn new() -> Self {
        let mut images = Images {
            obj: vec![0u8; OBJ_LEN],
            sub: vec![0u8; SUB_LEN],
            mover: vec![0u8; BLOCK_LEN],
            target: vec![0u8; OBJ_LEN],
            goal: vec![0u8; BLOCK_LEN],
            out: vec![0u8; OUT_LEN],
            flag: vec![0u8; 4],
        };
        let sub = images.sub.as_ptr() as usize as u32;
        wr32(&mut images.obj, SUB_PTR, sub);
        let mover = images.mover.as_ptr() as usize as u32;
        wr32(&mut images.sub, POS_PTR, mover);
        let goal = images.goal.as_ptr() as usize as u32;
        wr32(&mut images.target, POS_PTR, goal);
        images
    }

    /// Loads one case's position words. The record and the mode word start
    /// cleared, so a write the rewrite skips shows as a difference.
    fn load(&mut self, case: &Case) {
        self.mover.copy_from_slice(&case.mover);
        self.goal.copy_from_slice(&case.goal);
        self.out.iter_mut().for_each(|b| *b = 0);
        self.flag.iter_mut().for_each(|b| *b = 0);
    }
}

/// One generated case: the mover's position block and the goal block.
struct Case {
    mover: Vec<u8>,
    goal: Vec<u8>,
}

/// What one side left behind.
struct Outcome {
    out: Vec<u8>,
    flag: Vec<u8>,
    mover: Vec<u8>,
    goal: Vec<u8>,
    obj: Vec<u8>,
    sub: Vec<u8>,
    target: Vec<u8>,
    ret: u32,
}

impl Outcome {
    fn of(images: &Images, ret: u32) -> Self {
        Outcome {
            out: images.out.clone(),
            flag: images.flag.clone(),
            mover: images.mover.clone(),
            goal: images.goal.clone(),
            obj: images.obj.clone(),
            sub: images.sub.clone(),
            target: images.target.clone(),
            ret,
        }
    }
}

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

/// Edge positions: zeros, a value whose square overflows, one whose square
/// underflows, infinities and a quiet NaN.
const EDGE_POSITIONS: [f32; 10] = [
    0.0,
    -0.0,
    1.0,
    -1.0,
    1.0e30,
    -1.0e30,
    1.0e-30,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
];

fn position(rng: &mut Rng) -> f32 {
    match rng.u32() % 4 {
        0 => rng.pick(&EDGE_POSITIONS),
        1 => (rng.u32() % 64) as f32 * 0.25 - 8.0,
        _ => (rng.u32() % 200_000) as f32 / 1000.0 - 100.0,
    }
}

fn gen_case(rng: &mut Rng) -> Case {
    let mut mover = vec![0u8; BLOCK_LEN];
    for byte in mover.iter_mut() {
        *byte = rng.u32() as u8;
    }
    let mx = position(rng);
    let my = position(rng);
    wrf(&mut mover, PX, mx);
    wrf(&mut mover, PY, my);
    wrf(&mut mover, PZ, position(rng));

    let mut goal = vec![0u8; BLOCK_LEN];
    for byte in goal.iter_mut() {
        *byte = rng.u32() as u8;
    }
    // A goal equal to the mover is a quarter of the cases: the zero sum.
    let (gx, gy) = if rng.u32() % 4 == 0 {
        (mx, my)
    } else {
        (position(rng), position(rng))
    };
    wrf(&mut goal, PX, gx);
    wrf(&mut goal, PY, gy);
    Case { mover, goal }
}

/// Runs the rewrite on the case's images.
fn run_rewrite(images: &mut Images) -> u32 {
    let obj = images.obj.as_ptr() as usize as u32;
    let out = images.out.as_ptr() as usize as u32;
    let flag = images.flag.as_ptr() as usize as u32;
    let target = images.target.as_ptr() as usize as u32;
    rewrites::fn_00CFD810::rw_00cfd810(obj, out, flag, target)
}

/// The lifted side: reads the same words, builds the record and writes it
/// with the mode word, as the rewrite does.
fn run_lifted(images: &mut Images, build: fn(&SeekMover, &SeekGoal) -> SeekRecord) -> u32 {
    let mover = SeekMover {
        position: [rdf(&images.mover, PX), rdf(&images.mover, PY), rdf(&images.mover, PZ)],
        spare: rd32(&images.mover, PW),
    };
    let goal = SeekGoal {
        x: rdf(&images.goal, PX),
        y: rdf(&images.goal, PY),
    };
    let record = build(&mover, &goal);
    write_record(&mut images.out, &record);
    wr32(&mut images.flag, 0, SEEK_MODE);
    images.mover.as_ptr() as usize as u32
}

/// Writes a record in the rewrite's layout: sixteen words.
fn write_record(out: &mut [u8], record: &SeekRecord) {
    for (i, value) in record.at.iter().enumerate() {
        wrf(out, 4 * i, *value);
    }
    wr32(out, 0x0c, record.at_spare);
    for (i, value) in record.tuning_a.iter().enumerate() {
        wrf(out, 0x10 + 4 * i, *value);
    }
    for (i, value) in record.stepped.iter().enumerate() {
        wrf(out, 0x20 + 4 * i, *value);
    }
    wr32(out, 0x2c, record.frame_word);
    for (i, value) in record.tuning_b.iter().enumerate() {
        wrf(out, 0x30 + 4 * i, *value);
    }
}

/// Result of one differential run.
struct Report {
    trials: u32,
    mismatches: u32,
    first: Option<String>,
}

fn diff_run(
    seed: u32,
    trials: u32,
    lifted: fn(&SeekMover, &SeekGoal) -> SeekRecord,
) -> Report {
    let mut rng = Rng(seed);
    let mut images = Images::new();
    let mut report = Report {
        trials,
        mismatches: 0,
        first: None,
    };
    for trial in 0..trials {
        let case = gen_case(&mut rng);

        images.load(&case);
        let ret = run_rewrite(&mut images);
        let rewritten = Outcome::of(&images, ret);

        images.load(&case);
        let ret = run_lifted(&mut images, lifted);
        let lifted_outcome = Outcome::of(&images, ret);

        let verdict = if rewritten.out != lifted_outcome.out {
            Some("record differs")
        } else if rewritten.flag != lifted_outcome.flag {
            Some("mode word differs")
        } else if rewritten.mover != lifted_outcome.mover
            || rewritten.goal != lifted_outcome.goal
            || rewritten.obj != lifted_outcome.obj
            || rewritten.sub != lifted_outcome.sub
            || rewritten.target != lifted_outcome.target
        {
            Some("an input image differs")
        } else if rewritten.ret != lifted_outcome.ret {
            Some("return differs")
        } else {
            None
        };
        if let Some(what) = verdict {
            report.mismatches += 1;
            if report.first.is_none() {
                report.first = Some(format!(
                    "trial {trial}: {what}; rewrite record {:?}, lifted record {:?}",
                    rewritten.out, lifted_outcome.out
                ));
            }
        }
    }
    report
}

// The wrong versions. Each changes one decision of the routine.

/// Wrong: the step points away from the goal.
fn toward_wrong_sign(mover: &SeekMover, goal: &SeekGoal) -> SeekRecord {
    let [px, py, pz] = mover.position;
    let dx = goal.x - px;
    let dy = goal.y - py;
    let len2 = dx * dx + dy * dy;
    let inv = if len2 <= 0.0 { 0.0 } else { 1.0 / len2.sqrt() };
    let step_x = dx * inv * 8.0;
    let step_y = dy * inv * 8.0;
    let step_z = inv * 0.0 * 8.0;
    SeekRecord {
        at: [px, py, pz],
        at_spare: mover.spare,
        tuning_a: lf_peds_tasks::ped_task::TUNING,
        stepped: [px - step_x, py - step_y, pz - step_z],
        frame_word: 0,
        tuning_b: lf_peds_tasks::ped_task::TUNING,
    }
}

/// Wrong: a zero sum takes the division instead of the zero.
fn toward_wrong_threshold(mover: &SeekMover, goal: &SeekGoal) -> SeekRecord {
    let [px, py, pz] = mover.position;
    let dx = px - goal.x;
    let dy = py - goal.y;
    let len2 = dx * dx + dy * dy;
    let inv = if len2 < 0.0 { 0.0 } else { 1.0 / len2.sqrt() };
    let step_x = dx * inv * 8.0;
    let step_y = dy * inv * 8.0;
    let step_z = inv * 0.0 * 8.0;
    SeekRecord {
        at: [px, py, pz],
        at_spare: mover.spare,
        tuning_a: lf_peds_tasks::ped_task::TUNING,
        stepped: [px - step_x, py - step_y, pz - step_z],
        frame_word: 0,
        tuning_b: lf_peds_tasks::ped_task::TUNING,
    }
}

const TRIALS: u32 = 4000;

#[test]
fn seek_target_setup_matches_rewrite() {
    let _guard = lock();
    let report = diff_run(0x5EE0_0001, TRIALS, SeekRecord::toward_for_harness);
    assert_eq!(report.mismatches, 0, "{:?}", report.first);
    assert_eq!(report.trials, TRIALS);
}

#[test]
fn seek_target_setup_wrong_sign_is_caught() {
    let _guard = lock();
    let report = diff_run(0x5EE0_0001, TRIALS, toward_wrong_sign);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}

#[test]
fn seek_target_setup_wrong_threshold_is_caught() {
    let _guard = lock();
    let report = diff_run(0x5EE0_0001, TRIALS, toward_wrong_threshold);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}
