//! Differential cases for the layout frame's blend method.
//!
//! The lifted method and its verified rewrite run on the same generated
//! frames, modes, amounts and factors. The rewrite reads two shared constants
//! through the checker runtime's image-base mapping and calls the frame's
//! blend handler through the frame's vtable; the harness maps the constants
//! into a block it owns and points the vtable at a recording stub. The lifted
//! method reads the same constants through its host and calls the handler
//! through the same recording. Both sides must produce the same answer, the
//! same handler calls with the same mode and the same blended bits, and the
//! same frame bytes. Floats are compared by their bits, so a not-a-number
//! payload or a sign that differs shows. A deliberately wrong lift of the
//! method must differ from the rewrite.
//!
//! This is a separate test binary from the call cases: the image-base static
//! maps to one block, and these constants live at other image addresses than
//! the shared flags.
//!
//! The checker runtime's image-base static is set once, in the one test here.

// Writes the checker runtime's image-base static, which the checker worker sets before each trial in the real checker, and reads the block that static maps the constants into. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::hint::black_box;
    use std::sync::Mutex;

    use lf_checker_rt::CHECKER_XBASE;
    use lf_input_frontend::layout_frame::blend::BlendHost;
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame, field};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer the rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases.
    const TRIALS: u32 = 4000;

    /// Words in the fake vtable: enough for the blend handler's slot.
    const VTABLE_WORDS: usize = 0x200 / 4;

    /// Byte offset of the frame's blend handler in its vtable.
    const BLEND_HANDLER_AT: usize = 0x60;

    /// Image addresses of the two shared constants the blend reads. The
    /// second lies a fixed distance above the first, inside the same block.
    const HALF_VA: u32 = 0x00FE_8830;
    const HIGH_VA: u32 = 0x00FE_88E8;

    /// The constants as the executable holds them: one half and one.
    const HALF_BITS: u32 = 0x3F00_0000;
    const HIGH_BITS: u32 = 0x3F80_0000;

    /// Bytes of the block the constants are mapped into.
    const CONSTANT_BLOCK: usize = 0x100;

    /// Edge bit patterns for amounts and factors: zeros, ones, values just
    /// either side of one, infinities, quiet and signalling not-a-number with
    /// different payloads and signs, subnormals, the largest finite value.
    const F32_EDGE_BITS: [u32; 18] = [
        0x0000_0000,
        0x8000_0000,
        0x3F80_0000,
        0xBF80_0000,
        0x3F80_0001,
        0x3F7F_FFFF,
        0x3F00_0000,
        0x4020_0000,
        0x7F80_0000,
        0xFF80_0000,
        0x7FC0_0000,
        0xFFC0_1234,
        0x7FA0_0002,
        0x7F80_0001,
        0x0000_0001,
        0x7F7F_FFFF,
        0x4B00_0000,
        0xC0A0_0000,
    ];

    /// Edge mode words: the skip mode, the modes next to it, and a large one.
    const MODE_EDGE: [u32; 5] = [0, 1, 2, 3, 0xFFFF_FFFF];

    /// One call the blend makes through the frame's handler.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Blend {
        mode: u32,
        bits: u32,
    }

    /// The log the rewrite's stub writes to. Only the rewrite runs use it.
    static LOG: Mutex<Vec<Blend>> = Mutex::new(Vec::new());

    extern "thiscall" fn blend_stub(_this: u32, mode: u32, bits: u32) -> u32 {
        LOG.lock().unwrap().push(Blend { mode, bits });
        0
    }

    /// The host the lifted method is given: the two constants, and the log of
    /// handler calls.
    struct Host {
        log: Vec<Blend>,
    }

    impl BlendHost for Host {
        fn clamp_high(&mut self) -> f32 {
            f32::from_bits(HIGH_BITS)
        }

        fn half_factor(&mut self) -> f32 {
            f32::from_bits(HALF_BITS)
        }

        fn blend_handler(&mut self, _frame: &mut UILayoutFrame, mode: u32, blended_bits: u32) -> u32 {
            self.log.push(Blend { mode, bits: blended_bits });
            0
        }
    }

    /// Writes the two constants into the block the runtime maps them into,
    /// and points the image-base static at it, so that the constant at
    /// [`HALF_VA`] lands on the block's first word.
    fn install() -> (u32, *mut u8) {
        let vtable: &'static mut [u32; VTABLE_WORDS] = Box::leak(Box::new([0u32; VTABLE_WORDS]));
        vtable[BLEND_HANDLER_AT / 4] =
            (blend_stub as extern "thiscall" fn(u32, u32, u32) -> u32) as usize as u32;
        let vtable_addr = vtable.as_ptr() as usize as u32;

        let block_words: &'static mut [u32] =
            Box::leak(vec![0u32; CONSTANT_BLOCK / 4].into_boxed_slice());
        let block = block_words.as_mut_ptr().cast::<u8>();
        let block_addr = block as usize as u32;
        let xbase = block_addr.wrapping_sub(HALF_VA.wrapping_sub(0x0040_0000));
        let high_at = (HIGH_VA - HALF_VA) as usize;

        // SAFETY: the block holds CONSTANT_BLOCK bytes, and both written words
        // lie inside it (the high constant is at the offset computed above).
        unsafe {
            block.cast::<u32>().write_unaligned(HALF_BITS);
            block.add(high_at).cast::<u32>().write_unaligned(HIGH_BITS);
            // SAFETY-adjacent: the runtime reads this static only through its
            // accessors, and this test writes it once before any rewrite runs.
            core::ptr::addr_of_mut!(CHECKER_XBASE).write(xbase);
        }
        (vtable_addr, block)
    }

    /// Small deterministic generator (splitmix64).
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

        /// A float's bits: one time in two an edge pattern, otherwise any bits.
        fn float_bits(&mut self) -> u32 {
            if self.next() % 2 == 0 {
                F32_EDGE_BITS[(self.next() % F32_EDGE_BITS.len() as u64) as usize]
            } else {
                self.u32()
            }
        }

        /// A mode: one time in four the skip mode, otherwise an edge or any word.
        fn mode(&mut self) -> u32 {
            match self.next() % 4 {
                0 => 1,
                1 => MODE_EDGE[(self.next() % MODE_EDGE.len() as u64) as usize],
                _ => self.u32(),
            }
        }
    }

    fn seed_of(name: &str) -> u64 {
        let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
        for byte in name.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    /// The buffer the rewrite writes into, 4-byte aligned like the object.
    #[repr(C, align(4))]
    struct Buffer([u8; BUF]);

    /// The deliberate faults of the wrong lift. Each is a plausible mistake
    /// in the blend: the operands of the subtraction swapped, the upper clamp
    /// dropped, and the skip test inverted.
    #[derive(Clone, Copy)]
    enum Fault {
        SwapSubtraction,
        NoUpperClamp,
        InvertedSkip,
    }

    /// The blend with one deliberate fault, written over the frame's bytes.
    /// The rewrite's handler receives the frame's address; the wrong lift
    /// passes nothing, because the recording host does not read the frame.
    fn wrong_lift(
        fault: Fault,
        bytes: &mut [u8; SPAN],
        host: &mut Host,
        mode: u32,
        amount_bits: u32,
    ) -> u32 {
        bytes[field::SLOT_14..field::SLOT_14 + 4].copy_from_slice(&mode.to_le_bytes());
        let skip = match fault {
            Fault::InvertedSkip => mode == 1,
            _ => mode != 1,
        };
        if skip {
            let high = f32::from_bits(HIGH_BITS);
            let half = f32::from_bits(HALF_BITS);
            let mut clamped = f32::from_bits(amount_bits);
            if clamped < 0.0 {
                clamped = 0.0;
            } else if !matches!(fault, Fault::NoUpperClamp) && clamped > high {
                clamped = high;
            }
            let first = f32::from_bits(read_word(bytes, field::FLOAT_8));
            let second = f32::from_bits(read_word(bytes, field::BLOCK_10));
            let scaled = black_box(black_box(first) * black_box(half));
            let weighted = black_box(black_box(first) * black_box(clamped));
            let difference = match fault {
                Fault::SwapSubtraction => black_box(black_box(scaled) - black_box(second)),
                _ => black_box(black_box(second) - black_box(scaled)),
            };
            let blended = black_box(black_box(difference) + black_box(weighted));
            host.log.push(Blend { mode, bits: blended.to_bits() });
        }
        0
    }

    fn read_word(bytes: &[u8; SPAN], at: usize) -> u32 {
        let mut word = [0u8; 4];
        word.copy_from_slice(&bytes[at..at + 4]);
        u32::from_le_bytes(word)
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        trials_with_handler_calls: u32,
        first_mismatch: Option<String>,
    }

    fn run(name: &str, vtable_addr: u32, fault: Fault) -> Outcome {
        let mut rng = Rng(seed_of(name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let mut frame = [0u8; SPAN];
            for byte in &mut frame {
                *byte = rng.next() as u8;
            }
            frame[0..4].copy_from_slice(&vtable_addr.to_le_bytes());
            let mode = rng.mode();
            let amount = rng.float_bits();

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&frame);
            rewritten.0[SPAN..].fill(0x5A);
            LOG.lock().unwrap().clear();
            let rw_answer = r::fn_00DB19A0::rw_00db19a0(rewritten.0.as_mut_ptr() as usize as u32, mode, amount);
            let rw_log = std::mem::take(&mut *LOG.lock().unwrap());
            if !rw_log.is_empty() {
                outcome.trials_with_handler_calls += 1;
            }

            let mut lifted = UILayoutFrame::from_bytes(frame);
            let mut host = Host { log: Vec::new() };
            let lift_answer = lifted.store_mode_then_blend_and_forward(&mut host, mode, amount);

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            if rw_answer != lift_answer || rw_log != host.log || !frame_matches || !tail_matches {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: mode {mode:#x} amount {amount:#010x}: answers {rw_answer:#x} / {lift_answer:#x}, calls {rw_log:?} / {:?}, frame equal {frame_matches}, tail equal {tail_matches}",
                        host.log
                    ));
                }
            }

            let mut wrong_bytes = frame;
            let mut wrong_host = Host { log: Vec::new() };
            let wrong_answer = wrong_lift(fault, &mut wrong_bytes, &mut wrong_host, mode, amount);
            if wrong_answer != rw_answer || wrong_host.log != rw_log || wrong_bytes != rewritten.0[..SPAN] {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn blend_matches_its_rewrite_on_every_trial_and_a_wrong_lift_is_caught() {
        let (vtable_addr, _block) = install();
        let faults = [
            ("swap subtraction", Fault::SwapSubtraction),
            ("no upper clamp", Fault::NoUpperClamp),
            ("inverted skip", Fault::InvertedSkip),
        ];
        let outcome = run("store_mode_then_blend_and_forward", vtable_addr, Fault::SwapSubtraction);
        assert_eq!(
            outcome.lift_mismatches, 0,
            "store_mode_then_blend_and_forward: {}",
            outcome.first_mismatch.unwrap_or_default()
        );
        assert!(
            outcome.trials_with_handler_calls > TRIALS / 4,
            "only {} of {} trials reached the handler",
            outcome.trials_with_handler_calls,
            TRIALS
        );
        for (label, fault) in faults {
            let wrong = run(label, vtable_addr, fault);
            assert!(
                wrong.wrong_mismatches > 0,
                "{label}: the deliberately wrong lift matched the rewrite on every trial"
            );
        }
    }
}
