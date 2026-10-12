//! Differential cases for the layout frame's call-free methods.
//!
//! Each case runs a lifted method and its verified rewrite on the same
//! generated frames and arguments, and compares the answer and every byte of
//! the frame, floats by their bits. A deliberately wrong lift of each method
//! runs through the same generator and must differ from the rewrite at least
//! once. The rewrites take real object addresses, so these cases run on the
//! 32-bit target only; on other hosts the file compiles to nothing.
//!
//! Two cases answer their own address: the block copies return the address
//! the rewrite was given as the source, and the lifted method takes the block
//! by reference, so the harness supplies that answer. The narrowing is listed
//! in the registry.

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::layout_frame::registry::{self, State};
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame, field};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer each rewrite runs on: the frame's span plus a tail
    /// that must come back unchanged, so a write past the span shows.
    const BUF: usize = 0x100;

    /// Generated cases per method.
    const TRIALS: u32 = 4000;

    /// The buffer the rewrites write into, 4-byte aligned like the object.
    #[repr(C, align(4))]
    struct Buffer([u8; BUF]);

    /// Arguments of one call. Floats travel as their bits in `a` or `b`.
    #[derive(Clone, Copy)]
    struct Args {
        a: u32,
        b: u32,
        block: [u8; 24],
    }

    /// Which family of stores a method belongs to. The shape decides how
    /// arguments are generated and which deliberate fault its wrong lift
    /// carries.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Shape {
        /// Stores the low byte and answers it.
        ByteRet,
        /// Stores the low byte; answers zero.
        ByteVoid,
        /// Stores a word and answers it.
        WordRet,
        /// Stores a word; answers zero.
        WordVoid,
        /// Stores a float's bits; answers zero.
        FloatVoid,
        /// Stores a float and clears both tag words when the tag is set.
        ClearTags,
        /// Stores a float; replaces the tag and clears the word after it on change.
        TagStore,
        /// Stores a slot word and a fraction word.
        SlotFraction,
        /// Stores a word only when the cache word is zero.
        Cache,
        /// Copies 24 bytes into the frame.
        BlockCopy,
    }

    /// One method under test: its rewrite, its lift, and where it writes.
    struct Case {
        name: &'static str,
        shape: Shape,
        off: usize,
        rw: fn(u32, &Args) -> u32,
        lift: fn(&mut UILayoutFrame, &Args) -> u32,
    }

    fn case(
        name: &'static str,
        shape: Shape,
        off: usize,
        rw: fn(u32, &Args) -> u32,
        lift: fn(&mut UILayoutFrame, &Args) -> u32,
    ) -> Case {
        Case {
            name,
            shape,
            off,
            rw,
            lift,
        }
    }

    /// Every call-free method, with its rewrite and its lift. The lift
    /// closures answer zero for the stores that have no answer, which is the
    /// answer the rewrites give.
    fn cases() -> Vec<Case> {
        vec![
            case(
                "set_flag_c4",
                Shape::ByteRet,
                field::FLAG_C4,
                |t, x| r::fn_00C0F500::rw_00c0f500(t as *mut u8, x.a),
                |f, x| f.set_flag_c4(x.a),
            ),
            case(
                "set_flag_c7",
                Shape::ByteRet,
                field::FLAG_C7,
                |t, x| r::fn_00C0F5A0::rw_00c0f5a0(t as *mut u8, x.a),
                |f, x| f.set_flag_c7(x.a),
            ),
            case(
                "set_flag_c8",
                Shape::ByteRet,
                field::FLAG_C8,
                |t, x| r::fn_00C0F4F0::rw_00c0f4f0(t as *mut u8, x.a),
                |f, x| f.set_flag_c8(x.a),
            ),
            case(
                "set_flag_ca",
                Shape::ByteRet,
                field::FLAG_CA,
                |t, x| r::fn_00C0F8D0::rw_00c0f8d0(t as *mut u8, x.a),
                |f, x| f.set_flag_ca(x.a),
            ),
            case(
                "set_flag_d0",
                Shape::ByteRet,
                field::FLAG_D0,
                |t, x| r::fn_00C0F5B0::rw_00c0f5b0(t as *mut u8, x.a),
                |f, x| f.set_flag_d0(x.a),
            ),
            case(
                "set_flag_d1",
                Shape::ByteRet,
                field::FLAG_D1,
                |t, x| r::fn_00C0F530::rw_00c0f530(t as *mut u8, x.a),
                |f, x| f.set_flag_d1(x.a),
            ),
            case(
                "set_flag_eb",
                Shape::ByteRet,
                field::FLAG_EB,
                |t, x| r::fn_00C0F840::rw_00c0f840(t as *mut u8, x.a),
                |f, x| f.set_flag_eb(x.a),
            ),
            case(
                "set_flag_c9",
                Shape::ByteVoid,
                field::FLAG_C9,
                |t, x| r::fn_00DB1970::rw_00db1970(t, x.a),
                |f, x| {
                    f.set_flag_c9(x.a);
                    0
                },
            ),
            case(
                "set_flag_e8",
                Shape::ByteVoid,
                field::FLAG_E8,
                |t, x| r::fn_00DB1940::rw_00db1940(t, x.a),
                |f, x| {
                    f.set_flag_e8(x.a);
                    0
                },
            ),
            case(
                "set_flag_e9",
                Shape::ByteVoid,
                field::FLAG_E9,
                |t, x| r::fn_00DB1950::rw_00db1950(t, x.a),
                |f, x| {
                    f.set_flag_e9(x.a);
                    0
                },
            ),
            case(
                "set_flag_ea",
                Shape::ByteVoid,
                field::FLAG_EA,
                |t, x| r::fn_00DB1960::rw_00db1960(t, x.a),
                |f, x| {
                    f.set_flag_ea(x.a);
                    0
                },
            ),
            case(
                "set_word_a0",
                Shape::WordRet,
                field::WORD_A0,
                |t, x| r::fn_00C0F8C0::rw_00c0f8c0(t as *mut u8, x.a),
                |f, x| f.set_word_a0(x.a),
            ),
            case(
                "set_word_d8",
                Shape::WordRet,
                field::WORD_D8,
                |t, x| r::fn_00C0F860::rw_00c0f860(t as *mut u8, x.a),
                |f, x| f.set_word_d8(x.a),
            ),
            case(
                "set_word_e0",
                Shape::WordRet,
                field::WORD_E0,
                |t, x| r::fn_00C0F830::rw_00c0f830(t as *mut u8, x.a),
                |f, x| f.set_word_e0(x.a),
            ),
            case(
                "set_word_e4",
                Shape::WordRet,
                field::WORD_E4,
                |t, x| r::fn_00C0F850::rw_00c0f850(t as *mut u8, x.a),
                |f, x| f.set_word_e4(x.a),
            ),
            case(
                "set_word_d4",
                Shape::WordVoid,
                field::WORD_D4,
                |t, x| r::fn_00A64D50::rw_00a64d50(t, x.a),
                |f, x| {
                    f.set_word_d4(x.a);
                    0
                },
            ),
            case(
                "set_word_dc",
                Shape::WordVoid,
                field::WORD_DC,
                |t, x| r::fn_00A64D30::rw_00a64d30(t, x.a),
                |f, x| {
                    f.set_word_dc(x.a);
                    0
                },
            ),
            case(
                "set_float_4",
                Shape::FloatVoid,
                field::FLOAT_4,
                |t, x| r::fn_00C0F520::rw_00c0f520(t as *mut u8, f32::from_bits(x.a)),
                |f, x| {
                    f.set_float_4(f32::from_bits(x.a));
                    0
                },
            ),
            case(
                "set_float_8",
                Shape::FloatVoid,
                field::FLOAT_8,
                |t, x| r::fn_00C0F510::rw_00c0f510(t as *mut u8, f32::from_bits(x.a)),
                |f, x| {
                    f.set_float_8(f32::from_bits(x.a));
                    0
                },
            ),
            case(
                "store_float_b0_clear_tags",
                Shape::ClearTags,
                field::FLOAT_B0,
                |t, x| r::fn_00C0F540::rw_00c0f540(t as *mut u8, f32::from_bits(x.a)),
                |f, x| {
                    f.store_float_b0_clear_tags(f32::from_bits(x.a));
                    0
                },
            ),
            case(
                "store_float_b0_with_tag",
                Shape::TagStore,
                field::FLOAT_B0,
                |t, x| r::fn_00C0F570::rw_00c0f570(t as *mut u8, f32::from_bits(x.a), x.b),
                |f, x| f.store_float_b0_with_tag(f32::from_bits(x.a), x.b),
            ),
            case(
                "set_slot_and_fraction",
                Shape::SlotFraction,
                field::SLOT_14,
                |t, x| r::fn_00DB1980::rw_00db1980(t, x.a, x.b),
                |f, x| {
                    f.set_slot_and_fraction(x.a, x.b);
                    0
                },
            ),
            case(
                "install_cache_if_empty",
                Shape::Cache,
                field::CACHE_EC,
                |t, x| r::fn_00DB1AA0::rw_00db1aa0(t, x.a),
                |f, x| {
                    f.install_cache_if_empty(x.a);
                    0
                },
            ),
            case(
                "copy_block_to_10",
                Shape::BlockCopy,
                field::BLOCK_10,
                |t, x| r::fn_00C0F890::rw_00c0f890(t as *mut u8, x.block.as_ptr()),
                |f, x| {
                    f.copy_block_to_10(&x.block);
                    x.block.as_ptr() as usize as u32
                },
            ),
            case(
                "copy_block_to_a4",
                Shape::BlockCopy,
                field::BLOCK_A4,
                |t, x| r::fn_00C0F8F0::rw_00c0f8f0(t as *mut u8, x.block.as_ptr()),
                |f, x| {
                    f.copy_block_to_a4(&x.block);
                    x.block.as_ptr() as usize as u32
                },
            ),
        ]
    }

    /// Edge words: zero, one, the sign boundary, all ones, small counts,
    /// and a value that a 16-bit count would truncate.
    const U32_EDGE: [u32; 12] = [
        0,
        1,
        0xFF,
        0x100,
        0x7FFF_FFFF,
        0x8000_0000,
        0xFFFF_FFFF,
        0xFFFF_FF00,
        0x0001_0000,
        0x5E,
        0xDEAD_BEEF,
        0x0000_0080,
    ];

    /// Edge float bit patterns: zeros, ones, infinities, quiet and signalling
    /// not-a-number with and without sign, the largest and smallest normal
    /// and subnormal values, and the round-to-nearest pivot.
    const F32_EDGE_BITS: [u32; 16] = [
        0x0000_0000,
        0x8000_0000,
        0x3F80_0000,
        0xBF80_0000,
        0x3F00_0000,
        0x4020_0000,
        0x7F80_0000,
        0xFF80_0000,
        0x7FC0_0000,
        0xFFC0_1234,
        0x7F80_0001,
        0x7FBF_FFFF,
        0x0000_0001,
        0x7F7F_FFFF,
        0x0080_0000,
        0x4B00_0000,
    ];

    /// Small deterministic generator (splitmix64), seeded per method.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
        }

        /// A word: one time in three an edge value, otherwise any word.
        fn word(&mut self) -> u32 {
            if self.next() % 3 == 0 {
                U32_EDGE[(self.next() % U32_EDGE.len() as u64) as usize]
            } else {
                self.u32()
            }
        }

        /// A float's bits: one time in two an edge pattern, otherwise any bits.
        fn float_bits(&mut self) -> u32 {
            if self.next() % 2 == 0 {
                F32_EDGE_BITS[(self.next() % F32_EDGE_BITS.len() as u64) as usize]
            } else {
                self.u32()
            }
        }
    }

    /// Seed from the method's name (FNV-1a), so each method has its own stream.
    fn seed_of(name: &str) -> u64 {
        let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
        for byte in name.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    fn word_at(bytes: &[u8], at: usize) -> u32 {
        let mut word = [0u8; 4];
        word.copy_from_slice(&bytes[at..at + 4]);
        u32::from_le_bytes(word)
    }

    fn put_word(bytes: &mut [u8], at: usize, value: u32) {
        bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    /// Arguments for one trial of a shape. A tag is drawn from the frame's
    /// own tag word half the time, so the equal-tag branch runs.
    fn args_for(shape: Shape, rng: &mut Rng, frame: &[u8; SPAN]) -> Args {
        let block = [0u8; 24];
        match shape {
            Shape::ByteRet | Shape::ByteVoid | Shape::WordRet | Shape::WordVoid | Shape::Cache => {
                Args {
                    a: rng.word(),
                    b: 0,
                    block,
                }
            }
            Shape::FloatVoid | Shape::ClearTags => Args {
                a: rng.float_bits(),
                b: 0,
                block,
            },
            Shape::TagStore => {
                let tag = if rng.next() % 2 == 0 {
                    word_at(frame, field::TAG_B4)
                } else {
                    rng.word()
                };
                Args {
                    a: rng.float_bits(),
                    b: tag,
                    block,
                }
            }
            Shape::SlotFraction => Args {
                a: rng.word(),
                b: rng.float_bits(),
                block,
            },
            Shape::BlockCopy => {
                let mut block = [0u8; 24];
                for byte in &mut block {
                    *byte = rng.next() as u8;
                }
                Args { a: 0, b: 0, block }
            }
        }
    }

    /// The deliberate fault of each shape's wrong lift, applied at the
    /// method's own offset. Each one is a plausible mistake: the wrong byte,
    /// an unmasked answer, a swapped pair, a missed clear, an always-taken
    /// branch, a short copy. It answers the same way the lift does, except
    /// where the fault is in the answer itself.
    fn wrong_lift(shape: Shape, off: usize, frame: &mut [u8; SPAN], x: &Args) -> u32 {
        match shape {
            Shape::ByteRet => {
                frame[off + 1] = x.a.to_le_bytes()[0];
                x.a
            }
            Shape::ByteVoid => {
                frame[off + 1] = x.a.to_le_bytes()[0];
                0
            }
            Shape::WordRet => {
                put_word(frame, off, x.a ^ 1);
                x.a
            }
            Shape::WordVoid => {
                put_word(frame, off, x.a ^ 1);
                0
            }
            Shape::FloatVoid => {
                put_word(frame, off, x.a ^ 0x8000_0000);
                0
            }
            Shape::ClearTags => {
                put_word(frame, field::FLOAT_B0, x.a);
                put_word(frame, field::TAG_B4, 0);
                0
            }
            Shape::TagStore => {
                put_word(frame, field::FLOAT_B0, x.a);
                if word_at(frame, field::TAG_B4) != x.b {
                    put_word(frame, field::TAG_B4, x.b);
                }
                put_word(frame, field::WORD_B8, 0);
                x.b
            }
            Shape::SlotFraction => {
                put_word(frame, field::SLOT_14, x.b);
                put_word(frame, field::FRACTION_18, x.a);
                0
            }
            Shape::Cache => {
                put_word(frame, field::CACHE_EC, x.a);
                0
            }
            Shape::BlockCopy => {
                frame[off..off + 23].copy_from_slice(&x.block[..23]);
                x.block.as_ptr() as usize as u32
            }
        }
    }

    /// Outcome of one method's trials.
    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        first_mismatch: Option<String>,
    }

    /// Runs one method's trials: the lift against the rewrite, and the wrong
    /// lift against the rewrite.
    fn run(case: &Case) -> Outcome {
        let mut rng = Rng(seed_of(case.name));
        let mut outcome = Outcome::default();
        for trial in 0..TRIALS {
            let mut init = [0u8; BUF];
            for byte in &mut init {
                *byte = rng.next() as u8;
            }
            // The cache word is zero on half the trials, so the install runs.
            if rng.next() % 2 == 0 {
                init[field::CACHE_EC..field::CACHE_EC + 4].fill(0);
            }
            let mut frame_init = [0u8; SPAN];
            frame_init.copy_from_slice(&init[..SPAN]);
            let args = args_for(case.shape, &mut rng, &frame_init);

            let mut rewritten = Buffer(init);
            let rw_answer = (case.rw)(rewritten.0.as_mut_ptr() as usize as u32, &args);

            let mut lifted = UILayoutFrame::from_bytes(frame_init);
            let lift_answer = (case.lift)(&mut lifted, &args);

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == init[SPAN..];
            if rw_answer != lift_answer || !frame_matches || !tail_matches {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial}: rewrite answer {rw_answer:#010x}, lift answer {lift_answer:#010x}, frame equal {frame_matches}, tail equal {tail_matches}"
                    ));
                }
            }

            let mut wrong_frame = frame_init;
            let wrong_answer = wrong_lift(case.shape, case.off, &mut wrong_frame, &args);
            if wrong_answer != rw_answer || wrong_frame != rewritten.0[..SPAN] {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn every_method_matches_its_rewrite_on_every_trial() {
        for case in cases() {
            let outcome = run(&case);
            assert_eq!(
                outcome.lift_mismatches,
                0,
                "{}: {}",
                case.name,
                outcome.first_mismatch.unwrap_or_default()
            );
        }
    }

    #[test]
    fn every_wrong_lift_is_caught_by_the_same_trials() {
        for case in cases() {
            let outcome = run(&case);
            assert!(
                outcome.wrong_mismatches > 0,
                "{}: the deliberately wrong lift matched the rewrite on every trial",
                case.name
            );
        }
    }

    #[test]
    fn every_case_is_a_proven_row() {
        for case in cases() {
            assert!(
                registry::ROWS
                    .iter()
                    .any(|row| row.method == case.name && row.state == State::Proven),
                "{} is not a proven row",
                case.name
            );
        }
    }

    #[test]
    fn the_call_free_set_is_twenty_five_methods() {
        assert_eq!(cases().len(), 25);
    }
}
