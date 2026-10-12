//! Differential cases: the hosting task start (vf2).
//!
//! Each case runs the verified rewrite and the lifted `start` on the same
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order, with a deliberately wrong lift caught
//! alongside. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_hostsessdiff::rewrites::*;
    use lf_hostsessdiff::rt::{self};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, S_MODE, SLOT_COMPLETE, Stubs, T_SESSION, T_STARTED, T_STATE, TASK_SIZE,
        VTable, assert_only_changed, assert_task_matches, check_virtual, lift_task,
    };

    mod wrong {
        /// Forgets state 3 and admits state 4 into the live set.
        pub fn live(mode: u32) -> bool {
            mode == 2 || mode == 4
        }
    }

    #[test]
    fn start_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x51A7_7);
        let mut caught = 0;
        let mut cases = 0;
        for &started in &[0u32, 1, 5, 0xFFFF_FFFF] {
            for &mode in &[0u32, 1, 2, 3, 4, 0xFFFF_FFFF] {
                let mut obj = Image::random(TASK_SIZE, &mut rng);
                let mut session = Image::random(0x60, &mut rng);
                let mut vtable = VTable::random(8, &mut rng);
                let stubs = Stubs::new();
                vtable.set(SLOT_COMPLETE, stubs.complete);
                obj.w32(0, vtable.addr());
                obj.w32(T_SESSION, session.addr());
                obj.w32(T_STARTED, started);
                session.w32(S_MODE, mode);
                let state_before = obj.r32(T_STATE);
                let task = lift_task(&obj);
                let before = obj.buf.clone();
                rt::set_script(&[]);
                let mut fake = Fake::new();
                fake.answer("session_state", vec![mode]);
                let got = unsafe { fn_006621c0::rw_006621c0(obj.addr()) };
                let mut lifted = task.clone();
                lifted.start(&mut fake);
                assert_eq!(got, 0, "started {started:#x} mode {mode:#x}");
                assert_eq!(
                    obj.r32(T_STARTED),
                    if started == 0 { 1 } else { started },
                    "latch"
                );
                assert_eq!(
                    lifted.started,
                    if started == 0 { 1 } else { started },
                    "lift latch word"
                );
                let live = mode == 2 || mode == 3;
                if live {
                    assert_eq!(obj.r32(T_STATE), state_before, "live keeps state");
                    assert_eq!(lifted.state, state_before, "lift keeps state");
                    check_virtual(
                        rt::take_virtual(),
                        vec![("complete", vec![obj.addr(), 0, 0])],
                    );
                    assert_eq!(
                        fake.log,
                        vec![
                            ("session_state".to_string(), vec![session.addr()]),
                            ("own_complete".to_string(), vec![0, 0]),
                        ],
                        "lift calls"
                    );
                    assert_only_changed(&before, &obj.buf, &[(T_STARTED, T_STARTED + 4)]);
                } else {
                    assert_eq!(obj.r32(T_STATE), 0, "idle parks state");
                    assert_eq!(lifted.state, 0, "lift parks state");
                    assert!(rt::take_virtual().is_empty(), "no report when idle");
                    assert_eq!(
                        fake.log,
                        vec![("session_state".to_string(), vec![session.addr()])],
                        "lift calls"
                    );
                    assert_only_changed(
                        &before,
                        &obj.buf,
                        &[(T_STARTED, T_STARTED + 4), (T_STATE, T_STATE + 4)],
                    );
                }
                assert_task_matches(&obj, &lifted);
                assert!(rt::take_numbered().is_empty(), "no numbered calls");
                if wrong::live(mode) != live {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 24, "too few cases: {cases}");
        assert!(caught > 0, "wrong start never caught");
    }
}
