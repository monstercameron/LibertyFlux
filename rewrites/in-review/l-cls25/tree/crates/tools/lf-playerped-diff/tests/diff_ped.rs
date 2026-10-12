//! Differential tests: each lifted player pedestrian method against its
//! verified 32-bit rewrite, on generated inputs.
//!
//! Every method has two tests. The first runs the lifted method and the
//! rewrite on the same cases and requires the results, the collaborator
//! calls in order and the whole pedestrian image to agree. The second runs a
//! deliberately wrong copy of the lift on the same cases and requires that
//! the comparison catches it. The rewrites take real addresses, so the file
//! runs on the 32-bit target only.

#![cfg(target_arch = "x86")]

mod support;

use core::hint::black_box;

use lf_core::Handle32;
use lf_playerped_diff::rewrites::{
    fn_009e54c0, fn_009e5940, fn_009eaee0, fn_009f0fc0, fn_00a2a620, fn_00a2d300, fn_00a2d3a0,
    fn_00a2d3e0, fn_00a29e40, fn_009edb70,
};
use lf_playerped_diff::rt::{Effect, Kind, SLOT8_ID};
use lf_world::player_ped::offsets;
use lf_world::player_ped::offsets::{
    CURRENT_SLOT, JOB_WORD, LINK, MODE, MOTION_VALUE, SETUP_RESULT, SUB_A, SUB_B,
};
use lf_world::player_ped::{CPlayerPed, PedTag, PlayerPedWorld};
use support::{
    Case, Rng, TestWorld, Variant, compare, float_bits, handle_pick, put16, put32, put8,
    random_image, run_both, run_trials,
};

/// Generated cases per test.
const TRIALS: u32 = 3000;

/// One case with the default globals (no player context, empty model table)
/// and no kept memory; a trial fills in what it needs.
fn case(
    init: Vec<u32>,
    kinds: &[(u32, Kind)],
    answers: Vec<(u32, Vec<u32>)>,
    effect: Option<Effect>,
) -> Case {
    Case {
        init,
        kinds: kinds.to_vec(),
        answers,
        effect,
        player_context: 0,
        model_entries: [0; 8],
        keep: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Deliberately wrong copies. Each one differs from its lifted method in one
// place that a test can observe; each must be caught.
// ---------------------------------------------------------------------------

/// Wrong: the motion value sum takes its operands in the other order.
fn wrong_vf62(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    amount: f32,
    world: &mut TestWorld,
) -> u32 {
    let updated = black_box(ped.motion_value) + black_box(amount);
    ped.motion_value = updated;
    world.motion_helper(ped, this, updated.to_bits())
}

/// Wrong: the link test is inverted (notifies the trigger entry when the
/// link differs from the target).
fn wrong_vf44(ped: &mut CPlayerPed, this: Handle32<PedTag>, world: &mut TestWorld) -> u32 {
    match (ped.link, ped.sub_b) {
        (Some(link), Some(target)) if link != target => world.trigger_entry(ped, this, 0, 1, 0),
        _ => world.base_update(ped, this),
    }
}

/// Wrong: the second job reuses the job word read before the first job ran.
fn wrong_vf16(ped: &mut CPlayerPed, this: Handle32<PedTag>, world: &mut TestWorld) -> bool {
    if (world.ped_ready_check(ped, this) & 0xFF) == 0 {
        return false;
    }
    let job = ped.job_word;
    world.ped_job_a(ped, this, job.wrapping_add(0x80), job);
    world.ped_job_b(ped, this, job.wrapping_add(0x80), job);
    true
}

/// Wrong: the pre-pass runs when the argument's bit is set, not clear.
fn wrong_vf69(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    arg: u32,
    world: &mut TestWorld,
) -> u32 {
    let nibble = ped.mode & 0xF;
    ped.pre_words = [0, 0];
    if matches!(nibble, 6 | 8 | 9 | 0xA) && (arg & 0x10) != 0 {
        world.pre_step(ped, this, arg);
    }
    ped.apply_words = [u32::MAX, u32::MAX];
    world.apply_step(ped, this, arg);
    let obj = ped.sub_a;
    world.notify_object(ped, obj, 8, 0)
}

/// Wrong: the second sub-object is read before the first dispatch runs.
fn wrong_vf19(ped: &mut CPlayerPed, arg: u32, world: &mut TestWorld) -> u32 {
    let second = ped.sub_b;
    let mut answer = 0;
    if let Some(first) = ped.sub_a {
        answer = world.dispatch_slot8(ped, first, arg);
    }
    if let Some(second) = second {
        answer = world.dispatch_slot8(ped, second, arg);
    }
    answer
}

/// Wrong: the method returns the value it stored, not the word read back.
fn wrong_vf47(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    args: [u32; 5],
    world: &mut TestWorld,
) -> u32 {
    let context = world.player_context();
    let lookup = world.lookup_context(context);
    let result = if lookup != 0 {
        world.setup(ped, lookup, this, 0, args[0], args[1], args[2], args[3])
    } else {
        0
    };
    ped.setup_result = result;
    let _ = world.finalize(ped, result, args[4]);
    result
}

/// Wrong: the revive flag is inverted.
fn wrong_change_model(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    model_id: u32,
    world: &mut TestWorld,
) -> u32 {
    if world.model_state(model_id) == 0 {
        return world.lookup_model(ped, this, model_id);
    }
    world.preload_model(model_id);
    world.lookup_model(ped, this, model_id);
    let revive_flag = if ped.current_slot == u32::MAX { 1 } else { 0 };
    world.revive(ped, this, revive_flag);
    let bundle = ped.job_word;
    world.load_bundle(ped, this, bundle)
}

/// Wrong: the result is the first source word, not the fourth.
fn wrong_vf80(ped: &mut CPlayerPed, source: [u32; 4]) -> u32 {
    ped.transform = source;
    source[0]
}

/// Wrong: the descriptor words are stored in reverse order.
fn wrong_vf79(ped: &mut CPlayerPed, source: [u32; 4]) {
    ped.descriptor = [source[3], source[2], source[1], source[0]];
}

// ---------------------------------------------------------------------------
// Trials: one generated case each, run through the rewrite and the lift.
// ---------------------------------------------------------------------------

fn trial_vf62(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    put32(&mut init, MOTION_VALUE, float_bits(rng));
    let amount = f32::from_bits(float_bits(rng));
    let answer = rng.next_u32();
    let c = case(init, &[(1, Kind::T2)], vec![(1, vec![answer])], None);
    let (rw, lf) = run_both(
        &c,
        |this| fn_00a29e40::rw_a29e40(this, amount),
        |ped, this, world| match variant {
            Variant::Lifted => ped.add_motion_value(this, amount, world),
            Variant::Wrong => wrong_vf62(ped, this, amount, world),
        },
    );
    compare("vf62", &rw, &lf)
}

fn trial_vf44(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    put32(&mut init, LINK, handle_pick(rng));
    put32(&mut init, SUB_B, handle_pick(rng));
    let answers = vec![(2, vec![rng.next_u32()]), (3, vec![rng.next_u32()])];
    let c = case(init, &[(2, Kind::T4), (3, Kind::T1)], answers, None);
    let (rw, lf) = run_both(
        &c,
        |this| fn_009f0fc0::rw_009f0fc0(this),
        |ped, this, world| match variant {
            Variant::Lifted => ped.notify_while_linked(this, world),
            Variant::Wrong => wrong_vf44(ped, this, world),
        },
    );
    compare("vf44", &rw, &lf)
}

fn trial_vf16(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    let job = match rng.below(5) {
        0 => 0xFFFF_FF80,
        1 => 0xFFFF_FF7F,
        2 => 0,
        _ => rng.next_u32(),
    };
    put32(&mut init, JOB_WORD, job);
    // The ready check's low byte decides; one case in three is not ready.
    let ready = if rng.below(3) == 0 {
        rng.next_u32() & 0xFFFF_FF00
    } else {
        (rng.next_u32() & 0xFFFF_FF00) | (1 + rng.below(255))
    };
    let effect = if rng.coin() {
        Some(Effect {
            callee: 2,
            off: JOB_WORD,
            value: rng.next_u32(),
        })
    } else {
        None
    };
    let answers = vec![
        (1, vec![ready]),
        (2, vec![rng.next_u32()]),
        (3, vec![rng.next_u32()]),
    ];
    let c = case(init, &[(1, Kind::T1), (2, Kind::C3), (3, Kind::C3)], answers, effect);
    let (rw, lf) = run_both(
        &c,
        |this| fn_009e54c0::rw_009e54c0(this),
        |ped, this, world| match variant {
            Variant::Lifted => u32::from(ped.run_jobs_if_ready(this, world)),
            Variant::Wrong => u32::from(wrong_vf16(ped, this, world)),
        },
    );
    compare("vf16", &rw, &lf)
}

fn trial_vf69(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    put16(&mut init, MODE, rng.next_u32() as u16);
    put32(&mut init, SUB_A, handle_pick(rng));
    put32(&mut init, SUB_B, handle_pick(rng));
    let arg = if rng.coin() {
        rng.next_u32() | 0x10
    } else {
        rng.next_u32() & !0x10
    };
    let effect = if rng.coin() {
        Some(Effect {
            callee: 2,
            off: SUB_A,
            value: handle_pick(rng),
        })
    } else {
        None
    };
    let answers = vec![
        (1, vec![rng.next_u32()]),
        (2, vec![rng.next_u32()]),
        (3, vec![rng.next_u32()]),
    ];
    let c = case(
        init,
        &[(1, Kind::T2), (2, Kind::T2), (3, Kind::T3)],
        answers,
        effect,
    );
    let (rw, lf) = run_both(
        &c,
        |this| fn_009e5940::rw_009e5940(this, arg),
        |ped, this, world| match variant {
            Variant::Lifted => ped.reset_and_apply(this, arg, world),
            Variant::Wrong => wrong_vf69(ped, this, arg, world),
        },
    );
    compare("vf69", &rw, &lf)
}

fn trial_vf19(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    // Each sub-object's first word names a table whose byte-offset eight
    // holds the slot-8 stub; every other word holds the trap stub.
    let mut table = vec![rt_trap(); 8];
    table[2] = rt_slot8();
    let table_addr = table.as_ptr() as usize as u32;
    let objs: Vec<Vec<u32>> = (0..3).map(|_| vec![table_addr, 0, 0, 0]).collect();
    let addrs = [
        objs[0].as_ptr() as usize as u32,
        objs[1].as_ptr() as usize as u32,
        objs[2].as_ptr() as usize as u32,
    ];
    let pick = |rng: &mut Rng| match rng.below(4) {
        0 => 0,
        1 => addrs[0],
        2 => addrs[1],
        _ => addrs[2],
    };
    let a = pick(rng);
    let b = pick(rng);
    put32(&mut init, SUB_A, a);
    put32(&mut init, SUB_B, b);
    let effect = if rng.coin() {
        Some(Effect {
            callee: SLOT8_ID,
            off: SUB_B,
            value: pick(rng),
        })
    } else {
        None
    };
    let answers = vec![(SLOT8_ID, vec![rng.next_u32(), rng.next_u32()])];
    let arg = rng.next_u32();
    let mut c = case(init, &[], answers, effect);
    c.keep.push(table);
    c.keep.extend(objs);
    let (rw, lf) = run_both(
        &c,
        |this| fn_009eaee0::rw_009eaee0(this, arg),
        |ped, _this, world| match variant {
            Variant::Lifted => ped.notify_sub_objects(arg, world),
            Variant::Wrong => wrong_vf19(ped, arg, world),
        },
    );
    compare("vf19", &rw, &lf)
}

fn trial_vf47(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let init = random_image(rng);
    let args = [
        rng.next_u32(),
        rng.next_u32(),
        rng.next_u32(),
        rng.next_u32(),
        rng.next_u32(),
    ];
    let lookup = if rng.coin() { 0 } else { rng.next_u32() };
    let effect = if rng.coin() {
        Some(Effect {
            callee: 3,
            off: SETUP_RESULT,
            value: rng.next_u32(),
        })
    } else {
        None
    };
    let answers = vec![
        (1, vec![lookup]),
        (2, vec![rng.next_u32()]),
        (3, vec![rng.next_u32()]),
    ];
    let mut c = case(
        init,
        &[(1, Kind::T1), (2, Kind::T7), (3, Kind::T2)],
        answers,
        effect,
    );
    c.player_context = rng.next_u32();
    let (rw, lf) = run_both(
        &c,
        |this| fn_00a2a620::rw_a2a620(this, args[0], args[1], args[2], args[3], args[4]),
        |ped, this, world| match variant {
            Variant::Lifted => ped.setup_and_store(
                this, args[0], args[1], args[2], args[3], args[4], world,
            ),
            Variant::Wrong => wrong_vf47(ped, this, args, world),
        },
    );
    compare("vf47", &rw, &lf)
}

fn trial_change_model(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    let model_id = rng.below(8);
    let mut records: Vec<Vec<u32>> = Vec::new();
    let mut entries = [0u32; 8];
    for entry in &mut entries {
        let mut record = vec![0u32; 0x40];
        let state = if rng.below(3) == 0 {
            0
        } else {
            (1 + rng.below(255)) as u8
        };
        put8(&mut record, 0x8C, state);
        *entry = record.as_ptr() as usize as u32;
        records.push(record);
    }
    let slot = if rng.coin() {
        u32::MAX
    } else {
        rng.next_u32()
    };
    put32(&mut init, CURRENT_SLOT, slot);
    let effect = if rng.coin() {
        Some(Effect {
            callee: 1,
            off: CURRENT_SLOT,
            value: if rng.coin() {
                u32::MAX
            } else {
                rng.next_u32()
            },
        })
    } else {
        None
    };
    let answers = vec![
        (1, vec![rng.next_u32()]),
        (2, vec![rng.next_u32()]),
        (3, vec![rng.next_u32()]),
        (4, vec![rng.next_u32()]),
    ];
    let mut c = case(
        init,
        &[(1, Kind::T2), (2, Kind::C1), (3, Kind::T2), (4, Kind::C2)],
        answers,
        effect,
    );
    c.model_entries = entries;
    c.keep = records;
    let (rw, lf) = run_both(
        &c,
        |this| fn_00a2d300::rw_a2d300(this, model_id),
        |ped, this, world| match variant {
            Variant::Lifted => ped.change_model(this, model_id, world),
            Variant::Wrong => wrong_change_model(ped, this, model_id, world),
        },
    );
    compare("change_model", &rw, &lf)
}

fn trial_vf80(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let init = random_image(rng);
    let src: Vec<u32> = (0..4).map(|_| float_bits(rng)).collect();
    let src_addr = src.as_ptr() as usize as u32;
    let source = [src[0], src[1], src[2], src[3]];
    let mut c = case(init, &[], Vec::new(), None);
    c.keep.push(src);
    let (rw, lf) = run_both(
        &c,
        |this| fn_00a2d3e0::rw_a2d3e0(this, src_addr),
        |ped, _this, _world| match variant {
            Variant::Lifted => ped.set_transform(source),
            Variant::Wrong => wrong_vf80(ped, source),
        },
    );
    compare("vf80", &rw, &lf)
}

fn trial_vf79(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let init = random_image(rng);
    let src: Vec<u32> = (0..4).map(|_| float_bits(rng)).collect();
    let src_addr = src.as_ptr() as usize as u32;
    let source = [src[0], src[1], src[2], src[3]];
    let mut c = case(init, &[], Vec::new(), None);
    c.keep.push(src);
    let (mut rw, mut lf) = run_both(
        &c,
        |this| fn_00a2d3a0::rw_00a2d3a0(this, src_addr),
        |ped, _this, _world| {
            match variant {
                Variant::Lifted => ped.set_descriptor(source),
                Variant::Wrong => wrong_vf79(ped, source),
            }
            0
        },
    );
    // The original returns zero in its result register; the method has no
    // result, so that word is not compared.
    rw.ret = 0;
    lf.ret = 0;
    compare("vf79", &rw, &lf)
}

fn trial_vf32(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    let mut init = random_image(rng);
    put32(&mut init, offsets::CHAIN_BASE, rng.next_u32());
    if rng.below(4) == 0 {
        put32(&mut init, offsets::CHAIN_BASE, 0);
    }
    let answers = vec![
        (2, vec![rng.next_u32()]),
        (3, vec![rng.next_u32()]),
        (4, vec![rng.next_u32()]),
    ];
    let c = case(
        init,
        &[(2, Kind::T1), (3, Kind::T2), (4, Kind::T1)],
        answers,
        None,
    );
    let (rw, lf) = run_both(
        &c,
        |this| fn_009edb70::rw_009edb70(this),
        |ped, _this, world| match variant {
            Variant::Lifted => ped.run_entry_chain(world),
            Variant::Wrong => wrong_vf32(ped, world),
        },
    );
    compare("vf32", &rw, &lf)
}

/// Wrong: the entry chain runs on the chain base itself, without the offset.
fn wrong_vf32(ped: &mut CPlayerPed, world: &mut TestWorld) -> u32 {
    let sub = ped.chain_base;
    world.chain_reset(sub);
    world.chain_flag(sub, 1);
    world.chain_main(sub)
}

/// The slot-8 stub's address, as a word.
fn rt_slot8() -> u32 {
    lf_playerped_diff::rt::slot8_stub_addr()
}

/// The trap stub's address, as a word.
fn rt_trap() -> u32 {
    lf_playerped_diff::rt::trap_stub_addr()
}

// ---------------------------------------------------------------------------
// Tests: the lifted method agrees with its rewrite; the wrong copy is caught.
// ---------------------------------------------------------------------------

fn expect_agreement(name: &str, trial: fn(&mut Rng, Variant) -> Result<(), String>, seed: u64) {
    let (failures, first) = run_trials(TRIALS, seed, trial, Variant::Lifted);
    assert_eq!(failures, 0, "{name}: {failures} of {TRIALS} cases differ; first: {first:?}");
}

fn expect_wrong_caught(name: &str, trial: fn(&mut Rng, Variant) -> Result<(), String>, seed: u64) {
    let (failures, first) = run_trials(TRIALS, seed, trial, Variant::Wrong);
    eprintln!("{name}: wrong lift caught in {failures} of {TRIALS} cases; first: {first:?}");
    assert!(failures > 0, "{name}: the deliberately wrong lift was not caught in {TRIALS} cases");
}

#[test]
fn vf62_lift_matches_rewrite() {
    expect_agreement("vf62", trial_vf62, 0x62);
}

#[test]
fn vf62_wrong_lift_is_caught() {
    expect_wrong_caught("vf62", trial_vf62, 0x62);
}

#[test]
fn vf44_lift_matches_rewrite() {
    expect_agreement("vf44", trial_vf44, 0x44);
}

#[test]
fn vf44_wrong_lift_is_caught() {
    expect_wrong_caught("vf44", trial_vf44, 0x44);
}

#[test]
fn vf16_lift_matches_rewrite() {
    expect_agreement("vf16", trial_vf16, 0x16);
}

#[test]
fn vf16_wrong_lift_is_caught() {
    expect_wrong_caught("vf16", trial_vf16, 0x16);
}

#[test]
fn vf69_lift_matches_rewrite() {
    expect_agreement("vf69", trial_vf69, 0x69);
}

#[test]
fn vf69_wrong_lift_is_caught() {
    expect_wrong_caught("vf69", trial_vf69, 0x69);
}

#[test]
fn vf19_lift_matches_rewrite() {
    expect_agreement("vf19", trial_vf19, 0x19);
}

#[test]
fn vf19_wrong_lift_is_caught() {
    expect_wrong_caught("vf19", trial_vf19, 0x19);
}

#[test]
fn vf47_lift_matches_rewrite() {
    expect_agreement("vf47", trial_vf47, 0x47);
}

#[test]
fn vf47_wrong_lift_is_caught() {
    expect_wrong_caught("vf47", trial_vf47, 0x47);
}

#[test]
fn change_model_lift_matches_rewrite() {
    expect_agreement("change_model", trial_change_model, 0xC4);
}

#[test]
fn change_model_wrong_lift_is_caught() {
    expect_wrong_caught("change_model", trial_change_model, 0xC4);
}

#[test]
fn vf80_lift_matches_rewrite() {
    expect_agreement("vf80", trial_vf80, 0x80);
}

#[test]
fn vf80_wrong_lift_is_caught() {
    expect_wrong_caught("vf80", trial_vf80, 0x80);
}

#[test]
fn vf79_lift_matches_rewrite() {
    expect_agreement("vf79", trial_vf79, 0x79);
}

#[test]
fn vf79_wrong_lift_is_caught() {
    expect_wrong_caught("vf79", trial_vf79, 0x79);
}

#[test]
fn vf32_lift_matches_rewrite() {
    expect_agreement("vf32", trial_vf32, 0x32);
}

#[test]
fn vf32_wrong_lift_is_caught() {
    expect_wrong_caught("vf32", trial_vf32, 0x32);
}
