//! Differential cases, part 1: parameter blocks and the voice header.
//!
//! Each case builds the 32-bit object, runs the rewrite and the lift on
//! the same inputs, and compares the return, every written byte and (for
//! the sub-object reset) the helper call. Each method has a deliberately
//! wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_voice::params::{ParamBlock, ParamBlocks, VoiceHead, BLOCK_COUNT};
    use lf_audiovoicediff::rewrites::*;
    use lf_audiovoicediff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(this: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(this as *mut u8, len) }
    }

    fn decode_blocks(img: &[u8], base: usize) -> ParamBlocks {
        let mut blocks = [ParamBlock::DEFAULT; BLOCK_COUNT];
        for (i, b) in blocks.iter_mut().enumerate() {
            let o = base + i * 12;
            b.code = u32::from_le_bytes(img[o..o + 4].try_into().unwrap());
            b.gain = f32::from_bits(u32::from_le_bytes(img[o + 4..o + 8].try_into().unwrap()));
            b.flags = u16::from_le_bytes(img[o + 8..o + 10].try_into().unwrap());
        }
        ParamBlocks::new(blocks)
    }

    /// Encodes the ten modelled bytes per block, leaving the two pad bytes alone.
    fn encode_blocks(img: &mut [u8], base: usize, p: &ParamBlocks) {
        for (i, b) in p.blocks.iter().enumerate() {
            let o = base + i * 12;
            img[o..o + 4].copy_from_slice(&b.code.to_le_bytes());
            img[o + 4..o + 8].copy_from_slice(&b.gain.to_bits().to_le_bytes());
            img[o + 8..o + 10].copy_from_slice(&b.flags.to_le_bytes());
        }
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_audio::audio_voice::params::{ParamBlock, ParamBlocks, VoiceHead, BLOCK_COUNT};

        /// Resets the gain to 0.0 instead of 1.0.
        pub fn reset_gain_zero(p: &mut ParamBlocks) {
            p.blocks = [ParamBlock { code: 0, gain: 0.0, flags: 0 }; BLOCK_COUNT];
        }

        /// Resets only the first four blocks.
        pub fn reset_four(p: &mut ParamBlocks) {
            for b in p.blocks.iter_mut().take(BLOCK_COUNT - 1) {
                *b = ParamBlock::DEFAULT;
            }
        }

        /// Stamps 0xFFFE instead of 0xFFFF.
        pub fn reset_tag_fffe(h: &mut VoiceHead) {
            h.seq = 0;
            h.tag = 0xFFFE;
        }
    }

    #[test]
    fn defaults_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAFA0);
        let mut caught_gain = 0;
        let mut caught_four = 0;
        for trial in 0..60u32 {
            let mut initial = vec![0u8; 80];
            rng.bytes(&mut initial);
            if trial == 0 {
                // Pin non-default inputs in every field.
                initial.fill(0xA5);
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let mut lift = decode_blocks(&initial, 4);
            let ret = unsafe { fn_008AAFA0::rw_008aafa0(this as *mut u8) };
            assert_eq!(ret, this, "trial {trial}: the rewrite echoes this");
            lift.reset();
            let mut expected = initial.clone();
            encode_blocks(&mut expected, 4, &lift);
            let after = unsafe { image(this, 80) }.to_vec();
            assert_eq!(after, expected, "trial {trial}: image after reset");
            let mut w = decode_blocks(&initial, 4);
            wrong::reset_gain_zero(&mut w);
            let mut wimg = initial.clone();
            encode_blocks(&mut wimg, 4, &w);
            if wimg != after {
                caught_gain += 1;
            }
            let mut w = decode_blocks(&initial, 4);
            wrong::reset_four(&mut w);
            let mut wimg = initial.clone();
            encode_blocks(&mut wimg, 4, &w);
            if wimg != after {
                caught_four += 1;
            }
        }
        assert!(caught_gain > 0, "gain-zero mutant was never caught");
        assert!(caught_four > 0, "four-block mutant was never caught");
    }

    #[test]
    fn defaults_sub_matches() {
        let _guard = lock();
        let mut rng = Rng(0xB010);
        let mut caught = 0;
        for trial in 0..60u32 {
            let mut initial = vec![0u8; 0x104];
            rng.bytes(&mut initial);
            if trial == 0 {
                initial.fill(0x5A);
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_script(&[(1, StubKind::Thiscall1, vec![0])]);
            let mut lift = decode_blocks(&initial, 0xC0);
            let ret = unsafe { fn_008AB010::rw_008ab010(this as *mut u8) };
            assert_eq!(ret, this, "trial {trial}: the rewrite echoes this");
            let calls = rt::take_calls();
            assert_eq!(
                calls,
                vec![(1, vec![this.wrapping_add(0xFC)])],
                "trial {trial}: helper runs once on this+0xfc"
            );
            let mut helper_calls = 0u32;
            lift.reset_and_init_sub(&mut || helper_calls += 1);
            assert_eq!(helper_calls, 1, "trial {trial}: lift runs the helper once");
            let mut expected = initial.clone();
            encode_blocks(&mut expected, 0xC0, &lift);
            let after = unsafe { image(this, 0x104) }.to_vec();
            assert_eq!(after, expected, "trial {trial}: image after reset");
            let mut w = decode_blocks(&initial, 0xC0);
            wrong::reset_gain_zero(&mut w);
            let mut wimg = initial.clone();
            encode_blocks(&mut wimg, 0xC0, &w);
            if wimg != after {
                caught += 1;
            }
        }
        assert!(caught > 0, "gain-zero mutant was never caught");
    }

    #[test]
    fn header_reset_matches() {
        let _guard = lock();
        let mut rng = Rng(0x5530);
        let mut caught = 0;
        for trial in 0..60u32 {
            let mut initial = vec![0u8; 8];
            rng.bytes(&mut initial);
            if trial == 0 {
                initial.fill(0x81);
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let mut lift = VoiceHead {
                seq: u32::from_le_bytes(initial[0..4].try_into().unwrap()),
                tag: u16::from_le_bytes(initial[4..6].try_into().unwrap()),
            };
            let ret = unsafe { fn_00985530::rw_00985530(this) };
            assert_eq!(ret, this, "trial {trial}: the rewrite echoes this");
            lift.reset();
            let mut expected = initial.clone();
            expected[0..4].copy_from_slice(&lift.seq.to_le_bytes());
            expected[4..6].copy_from_slice(&lift.tag.to_le_bytes());
            let after = unsafe { image(this, 8) }.to_vec();
            assert_eq!(after, expected, "trial {trial}: image after reset");
            let mut w = VoiceHead {
                seq: u32::from_le_bytes(initial[0..4].try_into().unwrap()),
                tag: u16::from_le_bytes(initial[4..6].try_into().unwrap()),
            };
            wrong::reset_tag_fffe(&mut w);
            let mut wimg = initial.clone();
            wimg[0..4].copy_from_slice(&w.seq.to_le_bytes());
            wimg[4..6].copy_from_slice(&w.tag.to_le_bytes());
            if wimg != after {
                caught += 1;
            }
        }
        assert!(caught > 0, "tag-fffe mutant was never caught");
    }
}
