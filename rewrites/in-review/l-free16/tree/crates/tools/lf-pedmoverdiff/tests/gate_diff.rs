//! Differential tests: the lifted task request gate against its verified
//! 32-bit rewrite.
//!
//! The rewrite reads the task's vtable flag word, two global words planted
//! through the relocated table, the candidate's two words, and the request's
//! flags and mode. It asks one status callee on the task when the gate is
//! checked. Both sides run on the same images and the same answers; the
//! return value, the call log and every image must agree. Wrong versions run
//! through the same harness and must be caught.
#![cfg(target_arch = "x86")]

use std::sync::{Mutex, MutexGuard};

use lf_peds_tasks::ped_task::{GateCandidate, GateStatus, GateWords, admits};
use lf_pedmoverdiff::{rewrites, set_callee, set_relocated};

/// The file addresses of the two globals the rewrite reads through the
/// relocated table.
const GLOBAL_PRESENCE: u32 = 0x0116_0C68;
const GLOBAL_BUSY: u32 = 0x0105_C644;
/// The task's vtable flag word sits at this offset of its vtable.
const VT_FLAG_OFF: usize = 0x264;
const TASK_LEN: usize = 0x40;
const VTABLE_LEN: usize = 0x300;
const CAND_LEN: usize = 0x280;
const STATE_OFF: usize = 0x28;
const SUB_OFF: usize = 0x270;

static TEST_LOCK: Mutex<()> = Mutex::new(());
static RECORD: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static STATUS_ANSWER: Mutex<u8> = Mutex::new(0);

fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The status stub the rewrite calls through callee 1: records the task and
/// answers with the case's value.
extern "thiscall" fn stub_status(task: u32) -> u8 {
    RECORD.lock().unwrap().push(task);
    *STATUS_ANSWER.lock().unwrap()
}

/// The lifted side's probe: records the same task and gives the same answer.
struct Probe {
    calls: Vec<u32>,
    answer: u8,
}

impl GateStatus for Probe {
    type Handle = u32;

    fn status(&mut self, task: u32) -> u8 {
        self.calls.push(task);
        self.answer
    }
}

fn rd32(img: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([img[at], img[at + 1], img[at + 2], img[at + 3]])
}

fn wr32(img: &mut [u8], at: usize, value: u32) {
    img[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

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

/// The buffers one run uses. The task's first word points at its vtable.
struct Images {
    task: Vec<u8>,
    vtable: Vec<u8>,
    candidate: Vec<u8>,
    presence: Vec<u8>,
    busy: Vec<u8>,
}

/// One generated case.
#[derive(Clone)]
struct Case {
    vtable_flag: u32,
    presence: u32,
    busy: u32,
    state: u32,
    sub_flags: u32,
    flags: u32,
    mode: u32,
    status: u8,
    candidate_noise: Vec<u8>,
}

impl Images {
    fn new() -> Self {
        let mut images = Images {
            task: vec![0u8; TASK_LEN],
            vtable: vec![0u8; VTABLE_LEN],
            candidate: vec![0u8; CAND_LEN],
            presence: vec![0u8; 4],
            busy: vec![0u8; 4],
        };
        let vtable = images.vtable.as_ptr() as usize as u32;
        wr32(&mut images.task, 0, vtable);
        set_relocated(GLOBAL_PRESENCE, images.presence.as_ptr() as usize as u32);
        set_relocated(GLOBAL_BUSY, images.busy.as_ptr() as usize as u32);
        images
    }

    fn load(&mut self, case: &Case) {
        wr32(&mut self.vtable, VT_FLAG_OFF, case.vtable_flag);
        wr32(&mut self.presence, 0, case.presence);
        wr32(&mut self.busy, 0, case.busy);
        self.candidate.copy_from_slice(&case.candidate_noise);
        wr32(&mut self.candidate, STATE_OFF, case.state);
        wr32(&mut self.candidate, SUB_OFF, case.sub_flags);
    }
}

fn gen_case(rng: &mut Rng) -> Case {
    let mut noise = vec![0u8; CAND_LEN];
    for byte in noise.iter_mut() {
        *byte = rng.u32() as u8;
    }
    let random_busy = rng.u32();
    let random_mode = rng.u32();
    // The vtable flag bit is set in about half the cases.
    let vtable_flag = rng.u32();
    let presence = rng.pick(&[0u32, 1, 0x100, 0xffff_ffff]);
    // The busy word's high byte decides the second check.
    let busy = rng.pick(&[0u32, 0x0001_0000, 0x0100_0000, 0x00ff_ffff, random_busy]);
    // The busy state value is picked in about a third of the cases.
    let state = if rng.u32() % 3 == 0 {
        0xc0 | (rng.u32() & !0x3c0)
    } else {
        rng.u32()
    };
    let sub_flags = rng.u32();
    // The reject bit is set in about a quarter of the cases.
    let flags = if rng.u32() % 4 == 0 {
        rng.u32() | 0x1000
    } else {
        rng.u32() & !0x1000
    };
    let mode = rng.pick(&[0u32, 1, 0x100, 0x0000_0200, random_mode]);
    let status = rng.pick(&[0u8, 1, 0x80, 0xff]);
    Case {
        vtable_flag,
        presence,
        busy,
        state,
        sub_flags,
        flags,
        mode,
        status,
        candidate_noise: noise,
    }
}

/// Result of one differential run.
struct Report {
    trials: u32,
    mismatches: u32,
    first: Option<String>,
    probed: u32,
}

fn diff_run(
    seed: u32,
    trials: u32,
    lifted: fn(&mut Probe, u32, &GateWords, &GateCandidate, u32, u32) -> bool,
) -> Report {
    let mut rng = Rng(seed);
    let mut images = Images::new();
    let mut report = Report {
        trials,
        mismatches: 0,
        first: None,
        probed: 0,
    };
    for trial in 0..trials {
        let case = gen_case(&mut rng);
        *STATUS_ANSWER.lock().unwrap() = case.status;

        images.load(&case);
        let task = images.task.as_ptr() as usize as u32;
        let candidate = images.candidate.as_ptr() as usize as u32;
        RECORD.lock().unwrap().clear();
        let rw_ret = rewrites::fn_00A2B720::rw_00a2b720(task, candidate, case.flags, case.mode);
        let rw_calls = std::mem::take(&mut *RECORD.lock().unwrap());
        if !rw_calls.is_empty() {
            report.probed += 1;
        }
        let rw_image = (images.task.clone(), images.vtable.clone(), images.candidate.clone());

        images.load(&case);
        let words = GateWords {
            vtable_flag: rd32(&images.vtable, VT_FLAG_OFF),
            presence: rd32(&images.presence, 0),
            busy: rd32(&images.busy, 0),
        };
        let cand = GateCandidate {
            state: rd32(&images.candidate, STATE_OFF),
            sub_flags: rd32(&images.candidate, SUB_OFF),
        };
        let mut probe = Probe {
            calls: Vec::new(),
            answer: case.status,
        };
        let lf_ret = lifted(&mut probe, task, &words, &cand, case.flags, case.mode);
        let lf_image = (images.task.clone(), images.vtable.clone(), images.candidate.clone());

        let rw_bool = rw_ret != 0;
        let verdict = if rw_bool != lf_ret {
            Some("return differs")
        } else if rw_calls != probe.calls {
            Some("status calls differ")
        } else if rw_image != lf_image {
            Some("an image differs")
        } else {
            None
        };
        if let Some(what) = verdict {
            report.mismatches += 1;
            if report.first.is_none() {
                report.first = Some(format!(
                    "trial {trial}: {what}; rewrite {rw_ret} calls {rw_calls:?}; lifted {lf_ret} calls {:?}",
                    probe.calls
                ));
            }
        }
    }
    report
}

fn gate_lifted(
    probe: &mut Probe,
    task: u32,
    words: &GateWords,
    cand: &GateCandidate,
    flags: u32,
    mode: u32,
) -> bool {
    admits(probe, task, words, cand, flags, mode)
}

/// Wrong: the reject bit is ignored.
fn gate_wrong_reject(
    probe: &mut Probe,
    task: u32,
    words: &GateWords,
    cand: &GateCandidate,
    flags: u32,
    mode: u32,
) -> bool {
    admits(probe, task, words, cand, flags & !0x1000, mode)
}

/// Wrong: the status probe is not asked; the gate trusts the byte check alone.
fn gate_wrong_no_probe(
    probe: &mut Probe,
    task: u32,
    words: &GateWords,
    cand: &GateCandidate,
    flags: u32,
    mode: u32,
) -> bool {
    let _ = probe;
    let words = GateWords {
        vtable_flag: words.vtable_flag | 0x0080_0000,
        ..*words
    };
    admits(&mut Probe { calls: Vec::new(), answer: 1 }, task, &words, cand, flags, mode)
}

const TRIALS: u32 = 4000;

fn plant_probe() {
    set_callee(1, stub_status as *const () as usize as u32);
}

#[test]
fn task_request_gate_matches_rewrite() {
    let _guard = lock();
    plant_probe();
    let report = diff_run(0x6A7E_0001, TRIALS, gate_lifted);
    assert_eq!(report.mismatches, 0, "{:?}", report.first);
    assert_eq!(report.trials, TRIALS);
    assert!(report.probed > 0, "the probe was never asked");
}

#[test]
fn task_request_gate_wrong_reject_is_caught() {
    let _guard = lock();
    plant_probe();
    let report = diff_run(0x6A7E_0001, TRIALS, gate_wrong_reject);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}

#[test]
fn task_request_gate_wrong_probe_is_caught() {
    let _guard = lock();
    plant_probe();
    let report = diff_run(0x6A7E_0001, TRIALS, gate_wrong_no_probe);
    assert!(report.mismatches > 0, "the wrong version passed every case");
}
