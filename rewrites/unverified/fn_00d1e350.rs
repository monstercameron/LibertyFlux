// original: 0x00d1e350 CTaskComplexSeekCoverShooting::vf20
/// Run one cover-shooting think tick: poll the perception state, pick a
/// shooting sub-task and update the aim.
///
/// `this` is the task (subject at `+8`), `ped` the ped. The subject's state
/// slot (`+0xc`, id 1) routes first: 0x2ca exits with the subject. A memory
/// comparison below 30 continues, otherwise id 2 plus callees 10..12 run and
/// exit with the subject. Tag, flag bytes and three more state polls
/// (0x119/0x76d/0x41a) select the middle path; anything else falls into a
/// guard chain (callees 13..15) that either exits with id 4's answer or
/// drops into the late path.
///
/// The middle path samples callee 16 into four words (copied to
/// `this+0xa0..0xac`), fetches the pool worker (id 17) to run callee 18,
/// classifies the target (id 19, optional integer scaler id 20 and flag
/// setter id 21), subtracts the ped position, runs the aim-blend callee
/// (id 23, float answer plus out byte) and the range callee (id 24), then
/// either exits zero, exits with id 25's answer, or drops late.
///
/// The late path re-polls the state (0x11d), resolves the target (id 26),
/// checks its kind (id 5, 0x3ae/0x384) and index (id 27 against the signed
/// byte at `+0x98`, setting bits at the ped's `+0xa80` block on match),
/// \(optionally id 28 and another guard chain, then samples id 29 and runs
/// three float gates (a root distance strictly between two constants, a
/// threshold choice from id 6 (4.0/2.5) against `+0xb8`, a blended
/// difference strictly between two more constants) and a final dot sign.
/// Passing all of them runs the probe (id 32), the validator (id 33), the
/// guards again and the executor (id 34), exiting with its answer.
///
/// The tail re-polls (0x414), optionally toggles (id 8) and notifies
/// (ids 35..36), then either exits with the subject or runs the lock slot
/// (id 3) and the commit slot (id 9), exiting with the commit's answer.
/// Float operation order is the original's throughout; comparisons are
/// strict with NaN failing shut, matching the original's conditional jumps.
/// Original: 0x00d1e350 (thiscall, one stack arg).
lf_checker_rt::export!(thiscall, rw_00d1e350(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x08;
        const PED_POS: u32 = 0x20;
        const PED_EXTRA: u32 = 0x224;
        const PED_TAG: u32 = 0xd68;
        const PED_AUX: u32 = 0xa80;
        const MEM cruelty: u32 = 0;
        const POOL: u32 = 0x0167e2a0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn icall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn icall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0)
            }
        }
        #[inline(always)]
        unsafe fn icall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0, a1, a2)
            }
        }
        #[inline(always)]
        unsafe fn late(this: u32, ped: u32) -> u32;
