//! Host tests of the lifted player pedestrian methods on the 64-bit host.
//!
//! These run on every target: a recording world answers each collaborator
//! call with a fixed word and logs it, so the edge cases a reader would ask
//! about (the link test, the not-ready answer, the job word wrap, the mode
//! nibble, the model slot sentinel, the read back after a helper) are checked
//! without any rewrite. The differential cases in `diff_ped.rs` cover the
//! 32-bit comparison; nothing here touches an address.

use lf_core::Handle32;
use lf_world::player_ped::registry::{self, State};
use lf_world::player_ped::{
    ByteGlobal, CPlayerPed, GateWorld, ObjRef, ObjTag, PedTag, PlayerPedWorld, TaskWorld,
    WordGlobal,
};

/// A recording world. Every call is logged by name with its words and
/// answered with `answer`; the hooks below let a test change the pedestrian
/// or the context the way a collaborator could.
#[derive(Default)]
struct Fake {
    calls: Vec<(&'static str, Vec<u32>)>,
    answer: u32,
    context: u32,
    threshold_bits: u32,
    aim_state: u32,
    aim_factor_bits: u32,
    threshold: u32,
    words: Vec<(WordGlobal, u32)>,
    bytes: Vec<(ByteGlobal, u8)>,
    pose: u32,
    pose_flag: u8,
    memory: std::collections::BTreeMap<u32, u32>,
    model_state: u8,
    /// Written to the pedestrian's job word by the first job call.
    job_after_first: Option<u32>,
    /// Written to the pedestrian's stored result by the finalize call.
    result_after_finalize: Option<u32>,
}

impl Fake {
    fn log(&mut self, name: &'static str, words: &[u32]) -> u32 {
        self.calls.push((name, words.to_vec()));
        self.answer
    }
}

impl PlayerPedWorld for Fake {
    fn ped_ready_check(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("ready", &[this.get()])
    }

    fn ped_job_a(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32 {
        if let Some(word) = self.job_after_first.take() {
            ped.job_word = word;
        }
        self.log("job_a", &[this.get(), biased, job])
    }

    fn ped_job_b(
        &mut self,
        _ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32 {
        self.log("job_b", &[this.get(), biased, job])
    }

    fn trigger_entry(
        &mut self,
        _ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        first: u32,
        second: u32,
        third: u32,
    ) -> u32 {
        self.log("trigger", &[this.get(), first, second, third])
    }

    fn base_update(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("base", &[this.get()])
    }

    fn motion_helper(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, bits: u32) -> u32 {
        self.log("motion", &[this.get(), bits])
    }

    fn pre_step(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32 {
        self.log("pre", &[this.get(), arg])
    }

    fn apply_step(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32 {
        self.log("apply", &[this.get(), arg])
    }

    fn notify_object(
        &mut self,
        _ped: &mut CPlayerPed,
        obj: ObjRef,
        first: u32,
        second: u32,
    ) -> u32 {
        self.log("notify", &[Handle32::raw_or_zero(obj), first, second])
    }

    fn dispatch_slot8(&mut self, _ped: &mut CPlayerPed, obj: Handle32<ObjTag>, arg: u32) -> u32 {
        self.log("slot8", &[obj.get(), arg])
    }

    fn player_context(&mut self) -> u32 {
        self.context
    }

    fn lookup_context(&mut self, context: u32) -> u32 {
        self.log("lookup_context", &[context])
    }

    fn setup(
        &mut self,
        _ped: &mut CPlayerPed,
        lookup: u32,
        this: Handle32<PedTag>,
        flag: u32,
        first: u32,
        second: u32,
        third: u32,
        tail: u32,
    ) -> u32 {
        self.log(
            "setup",
            &[lookup, this.get(), flag, first, second, third, tail],
        )
    }

    fn finalize(&mut self, ped: &mut CPlayerPed, target: u32, arg: u32) -> u32 {
        if let Some(word) = self.result_after_finalize {
            ped.setup_result = word;
        }
        self.log("finalize", &[target, arg])
    }

    fn model_state(&mut self, _model_id: u32) -> u8 {
        self.model_state
    }

    fn base_entry(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, flag: u32) -> u32 {
        self.log("base_entry", &[this.get(), flag])
    }

    fn read_word(&mut self, address: u32) -> u32 {
        self.memory.get(&address).copied().unwrap_or(0)
    }

    fn distance_threshold_bits(&mut self) -> u32 {
        self.threshold
    }

    fn relink(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, one: u32) -> u32 {
        self.log("relink", &[this.get(), one])
    }

    fn get_context(&mut self, targeting: u32) -> u32 {
        self.log("get_context", &[targeting])
    }

    fn context_property_argument(&mut self, context: u32) -> u32 {
        self.log("property_argument", &[context])
    }

    fn get_property(&mut self, argument: u32) -> u32 {
        self.log("get_property", &[argument])
    }

    fn property_threshold_bits(&mut self, property: u32) -> u32 {
        self.log("threshold", &[property]);
        self.threshold_bits
    }

    fn aim_state(&mut self) -> u32 {
        self.aim_state
    }

    fn state_gate(&mut self) -> u32 {
        self.log("state_gate", &[])
    }

    fn aim_factor_bits(&mut self, context: u32) -> u32 {
        self.log("aim_factor", &[context]);
        self.aim_factor_bits
    }

    fn create_target(
        &mut self,
        component: u32,
        helper: u32,
        threshold: u32,
        key: u32,
        zero_first: u32,
        zero_second: u32,
    ) -> u32 {
        self.log(
            "create_target",
            &[component, helper, threshold, key, zero_first, zero_second],
        )
    }

    fn attach_target(&mut self, component: u32, target: u32) {
        self.log("attach_target", &[component, target]);
    }

    fn mark_selected(&mut self, context: u32) {
        self.log("mark_selected", &[context]);
    }

    fn chain_reset(&mut self, sub: u32) {
        self.log("chain_reset", &[sub]);
    }

    fn chain_flag(&mut self, sub: u32, flag: u32) {
        self.log("chain_flag", &[sub, flag]);
    }

    fn chain_main(&mut self, sub: u32) -> u32 {
        self.log("chain_main", &[sub])
    }

    fn lookup_model(
        &mut self,
        _ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        model_id: u32,
    ) -> u32 {
        self.log("lookup_model", &[this.get(), model_id])
    }

    fn preload_model(&mut self, model_id: u32) -> u32 {
        self.log("preload", &[model_id])
    }

    fn revive(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, flag: u32) -> u32 {
        self.log("revive", &[this.get(), flag])
    }

    fn load_bundle(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>, bundle: u32) -> u32 {
        self.log("bundle", &[this.get(), bundle])
    }
}

impl GateWorld for Fake {
    fn hook_argument(&mut self) -> u32 {
        self.log("hook_argument", &[]);
        7
    }

    fn hook_call(&mut self, argument: u32) -> u32 {
        self.log("hook_call", &[argument])
    }

    fn global_word(&mut self, which: WordGlobal) -> u32 {
        self.calls.push(("global_word", vec![which as u32]));
        self.words
            .iter()
            .find(|(w, _)| *w == which)
            .map_or(0, |(_, v)| *v)
    }

    fn global_byte(&mut self, which: ByteGlobal) -> u8 {
        self.calls.push(("global_byte", vec![which as u32]));
        self.bytes
            .iter()
            .find(|(b, _)| *b == which)
            .map_or(0, |(_, v)| *v)
    }

    fn pose_entry(&mut self, index: i32) -> u32 {
        self.log("pose_entry", &[index as u32]);
        self.pose
    }

    fn entry_flag(&mut self, entry: u32) -> u8 {
        self.log("entry_flag", &[entry]);
        self.pose_flag
    }

    fn refresh_virtual(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("refresh", &[this.get(), this.get()])
    }

    fn refresh_follow(&mut self, refreshed: u32) {
        self.log("refresh_follow", &[refreshed]);
    }

    fn attach_virtual(&mut self, inner: u32, this: Handle32<PedTag>) {
        self.log("attach", &[inner, this.get()]);
    }

    fn probe_first(&mut self, obj: u32) -> u32 {
        self.log("probe_first", &[obj])
    }

    fn probe_second(&mut self, obj: u32) -> u32 {
        self.log("probe_second", &[obj])
    }

    fn deep_refresh_gate(&mut self) -> u32 {
        self.log("deep_gate", &[])
    }

    fn deep_refresh_entry(&mut self, sub: u32) {
        self.log("deep_entry", &[sub]);
    }

    fn sub_virtual(&mut self, sub: u32) {
        self.log("sub", &[sub]);
    }

    fn ready_probe(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("ready", &[this.get()])
    }

    fn tail_virtual(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) {
        self.log("tail", &[this.get()]);
    }
}

impl TaskWorld for Fake {
    fn base_step(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("base", &[this.get()])
    }

    fn current_item(&mut self, _ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32 {
        self.log("item", &[this.get()])
    }

    fn classify_task(&mut self, _ped: &mut CPlayerPed, task: u32) -> (u32, u32) {
        let answer = self.log("classify", &[task]);
        (answer, 0)
    }

    fn task_present(&mut self, _ped: &mut CPlayerPed, list: u32, task: u32, zero: u32) -> u32 {
        self.log("task_present", &[list, task, zero])
    }

    fn award(&mut self, _ped: &mut CPlayerPed) -> u32 {
        self.log("award", &[])
    }

    fn frame_delta_bits(&mut self) -> u32 {
        self.threshold
    }

    fn cost_scale_bits(&mut self) -> u32 {
        self.threshold
    }

    fn heat_limit(&mut self) -> u8 {
        0
    }

    fn read_byte(&mut self, _address: u32) -> u8 {
        0
    }
}

/// A non-zero identity for the pedestrian in these tests.
fn this() -> Handle32<PedTag> {
    Handle32::new(0x0040_0000).expect("non-zero")
}

fn obj(raw: u32) -> ObjRef {
    Handle32::new(raw)
}

#[test]
fn absent_link_takes_the_base_update() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        answer: 9,
        ..Fake::default()
    };
    assert_eq!(ped.notify_while_linked(this(), &mut world), 9);
    assert_eq!(world.calls, vec![("base", vec![0x0040_0000])]);
}

#[test]
fn equal_link_and_target_fire_the_trigger_entry() {
    let mut ped = CPlayerPed {
        link: obj(0x1000),
        sub_b: obj(0x1000),
        ..CPlayerPed::default()
    };
    let mut world = Fake::default();
    ped.notify_while_linked(this(), &mut world);
    assert_eq!(world.calls, vec![("trigger", vec![0x0040_0000, 0, 1, 0])]);
}

#[test]
fn differing_link_and_target_take_the_base_update() {
    let mut ped = CPlayerPed {
        link: obj(0x1000),
        sub_b: obj(0x2000),
        ..CPlayerPed::default()
    };
    let mut world = Fake::default();
    ped.notify_while_linked(this(), &mut world);
    assert_eq!(world.calls, vec![("base", vec![0x0040_0000])]);
}

#[test]
fn ready_check_tests_only_the_low_byte() {
    let mut ped = CPlayerPed {
        job_word: 0x40,
        ..CPlayerPed::default()
    };
    let mut not_ready = Fake {
        answer: 0x0000_0100,
        ..Fake::default()
    };
    assert!(!ped.run_jobs_if_ready(this(), &mut not_ready));
    assert_eq!(not_ready.calls, vec![("ready", vec![0x0040_0000])]);

    let mut ready = Fake {
        answer: 0x0000_0101,
        ..Fake::default()
    };
    assert!(ped.run_jobs_if_ready(this(), &mut ready));
    assert_eq!(
        ready.calls,
        vec![
            ("ready", vec![0x0040_0000]),
            ("job_a", vec![0x0040_0000, 0xC0, 0x40]),
            ("job_b", vec![0x0040_0000, 0xC0, 0x40]),
        ]
    );
}

#[test]
fn job_bias_wraps_at_the_top_of_the_word() {
    let mut ped = CPlayerPed {
        job_word: 0xFFFF_FF80,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 1,
        ..Fake::default()
    };
    ped.run_jobs_if_ready(this(), &mut world);
    assert_eq!(world.calls[1], ("job_a", vec![0x0040_0000, 0, 0xFFFF_FF80]));
}

#[test]
fn second_job_reads_the_job_word_the_first_job_left() {
    let mut ped = CPlayerPed {
        job_word: 0x100,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 1,
        job_after_first: Some(0x200),
        ..Fake::default()
    };
    ped.run_jobs_if_ready(this(), &mut world);
    assert_eq!(world.calls[2], ("job_b", vec![0x0040_0000, 0x280, 0x200]));
}

#[test]
fn pre_pass_runs_only_for_listed_nibbles_with_the_bit_clear() {
    let run = |mode: u16, arg: u32| {
        let mut ped = CPlayerPed {
            mode,
            ..CPlayerPed::default()
        };
        let mut world = Fake::default();
        ped.reset_and_apply(this(), arg, &mut world);
        world.calls.iter().any(|(name, _)| *name == "pre")
    };
    assert!(run(0x16, 0));
    assert!(run(0x0008, 0));
    assert!(run(0x0009, 0));
    assert!(run(0x000A, 0));
    assert!(!run(0x0007, 0));
    assert!(!run(0x0010, 0));
    assert!(!run(0x0016, 0x10));
}

#[test]
fn reset_and_apply_clears_then_sets_its_words_and_notifies_the_first_sub_object() {
    let mut ped = CPlayerPed {
        pre_words: [5, 6],
        sub_a: obj(0x3000),
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 77,
        ..Fake::default()
    };
    let answer = ped.reset_and_apply(this(), 0x20, &mut world);
    assert_eq!(answer, 77);
    assert_eq!(ped.pre_words, [0, 0]);
    assert_eq!(ped.apply_words, [u32::MAX, u32::MAX]);
    assert_eq!(world.calls.last(), Some(&("notify", vec![0x3000, 8, 0])));
}

#[test]
fn sub_object_dispatch_runs_present_objects_in_order() {
    let mut ped = CPlayerPed::default();
    let mut none = Fake::default();
    assert_eq!(ped.notify_sub_objects(4, &mut none), 0);
    assert!(none.calls.is_empty());

    ped.sub_b = obj(0x5000);
    let mut only_b = Fake {
        answer: 3,
        ..Fake::default()
    };
    assert_eq!(ped.notify_sub_objects(4, &mut only_b), 3);
    assert_eq!(only_b.calls, vec![("slot8", vec![0x5000, 4])]);

    ped.sub_a = obj(0x4000);
    let mut both = Fake {
        answer: 8,
        ..Fake::default()
    };
    assert_eq!(ped.notify_sub_objects(4, &mut both), 8);
    assert_eq!(
        both.calls,
        vec![("slot8", vec![0x4000, 4]), ("slot8", vec![0x5000, 4])]
    );
}

#[test]
fn motion_value_sum_is_stored_and_handed_to_the_helper() {
    let mut ped = CPlayerPed {
        motion_value: 2.25,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 11,
        ..Fake::default()
    };
    let answer = ped.add_motion_value(this(), 1.5, &mut world);
    assert_eq!(answer, 11);
    assert_eq!(ped.motion_value.to_bits(), 3.75f32.to_bits());
    assert_eq!(
        world.calls,
        vec![("motion", vec![0x0040_0000, 3.75f32.to_bits()])]
    );
}

#[test]
fn setup_and_store_skips_the_setup_when_there_is_no_lookup() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        context: 5,
        answer: 0,
        ..Fake::default()
    };
    let answer = ped.setup_and_store(this(), 1, 2, 3, 4, 6, &mut world);
    assert_eq!(answer, 0);
    assert_eq!(
        world.calls,
        vec![("lookup_context", vec![5]), ("finalize", vec![0, 6])]
    );
}

#[test]
fn setup_and_store_returns_the_word_read_back_after_the_helper() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        context: 5,
        answer: 0x44,
        result_after_finalize: Some(0xABCD),
        ..Fake::default()
    };
    let answer = ped.setup_and_store(this(), 1, 2, 3, 4, 6, &mut world);
    assert_eq!(answer, 0xABCD);
    assert_eq!(ped.setup_result, 0xABCD);
    assert_eq!(
        world.calls[1],
        ("setup", vec![0x44, 0x0040_0000, 0, 1, 2, 3, 4])
    );
}

#[test]
fn change_model_with_an_inactive_model_only_looks_up() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        answer: 21,
        model_state: 0,
        ..Fake::default()
    };
    assert_eq!(ped.change_model(this(), 3, &mut world), 21);
    assert_eq!(world.calls, vec![("lookup_model", vec![0x0040_0000, 3])]);
}

#[test]
fn change_model_revives_with_flag_zero_only_for_the_all_ones_slot() {
    let run = |slot: u32| {
        let mut ped = CPlayerPed {
            current_slot: slot,
            job_word: 0x1234,
            ..CPlayerPed::default()
        };
        let mut world = Fake {
            answer: 1,
            model_state: 2,
            ..Fake::default()
        };
        ped.change_model(this(), 3, &mut world);
        world.calls
    };
    let taken = run(u32::MAX);
    assert_eq!(taken[0], ("preload", vec![3]));
    assert_eq!(taken[2], ("revive", vec![0x0040_0000, 0]));
    assert_eq!(taken[3], ("bundle", vec![0x0040_0000, 0x1234]));
    let free = run(7);
    assert_eq!(free[2], ("revive", vec![0x0040_0000, 1]));
}

#[test]
fn set_transform_returns_the_fourth_word_and_keeps_bits() {
    let mut ped = CPlayerPed::default();
    let nan = 0x7FC0_0001;
    let answer = ped.set_transform([1, 0x8000_0000, nan, 0xDEAD_BEEF]);
    assert_eq!(answer, 0xDEAD_BEEF);
    assert_eq!(ped.transform, [1, 0x8000_0000, nan, 0xDEAD_BEEF]);
}

#[test]
fn set_descriptor_copies_the_four_words() {
    let mut ped = CPlayerPed::default();
    ped.set_descriptor([9, 8, 7, 6]);
    assert_eq!(ped.descriptor, [9, 8, 7, 6]);
}

#[test]
fn registry_lists_every_verified_method_once() {
    assert_eq!(registry::proven_count(), 14);
    assert_eq!(registry::not_lifted_count(), 7);
    let proven = registry::ROWS
        .iter()
        .filter(|r| r.state == State::Proven)
        .count();
    assert_eq!(proven + registry::not_lifted_count(), 21);
}

#[test]
fn entry_chain_offsets_the_base_once_and_runs_three_entries() {
    let mut ped = CPlayerPed {
        chain_base: 0x1000,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 42,
        ..Fake::default()
    };
    assert_eq!(ped.run_entry_chain(&mut world), 42);
    assert_eq!(
        world.calls,
        vec![
            ("chain_reset", vec![0x1084]),
            ("chain_flag", vec![0x1084, 1]),
            ("chain_main", vec![0x1084]),
        ]
    );
}

#[test]
fn entry_chain_on_a_zero_base_still_runs_on_the_offset() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake::default();
    ped.run_entry_chain(&mut world);
    assert_eq!(world.calls[0], ("chain_reset", vec![0x84]));
}

fn aim_fake(threshold_bits: u32, aim_state: u32, aim_factor_bits: u32) -> Fake {
    Fake {
        answer: 1,
        threshold_bits,
        aim_state,
        aim_factor_bits,
        ..Fake::default()
    }
}

#[test]
fn aim_acquisition_stops_when_there_is_no_context() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake::default();
    assert!(!ped.acquire_aim_target(this(), 9, 0, &mut world));
    assert_eq!(
        world.calls,
        vec![("get_context", vec![0x0040_0000 + 0x2B0])]
    );
}

#[test]
fn aim_acquisition_uses_the_plain_threshold_when_the_state_is_not_scaled() {
    let mut ped = CPlayerPed {
        aim_context: 0x7000,
        ..CPlayerPed::default()
    };
    let mut world = aim_fake(1.5f32.to_bits(), 1, 2.0f32.to_bits());
    assert!(ped.acquire_aim_target(this(), 9, 1, &mut world));
    assert!(!world.calls.iter().any(|(n, _)| *n == "aim_factor"));
    let create = world
        .calls
        .iter()
        .find(|(n, _)| *n == "create_target")
        .expect("created");
    assert_eq!(create.1[1], 0x22E4);
    assert_eq!(create.1[2], 1.5f32.to_bits());
    assert_eq!(world.calls.last(), Some(&("mark_selected", vec![0x7000])));
}

#[test]
fn aim_acquisition_scales_the_threshold_by_the_aim_factor() {
    let mut ped = CPlayerPed {
        aim_context: 0x7000,
        ..CPlayerPed::default()
    };
    let mut world = aim_fake(1.5f32.to_bits(), 2, 2.0f32.to_bits());
    assert!(ped.acquire_aim_target(this(), 9, 0, &mut world));
    let create = world
        .calls
        .iter()
        .find(|(n, _)| *n == "create_target")
        .expect("created");
    assert_eq!(create.1[1], 0x22E8);
    assert_eq!(create.1[2], 3.0f32.to_bits());
}

#[test]
fn aim_acquisition_keeps_a_not_a_number_payload_quiet() {
    let quiet_nan = 0x7F80_0001u32;
    let mut ped = CPlayerPed {
        aim_context: 0x7000,
        ..CPlayerPed::default()
    };
    let mut world = aim_fake(1.5f32.to_bits(), 2, quiet_nan);
    ped.acquire_aim_target(this(), 9, 0, &mut world);
    let create = world
        .calls
        .iter()
        .find(|(n, _)| *n == "create_target")
        .expect("created");
    assert_eq!(create.1[2], quiet_nan | 0x0040_0000);
}

#[test]
fn aim_acquisition_without_an_aim_context_creates_but_does_not_mark() {
    let mut ped = CPlayerPed::default();
    let mut world = aim_fake(1.5f32.to_bits(), 2, 2.0f32.to_bits());
    assert!(!ped.acquire_aim_target(this(), 9, 0, &mut world));
    assert!(world.calls.iter().any(|(n, _)| *n == "create_target"));
    assert!(!world.calls.iter().any(|(n, _)| *n == "mark_selected"));
}

/// A recording world whose point memory and threshold the relink tests set.
fn relink_fake(threshold: f32, answer: u32) -> Fake {
    let mut memory = std::collections::BTreeMap::new();
    // Source point at zero; target point three, four, zero away.
    for (off, value) in [(0x30u32, 0.0f32), (0x34, 0.0), (0x38, 0.0)] {
        memory.insert(0x100 + off, value.to_bits());
    }
    for (off, value) in [(0x40u32, 3.0f32), (0x44, 4.0), (0x48, 0.0)] {
        memory.insert(0x200 + off, value.to_bits());
    }
    Fake {
        answer,
        threshold: threshold.to_bits(),
        memory,
        ..Fake::default()
    }
}

#[test]
fn relink_runs_when_the_flag_is_set() {
    let mut ped = CPlayerPed {
        source: 0x100,
        link: obj(0x1000),
        sub_b: obj(0x200),
        ..CPlayerPed::default()
    };
    let mut world = relink_fake(1.0, 77);
    assert_eq!(ped.relink_on_move(this(), 1, &mut world), 77);
    assert_eq!(world.calls.last(), Some(&("relink", vec![0x0040_0000, 1])));
}

#[test]
fn relink_is_skipped_for_a_short_move_without_the_flag() {
    let mut ped = CPlayerPed {
        source: 0x100,
        link: obj(0x1000),
        sub_b: obj(0x200),
        ..CPlayerPed::default()
    };
    // Squared distance 25 against a threshold of 100: not far enough.
    let mut world = relink_fake(100.0, 77);
    assert_eq!(ped.relink_on_move(this(), 0x100, &mut world), 0x100);
    assert!(!world.calls.iter().any(|(n, _)| *n == "relink"));
}

#[test]
fn relink_with_an_equal_link_only_runs_the_base_entry() {
    let mut ped = CPlayerPed {
        source: 0x100,
        link: obj(0x200),
        sub_b: obj(0x200),
        ..CPlayerPed::default()
    };
    let mut world = relink_fake(0.0, 5);
    assert_eq!(ped.relink_on_move(this(), 1, &mut world), 0x200);
    assert_eq!(world.calls, vec![("base_entry", vec![0x0040_0000, 1])]);
}

#[test]
fn relink_flag_clears_the_block_and_marks_only_with_the_marker() {
    let mut ped = CPlayerPed {
        source: 0x100,
        link: obj(0x1000),
        sub_b: obj(0x200),
        relink_block: [1; 9],
        relink_flags: 0x10,
        marker_219: 1,
        ..CPlayerPed::default()
    };
    let mut world = relink_fake(1.0, 77);
    ped.relink_on_move(this(), 1, &mut world);
    assert_eq!(ped.relink_block, [0; 9]);
    assert_eq!(ped.relink_flags, 0x1C);

    let mut quiet = CPlayerPed {
        relink_block: [1; 9],
        relink_flags: 0x10,
        marker_219: 0,
        source: 0x100,
        link: obj(0x1000),
        sub_b: obj(0x200),
        ..CPlayerPed::default()
    };
    let mut world = relink_fake(1.0, 77);
    quiet.relink_on_move(this(), 1, &mut world);
    assert_eq!(quiet.relink_flags, 0x10);
}

fn gate_fake(answer: u32) -> Fake {
    Fake {
        answer,
        ..Fake::default()
    }
}

#[test]
fn refresh_gate_refuses_when_an_or_byte_is_set_without_the_guard() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        bytes: vec![(ByteGlobal::OrFirst, 1)],
        ..gate_fake(1)
    };
    assert_eq!(ped.refresh_gate(this(), &mut world), 0);
    assert!(!world.calls.iter().any(|(n, _)| *n == "refresh"));
}

#[test]
fn refresh_gate_refuses_on_the_state_word_after_the_refresh() {
    let mut ped = CPlayerPed::default();
    let mut world = Fake {
        words: vec![(WordGlobal::State, 1)],
        ..gate_fake(0)
    };
    assert_eq!(ped.refresh_gate(this(), &mut world), 0);
    assert!(world.calls.iter().any(|(n, _)| *n == "refresh"));
    assert!(!world.calls.iter().any(|(n, _)| *n == "ready"));
}

#[test]
fn refresh_gate_with_a_zero_readiness_runs_the_tail_and_arms_the_word() {
    let mut ped = CPlayerPed {
        status_29c: 0x2,
        ..CPlayerPed::default()
    };
    let mut world = gate_fake(0);
    assert_eq!(ped.refresh_gate(this(), &mut world), 0);
    assert_eq!(world.calls.last(), Some(&("tail", vec![0x0040_0000])));
    assert_eq!(ped.status_29c, 0x0100_0002);
}

#[test]
fn refresh_gate_attaches_only_with_an_entry_flag_and_the_marker() {
    let mut ped = CPlayerPed {
        outer: 0x9000,
        marker_219: 1,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        pose: 0x5000,
        pose_flag: 1,
        bytes: vec![(ByteGlobal::Guard, 1)],
        ..gate_fake(1)
    };
    ped.refresh_gate(this(), &mut world);
    assert!(world.calls.iter().any(|(n, _)| *n == "attach"));

    let mut quiet = CPlayerPed {
        outer: 0x9000,
        marker_219: 0,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        pose: 0x5000,
        pose_flag: 1,
        bytes: vec![(ByteGlobal::Guard, 1)],
        ..gate_fake(1)
    };
    quiet.refresh_gate(this(), &mut world);
    assert!(!world.calls.iter().any(|(n, _)| *n == "attach"));
}

#[test]
fn budget_step_without_the_marker_answers_the_base_step_only() {
    let mut ped = CPlayerPed {
        chain_base: 0x1234,
        ..CPlayerPed::default()
    };
    let mut world = Fake {
        answer: 42,
        ..Fake::default()
    };
    assert_eq!(ped.settle_budget(this(), &mut world), 42);
    assert_eq!(world.calls, vec![("base", vec![0x0040_0000])]);
}

#[test]
fn budget_step_with_an_empty_task_list_answers_the_task_record() {
    let mut ped = CPlayerPed {
        marker_219: 1,
        chain_base: 0x1234,
        ..CPlayerPed::default()
    };
    let mut world = Fake::default();
    assert_eq!(ped.settle_budget(this(), &mut world), 0x1234);
    assert_eq!(world.calls, vec![("base", vec![0x0040_0000])]);
}
