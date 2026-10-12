//! Differential cases: the hosting task's gamer-list setup.
//!
//! The rewrite and lift receive the same list words, source array and
//! scripted trailer result. The case compares the return, stored fields,
//! copied list, trailer write, and the one collaborator call when present.
//! A deliberately wrong empty-list test and clamp threshold must be caught.

#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_hostsessdiff::rewrites::*;
    use lf_hostsessdiff::rt::{self, Spec, StubKind};
    use lf_network::host_session_task::{GAMER_WORDS, GamerListParams};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, T_CONFIG, T_EMPTY, T_EXTRA, T_FIRST, T_GAMERS, T_SECOND, T_SPLIT,
        T_TRAILER, T_TSIZE, TASK_SIZE, assert_only_changed, assert_task_matches, lift_task,
    };

    mod wrong {
        /// Uses a strict positive test, missing negative signed counts.
        pub fn empty(value: u32) -> u32 {
            u32::from((value as i32) == 0)
        }

        /// Uses a maximum one entry below the verified rewrite's value.
        pub fn counts_with_low_cap(first: u32, second: u32) -> (u32, u32) {
            let mut first = first as i32;
            let mut second = second as i32;
            if first.wrapping_add(second) > 31 {
                let over = 31i32.wrapping_sub(first).wrapping_sub(second);
                first = (first.wrapping_sub(over)).max(0);
                if first.wrapping_add(second) > 31 {
                    second = (second.wrapping_sub(over)).max(0);
                }
            }
            (first as u32, second as u32)
        }
    }

    #[test]
    fn setup_matches_stores_copy_trailer_and_return() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x61_20E0);
        let source_words = core::array::from_fn(|_| rng.u32());
        let mut source = Image::random(GAMER_WORDS * 4, &mut rng);
        for (i, word) in source_words.iter().enumerate() {
            source.w32(i * 4, *word);
        }
        let scenarios = [
            (0, 32, 0, 0, 0),
            (1, 20, 12, 1, 2),
            (0, u32::MAX, 0, 0, 1),
            (0, 36, 0, 0, 2),
        ];
        let mut caught_empty = false;
        let mut caught_threshold = false;

        for (split, first, second, trailer_size, call_answer) in scenarios {
            let obj = Image::random(TASK_SIZE, &mut rng);
            let params = GamerListParams {
                list: 7,
                split,
                first,
                second,
                extra: 9,
                trailer_arg: 11,
                trailer_size,
            };
            let before = obj.buf.clone();
            let mut lifted = lift_task(&obj);
            let mut fake = Fake::new();
            let words = vec![0x1234_5678, 0xA5A5_5A5A];
            if trailer_size != 0 {
                fake.answer("trailer", vec![call_answer]);
                fake.out_words("trailer.words", vec![words.clone()]);
                rt::set_script(&[Spec {
                    id: 1,
                    kind: StubKind::Cdecl3,
                    answers: vec![call_answer],
                    snaps: vec![],
                    outs: vec![vec![(0, words.clone())]],
                }]);
            } else {
                rt::set_script(&[]);
            }
            let got = fn_006620e0::rw_006620e0(
                obj.addr(),
                0xABCD,
                params.list,
                params.split,
                params.first,
                params.second,
                source.addr(),
                params.extra,
                params.trailer_arg,
                params.trailer_size,
            );
            let want = lifted.setup_gamer_data(&mut fake, &params, &source_words);

            assert_eq!(
                got, want,
                "split {split:#x}, first {first:#x}, second {second:#x}"
            );
            assert_eq!(lifted.config, params.list);
            assert_eq!(lifted.list_split, params.split);
            assert_eq!(lifted.gamers, source_words);
            assert_eq!(
                lifted.trailer,
                if trailer_size == 0 {
                    vec![]
                } else {
                    words.clone()
                }
            );
            assert_eq!(obj.r32(T_CONFIG), lifted.config);
            assert_eq!(obj.r32(T_SPLIT), lifted.list_split);
            assert_eq!(obj.r32(T_EMPTY), lifted.first_empty);
            assert_eq!(obj.r32(T_FIRST), lifted.first_count);
            assert_eq!(obj.r32(T_SECOND), lifted.second_count);
            assert_eq!(obj.r32(T_EXTRA), lifted.open_extra);
            assert_eq!(obj.r32(T_TSIZE), lifted.trailer_size);
            assert_task_matches(&obj, &lifted);
            for (i, word) in lifted.gamers.iter().enumerate() {
                assert_eq!(obj.r32(T_GAMERS + i * 4), *word, "gamer word {i}");
            }
            if trailer_size != 0 {
                for (i, word) in words.iter().enumerate() {
                    assert_eq!(obj.r32(T_TRAILER + i * 4), *word, "trailer word {i}");
                }
                assert_eq!(
                    rt::take_numbered(),
                    vec![(
                        1,
                        vec![obj.at(T_TRAILER), params.trailer_arg, params.trailer_size]
                    )]
                );
                assert_eq!(
                    fake.log,
                    vec![(
                        "trailer".to_string(),
                        vec![params.trailer_arg, params.trailer_size]
                    )]
                );
            } else {
                assert!(rt::take_numbered().is_empty());
                assert!(fake.log.is_empty());
            }
            assert!(rt::take_snaps().is_empty());
            let mut changed = vec![
                (T_CONFIG, T_CONFIG + 4),
                (T_EMPTY, T_EMPTY + 4),
                (T_SPLIT, T_SPLIT + 4),
                (T_FIRST, T_FIRST + 4),
                (T_SECOND, T_SECOND + 4),
                (T_GAMERS, T_GAMERS + GAMER_WORDS * 4),
                (T_EXTRA, T_EXTRA + 4),
                (T_TSIZE, T_TSIZE + 4),
            ];
            if trailer_size != 0 {
                changed.push((T_TRAILER, T_TRAILER + words.len() * 4));
            }
            assert_only_changed(&before, &obj.buf, &changed);
            if first == u32::MAX {
                caught_empty |= wrong::empty(lifted.first_count) != lifted.first_empty;
            }
            if first == 32 && second == 0 {
                caught_threshold |= wrong::counts_with_low_cap(first, second)
                    != (lifted.first_count, lifted.second_count);
            }
        }

        assert!(caught_empty, "wrong empty-list check was not caught");
        assert!(
            caught_threshold,
            "wrong exact-maximum threshold was not caught"
        );
    }
}
