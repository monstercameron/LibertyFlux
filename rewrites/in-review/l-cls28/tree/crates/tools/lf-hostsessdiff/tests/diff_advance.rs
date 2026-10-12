//! Differential cases: the hosting task's countdown and state machine.
//!
//! The cases exercise each state, both open forms, key resolution, poll
//! outcomes, the repeat edge, virtual reports, and the task words written.
//! The wrong countdown and wrong open selector must be caught.

#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_hostsessdiff::rewrites::*;
    use lf_hostsessdiff::rt::{self, Spec, StubKind};

    #[path = "support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, Stubs, VTable, S_MODE, S_PORT, S_PRIMARY, SLOT_COMPLETE,
        T_CONFIG, T_COUNTDOWN, T_EMPTY, T_FIRST, T_GAMERS, T_PROBE, T_SECOND, T_SESSION,
        T_SPLIT, T_STATE, TASK_SIZE, SESSION_SIZE, assert_only_changed, assert_task_matches,
        lift_task,
    };

    mod wrong {
        /// Omits the original's exact-zero replacement after subtraction.
        pub fn countdown(value: u32, delta: u32) -> u32 {
            value.wrapping_sub(delta)
        }

        /// Selects the plain open path even when the split word is nonzero.
        pub fn open_role(_: u32) -> &'static str {
            "open_plain"
        }

    }

    fn spec(
        id: u32,
        kind: StubKind,
        answers: Vec<u32>,
        snaps: Vec<(usize, usize)>,
        outs: Vec<Vec<(usize, Vec<u32>)>>,
    ) -> Spec {
        Spec { id, kind, answers, snaps, outs }
    }

    fn fixture(rng: &mut Rng, mode: u32) -> (Image, Image, VTable, Stubs) {
        let mut obj = Image::random(TASK_SIZE, rng);
        let mut session = Image::random(SESSION_SIZE, rng);
        let mut vtable = VTable::random(8, rng);
        let stubs = Stubs::new();
        vtable.set(SLOT_COMPLETE, stubs.complete);
        obj.w32(0, vtable.addr());
        obj.w32(T_SESSION, session.addr());
        session.w32(S_MODE, mode);
        (obj, session, vtable, stubs)
    }

    #[test]
    fn advance_hosted_path_matches_in_call_order() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x62_2440);
        let (mut obj, session, _vtable, _stubs) = fixture(&mut rng, 3);
        obj.w32(T_COUNTDOWN, 1);
        obj.w32(T_STATE, 0);
        obj.w32(T_PROBE, 0);
        obj.w32(T_CONFIG, 0);
        obj.w32(T_SPLIT, 1);
        obj.w32(T_FIRST, 4);
        obj.w32(T_SECOND, 5);
        obj.w32(T_EMPTY, 2);
        obj.w32(T_GAMERS, 0x1122_3344);
        obj.w32(T_GAMERS + 4, 0x5566_7788);
        obj.w32(T_GAMERS + 8, 0x99AA_BBCC);
        let before = obj.buf.clone();
        let mut lifted = lift_task(&obj);
        let keys = [0x10, 0x20, 0x30, 0x40];
        rt::set_image_base(0x1000_0000);
        rt::set_cells(0x1234_5678, 2, keys);
        rt::set_script(&[
            spec(1, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(2, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(3, StubKind::Thiscall4, vec![0], vec![], vec![]),
            spec(
                4,
                StubKind::Thiscall7,
                vec![1],
                vec![],
                vec![vec![(7, vec![3])]],
            ),
            spec(5, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(6, StubKind::Thiscall4, vec![1], vec![(1, 4)], vec![]),
            spec(7, StubKind::Thiscall1, vec![0], vec![], vec![]),
            spec(8, StubKind::Thiscall0, vec![1, 1, 1, 0], vec![], vec![]),
        ]);
        let mut fake = Fake::new();
        fake.answer("open_hosted", vec![1])
            .out_words("open_hosted.probe", vec![vec![3]])
            .answer("resolve_key", vec![1])
            .answer("progress", vec![1, 1, 1, 0]);

        let got = unsafe { fn_00662440::rw_00662440(obj.addr(), 1) };
        lifted.advance(
            &mut fake,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base: 0x1000_0000,
                cookie: 0x1234_5678,
                setup_flag: 2,
                setup_key: keys,
            },
            1,
        );

        assert_eq!(got, 0);
        assert_task_matches(&obj, &lifted);
        assert_eq!(lifted.countdown, u32::MAX);
        assert_eq!(wrong::countdown(1, 1), 0);
        assert_ne!(wrong::countdown(1, 1), lifted.countdown, "wrong countdown must fail");
        assert_eq!(wrong::open_role(lifted.list_split), "open_plain");
        assert_eq!(fake.log.iter().find(|(n, _)| n == "open_hosted").unwrap().0, "open_hosted");
        assert_ne!(wrong::open_role(lifted.list_split), "open_hosted", "wrong selector must fail");

        let calls = rt::take_numbered();
        assert_eq!(calls.iter().map(|(id, _)| *id).collect::<Vec<_>>(), [1, 2, 4, 5, 6, 8, 8, 8, 8]);
        assert_eq!(calls[0], (1, vec![session.addr().wrapping_add(S_PRIMARY as u32)]));
        assert_eq!(calls[1], (2, vec![session.addr()]));
        assert_eq!(
            calls[2],
            (
                4,
                vec![
                    session.addr().wrapping_add(S_PORT as u32),
                    lifted.config,
                    lifted.list_split,
                    lifted.first_count,
                    lifted.second_count,
                    obj.at(T_GAMERS),
                    lifted.open_extra,
                    obj.at(T_PROBE),
                ],
            )
        );
        assert_eq!(calls[3], (5, vec![obj.addr()]));
        assert_eq!(calls[4].0, 6);
        assert_eq!(calls[4].1[0], session.addr().wrapping_add(S_PORT as u32));
        assert_eq!(&calls[4].1[2..], &[obj.at(T_EMPTY), 1, obj.at(T_PROBE)]);
        assert_eq!(calls[4].1[1] != 0, true, "frame pointer is present");
        for call in &calls[5..] {
            assert_eq!(*call, (8, vec![obj.addr()]));
        }
        assert_eq!(rt::take_snaps(), vec![(6, 1, keys.to_vec())]);
        let virtuals = rt::take_virtual();
        assert_eq!(virtuals, vec![("complete".to_string(), vec![obj.addr(), 1, 0])]);
        assert_eq!(
            fake.log,
            vec![
                ("prepare_primary".to_string(), vec![session.addr()]),
                ("prepare_session".to_string(), vec![session.addr()]),
                ("open_hosted".to_string(), vec![session.addr(), 0, 1, 4, 5, obj.r32(0x4bc), 0]),
                ("promote_hosted".to_string(), vec![]),
                ("resolve_key".to_string(), vec![session.addr(), 0x10, 0x20, 0x30, 0x40, 2, 3]),
                ("progress".to_string(), vec![]),
                ("progress".to_string(), vec![]),
                ("progress".to_string(), vec![]),
                ("own_complete".to_string(), vec![1, 0]),
                ("progress".to_string(), vec![]),
            ]
        );
        assert_only_changed(
            &before,
            &obj.buf,
            &[(T_COUNTDOWN, T_COUNTDOWN + 4), (T_STATE, T_STATE + 4), (T_PROBE, T_PROBE + 4)],
        );
    }

    #[test]
    fn advance_plain_failure_and_probe_short_circuit_match() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x62_2441);
        let (mut obj, session, _vtable, _stubs) = fixture(&mut rng, 2);
        obj.w32(T_STATE, 0);
        obj.w32(T_PROBE, 0);
        obj.w32(T_CONFIG, 5);
        obj.w32(T_SPLIT, 0);
        let mut lifted = lift_task(&obj);
        let before = obj.buf.clone();
        rt::set_cells(0, 0, [0; 4]);
        rt::set_script(&[
            spec(1, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(2, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(3, StubKind::Thiscall4, vec![0], vec![], vec![]),
            spec(4, StubKind::Thiscall7, vec![0], vec![], vec![]),
            spec(5, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(6, StubKind::Thiscall4, vec![0], vec![(1, 4)], vec![]),
            spec(7, StubKind::Thiscall1, vec![0], vec![], vec![]),
            spec(8, StubKind::Thiscall0, vec![0], vec![], vec![]),
        ]);
        let mut fake = Fake::new();
        fake.answer("open_plain", vec![0]).answer("progress", vec![0]);
        let got = unsafe { fn_00662440::rw_00662440(obj.addr(), 0) };
        lifted.advance(
            &mut fake,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base: 0,
                cookie: 0,
                setup_flag: 0,
                setup_key: [0; 4],
            },
            0,
        );
        assert_eq!(got, 0);
        assert_task_matches(&obj, &lifted);
        assert_eq!(rt::take_numbered().iter().map(|(id, _)| *id).collect::<Vec<_>>(), [1, 2, 3, 8]);
        assert_eq!(rt::take_virtual(), vec![("complete".to_string(), vec![obj.addr(), 0, 0])]);
        assert_eq!(
            fake.log,
            vec![
                ("prepare_primary".to_string(), vec![session.addr()]),
                ("prepare_session".to_string(), vec![session.addr()]),
                ("open_plain".to_string(), vec![session.addr(), 5, lifted.first_count, 0]),
                ("own_complete".to_string(), vec![0, 0]),
                ("progress".to_string(), vec![]),
            ]
        );
        assert_only_changed(&before, &obj.buf, &[]);

        let (mut obj, _session, _vtable, _stubs) = fixture(&mut rng, 2);
        obj.w32(T_STATE, 1);
        obj.w32(T_PROBE, 1);
        let mut lifted = lift_task(&obj);
        rt::set_script(&[
            spec(1, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(2, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(3, StubKind::Thiscall4, vec![0], vec![], vec![]),
            spec(4, StubKind::Thiscall7, vec![0], vec![], vec![]),
            spec(5, StubKind::Thiscall0, vec![0], vec![], vec![]),
            spec(6, StubKind::Thiscall4, vec![0], vec![(1, 4)], vec![]),
            spec(7, StubKind::Thiscall1, vec![0], vec![], vec![]),
            spec(8, StubKind::Thiscall0, vec![0], vec![], vec![]),
        ]);
        let mut fake = Fake::new();
        let _ = unsafe { fn_00662440::rw_00662440(obj.addr(), 0) };
        lifted.advance(
            &mut fake,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base: 0,
                cookie: 0,
                setup_flag: 0,
                setup_key: [0; 4],
            },
            0,
        );
        assert_task_matches(&obj, &lifted);
        assert!(rt::take_numbered().is_empty());
        assert!(rt::take_virtual().is_empty());
        assert!(fake.log.is_empty());
    }

    #[test]
    fn advance_routes_all_remaining_states() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x62_2442);
        let scenarios = [
            (1, 3, 0, 0, 0, vec![0], 2, vec![5, 8]),
            (2, 0, 1, 0, 0, vec![0], 4, vec![8]),
            (2, 0, 0, 0, 0, vec![0], 4, vec![8]),
            (2, 0, 0, 2, 0, vec![0], 4, vec![6, 8]),
            (2, 3, 0, 2, 1, vec![0], 3, vec![6, 8]),
            (3, 3, 0, 0, 0, vec![0], 3, vec![8]),
            (3, 1, 0, 0, 0, vec![], 3, vec![]),
            (3, 7, 0, 0, 0, vec![1, 0], 4, vec![7, 8, 8]),
            (4, 0, 0, 0, 0, vec![0], 4, vec![7, 8]),
            (4, 0, 0, 0, 1, vec![1, 0], 5, vec![7, 8, 8]),
            (5, 1, 0, 0, 0, vec![], 5, vec![]),
            (5, 0, 0, 0, 0, vec![0], 5, vec![8]),
            (6, 0, 0, 0, 0, vec![0], 6, vec![8]),
        ];
        for (state, probe, config, flag, answer, progress, final_state, expected_ids) in scenarios {
            let (mut obj, session, _vtable, _stubs) = fixture(&mut rng, 2);
            obj.w32(T_STATE, state);
            obj.w32(T_PROBE, probe);
            obj.w32(T_CONFIG, config);
            obj.w32(T_SPLIT, 1);
            let mut lifted = lift_task(&obj);
            rt::set_cells(0, flag, [1, 2, 3, 4]);
            rt::set_script(&[
                spec(1, StubKind::Thiscall0, vec![0], vec![], vec![]),
                spec(2, StubKind::Thiscall0, vec![0], vec![], vec![]),
                spec(3, StubKind::Thiscall4, vec![0], vec![], vec![]),
                spec(4, StubKind::Thiscall7, vec![0], vec![], vec![]),
                spec(5, StubKind::Thiscall0, vec![answer], vec![], vec![]),
                spec(6, StubKind::Thiscall4, vec![answer], vec![(1, 4)], vec![]),
                spec(7, StubKind::Thiscall1, vec![answer], vec![], vec![]),
                spec(8, StubKind::Thiscall0, progress.clone(), vec![], vec![]),
            ]);
            let mut fake = Fake::new();
            if !progress.is_empty() {
                fake.answer("progress", progress.clone());
            }
            if state == 2 && config == 0 && flag & 2 != 0 {
                fake.answer("resolve_key", vec![answer]);
            }
            if state == 4 {
                fake.answer("poll_host", vec![answer]);
            }
            let _ = unsafe { fn_00662440::rw_00662440(obj.addr(), 0) };
            lifted.advance(
                &mut fake,
                &lf_network::host_session_task::HostSessionGlobals {
                    image_base: 0,
                    cookie: 0,
                    setup_flag: flag as u8,
                    setup_key: [1, 2, 3, 4],
                },
                0,
            );
            assert_eq!(lifted.state, final_state, "state {state}, probe {probe}");
            assert_task_matches(&obj, &lifted);
            let ids = rt::take_numbered().iter().map(|(id, _)| *id).collect::<Vec<_>>();
            assert_eq!(ids, expected_ids, "state {state}, probe {probe}");
            let snaps = rt::take_snaps();
            if state == 2 && config == 0 && flag & 2 != 0 {
                assert_eq!(snaps, vec![(6, 1, vec![1, 2, 3, 4])]);
            } else {
                assert!(snaps.is_empty(), "unexpected snapshots in state {state}");
            }
            let virtuals = rt::take_virtual();
            let expected_virtual = if state == 3 && probe == 3 {
                vec![("complete".to_string(), vec![obj.addr(), 1, 0])]
            } else if (state == 4 && answer == 0)
                || (state == 5 && probe != 1)
                || (state == 3 && probe != 1 && probe != 3)
            {
                vec![("complete".to_string(), vec![obj.addr(), 0, 0])]
            } else {
                vec![]
            };
            assert_eq!(virtuals, expected_virtual, "state {state}, probe {probe}");
            let _ = session;
        }
    }
}
