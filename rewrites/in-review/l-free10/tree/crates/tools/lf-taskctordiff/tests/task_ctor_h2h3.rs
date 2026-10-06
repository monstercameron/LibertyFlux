//! Differential cases: the eight main-only kinds and the three
//! `0x3e`-derived kinds against their verified rewrites.
//!
//! Same shape as the base+main cases: each case plants the shared
//! header word (plus the float constants for kind `0x3b`), runs the
//! rewrite and the lift on the same arguments and starting bytes, and
//! compares the return, every block byte and the collaborator calls in
//! order, with a deliberately wrong lift caught alongside. 32-bit
//! target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;
    use std::sync::atomic::AtomicU32;
    use std::sync::atomic::Ordering;

    use lf_peds_tasks::task_ctor::{PARAM_LEN, TaskInit, TaskParams};
    use lf_taskctordiff::rewrites::fn_00BF8F90::rw_00bf8f90;
    use lf_taskctordiff::rewrites::fn_00BF8FF0::rw_00bf8ff0;
    use lf_taskctordiff::rewrites::fn_00BF9E30::rw_00bf9e30;
    use lf_taskctordiff::rewrites::fn_00BF9EB0::rw_00bf9eb0;
    use lf_taskctordiff::rewrites::fn_00BF9F00::rw_00bf9f00;
    use lf_taskctordiff::rewrites::fn_00BF9FA0::rw_00bf9fa0;
    use lf_taskctordiff::rewrites::fn_00BF90A0::rw_00bf90a0;
    use lf_taskctordiff::rewrites::fn_00BF7860::rw_00bf7860;
    use lf_taskctordiff::rewrites::fn_00BF7930::rw_00bf7930;
    use lf_taskctordiff::rewrites::fn_00BF9070::rw_00bf9070;
    use lf_taskctordiff::rewrites::fn_00BFA080::rw_00bfa080;
    use lf_taskctordiff::{clear_callees, set_callee, set_f1, set_f2, set_gg};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{EDGE_BYTES, EDGE_WORDS, Rng, addr, lock};

    /// Calls seen by the stubs: tag plus arguments without `this`.
    /// Tags match the [`TaskInit`] method names so both logs compare directly.
    static STUB_LOG: Mutex<Vec<(&'static str, Vec<u32>)>> = Mutex::new(Vec::new());
    /// The object address every stub must receive as `this`.
    static EXPECTED_THIS: AtomicU32 = AtomicU32::new(0);

    fn check_this(tag: &'static str, this: u32, args: Vec<u32>) -> u32 {
        assert_eq!(
            this,
            EXPECTED_THIS.load(Ordering::Relaxed),
            "{tag} got the wrong object address"
        );
        STUB_LOG.lock().unwrap().push((tag, args));
        0
    }

    extern "thiscall" fn stub_main(
        this: u32,
        kind: u32,
        g: u32,
        a1: u32,
        a0: u32,
        flag: u32,
    ) -> u32 {
        check_this("main", this, vec![kind, g, a1, a0, flag])
    }
    extern "thiscall" fn stub_copy_block(this: u32, arg: u32) -> u32 {
        check_this("copy_block", this, vec![arg])
    }
    extern "thiscall" fn stub_quant_vec(this: u32, arg: u32) -> u32 {
        check_this("quant_vec", this, vec![arg])
    }
    extern "thiscall" fn stub_quant_byte(this: u32, arg: u32) -> u32 {
        check_this("quant_byte", this, vec![arg])
    }
    extern "thiscall" fn stub_pair_first(this: u32, arg: u32) -> u32 {
        check_this("pair_first", this, vec![arg])
    }
    extern "thiscall" fn stub_pair_second(this: u32, arg: u32) -> u32 {
        check_this("pair_second", this, vec![arg])
    }
    extern "thiscall" fn stub_aux(this: u32, arg: u32) -> u32 {
        check_this("block_aux", this, vec![arg])
    }
    extern "thiscall" fn stub_sub_3e(this: u32, a0: u32, a1: u32, a_last: u32) -> u32 {
        check_this("sub_3e", this, vec![a0, a1, a_last])
    }
    extern "thiscall" fn stub_block_3a(this: u32, arg: u32) -> u32 {
        check_this("block_3a", this, vec![arg])
    }

    /// Recording collaborator for the lift side.
    #[derive(Default)]
    struct Fake {
        log: Vec<(&'static str, Vec<u32>)>,
    }

    impl TaskInit for Fake {
        fn base(&mut self, kind: u32) {
            self.log.push(("base", vec![kind]));
        }
        fn main(&mut self, kind: u32, g: u32, a1: u32, a0: u32, flag: u32) {
            self.log.push(("main", vec![kind, g, a1, a0, flag]));
        }
        fn block_2f(&mut self, arg: u32) {
            self.log.push(("block_2f", vec![arg]));
        }
        fn quant_word(&mut self, arg: u32) {
            self.log.push(("quant_word", vec![arg]));
        }
        fn chain_43_first(&mut self, arg: u32) {
            self.log.push(("chain_43_first", vec![arg]));
        }
        fn chain_43_second(&mut self, arg: u32) {
            self.log.push(("chain_43_second", vec![arg]));
        }
        fn copy_block(&mut self, arg: u32) {
            self.log.push(("copy_block", vec![arg]));
        }
        fn quant_vec(&mut self, arg: u32) {
            self.log.push(("quant_vec", vec![arg]));
        }
        fn quant_byte(&mut self, arg: u32) {
            self.log.push(("quant_byte", vec![arg]));
        }
        fn pair_first(&mut self, arg: u32) {
            self.log.push(("pair_first", vec![arg]));
        }
        fn pair_second(&mut self, arg: u32) {
            self.log.push(("pair_second", vec![arg]));
        }
        fn block_aux(&mut self, arg: u32) {
            self.log.push(("block_aux", vec![arg]));
        }
        fn sub_3e(&mut self, a0: u32, a1: u32, a_last: u32) {
            self.log.push(("sub_3e", vec![a0, a1, a_last]));
        }
        fn block_3a(&mut self, arg: u32) {
            self.log.push(("block_3a", vec![arg]));
        }
    }

    /// A word argument: edge values interleaved with random words.
    fn word_arg(rng: &mut Rng, i: usize, phase: usize) -> u32 {
        let j = i.wrapping_add(phase);
        if j % 2 == 0 {
            EDGE_WORDS[(j / 2) % EDGE_WORDS.len()]
        } else {
            rng.u32()
        }
    }

    /// A byte argument's full word: edge bytes interleaved with random
    /// words (random high bytes prove the rewrite ignores them).
    fn byte_arg(rng: &mut Rng, i: usize, phase: usize) -> u32 {
        let j = i.wrapping_add(phase);
        if j % 2 == 0 {
            u32::from(EDGE_BYTES[(j / 2) % EDGE_BYTES.len()]) | (rng.u32() & 0xffff_ff00)
        } else {
            rng.u32()
        }
    }

    /// Starting block bytes: zeros once, all ones once, random after.
    fn start_bytes(rng: &mut Rng, i: usize) -> [u8; PARAM_LEN] {
        let mut raw = [0u8; PARAM_LEN];
        if i == 1 {
            raw.fill(0xff);
        } else if i > 1 {
            rng.bytes(&mut raw);
        }
        raw
    }

    /// Reads back the block the rewrite wrote, through a raw pointer:
    /// the borrow checker cannot see writes through the passed address.
    unsafe fn snap(this: u32) -> [u8; PARAM_LEN] {
        unsafe { (this as *const [u8; PARAM_LEN]).read_unaligned() }
    }

    /// Compares the lift's call log with the stubs' log.
    fn check_calls(fake: &Fake, case: usize) {
        assert_eq!(&*STUB_LOG.lock().unwrap(), &fake.log, "case {case} calls");
    }

    /// Float bits exercising truncation: zeros, halves, fractions,
    /// whole magnitudes, the 32-bit range edges, infinities and NaN.
    const TRUNC_BITS: [u32; 16] = [
        0x0000_0000,
        0x8000_0000,
        0x3f00_0000,
        0xbf00_0000,
        0x3fc0_0000,
        0xbfc0_0000,
        0x437f_ff00,
        0x4380_0000,
        0x4f00_0000,
        0xcf00_0000,
        0x4f80_0000,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_0000,
        0x42c8_0000,
    ];

    /// Float-constant pairs for kind `0x3b`: equal, sign-mirrored zero,
    /// unequal, infinities, and NaN pairs that must never set the bit.
    const CONST_PAIRS: [(u32, u32); 8] = [
        (0x3f80_0000, 0x3f80_0000),
        (0x0000_0000, 0x8000_0000),
        (0x3f80_0000, 0x4000_0000),
        (0x7f80_0000, 0x7f80_0000),
        (0x7fc0_0000, 0x7fc0_0000),
        (0x7fc0_0000, 0x3f80_0000),
        (0x3f80_0000, 0x7fc0_0000),
        (0xc000_0000, 0xc000_0000),
    ];

    // Deliberately wrong lifts, each a one-change mirror of its method.
    struct Wrong {
        raw: [u8; PARAM_LEN],
    }

    impl Wrong {
        fn from(start: &[u8; PARAM_LEN]) -> Self {
            Self { raw: *start }
        }
        fn rd(&self, off: usize) -> u8 {
            self.raw[off]
        }
        fn wr8(&mut self, off: usize, v: u8) {
            self.raw[off] = v;
        }
        fn wr32(&mut self, off: usize, v: u32) {
            self.raw[off..off + 4].copy_from_slice(&v.to_le_bytes());
        }
        // Wrong header flag: main runs with 1 instead of 0.
        fn w3d(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32) {
            init.main(0x3d, g, a1, a0, 1);
            self.wr8(0x02, 6);
        }
        // Wrong tag: 6 instead of 7.
        fn w42(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32) {
            init.main(0x42, g, a1, a0, 0);
            init.block_aux(a2);
            self.wr8(0x02, 6);
        }
        // Wrong offset: the word lands at +0x20 instead of +0x1c.
        fn w46(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32, a3: u32) {
            init.main(0x46, g, a1, a0, 0);
            init.copy_block(a2);
            self.wr32(0x20, a3);
            self.wr8(0x02, 7);
        }
        // Wrong shift: the pack shifts by 2 instead of 3.
        #[allow(clippy::too_many_arguments)]
        fn w35(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u8,
            a4: u8,
            a5: u8,
            a6: u8,
        ) {
            init.main(0x35, g, a1, a0, 0);
            init.copy_block(a2);
            let mut cl = a6 & 1;
            cl = cl.wrapping_add(cl) | (a5 & 1);
            cl = cl.wrapping_add(cl) | (a4 & 1);
            cl = (cl << 2) | (a3.wrapping_sub(1) & 7);
            cl = cl.wrapping_add(cl) | (self.rd(0x1c) & 0x81);
            self.wr8(0x1c, cl);
            self.wr8(0x02, 1);
        }
        // Wrong mask: the fold keeps bit 0 too.
        fn w33(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u8) {
            init.main(0x33, g, a1, a0, 0);
            init.copy_block(a2);
            init.quant_vec(a3);
            let old = self.rd(0x1c);
            let t = a4.wrapping_sub(1).wrapping_mul(2) ^ old;
            self.wr8(0x02, 1);
            self.wr8(0x1c, old ^ (t & 0x0f));
        }
        // Wrong mask: keeps bits 6..7 of the old byte, not 5..7.
        #[allow(clippy::too_many_arguments)]
        fn w41(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u8,
            a5: u32,
            a6: u8,
            a7: u8,
            a8: u8,
        ) {
            init.main(0x41, g, a1, a0, 0);
            init.copy_block(a2);
            init.quant_byte(a3);
            let mut cl = a8 & 3;
            cl = cl.wrapping_add(cl) | (a4 & 1);
            cl = cl.wrapping_add(cl) | (a7 & 1);
            cl = cl.wrapping_add(cl) | (self.rd(0x24) & 0xc0);
            cl |= a6 & 1;
            self.wr32(0x20, a5);
            self.wr8(0x24, cl);
            self.wr8(0x02, 7);
        }
        // Wrong pack: forgets to double the truncated low byte.
        #[allow(clippy::too_many_arguments)]
        fn w44(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u32,
            a5: u8,
        ) {
            init.main(0x44, g, a1, a0, 0);
            init.pair_first(a2);
            init.pair_second(a3);
            let x = f32::from_bits(a4);
            let c = if x.is_nan() || x >= 2147483648.0 || x <= -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            };
            let cl = (c as u8) | (a5 & 1);
            self.wr8(0x20, cl);
            self.wr8(0x02, 7);
        }
        // Wrong mask: the keep-branch clears bit 2 too.
        #[allow(clippy::too_many_arguments)]
        fn w45(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u32,
            a5: u8,
            a6: u8,
            a7: u8,
            a8: u32,
            a9: u32,
        ) {
            init.main(0x45, g, a1, a0, 0);
            if a1 != 0 {
                init.block_aux(a2);
            } else {
                init.pair_first(a3);
                init.pair_second(a4);
            }
            self.wr32(0x20, a9);
            if a7.wrapping_sub(1) > 2 {
                self.wr8(0x28, self.rd(0x28) & 0xf0);
            } else {
                let old = self.rd(0x28);
                let t = (a7 << 2) ^ old;
                self.wr8(0x28, old ^ (t & 0x0c));
            }
            let cl = (self.rd(0x28) & 0xfc) | (((a6 & 1) << 1) | (a5 & 1));
            self.wr8(0x28, cl);
            self.wr32(0x24, a8);
            self.wr8(0x02, 7);
        }
        // Wrong mask: keeps bits 4..7 of the old byte, not 3..7.
        #[allow(clippy::too_many_arguments)]
        fn w3a(
            &mut self,
            init: &mut Fake,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u8,
            a5: u8,
            a6: u8,
        ) {
            init.sub_3e(a0, a1, a3);
            self.wr8(0x00, 0x3a);
            init.block_3a(a2);
            let mut cl = (a6 & 1).wrapping_mul(2) | (a5 & 1);
            cl = cl.wrapping_mul(2) | (self.rd(0x2c) & 0xf0);
            cl |= a4 & 1;
            self.wr8(0x14, 2);
            self.wr8(0x2c, cl);
            self.wr8(0x02, 4);
        }
        // Wrong bit: the equality sets bit 2 instead of bit 1.
        #[allow(clippy::too_many_arguments)]
        fn w3b(
            &mut self,
            init: &mut Fake,
            a0: u32,
            a1: u32,
            a2: u8,
            a3: u32,
            a4: u32,
            a5: u8,
            a6: u8,
            a7: u8,
            f1_bits: u32,
            f2_bits: u32,
        ) {
            init.sub_3e(a0, a1, a3);
            self.wr8(0x14, a2.wrapping_add(2));
            self.wr8(0x20, a5);
            self.wr8(0x21, a6);
            let old = self.rd(0x23);
            let t = old ^ a7;
            self.wr8(0x00, 0x3b);
            self.wr8(0x23, old ^ (t & 1));
            self.wr32(0x1c, a4);
            self.wr8(0x22, a2);
            let f1 = f32::from_bits(f1_bits);
            let f2 = f32::from_bits(f2_bits);
            self.wr8(0x02, 4);
            if f1 == f2 {
                self.wr8(0x03, self.rd(0x03) | 4);
            }
        }
        // Wrong base: copies from one word past the block start.
        fn w3c(&mut self, init: &mut Fake, a0: u32, a1: u32, a2: u32, src: &[u32; 4]) {
            init.sub_3e(a0, a1, a2);
            self.wr8(0x00, 0x3c);
            self.wr32(0x1c, src[1]);
            self.wr32(0x20, src[2]);
            self.wr32(0x24, src[3]);
            self.wr8(0x14, 0);
            self.wr8(0x02, 5);
        }
    }

    /// Plants the main-only header stub shared by every H2 case.
    fn plant_h2() {
        clear_callees();
        STUB_LOG.lock().unwrap().clear();
        set_callee(1, stub_main as *const () as usize as u32);
    }

    /// Plants the sub-initialiser stub shared by every H3 case.
    fn plant_h3() {
        clear_callees();
        STUB_LOG.lock().unwrap().clear();
        set_callee(1, stub_sub_3e as *const () as usize as u32);
    }

    const CASES: usize = 96;

    #[test]
    fn kind_3d_matches() {
        let _guard = lock();
        plant_h2();
        let mut rng = Rng(0x3d01);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
            );
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf9070(this, a0, a1) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3d(&mut fake, g, a0, a1);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3d(&mut wfake, g, a0, a1);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x3d lift never caught");
    }

    #[test]
    fn kind_42_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_aux as *const () as usize as u32);
        let mut rng = Rng(0x4201);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
            );
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bfa080(this, a0, a1, a2) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_42(&mut fake, g, a0, a1, a2);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w42(&mut wfake, g, a0, a1, a2);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x42 lift never caught");
    }

    #[test]
    fn kind_46_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_copy_block as *const () as usize as u32);
        let mut rng = Rng(0x4601);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
                word_arg(&mut rng, i, 4),
            );
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf9fa0(this, a0, a1, a2, a3) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_46(&mut fake, g, a0, a1, a2, a3);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w46(&mut wfake, g, a0, a1, a2, a3);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x46 lift never caught");
    }

    #[test]
    fn kind_35_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_copy_block as *const () as usize as u32);
        let mut rng = Rng(0x3501);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
            );
            let (w3, w4, w5, w6) = (
                byte_arg(&mut rng, i, 4),
                byte_arg(&mut rng, i, 5),
                byte_arg(&mut rng, i, 6),
                byte_arg(&mut rng, i, 7),
            );
            let (a3, a4, a5, a6) = (w3 as u8, w4 as u8, w5 as u8, w6 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf7860(this, a0, a1, a2, w3, w4, w5, w6) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_35(&mut fake, g, a0, a1, a2, a3, a4, a5, a6);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w35(&mut wfake, g, a0, a1, a2, a3, a4, a5, a6);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x35 lift never caught");
    }

    #[test]
    fn kind_33_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_copy_block as *const () as usize as u32);
        set_callee(3, stub_quant_vec as *const () as usize as u32);
        let mut rng = Rng(0x3301);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
                word_arg(&mut rng, i, 4),
            );
            let w4 = byte_arg(&mut rng, i, 5);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf7930(this, a0, a1, a2, a3, w4) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_33(&mut fake, g, a0, a1, a2, a3, w4 as u8);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w33(&mut wfake, g, a0, a1, a2, a3, w4 as u8);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x33 lift never caught");
    }

    #[test]
    fn kind_41_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_copy_block as *const () as usize as u32);
        set_callee(3, stub_quant_byte as *const () as usize as u32);
        let mut rng = Rng(0x4101);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2, a3, a5) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
                word_arg(&mut rng, i, 4),
                word_arg(&mut rng, i, 6),
            );
            let (w4, w6, w7, w8) = (
                byte_arg(&mut rng, i, 5),
                byte_arg(&mut rng, i, 7),
                byte_arg(&mut rng, i, 8),
                byte_arg(&mut rng, i, 9),
            );
            let (a4, a6, a7, a8) = (w4 as u8, w6 as u8, w7 as u8, w8 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf9e30(this, a0, a1, a2, a3, w4, a5, w6, w7, w8) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_41(&mut fake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w41(&mut wfake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x41 lift never caught");
    }

    #[test]
    fn kind_44_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_pair_first as *const () as usize as u32);
        set_callee(3, stub_pair_second as *const () as usize as u32);
        let mut rng = Rng(0x4401);
        let mut caught = 0u32;
        let mut saw_nan = 0u32;
        let mut saw_huge = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
                word_arg(&mut rng, i, 4),
            );
            let a4 = if i < TRUNC_BITS.len() {
                TRUNC_BITS[i]
            } else {
                rng.u32()
            };
            let w5 = byte_arg(&mut rng, i, 6);
            let x = f32::from_bits(a4);
            if x.is_nan() {
                saw_nan += 1;
            }
            if x.abs() >= 2147483648.0 {
                saw_huge += 1;
            }
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf9eb0(this, a0, a1, a2, a3, a4, w5) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_44(&mut fake, g, a0, a1, a2, a3, a4, w5 as u8);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w44(&mut wfake, g, a0, a1, a2, a3, a4, w5 as u8);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(saw_nan > 0 && saw_huge > 0, "truncation edges missing");
        assert!(caught > 0, "wrong 0x44 lift never caught");
    }

    #[test]
    fn kind_45_matches() {
        let _guard = lock();
        plant_h2();
        set_callee(2, stub_aux as *const () as usize as u32);
        set_callee(3, stub_pair_first as *const () as usize as u32);
        set_callee(4, stub_pair_second as *const () as usize as u32);
        let mut rng = Rng(0x4501);
        let mut caught = 0u32;
        let mut arms = (0u32, 0u32);
        for i in 0..CASES {
            let g = word_arg(&mut rng, i, 0);
            let a0 = word_arg(&mut rng, i, 1);
            // Alternate zero and nonzero branch words so both arms run.
            let a1 = if i % 2 == 0 {
                0
            } else {
                word_arg(&mut rng, i, 2) | 1
            };
            let a2 = word_arg(&mut rng, i, 3);
            let a3 = word_arg(&mut rng, i, 4);
            let a4 = word_arg(&mut rng, i, 5);
            let w5 = byte_arg(&mut rng, i, 6);
            let w6 = byte_arg(&mut rng, i, 7);
            // Favour the small fold selectors 1..3 so both folds run.
            let w7 = if i % 3 == 0 {
                1 + (i as u32 % 3)
            } else {
                byte_arg(&mut rng, i, 8)
            };
            let a8 = word_arg(&mut rng, i, 9);
            let a9 = word_arg(&mut rng, i, 10);
            if a1 == 0 {
                arms.1 += 1;
            } else {
                arms.0 += 1;
            }
            let (a5, a6, a7) = (w5 as u8, w6 as u8, w7 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf9f00(this, a0, a1, a2, a3, a4, w5, w6, w7, a8, a9) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_45(&mut fake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w45(&mut wfake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(arms.0 > 0 && arms.1 > 0, "one branch arm never ran");
        assert!(caught > 0, "wrong 0x45 lift never caught");
    }

    #[test]
    fn kind_3a_matches() {
        let _guard = lock();
        plant_h3();
        set_callee(2, stub_block_3a as *const () as usize as u32);
        let mut rng = Rng(0x3a01);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (a0, a1, a2, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 3),
            );
            let (w4, w5, w6) = (
                byte_arg(&mut rng, i, 4),
                byte_arg(&mut rng, i, 5),
                byte_arg(&mut rng, i, 6),
            );
            let (a4, a5, a6) = (w4 as u8, w5 as u8, w6 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf8f90(this, a0, a1, a2, a3, w4, w5, w6) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3a(&mut fake, a0, a1, a2, a3, a4, a5, a6);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3a(&mut wfake, a0, a1, a2, a3, a4, a5, a6);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x3a lift never caught");
    }

    #[test]
    fn kind_3b_matches() {
        let _guard = lock();
        plant_h3();
        let mut rng = Rng(0x3b01);
        let mut caught = 0u32;
        let mut outcomes = (0u32, 0u32);
        for i in 0..CASES {
            let (a0, a1, a3, a4) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 3),
                word_arg(&mut rng, i, 4),
            );
            let (w2, w5, w6, w7) = (
                byte_arg(&mut rng, i, 2),
                byte_arg(&mut rng, i, 5),
                byte_arg(&mut rng, i, 6),
                byte_arg(&mut rng, i, 7),
            );
            let (a2, a5, a6, a7) = (w2 as u8, w5 as u8, w6 as u8, w7 as u8);
            let (f1, f2) = if i < CONST_PAIRS.len() * 4 {
                CONST_PAIRS[i % CONST_PAIRS.len()]
            } else {
                (rng.u32(), rng.u32())
            };
            if f32::from_bits(f1) == f32::from_bits(f2) {
                outcomes.0 += 1;
            } else {
                outcomes.1 += 1;
            }
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_f1(f1);
            set_f2(f2);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf8ff0(this, a0, a1, w2, a3, a4, w5, w6, w7) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3b(&mut fake, a0, a1, a2, a3, a4, a5, a6, a7, f1, f2);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3b(&mut wfake, a0, a1, a2, a3, a4, a5, a6, a7, f1, f2);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(outcomes.0 > 0 && outcomes.1 > 0, "compare outcomes missing");
        assert!(caught > 0, "wrong 0x3b lift never caught");
    }

    #[test]
    fn kind_3c_matches() {
        let _guard = lock();
        plant_h3();
        let mut rng = Rng(0x3c01);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (a0, a1, a2) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
            );
            let mut src = Box::new([0u32; 4]);
            if i == 1 {
                src.fill(0xffff_ffff);
            } else if i > 1 {
                for w in src.iter_mut() {
                    *w = rng.u32();
                }
            }
            let snap_src = [src[0], src[1], src[2]];
            let src_addr = addr(&src[0]);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf90a0(this, a0, a1, a2, src_addr) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3c(&mut fake, a0, a1, a2, &snap_src);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3c(&mut wfake, a0, a1, a2, &src);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
            std::hint::black_box(&mut src);
        }
        assert!(caught > 0, "wrong 0x3c lift never caught");
    }
}
