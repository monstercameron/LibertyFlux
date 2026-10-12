//! Deliberately wrong lifts, one plausible mistake per variant.
//!
//! Each variant is the lifted method with one change: an off-by-one
//! predicate, a swapped argument, the wrong flag bit, a signed compare where
//! the original is unsigned. The differential cases must catch every one; a
//! variant that passes would mean the cases cannot see that change.

use lf_core::Handle32;
use lf_input_frontend::replay_progress_bar::{
    BarTag, Part, ReplayBarGlobals, ReplayBarWorld, ReplayProgressBar, UpdateOutcome,
};

/// The wrong teardown variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeardownMutant {
    /// The second finalizer is left out.
    SkipFinalizeB,
    /// The first three slots are released but not cleared.
    KeepFirstSlots,
    /// The token given to the attach call is zero.
    AttachZeroToken,
}

/// The wrong deleting destructor variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeletingMutant {
    /// The storage is released on bit 1 instead of bit 0.
    FreeOnBitOne,
    /// The returned object is the inner one, not the object.
    ReturnInner,
}

/// The wrong update variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateMutant {
    /// The second phase runs for every phase word (the skip test is lost).
    SecondPhaseAlways,
    /// The consume call takes the two words in the other order.
    ConsumeSwapped,
    /// The total takes the first word instead of the second.
    TotalFromFirst,
    /// The compared value is compared as signed.
    SignedCompare,
    /// The compare is "at most" where the original is "below".
    CompareAtMost,
    /// The long signal is chosen for mode 6 instead of mode 7.
    LongSignalMode6,
    /// The gate steps run when the inner flag is set, not when it is clear.
    GateInverted,
    /// The signal step does not OR the bit into the or-word.
    NoOrBit,
}

/// The teardown with one wrong step.
pub fn teardown(bar: &mut ReplayProgressBar, world: &mut impl ReplayBarWorld, m: TeardownMutant) {
    world.base_teardown(bar.handle);
    let token = world.open_token();
    world.attach_token(if m == TeardownMutant::AttachZeroToken {
        0
    } else {
        token
    });
    for slot in &mut bar.owned[..3] {
        if let Some(owned) = *slot {
            world.destruct(owned);
            world.free_owned(owned);
            if m != TeardownMutant::KeepFirstSlots {
                *slot = None;
            }
        }
    }
    for index in 3..bar.owned.len() {
        if let Some(owned) = bar.owned[index] {
            world.destruct(owned);
            if let Some(still) = bar.owned[index] {
                world.destruct(still);
                world.free_owned(still);
            }
            bar.owned[index] = None;
        }
    }
    world.close_token(token);
    world.finalize_a();
    if m != TeardownMutant::SkipFinalizeB {
        world.finalize_b();
    }
    bar.torn = true;
}

/// The deleting destructor with one wrong step. Its teardown is the lifted
/// one, so the variants change only the destructor's own steps.
pub fn deleting(
    bar: &mut ReplayProgressBar,
    world: &mut impl ReplayBarWorld,
    flags: u32,
    m: DeletingMutant,
) -> Handle32<BarTag> {
    bar.teardown(world);
    let bit = if m == DeletingMutant::FreeOnBitOne {
        2
    } else {
        1
    };
    if flags & bit != 0 {
        world.release_storage(bar.handle);
    }
    if m == DeletingMutant::ReturnInner {
        return Handle32::new(bar.inner.get()).expect("inner is non-null");
    }
    bar.handle
}

/// The update with one wrong step.
pub fn update(
    bar: &mut ReplayProgressBar,
    world: &mut impl ReplayBarWorld,
    globals: &mut ReplayBarGlobals,
    m: UpdateMutant,
) -> UpdateOutcome {
    world.setup_inner(bar.inner, 0);
    let [_, produced_b, produced_c] = world.produce(bar.handle);
    bar.mark = u32::MAX;
    let (token, mut first, mut second) = world.fill_pair();
    if m == UpdateMutant::ConsumeSwapped {
        let (mut x, mut y) = (second, first);
        world.consume(bar.handle, token, &mut x, &mut y);
    } else {
        world.consume(bar.handle, token, &mut first, &mut second);
    }
    let measured = world.measure();
    bar.total = if m == UpdateMutant::TotalFromFirst {
        measured.wrapping_add(first)
    } else {
        measured.wrapping_add(second)
    };

    let skip_second = bar.phase == 1 || bar.phase == 2;
    if m == UpdateMutant::SecondPhaseAlways || !skip_second {
        let [_, refreshed_a, refreshed_b] = world.produce(bar.handle);
        world.use_a(refreshed_a);
        let used = world.use_b(bar.handle, refreshed_a, 0);
        world.use_c(used);
        world.use_d(refreshed_b, used);
        let used_again = world.use_b(bar.handle, refreshed_b, 0);
        world.use_e(used_again);
    }

    world.combine(Part::Sub1c, first, second);
    world.step(bar.handle);
    let compared = world.compare();
    let below = if m == UpdateMutant::SignedCompare {
        (compared as i32) < (produced_b as i32)
    } else if m == UpdateMutant::CompareAtMost {
        compared <= produced_b
    } else {
        compared < produced_b
    };
    if below {
        let mode_long = if m == UpdateMutant::LongSignalMode6 {
            globals.mode == 2 || globals.mode == 6
        } else {
            globals.mode == 2 || globals.mode == 7
        };
        let long = mode_long && globals.flag != 0;
        world.signal(if long { 7 } else { 6 });
        if m != UpdateMutant::NoOrBit {
            globals.or_word |= 2;
        }
        globals.save_word = produced_b;
    }

    world.push_s4(bar.handle, produced_c);
    if globals.gate != 0 {
        let flag_set = world.inner_flag_set(bar.inner);
        let run_gate_ab = if m == UpdateMutant::GateInverted {
            flag_set
        } else {
            !flag_set
        };
        if run_gate_ab {
            world.gate_a(bar.handle);
            world.gate_b(bar.handle);
        }
        world.gate_c(bar.handle);
    }

    if world.inner_flag_set(bar.inner) {
        UpdateOutcome::Inner(bar.inner)
    } else {
        world.finish(bar.handle);
        UpdateOutcome::Base(world.base_update(bar.handle))
    }
}
