//! Differential cases: the hosting task's completion entry.
//!
//! The cases cover both main-path list variants, key-block variants, the
//! live side path, and the non-live tail. They compare task/session/entry
//! writes, ordered calls, descriptor tags and snapshotted frame words.

#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_hostsessdiff::rewrites::*;
    use lf_hostsessdiff::rt::{self, Spec, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, S_AUX, S_FLAG, S_MIR_DST, S_MIR_SRC, S_MODE, S_PORT, S_SUB, S_TIMER,
        SESSION_SIZE, SLOT_COMPLETE, Stubs, T_CONFIG, T_EMPTY, T_PROBE, T_SESSION, T_SPLIT,
        T_TRAILER, T_TSIZE, TASK_SIZE, VTable, assert_only_changed, assert_task_matches, lift_task,
    };

    const ENTRY_FLAG: usize = 0x80;

    mod wrong {
        /// Incorrectly admits state 4 into the live range.
        pub fn live(mode: u32) -> bool {
            (2..=4).contains(&(mode as i32))
        }

        /// Sets the wrong key-ready bit in the session flag byte.
        pub fn keyed_flags(flags: u8) -> u8 {
            flags | 0x40
        }
    }

    fn spec(id: u32, kind: StubKind, answers: Vec<u32>, snaps: Vec<(usize, usize)>) -> Spec {
        Spec {
            id,
            kind,
            answers,
            snaps,
            outs: vec![],
        }
    }

    fn main_case(split: u32, config: u32, trailer_size: u32) {
        let mut rng = Rng(0x62_2200 ^ u64::from(split) ^ u64::from(config));
        let mut obj = Image::random(TASK_SIZE, &mut rng);
        let mut session = Image::random(SESSION_SIZE, &mut rng);
        let mut entry = Image::random(0x100, &mut rng);
        let mut vtable = VTable::random(8, &mut rng);
        let stubs = Stubs::new();
        vtable.set(SLOT_COMPLETE, stubs.complete);
        obj.w32(0, vtable.addr());
        obj.w32(T_SESSION, session.addr());
        obj.w32(T_CONFIG, config);
        obj.w32(T_SPLIT, split);
        obj.w32(T_EMPTY, 7);
        obj.w32(T_TSIZE, trailer_size);
        obj.w32(T_TRAILER, 0x1111_2222);
        obj.w32(T_TRAILER + 4, 0x3333_4444);
        session.w32(S_MODE, 3);
        session.w32(S_MIR_SRC, 0xAABB_CCDD);
        session.w32(S_MIR_SRC + 4, 0x5566_7788);
        session.w32(S_MIR_DST, 0xDEAD_BEEF);
        session.w32(S_MIR_DST + 4, 0xDEAD_BEEF);
        session.w8(S_FLAG, 0x0A);
        entry.w8(ENTRY_FLAG, 0x41);

        let obj_before = obj.buf.clone();
        let session_before = session.buf.clone();
        let entry_before = entry.buf.clone();
        let mut lifted = lift_task(&obj);
        lifted.trailer = if trailer_size == 0 {
            vec![]
        } else {
            vec![obj.r32(T_TRAILER), obj.r32(T_TRAILER + 4)]
        };
        let mut world = Fake::new();
        world.session_flag_byte = session.r8(S_FLAG);
        world.entry_flag_byte = entry.r8(ENTRY_FLAG);
        world.mirror_source = [session.r32(S_MIR_SRC), session.r32(S_MIR_SRC + 4)];
        world.answer("session_state", vec![3]);
        world.answer("alloc_entry", vec![entry.addr()]);

        let image_base = 0x1000_0000;
        let cookie = 0xCAFE_BABE;
        rt::set_image_base(image_base);
        rt::set_cells(cookie, 0, [0; 4]);
        let mut specs = vec![
            spec(2, StubKind::Thiscall2, vec![0], vec![]),
            spec(3, StubKind::Thiscall1, vec![0, 0], vec![(1, 1)]),
            spec(4, StubKind::Thiscall0, vec![0], vec![(0, 1)]),
            spec(5, StubKind::Thiscall0, vec![0], vec![(0, 1)]),
            spec(7, StubKind::Thiscall4, vec![0], vec![]),
            spec(8, StubKind::Thiscall4, vec![0], vec![(1, 1)]),
            spec(9, StubKind::Thiscall1, vec![entry.addr()], vec![(1, 1)]),
            spec(10, StubKind::Thiscall0, vec![0], vec![]),
        ];
        if split != 0 {
            specs.push(spec(1, StubKind::Thiscall1, vec![0], vec![]));
        }
        if config == 0 {
            specs.push(spec(6, StubKind::Thiscall1, vec![0], vec![(0, 1)]));
        }
        rt::set_script(&specs);

        let got = fn_00662200::rw_00662200(obj.addr(), 1, 0xBEEF);
        lifted.complete(
            &mut world,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base,
                cookie,
                setup_flag: 0,
                setup_key: [0; 4],
            },
            1,
            0xBEEF,
        );

        assert_eq!(got, 0);
        assert_task_matches(&obj, &lifted);
        assert_eq!(obj.r32(T_SESSION), 0);
        let expected_flags = if split == 0 { 0x28 } else { 0x2A };
        assert_eq!(session.r8(S_FLAG), expected_flags);
        assert_eq!(world.session_flag_byte, expected_flags);
        assert_eq!(session.r32(S_MIR_DST), world.mirror_result.unwrap()[0]);
        assert_eq!(session.r32(S_MIR_DST + 4), world.mirror_result.unwrap()[1]);
        assert_eq!(entry.r8(ENTRY_FLAG), 0x43);
        assert_eq!(world.entry_flag_byte, 0x43);
        assert_ne!(
            wrong::keyed_flags(0x08),
            session.r8(S_FLAG),
            "wrong bit must fail"
        );

        let numbers = rt::take_numbered();
        let ids = numbers.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        let mut expected_ids = vec![2, 3, 3, 4, 5];
        if config == 0 {
            expected_ids.push(6);
        }
        expected_ids.extend([7, 8, 9, 10]);
        if split != 0 {
            expected_ids.insert(0, 1);
        }
        assert_eq!(ids, expected_ids);
        let mut call_index = 0;
        if split != 0 {
            assert_eq!(numbers[call_index], (1, vec![session.addr(), 1]));
            call_index += 1;
        }
        assert_eq!(
            numbers[call_index],
            (2, vec![obj.addr(), 1, 0xBEEF]),
            "notify arguments"
        );
        call_index += 1;
        assert_eq!(numbers[call_index].0, 3);
        assert_eq!(numbers[call_index].1[0], session.addr());
        assert_ne!(
            numbers[call_index].1[1], 0,
            "first descriptor pointer is present"
        );
        call_index += 1;
        assert_eq!(numbers[call_index].0, 3);
        assert_eq!(numbers[call_index].1[0], session.addr());
        assert_ne!(
            numbers[call_index].1[1], 0,
            "second descriptor pointer is present"
        );
        call_index += 1;
        assert_ne!(
            numbers[call_index].1[0], 0,
            "first frame word pointer is present"
        );
        call_index += 1;
        assert_ne!(
            numbers[call_index].1[0], 0,
            "second frame word pointer is present"
        );
        call_index += 1;
        if config == 0 {
            assert_ne!(
                numbers[call_index].1[0], 0,
                "table frame pointer is present"
            );
            assert_eq!(numbers[call_index].1[1], image_base + 0x019F_3230);
            call_index += 1;
        }
        assert_eq!(numbers[call_index], (7, vec![session.addr(), 0, 0, 0, 0]));
        call_index += 1;
        assert_eq!(numbers[call_index].0, 8);
        assert_eq!(numbers[call_index].1[0], session.addr());
        assert_ne!(numbers[call_index].1[1], 0, "key record pointer is present");
        assert_eq!(numbers[call_index].1[2], 7);
        assert_eq!(
            numbers[call_index].1[3],
            if trailer_size == 0 {
                0
            } else {
                obj.at(T_TRAILER)
            }
        );
        assert_eq!(numbers[call_index].1[4], trailer_size);
        call_index += 1;
        assert_eq!(numbers[call_index].0, 9);
        assert_eq!(numbers[call_index].1[0], session.addr());
        assert_ne!(
            numbers[call_index].1[1], 0,
            "sentinel word pointer is present"
        );
        call_index += 1;
        assert_eq!(numbers[call_index], (10, vec![cookie]));
        assert_eq!(call_index + 1, numbers.len());
        let tag_one = image_base.wrapping_add(0x00FE_3504);
        let tag_two = image_base.wrapping_add(0x00FE_341C);
        let d1 = numbers.iter().find(|(id, _)| *id == 3).unwrap();
        let descriptors: Vec<&(u32, Vec<u32>)> =
            numbers.iter().filter(|(id, _)| *id == 3).collect();
        assert_eq!(descriptors.len(), 2);
        assert_eq!(descriptors[0].1[0], session.addr());
        assert_ne!(
            descriptors[0].1[1], 0,
            "first descriptor pointer is present"
        );
        assert_eq!(descriptors[1].1[0], session.addr());
        assert_ne!(
            descriptors[1].1[1], 0,
            "second descriptor pointer is present"
        );
        assert_eq!(d1.0, 3);
        let mut expected_snaps = vec![
            (3, 1, vec![tag_one]),
            (3, 1, vec![tag_two]),
            (4, 0, vec![0]),
            (5, 0, vec![0]),
        ];
        if config == 0 {
            expected_snaps.push((6, 0, vec![0]));
        }
        expected_snaps.extend([(8, 1, vec![0]), (9, 1, vec![u32::MAX])]);
        assert_eq!(rt::take_snaps(), expected_snaps);

        let mut expected = vec![
            ("session_state".to_string(), vec![session.addr()]),
            ("mirror".to_string(), vec![session.addr()]),
        ];
        if split != 0 {
            expected.push(("session_up".to_string(), vec![session.addr()]));
        } else {
            expected.push(("session_flag".to_string(), vec![session.addr()]));
            expected.push(("session_flag.set".to_string(), vec![session.addr(), 8]));
        }
        expected.extend([
            ("notify".to_string(), vec![1, 0xBEEF]),
            ("descriptor".to_string(), vec![session.addr(), tag_one]),
            ("session_flag".to_string(), vec![session.addr()]),
            (
                "session_flag.set".to_string(),
                vec![session.addr(), u32::from(expected_flags)],
            ),
            ("descriptor".to_string(), vec![session.addr(), tag_two]),
            ("derive_a".to_string(), vec![]),
            ("derive_b".to_string(), vec![]),
        ]);
        if config == 0 {
            expected.push(("derive_c".to_string(), vec![image_base + 0x019F_3230]));
        }
        let trailer_args = if trailer_size == 0 {
            vec![session.addr(), 7, 0]
        } else {
            vec![session.addr(), 7, trailer_size, 0x1111_2222, 0x3333_4444]
        };
        expected.extend([
            ("derive_d".to_string(), vec![session.addr()]),
            ("finish_key".to_string(), trailer_args),
            ("alloc_entry".to_string(), vec![session.addr()]),
            ("entry_flags".to_string(), vec![entry.addr()]),
            ("entry_flags.set".to_string(), vec![entry.addr(), 0x43]),
            ("cookie".to_string(), vec![cookie]),
        ]);
        assert_eq!(world.log, expected);

        assert_only_changed(&obj_before, &obj.buf, &[(T_SESSION, T_SESSION + 4)]);
        assert_only_changed(
            &session_before,
            &session.buf,
            &[(S_MIR_DST, S_MIR_DST + 8), (S_FLAG, S_FLAG + 1)],
        );
        assert_only_changed(&entry_before, &entry.buf, &[(ENTRY_FLAG, ENTRY_FLAG + 1)]);
    }

    #[test]
    fn complete_main_paths_match_tags_flags_allocations_and_data() {
        let _lock = rt::script_lock();
        main_case(0, 0, 2);
        main_case(1, 5, 0);
    }

    #[test]
    fn complete_live_side_path_matches_and_wrong_live_range_is_caught() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x62_2201);
        let mut obj = Image::random(TASK_SIZE, &mut rng);
        let mut session = Image::random(SESSION_SIZE, &mut rng);
        let timer = Image::random(0x40, &mut rng);
        obj.w32(T_SESSION, session.addr());
        obj.w32(T_PROBE, 1);
        obj.w32(T_SPLIT, 1);
        session.w32(S_MODE, 2);
        session.w32(S_SUB, timer.addr());
        session.w8(S_FLAG, 0x03);
        let obj_before = obj.buf.clone();
        let session_before = session.buf.clone();
        let mut lifted = lift_task(&obj);
        let mut world = Fake::new();
        world.session_flag_byte = 3;
        world
            .answer("session_state", vec![2])
            .answer("poll_host", vec![0]);
        rt::set_image_base(0x1000_0000);
        rt::set_cells(0xABCD_1234, 0, [0; 4]);
        rt::set_script(&[
            spec(2, StubKind::Thiscall2, vec![0], vec![]),
            spec(10, StubKind::Thiscall0, vec![0], vec![]),
            spec(11, StubKind::Thiscall1, vec![0], vec![]),
            spec(12, StubKind::Thiscall0, vec![0], vec![]),
            spec(13, StubKind::Thiscall1, vec![0], vec![]),
        ]);

        let _ = fn_00662200::rw_00662200(obj.addr(), 0, 9);
        lifted.complete(
            &mut world,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base: 0x1000_0000,
                cookie: 0xABCD_1234,
                setup_flag: 0,
                setup_key: [0; 4],
            },
            0,
            9,
        );

        assert_eq!(lifted.session, None);
        assert_task_matches(&obj, &lifted);
        assert_eq!(session.r8(S_FLAG), 2);
        assert_eq!(world.session_flag_byte, 2);
        let calls = rt::take_numbered();
        assert_eq!(
            calls.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            [11, 12, 13, 2, 10]
        );
        assert_eq!(
            calls[0],
            (11, vec![session.addr() + S_PORT as u32, obj.at(T_PROBE)])
        );
        assert_eq!(calls[1], (12, vec![session.addr() + S_AUX as u32]));
        assert_eq!(
            calls[2],
            (13, vec![timer.addr(), session.addr() + S_TIMER as u32])
        );
        assert_eq!(calls[3], (2, vec![obj.addr(), 0, 9]));
        assert_eq!(calls[4], (10, vec![0xABCD_1234]));
        assert!(rt::take_snaps().is_empty());
        assert_eq!(
            world.log,
            vec![
                ("session_state".to_string(), vec![session.addr()]),
                ("poll_host".to_string(), vec![session.addr(), 1]),
                ("session_flag".to_string(), vec![session.addr()]),
                ("close_aux".to_string(), vec![session.addr()]),
                ("release_timer".to_string(), vec![session.addr()]),
                ("session_flag".to_string(), vec![session.addr()]),
                ("session_flag.set".to_string(), vec![session.addr(), 2]),
                ("notify".to_string(), vec![0, 9]),
                ("cookie".to_string(), vec![0xABCD_1234]),
            ]
        );
        assert_only_changed(&obj_before, &obj.buf, &[(T_SESSION, T_SESSION + 4)]);
        assert_only_changed(&session_before, &session.buf, &[(S_FLAG, S_FLAG + 1)]);
    }

    #[test]
    fn complete_nonlive_state_uses_tail_only() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x62_2202);
        let mut obj = Image::random(TASK_SIZE, &mut rng);
        let mut session = Image::random(SESSION_SIZE, &mut rng);
        obj.w32(T_SESSION, session.addr());
        session.w32(S_MODE, 4);
        let mut lifted = lift_task(&obj);
        let mut world = Fake::new();
        world.answer("session_state", vec![4]);
        rt::set_cells(0xAABB_CCDD, 0, [0; 4]);
        rt::set_script(&[
            spec(2, StubKind::Thiscall2, vec![0], vec![]),
            spec(10, StubKind::Thiscall0, vec![0], vec![]),
        ]);

        let _ = fn_00662200::rw_00662200(obj.addr(), 1, 3);
        lifted.complete(
            &mut world,
            &lf_network::host_session_task::HostSessionGlobals {
                image_base: 0,
                cookie: 0xAABB_CCDD,
                setup_flag: 0,
                setup_key: [0; 4],
            },
            1,
            3,
        );

        assert_eq!(lifted.session, None);
        assert_task_matches(&obj, &lifted);
        assert!(
            wrong::live(4) != world.log.iter().any(|(name, _)| name == "mirror"),
            "wrong live-range version must be caught by the tail-only path"
        );
        assert_eq!(
            rt::take_numbered()
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<_>>(),
            [2, 10]
        );
        assert!(rt::take_snaps().is_empty());
        assert_eq!(
            world.log,
            vec![
                ("session_state".to_string(), vec![session.addr()]),
                ("notify".to_string(), vec![1, 3]),
                ("cookie".to_string(), vec![0xAABB_CCDD]),
            ]
        );
    }
}
