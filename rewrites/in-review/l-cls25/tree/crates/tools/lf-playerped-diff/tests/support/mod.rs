//! Shared differential machinery: the pedestrian image codec, the test
//! world the lifted methods call through, the case runner and the
//! comparison.
//!
//! A case has one initial image of the pedestrian. The rewrite runs on its
//! own copy of that image, with the pedestrian's identity set to the copy's
//! address; the lifted method runs on a native value decoded from the other
//! copy, and the native value is encoded back afterwards. Both runs see the
//! same scripted answers, the same globals and the same effect, and both
//! log every collaborator call in order. The outcome compares the result,
//! the call logs and the whole image.

#![allow(dead_code)]

use lf_core::Handle32;
use lf_playerped_diff::rt::{self, Call, Effect, Kind};
use lf_world::player_ped::offsets;
use lf_world::player_ped::{CPlayerPed, ObjRef, ObjTag, PedTag, PlayerPedWorld};

/// Words in a pedestrian image: four kilobytes, past the last field the
/// rewrites touch.
pub const IMAGE_WORDS: usize = 0x400;

/// Deterministic generator (xorshift64): the same seed gives the same cases.
pub struct Rng(u64);

impl Rng {
    /// A generator from a non-zero seed.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    /// The next 32 bits.
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 32) as u32
    }

    /// A value below `n`.
    pub fn below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }

    /// A fair coin.
    pub fn coin(&mut self) -> bool {
        self.next_u32() & 1 == 1
    }
}

/// Byte write into an image of words.
pub fn put8(img: &mut [u32], off: u32, v: u8) {
    let i = off as usize;
    let shift = (i % 4) * 8;
    let w = i / 4;
    img[w] = (img[w] & !(0xFF << shift)) | (u32::from(v) << shift);
}

/// Byte read from an image of words.
#[must_use]
pub fn get8(img: &[u32], off: u32) -> u8 {
    let i = off as usize;
    (img[i / 4] >> ((i % 4) * 8)) as u8
}

/// Half-word write (little-endian).
pub fn put16(img: &mut [u32], off: u32, v: u16) {
    put8(img, off, v as u8);
    put8(img, off + 1, (v >> 8) as u8);
}

/// Half-word read (little-endian).
#[must_use]
pub fn get16(img: &[u32], off: u32) -> u16 {
    u16::from(get8(img, off)) | (u16::from(get8(img, off + 1)) << 8)
}

/// Word write (little-endian).
pub fn put32(img: &mut [u32], off: u32, v: u32) {
    for k in 0..4 {
        put8(img, off + k, (v >> (8 * k)) as u8);
    }
}

/// Word read (little-endian).
#[must_use]
pub fn get32(img: &[u32], off: u32) -> u32 {
    (0..4).fold(0u32, |acc, k| acc | (u32::from(get8(img, off + k)) << (8 * k)))
}

/// Writes every field the native pedestrian owns into the image.
pub fn encode(ped: &CPlayerPed, img: &mut [u32]) {
    put32(img, offsets::CURRENT_SLOT, ped.current_slot);
    put32(img, offsets::SETUP_RESULT, ped.setup_result);
    put32(img, offsets::LINK, Handle32::raw_or_zero(ped.link));
    put32(img, offsets::SUB_A, Handle32::raw_or_zero(ped.sub_a));
    put32(img, offsets::SUB_B, Handle32::raw_or_zero(ped.sub_b));
    put16(img, offsets::MODE, ped.mode);
    put32(img, offsets::MOTION_VALUE, ped.motion_value.to_bits());
    for k in 0..2u32 {
        put32(img, offsets::PRE_WORDS + 4 * k, ped.pre_words[k as usize]);
        put32(img, offsets::APPLY_WORDS + 4 * k, ped.apply_words[k as usize]);
    }
    for k in 0..4u32 {
        put32(img, offsets::DESCRIPTOR + 4 * k, ped.descriptor[k as usize]);
        put32(img, offsets::TRANSFORM + 4 * k, ped.transform[k as usize]);
    }
    put32(img, offsets::JOB_WORD, ped.job_word);
}

/// Reads every field the native pedestrian owns from the image.
#[must_use]
pub fn decode(img: &[u32]) -> CPlayerPed {
    let obj = |off: u32| -> ObjRef { Handle32::<ObjTag>::new(get32(img, off)) };
    let mut ped = CPlayerPed {
        current_slot: get32(img, offsets::CURRENT_SLOT),
        setup_result: get32(img, offsets::SETUP_RESULT),
        link: obj(offsets::LINK),
        sub_a: obj(offsets::SUB_A),
        sub_b: obj(offsets::SUB_B),
        mode: get16(img, offsets::MODE),
        motion_value: f32::from_bits(get32(img, offsets::MOTION_VALUE)),
        job_word: get32(img, offsets::JOB_WORD),
        ..CPlayerPed::default()
    };
    for k in 0..2u32 {
        ped.pre_words[k as usize] = get32(img, offsets::PRE_WORDS + 4 * k);
        ped.apply_words[k as usize] = get32(img, offsets::APPLY_WORDS + 4 * k);
    }
    for k in 0..4u32 {
        ped.descriptor[k as usize] = get32(img, offsets::DESCRIPTOR + 4 * k);
        ped.transform[k as usize] = get32(img, offsets::TRANSFORM + 4 * k);
    }
    ped
}

/// Applies a scripted effect to the native pedestrian: encodes it, writes
/// the effect's word, and decodes it back.
fn apply_effect(ped: &mut CPlayerPed, e: Effect) {
    let mut scratch = vec![0u32; IMAGE_WORDS];
    encode(ped, &mut scratch);
    put32(&mut scratch, e.off, e.value);
    *ped = decode(&scratch);
}

/// The test world: every collaborator call is logged through the runtime
/// and answered from the script, and the scripted effect of a callee is
/// applied to the native pedestrian.
pub struct TestWorld;

impl TestWorld {
    fn call(ped: &mut CPlayerPed, id: u32, args: Vec<u32>) -> u32 {
        let answer = rt::script_call(id, args);
        if let Some(e) = rt::effect_for(id) {
            apply_effect(ped, e);
        }
        answer
    }
}

impl PlayerPedWorld for TestWorld {
    fn ped_ready_check(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        Self::call(ped, 1, vec![this.get()])
    }

    fn ped_job_a(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32 {
        Self::call(ped, 2, vec![this.get(), biased, job])
    }

    fn ped_job_b(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32 {
        Self::call(ped, 3, vec![this.get(), biased, job])
    }

    fn trigger_entry(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        first: u32,
        second: u32,
        third: u32,
    ) -> u32 {
        Self::call(ped, 2, vec![this.get(), first, second, third])
    }

    fn base_update(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        Self::call(ped, 3, vec![this.get()])
    }

    fn motion_helper(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, bits: u32) -> u32 {
        Self::call(ped, 1, vec![this.get(), bits])
    }

    fn pre_step(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32 {
        Self::call(ped, 1, vec![this.get(), arg])
    }

    fn apply_step(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32 {
        Self::call(ped, 2, vec![this.get(), arg])
    }

    fn notify_object(
        &mut self,
        ped: &mut CPlayerPed,
        obj: ObjRef,
        first: u32,
        second: u32,
    ) -> u32 {
        Self::call(ped, 3, vec![Handle32::raw_or_zero(obj), first, second])
    }

    fn dispatch_slot8(&mut self, ped: &mut CPlayerPed, obj: Handle32<ObjTag>, arg: u32) -> u32 {
        Self::call(ped, rt::SLOT8_ID, vec![obj.get(), arg])
    }

    fn player_context(&mut self) -> u32 {
        rt::player_context()
    }

    fn lookup_context(&mut self, context: u32) -> u32 {
        rt::script_call(1, vec![context])
    }

    fn setup(
        &mut self,
        ped: &mut CPlayerPed,
        lookup: u32,
        this: Handle32<PedTag>,
        flag: u32,
        first: u32,
        second: u32,
        third: u32,
        tail: u32,
    ) -> u32 {
        Self::call(ped, 2, vec![lookup, this.get(), flag, first, second, third, tail])
    }

    fn finalize(&mut self, ped: &mut CPlayerPed, target: u32, arg: u32) -> u32 {
        Self::call(ped, 3, vec![target, arg])
    }

    fn model_state(&mut self, model_id: u32) -> u8 {
        rt::model_state_byte(model_id)
    }

    fn lookup_model(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, model_id: u32) -> u32 {
        Self::call(ped, 1, vec![this.get(), model_id])
    }

    fn preload_model(&mut self, model_id: u32) -> u32 {
        rt::script_call(2, vec![model_id])
    }

    fn revive(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, flag: u32) -> u32 {
        Self::call(ped, 3, vec![this.get(), flag])
    }

    fn load_bundle(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, bundle: u32) -> u32 {
        Self::call(ped, 4, vec![this.get(), bundle])
    }
}

/// What one run leaves behind: the result, the collaborator calls in order,
/// and the whole pedestrian image.
pub struct Outcome {
    pub ret: u32,
    pub calls: Vec<Call>,
    pub image: Vec<u32>,
}

/// One generated case: the initial image, the callee shapes, the scripted
/// answers, the optional effect, the globals, and the memory the addresses
/// in the image point at.
pub struct Case {
    pub init: Vec<u32>,
    pub kinds: Vec<(u32, Kind)>,
    pub answers: Vec<(u32, Vec<u32>)>,
    pub effect: Option<Effect>,
    pub player_context: u32,
    pub model_entries: [u32; 8],
    /// Test-owned memory the image's addresses name; kept alive for the run.
    pub keep: Vec<Vec<u32>>,
}

/// Runs one case through the rewrite and through the lifted method.
///
/// The lifted closure receives the decoded pedestrian, its identity and the
/// test world. Returns the rewrite's outcome and the lifted outcome.
pub fn run_both(
    case: &Case,
    rewrite: impl FnOnce(u32) -> u32,
    lifted: impl FnOnce(&mut CPlayerPed, Handle32<PedTag>, &mut TestWorld) -> u32,
) -> (Outcome, Outcome) {
    let _guard = rt::lock();
    rt::set_player_context(case.player_context);
    for (i, record) in case.model_entries.iter().enumerate() {
        rt::set_model_entry(i, *record);
    }
    let mut mem = case.init.clone();
    let this = mem.as_mut_ptr() as usize as u32;
    rt::begin(&case.kinds, &case.answers, case.effect, this);
    let ret_rw = rewrite(this);
    let calls_rw = rt::take_calls();
    let rw = Outcome {
        ret: ret_rw,
        calls: calls_rw,
        image: mem,
    };

    let mut img = case.init.clone();
    let mut ped = decode(&img);
    rt::begin(&case.kinds, &case.answers, case.effect, this);
    let identity = Handle32::<PedTag>::new(this).expect("the image address is non-zero");
    let mut world = TestWorld;
    let ret_lf = lifted(&mut ped, identity, &mut world);
    let calls_lf = rt::take_calls();
    encode(&ped, &mut img);
    let lf = Outcome {
        ret: ret_lf,
        calls: calls_lf,
        image: img,
    };
    (rw, lf)
}

/// Compares the two outcomes: the result, every call with its arguments in
/// order, and every word of the image. The error names the first
/// difference only.
pub fn compare(name: &str, rw: &Outcome, lf: &Outcome) -> Result<(), String> {
    if rw.ret != lf.ret {
        return Err(format!(
            "{name}: result {:#010x} from the rewrite, {:#010x} from the lift",
            rw.ret, lf.ret
        ));
    }
    if rw.calls != lf.calls {
        return Err(format!(
            "{name}: collaborator calls differ (rewrite {} calls, lift {} calls); first call differs at {}",
            rw.calls.len(),
            lf.calls.len(),
            rw.calls
                .iter()
                .zip(&lf.calls)
                .position(|(a, b)| a != b)
                .map_or_else(|| "the end".to_string(), |i| i.to_string())
        ));
    }
    if let Some(i) = rw.image.iter().zip(&lf.image).position(|(a, b)| a != b) {
        return Err(format!("{name}: image word at byte {:#x} differs", i * 4));
    }
    Ok(())
}

/// A random pedestrian image: every word random, so the bytes the rewrite
/// must leave alone are random too.
#[must_use]
pub fn random_image(rng: &mut Rng) -> Vec<u32> {
    (0..IMAGE_WORDS).map(|_| rng.next_u32()).collect()
}

/// A float bit pattern from a mix that always includes the special values:
/// zeros, infinities, quiet and signalling not-a-number with distinct
/// payloads, denormals, and ordinary numbers.
#[must_use]
pub fn float_bits(rng: &mut Rng) -> u32 {
    match rng.below(14) {
        0 => 0x0000_0000,
        1 => 0x8000_0000,
        2 => 0x7F80_0000,
        3 => 0xFF80_0000,
        4 => 0x7FC0_0000,
        5 => 0x7FC0_0001,
        6 => 0xFFC0_1234,
        7 => 0x7F80_0001,
        8 => 0x0000_0001,
        9 => 0x3F80_0000,
        10 => 0x4F00_0000,
        11 => 0xBF80_0000,
        _ => rng.next_u32(),
    }
}

/// An object identity: zero, or one of three fixed non-zero values, so that
/// equal and unequal pairs both come up often.
#[must_use]
pub fn handle_pick(rng: &mut Rng) -> u32 {
    match rng.below(4) {
        0 => 0,
        1 => 0x0040_1000,
        2 => 0x0040_2000,
        _ => 0x0040_3000,
    }
}

/// Which side of a case the trial runs: the lifted method, or the deliberately
/// wrong copy that must be caught.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Lifted,
    Wrong,
}

/// Runs `trials` cases of one trial function. Returns the number of trials
/// whose comparison failed and the first failure.
pub fn run_trials(
    trials: u32,
    seed: u64,
    mut trial: impl FnMut(&mut Rng, Variant) -> Result<(), String>,
    variant: Variant,
) -> (u32, Option<String>) {
    let mut rng = Rng::new(seed);
    let mut failures = 0;
    let mut first = None;
    for _ in 0..trials {
        if let Err(e) = trial(&mut rng, variant) {
            failures += 1;
            if first.is_none() {
                first = Some(e);
            }
        }
    }
    (failures, first)
}
