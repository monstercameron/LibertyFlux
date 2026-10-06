//! Differential cases: the seven base+main kind initialisers against
//! their verified rewrites.
//!
//! Each case plants the shared header word, fills one 64-byte block with
//! starting bytes, runs the rewrite and
//! [`TaskParams`](lf_peds_tasks::task_ctor::TaskParams) on the same
//! arguments, and compares the return, every block byte and the
//! collaborator calls in order. Byte arguments reach the rewrite as
//! full words with random high bytes and the lift as low bytes, which
//! also proves the narrowing. A deliberately wrong lift of each method
//! must be caught. 32-bit target only.

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
    use lf_taskctordiff::rewrites::fn_00BF9FE0::rw_00bf9fe0;
    use lf_taskctordiff::rewrites::fn_00BF78D0::rw_00bf78d0;
    use lf_taskctordiff::rewrites::fn_00BF90E0::rw_00bf90e0;
    use lf_taskctordiff::rewrites::fn_00BF7810::rw_00bf7810;
    use lf_taskctordiff::rewrites::fn_00BF7980::rw_00bf7980;
    use lf_taskctordiff::rewrites::fn_00BFA0B0::rw_00bfa0b0;
    use lf_taskctordiff::rewrites::fn_00BFA030::rw_00bfa030;
    use lf_taskctordiff::{clear_callees, set_callee, set_gg};

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

    extern "thiscall" fn stub_base(this: u32, kind: u32) -> u32 {
        check_this("base", this, vec![kind])
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
    extern "thiscall" fn stub_block_2f(this: u32, arg: u32) -> u32 {
        check_this("block_2f", this, vec![arg])
    }
    extern "thiscall" fn stub_quant_word(this: u32, arg: u32) -> u32 {
        check_this("quant_word", this, vec![arg])
    }
    extern "thiscall" fn stub_chain_first(this: u32, arg: u32) -> u32 {
        check_this("chain_43_first", this, vec![arg])
    }
    extern "thiscall" fn stub_chain_second(this: u32, arg: u32) -> u32 {
        check_this("chain_43_second", this, vec![arg])
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

    // Deliberately wrong lifts, each a one-change mirror of its method.
    // Every case counts one when the wrong lift's bytes or calls differ
    // from the rewrite's; the test fails when none ever does.
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
        // Wrong offset: the word lands at +0x1c instead of +0x18.
        fn w3e(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32) {
            init.base(0x3e);
            init.main(0x3e, g, a1, a0, 1);
            self.wr8(0x14, 1);
            self.wr32(0x1c, a2);
            self.wr8(0x02, 4);
        }
        // Wrong bias: the mode adds 10 instead of 11.
        fn w36(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u8,
            a3: u32,
            a4: u8,
            a5: u8,
        ) {
            init.base(0x36);
            init.main(0x36, g, a1, a0, 1);
            self.wr8(0x1c, a5);
            self.wr32(0x18, a3);
            self.wr8(0x1d, a2);
            self.wr8(0x1e, a4);
            self.wr8(0x14, a4.wrapping_add(10));
            self.wr8(0x02, 1);
        }
        // Wrong mask: the fold keeps bit 0 too.
        fn w34(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u8, a3: u32, a4: u32) {
            init.base(0x34);
            init.main(0x34, g, a1, a0, 1);
            let old = self.rd(0x24);
            let t = a2.wrapping_sub(1).wrapping_mul(2) ^ old;
            self.wr32(0x18, a3);
            self.wr32(0x1c, a4);
            self.wr8(0x24, old ^ (t & 0x0f));
            self.wr8(0x14, 4);
            self.wr8(0x02, 1);
        }
        // Wrong bit: toggles bit 0 instead of bit 1.
        fn w3f(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u8) {
            init.base(0x3f);
            init.main(0x3f, g, a1, a0, 1);
            let old = self.rd(0x03);
            let t = a2.wrapping_mul(2) ^ old;
            self.wr8(0x02, 9);
            self.wr8(0x03, old ^ (t & 1));
        }
        // Wrong bit: tests bit 1 instead of bit 0.
        fn w40(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u8) {
            init.base(0x40);
            init.main(0x40, g, a1, a0, 1);
            let old = self.rd(0x22);
            let t = old ^ a4;
            self.wr8(0x22, old ^ (t & 2));
            self.wr32(0x18, a2);
            init.quant_word(a3);
            self.wr8(0x02, 8);
        }
        // Wrong threshold: the mode bonus tests bit 4, not bit 3.
        #[allow(clippy::too_many_arguments)]
        fn w2f(
            &mut self,
            init: &mut Fake,
            g: u32,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u8,
            a4: u32,
            a5: u32,
            a6: u32,
            a7: u32,
            a8: u32,
            a9: u8,
            a10: u8,
            a11: u8,
            a12: u8,
        ) {
            init.base(0x2f);
            init.main(0x2f, g, a1, a0, 1);
            init.block_2f(a2);
            let mut packed = (a11 & 1) | a12.wrapping_mul(2);
            packed = (packed << 2) | (a9 & 3);
            packed = packed.wrapping_add(packed) | (a10 & 1);
            self.wr32(0x18, a4);
            self.wr32(0x1c, a5);
            self.wr32(0x20, a6);
            self.wr32(0x24, a7);
            self.wr32(0x28, a8);
            packed = (packed << 3) | (a3 & 7);
            self.wr8(0x3c, packed);
            let mut mode = packed & 7;
            self.wr8(0x02, 2);
            if packed & 16 != 0 {
                mode = mode.wrapping_add(2);
            }
            self.wr8(0x14, mode);
        }
        // Wrong header flag: main runs with 0 instead of 1.
        fn w43(&mut self, init: &mut Fake, g: u32, a0: u32, a1: u32, a2: u32, a3: u32) {
            init.base(0x43);
            init.main(0x43, g, a1, a0, 0);
            init.chain_43_first(a2);
            init.chain_43_second(a3);
            self.wr8(0x02, 7);
        }
    }

    /// Plants the base+main stubs shared by every H1 case.
    fn plant_header() {
        clear_callees();
        STUB_LOG.lock().unwrap().clear();
        set_callee(1, stub_base as *const () as usize as u32);
        set_callee(2, stub_main as *const () as usize as u32);
    }

    const CASES: usize = 96;

    #[test]
    fn kind_3e_matches() {
        let _guard = lock();
        plant_header();
        let mut rng = Rng(0x3e01);
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
            let ret = unsafe { rw_00bf90e0(this, a0, a1, a2) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3e(&mut fake, g, a0, a1, a2);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3e(&mut wfake, g, a0, a1, a2);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x3e lift never caught");
    }

    #[test]
    fn kind_36_matches() {
        let _guard = lock();
        plant_header();
        let mut rng = Rng(0x3601);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 4),
            );
            let (w2, w4, w5) = (
                byte_arg(&mut rng, i, 3),
                byte_arg(&mut rng, i, 5),
                byte_arg(&mut rng, i, 6),
            );
            let (a2, a4, a5) = (w2 as u8, w4 as u8, w5 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf7810(this, a0, a1, w2, a3, w4, w5) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_36(&mut fake, g, a0, a1, a2, a3, a4, a5);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w36(&mut wfake, g, a0, a1, a2, a3, a4, a5);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x36 lift never caught");
    }

    #[test]
    fn kind_34_matches() {
        let _guard = lock();
        plant_header();
        let mut rng = Rng(0x3401);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a3, a4) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 4),
                word_arg(&mut rng, i, 5),
            );
            let w2 = byte_arg(&mut rng, i, 3);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bf78d0(this, a0, a1, w2, a3, a4) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_34(&mut fake, g, a0, a1, w2 as u8, a3, a4);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w34(&mut wfake, g, a0, a1, w2 as u8, a3, a4);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x34 lift never caught");
    }

    #[test]
    fn kind_3f_matches() {
        let _guard = lock();
        plant_header();
        let mut rng = Rng(0x3f01);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
            );
            let w2 = byte_arg(&mut rng, i, 3);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bfa0b0(this, a0, a1, w2) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_3f(&mut fake, g, a0, a1, w2 as u8);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w3f(&mut wfake, g, a0, a1, w2 as u8);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x3f lift never caught");
    }

    #[test]
    fn kind_40_matches() {
        let _guard = lock();
        plant_header();
        set_callee(3, stub_quant_word as *const () as usize as u32);
        let mut rng = Rng(0x4001);
        let mut caught = 0u32;
        for i in 0..CASES {
            let (g, a0, a1, a2, a3) = (
                word_arg(&mut rng, i, 0),
                word_arg(&mut rng, i, 1),
                word_arg(&mut rng, i, 2),
                word_arg(&mut rng, i, 4),
                word_arg(&mut rng, i, 5),
            );
            let w4 = byte_arg(&mut rng, i, 3);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret = unsafe { rw_00bfa030(this, a0, a1, a2, a3, w4) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_40(&mut fake, g, a0, a1, a2, a3, w4 as u8);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w40(&mut wfake, g, a0, a1, a2, a3, w4 as u8);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x40 lift never caught");
    }

    #[test]
    fn kind_2f_matches() {
        let _guard = lock();
        plant_header();
        set_callee(3, stub_block_2f as *const () as usize as u32);
        let mut rng = Rng(0x2f01);
        let mut caught = 0u32;
        for i in 0..CASES {
            let g = word_arg(&mut rng, i, 0);
            let a0 = word_arg(&mut rng, i, 1);
            let a1 = word_arg(&mut rng, i, 2);
            let a2 = word_arg(&mut rng, i, 3);
            let w3 = byte_arg(&mut rng, i, 4);
            let a4 = word_arg(&mut rng, i, 5);
            let a5 = word_arg(&mut rng, i, 6);
            let a6 = word_arg(&mut rng, i, 7);
            let a7 = word_arg(&mut rng, i, 8);
            let a8 = word_arg(&mut rng, i, 9);
            let w9 = byte_arg(&mut rng, i, 10);
            let w10 = byte_arg(&mut rng, i, 11);
            let w11 = byte_arg(&mut rng, i, 12);
            let w12 = byte_arg(&mut rng, i, 13);
            let (a3, a9, a10, a11, a12) = (w3 as u8, w9 as u8, w10 as u8, w11 as u8, w12 as u8);
            let start = start_bytes(&mut rng, i);
            let mut obj = Box::new(start);
            let this = addr(&obj[0]);
            EXPECTED_THIS.store(this, Ordering::Relaxed);
            set_gg(g);
            STUB_LOG.lock().unwrap().clear();
            let ret =
                unsafe { rw_00bf7980(this, a0, a1, a2, w3, a4, a5, a6, a7, a8, w9, w10, w11, w12) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_2f(
                &mut fake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12,
            );
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w2f(
                &mut wfake, g, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12,
            );
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x2f lift never caught");
    }

    #[test]
    fn kind_43_matches() {
        let _guard = lock();
        plant_header();
        set_callee(3, stub_chain_first as *const () as usize as u32);
        set_callee(4, stub_chain_second as *const () as usize as u32);
        let mut rng = Rng(0x4301);
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
            let ret = unsafe { rw_00bf9fe0(this, a0, a1, a2, a3) };
            let after = unsafe { snap(this) };
            assert_eq!(ret, 0, "case {i}");
            let mut lift = TaskParams::from_bytes(start);
            let mut fake = Fake::default();
            lift.init_43(&mut fake, g, a0, a1, a2, a3);
            assert_eq!(lift.bytes(), &after, "case {i} bytes");
            check_calls(&fake, i);
            let mut wrong = Wrong::from(&start);
            let mut wfake = Fake::default();
            wrong.w43(&mut wfake, g, a0, a1, a2, a3);
            if wrong.raw != after || wfake.log != fake.log {
                caught += 1;
            }
            std::hint::black_box(&mut obj);
        }
        assert!(caught > 0, "wrong 0x43 lift never caught");
    }
}
