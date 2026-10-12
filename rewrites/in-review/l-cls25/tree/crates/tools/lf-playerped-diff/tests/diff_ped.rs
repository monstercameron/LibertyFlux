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
    fn_00a2d3e0, fn_00a29e40, fn_009edb70, fn_00a2a870, fn_009f0e20, fn_009ebed0,
};
use lf_playerped_diff::rt::{self, Effect, Kind, SLOT8_ID};
use lf_world::player_ped::{ByteGlobal, GateWorld, WordGlobal};
use lf_world::player_ped::offsets;
use lf_world::player_ped::offsets::{
    CURRENT_SLOT, JOB_WORD, LINK, MODE, MOTION_VALUE, SETUP_RESULT, SUB_A, SUB_B,
};
use lf_world::player_ped::{CPlayerPed, PedTag, PlayerPedWorld};
use support::cells as cells_index;
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
        aim_state: 0,
        dist_threshold: 0,
        cells: [0; rt::CELL_COUNT],
        poses: [0; 16],
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
    let mut c = case(init, &[(1, Kind::T2)], vec![(1, vec![answer])], None);
    let (rw, lf) = run_both(
        &mut c,
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
    let mut c = case(init, &[(2, Kind::T4), (3, Kind::T1)], answers, None);
    let (rw, lf) = run_both(
        &mut c,
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
    let mut c = case(init, &[(1, Kind::T1), (2, Kind::C3), (3, Kind::C3)], answers, effect);
    let (rw, lf) = run_both(
        &mut c,
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
    let mut c = case(
        init,
        &[(1, Kind::T2), (2, Kind::T2), (3, Kind::T3)],
        answers,
        effect,
    );
    let (rw, lf) = run_both(
        &mut c,
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
        &mut c,
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
        &mut c,
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
        &mut c,
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
        &mut c,
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
        &mut c,
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
    let mut c = case(
        init,
        &[(2, Kind::T1), (3, Kind::T2), (4, Kind::T1)],
        answers,
        None,
    );
    let (rw, lf) = run_both(
        &mut c,
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

/// Wrong: when both the threshold and the aim factor are not-a-number, the
/// threshold's payload is kept instead of the aim factor's.
fn wrong_acquire(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    key: u32,
    flag: u32,
    world: &mut TestWorld,
) -> bool {
    let targeting = this.get().wrapping_add(0x2B0);
    let context = world.get_context(targeting);
    if context == 0 {
        return false;
    }
    let argument = world.context_property_argument(context);
    let property = world.get_property(argument);
    let initial = f32::from_bits(world.property_threshold_bits(property));
    let state = world.aim_state();
    let threshold = if state == 2 && (world.state_gate() as u8) != 0 {
        if ped.aim_context == 0 {
            initial
        } else {
            let aim = f32::from_bits(world.aim_factor_bits(ped.aim_context));
            let product = black_box(aim) * black_box(initial);
            if initial.is_nan() && aim.is_nan() {
                f32::from_bits(initial.to_bits() | 0x0040_0000)
            } else if initial.is_nan() {
                f32::from_bits(initial.to_bits() | 0x0040_0000)
            } else if aim.is_nan() {
                f32::from_bits(aim.to_bits() | 0x0040_0000)
            } else {
                product
            }
        }
    } else {
        initial
    };
    let helper = if (flag as u8) == 0 { 0x22E8 } else { 0x22E4 };
    let target = world.create_target(
        this.get().wrapping_add(0xE6C),
        helper,
        threshold.to_bits(),
        key,
        0,
        0,
    );
    if target == 0 {
        return false;
    }
    world.attach_target(targeting, target);
    if ped.aim_context == 0 {
        return false;
    }
    world.mark_selected(ped.aim_context);
    true
}

fn trial_acquire(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    // The context record holds the property argument and the aim factor,
    // and receives the selected marker; the property record holds the
    // threshold. Both live in the case's kept memory at fixed addresses.
    let mut context_rec = vec![0u32; 0x180];
    put32(&mut context_rec, 0x18, rng.next_u32());
    put32(&mut context_rec, 0x5B4, float_bits(rng));
    put8(&mut context_rec, 0x485, rng.next_u32() as u8);
    let mut prop_rec = vec![0u32; 0x10];
    put32(&mut prop_rec, 0x14, float_bits(rng));
    let ctx_addr = context_rec.as_ptr() as usize as u32;
    let prop_addr = prop_rec.as_ptr() as usize as u32;

    let mut init = random_image(rng);
    let aim_context = if rng.below(4) == 0 { 0 } else { ctx_addr };
    put32(&mut init, offsets::AIM_CONTEXT, aim_context);
    let context_answer = if rng.below(5) == 0 { 0 } else { ctx_addr };
    let key = rng.next_u32();
    let flag = rng.next_u32();
    let answers = vec![
        (1, vec![context_answer]),
        (2, vec![prop_addr]),
        (3, vec![rng.next_u32()]),
        (4, vec![rng.next_u32()]),
        (5, vec![rng.next_u32()]),
    ];
    let mut c = case(
        init,
        &[
            (1, Kind::T1),
            (2, Kind::C1),
            (3, Kind::C0),
            (4, Kind::T6),
            (5, Kind::T2),
        ],
        answers,
        None,
    );
    c.aim_state = if rng.coin() { 2 } else { rng.below(4) };
    c.keep = vec![context_rec, prop_rec];
    let (rw, lf) = run_both(
        &mut c,
        |this| u32::from(fn_00a2a870::fn_00a2a870(this as *mut u8, key, flag)),
        |ped, this, world| match variant {
            Variant::Lifted => u32::from(ped.acquire_aim_target(this, key, flag, world)),
            Variant::Wrong => u32::from(wrong_acquire(ped, this, key, flag, world)),
        },
    );
    compare("acquire_aim_target", &rw, &lf)
}

/// Wrong: the marker bits are never set on the relink word.
fn wrong_vf45(
    ped: &mut CPlayerPed,
    this: Handle32<PedTag>,
    flag: u32,
    world: &mut TestWorld,
) -> u32 {
    world.base_entry(ped, this, flag);
    let mut answer = Handle32::raw_or_zero(ped.link);
    let target = Handle32::raw_or_zero(ped.sub_b);
    if answer != target && target != 0 {
        let src = ped.source;
        let s_x = f32::from_bits(world.read_word(src.wrapping_add(0x30)));
        let s_y = f32::from_bits(world.read_word(src.wrapping_add(0x34)));
        let s_z = f32::from_bits(world.read_word(src.wrapping_add(0x38)));
        let t_x = f32::from_bits(world.read_word(target.wrapping_add(0x40)));
        let t_y = f32::from_bits(world.read_word(target.wrapping_add(0x44)));
        let t_z = f32::from_bits(world.read_word(target.wrapping_add(0x48)));
        let dx = black_box(s_x) - black_box(t_x);
        let dy = black_box(s_y) - black_box(t_y);
        let dz = black_box(s_z) - black_box(t_z);
        let dist_sq = black_box(black_box(dx * dx) + black_box(dy * dy)) + black_box(dz * dz);
        let threshold = f32::from_bits(world.distance_threshold_bits());
        if (flag & 0xFF) != 0 || dist_sq > threshold {
            answer = world.relink(ped, this, 1);
        } else {
            return src;
        }
    }
    if (flag & 0xFF) == 0 {
        return answer;
    }
    ped.relink_block = [0; 9];
    answer
}

/// A coordinate: mostly small values near the threshold range, sometimes any
/// bit pattern (not-a-number and infinities included).
fn coordinate_bits(rng: &mut Rng) -> u32 {
    if rng.coin() {
        float_bits(rng)
    } else {
        ((rng.below(2000) as f32) / 100.0 - 10.0).to_bits()
    }
}

fn trial_vf45(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    // The source and target points are kept records at fixed addresses.
    let mut src_rec = vec![0u32; 0x20];
    for off in [0x30u32, 0x34, 0x38] {
        put32(&mut src_rec, off, coordinate_bits(rng));
    }
    let mut tgt_rec = vec![0u32; 0x20];
    for off in [0x40u32, 0x44, 0x48] {
        put32(&mut tgt_rec, off, coordinate_bits(rng));
    }
    let src_addr = src_rec.as_ptr() as usize as u32;
    let tgt_addr = tgt_rec.as_ptr() as usize as u32;

    let mut init = random_image(rng);
    put32(&mut init, offsets::SOURCE, src_addr);
    // The target is dereferenced by the original, so it is either absent or
    // the kept point; the link is only compared, so any identity will do.
    let target = if rng.coin() { 0 } else { tgt_addr };
    put32(&mut init, offsets::SUB_B, target);
    let link = match rng.below(3) {
        0 => 0,
        1 => tgt_addr,
        _ => 0x0040_1000,
    };
    put32(&mut init, LINK, link);
    put8(&mut init, offsets::MARKER_219, rng.next_u32() as u8);
    let flag = if rng.below(3) == 0 {
        rng.next_u32() & !0xFF
    } else {
        rng.next_u32() | 1
    };
    let answers = vec![(2, vec![rng.next_u32()]), (3, vec![rng.next_u32()])];
    let mut c = case(init, &[(2, Kind::T2), (3, Kind::T2)], answers, None);
    c.dist_threshold = if rng.coin() {
        float_bits(rng)
    } else {
        ((rng.below(4000) as f32) / 10.0).to_bits()
    };
    c.keep = vec![src_rec, tgt_rec];
    let (rw, lf) = run_both(
        &mut c,
        |this| fn_009f0e20::rw_009f0e20(this, flag),
        |ped, this, world| match variant {
            Variant::Lifted => ped.relink_on_move(this, flag, world),
            Variant::Wrong => wrong_vf45(ped, this, flag, world),
        },
    );
    compare("vf45", &rw, &lf)
}


/// Wrong: the attachment step is left out of the refresh gate.
fn wrong_vf33(ped: &mut CPlayerPed, this: Handle32<PedTag>, world: &mut TestWorld) -> u32 {
    let argument = world.hook_argument();
    let hooked = world.hook_call(argument) != 0
        || (world.global_byte(ByteGlobal::FlagA) != 0 && world.global_byte(ByteGlobal::FlagB) != 0);
    let mut acc = u8::from(hooked);
    acc |= world.global_byte(ByteGlobal::OrFirst);
    acc |= world.global_byte(ByteGlobal::OrSecond);
    if acc != 0 && world.global_byte(ByteGlobal::Guard) == 0 {
        return 0;
    }
    let got = world.refresh_virtual(ped, this);
    world.refresh_follow(got);
    if world.global_word(WordGlobal::State) == 1 {
        return 0;
    }
    if world.global_word(WordGlobal::PairA) != world.global_word(WordGlobal::PairB) {
        return 0;
    }
    if world.global_word(WordGlobal::Mode) == 0x12 {
        return 0;
    }
    let obj = ped.chain_base;
    if (world.probe_first(obj) & 0xFF) != 0 {
        ped.relink_flags &= !2;
    }
    if (world.probe_second(obj) & 0xFF) == 0 {
        if (ped.relink_flags >> 1) & 1 != 0 {
            if (world.deep_refresh_gate() & 0xFF) == 0 && ped.status_28 & 0x0080_0000 == 0 {
                world.deep_refresh_entry(this.get().wrapping_add(0xBB0));
            }
        } else if ped.marker_219 == 0 {
            let current = f32::from_bits(ped.fval_bits);
            let constant = f32::from_bits(world.global_word(WordGlobal::FloatConst));
            if current != constant
                && (world.deep_refresh_gate() & 0xFF) == 0
                && ped.status_28 & 0x0080_0000 == 0
            {
                world.deep_refresh_entry(this.get().wrapping_add(0xBB0));
            }
        }
    }
    if ped.flag_f4 & 2 != 0 {
        world.sub_virtual(this.get().wrapping_add(0xBB0));
    }
    ped.status_29c |= 0x0100_0000;
    let ready = world.ready_probe(ped, this);
    if ready == 0 {
        world.tail_virtual(ped, this);
    }
    ready
}

fn trial_vf33(rng: &mut Rng, variant: Variant) -> Result<(), String> {
    // The pedestrian's table: the refresh slot at byte 0xD0 and the tail slot
    // at byte 0xCC; every other word holds the two-argument trap.
    let mut ped_table = vec![rt::trap_stub_addr(); 53];
    ped_table[51] = rt::tail_stub_addr();
    ped_table[52] = rt::refresh_stub_addr();
    let ped_table_addr = ped_table.as_ptr() as usize as u32;
    // The inner object's table: the attach slot at byte 0x0C.
    let mut inner_table = vec![rt::trap_stub_addr(); 4];
    inner_table[3] = rt::attach_stub_addr();
    let mut inner_rec = vec![0u32; 0x10];
    inner_rec[0] = inner_table.as_ptr() as usize as u32;
    let inner_addr = inner_rec.as_ptr() as usize as u32;
    // The outer object: its inner pointer at byte 0x58.
    let mut outer_rec = vec![0u32; 0x30];
    outer_rec[0x58 / 4] = inner_addr;
    let outer_addr = outer_rec.as_ptr() as usize as u32;
    // The sub-object's table: the entry at byte 4 is the sub-object slot.
    let mut sub_table = vec![rt::trap_one_addr(); 2];
    sub_table[1] = rt::vsub_stub_addr();
    let sub_table_addr = sub_table.as_ptr() as usize as u32;

    // Pose records: one per table entry, each with a state byte at 0x8C.
    let mut poses = [0u32; 16];
    let mut pose_recs: Vec<Vec<u32>> = Vec::new();
    for slot in &mut poses {
        let mut rec = vec![0u32; 0x30];
        let state = if rng.below(3) == 0 { 0 } else { (1 + rng.below(255)) as u8 };
        put8(&mut rec, 0x8C, state);
        *slot = if rng.below(5) == 0 { 0 } else { rec.as_ptr() as usize as u32 };
        pose_recs.push(rec);
    }

    let mut init = random_image(rng);
    put32(&mut init, 0, ped_table_addr);
    put32(&mut init, 0x0BB0, sub_table_addr);
    put32(&mut init, offsets::OUTER, outer_addr);
    let index = rng.below(16) as i32 - 8;
    put16(&mut init, offsets::CHAIN_INDEX, index as i16 as u16);
    put8(&mut init, offsets::MARKER_219, if rng.coin() { 0 } else { 1 + rng.below(255) as u8 });
    put32(&mut init, offsets::FVAL, float_bits(rng));
    put32(&mut init, offsets::CHAIN_BASE, handle_pick(rng));
    put8(&mut init, offsets::FLAG_F4, rng.next_u32() as u8);

    let mut cells = [0u32; rt::CELL_COUNT];
    cells[cells_index::HOOK_ARG] = rng.next_u32();
    cells[cells_index::HOOK_SLOT] = rt::hook_stub_addr();
    cells[cells_index::FLAG_A] = if rng.below(3) == 0 { 0 } else { rng.below(256) };
    cells[cells_index::FLAG_B] = if rng.coin() { 0 } else { rng.below(256) };
    cells[cells_index::OR_FIRST] = if rng.below(4) == 0 { 1 } else { 0 };
    cells[cells_index::OR_SECOND] = if rng.below(4) == 0 { 2 } else { 0 };
    cells[cells_index::GUARD] = if rng.below(3) == 0 { 0 } else { 1 };
    cells[cells_index::STATE] = if rng.below(4) == 0 { 1 } else { rng.below(4) };
    cells[cells_index::PAIR_A] = rng.below(3);
    cells[cells_index::PAIR_B] = if rng.coin() { cells[cells_index::PAIR_A] } else { rng.below(3) };
    cells[cells_index::MODE] = if rng.below(5) == 0 { 0x12 } else { rng.below(0x20) };
    cells[cells_index::FLOAT_CONST] = float_bits(rng);

    let answers = vec![
        (3, vec![rng.next_u32()]),
        (5, vec![rng.next_u32()]),
        (6, vec![rng.next_u32()]),
        (7, vec![rng.next_u32()]),
        (8, vec![rng.next_u32()]),
        (10, vec![rng.next_u32()]),
        (rt::HOOK_ID, vec![rng.next_u32()]),
        (rt::REFRESH_ID, vec![rng.next_u32()]),
        (rt::ATTACH_ID, vec![rng.next_u32()]),
        (rt::TAIL_ID, vec![rng.next_u32()]),
        (rt::VSUB_ID, vec![rng.next_u32()]),
    ];
    let mut c = case(
        init,
        &[(3, Kind::T1), (5, Kind::T1), (6, Kind::T1), (7, Kind::C0), (8, Kind::T1), (10, Kind::T1)],
        answers,
        None,
    );
    c.cells = cells;
    c.poses = poses;
    c.keep = vec![ped_table, inner_table, inner_rec, outer_rec, sub_table];
    c.keep.extend(pose_recs);
    let (rw, lf) = run_both(
        &mut c,
        |this| fn_009ebed0::rw_009EBED0(this),
        |ped, this, world| match variant {
            Variant::Lifted => ped.refresh_gate(this, world),
            Variant::Wrong => wrong_vf33(ped, this, world),
        },
    );
    compare("vf33", &rw, &lf)
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

#[test]
fn acquire_aim_target_lift_matches_rewrite() {
    expect_agreement("acquire_aim_target", trial_acquire, 0xA1);
}

#[test]
fn acquire_aim_target_wrong_lift_is_caught() {
    expect_wrong_caught("acquire_aim_target", trial_acquire, 0xA1);
}

#[test]
fn vf45_lift_matches_rewrite() {
    expect_agreement("vf45", trial_vf45, 0x45);
}

#[test]
fn vf45_wrong_lift_is_caught() {
    expect_wrong_caught("vf45", trial_vf45, 0x45);
}

#[test]
fn vf33_lift_matches_rewrite() {
    expect_agreement("vf33", trial_vf33, 0x33);
}

#[test]
fn vf33_wrong_lift_is_caught() {
    expect_wrong_caught("vf33", trial_vf33, 0x33);
}
