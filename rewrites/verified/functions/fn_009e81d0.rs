// original: 0x009e81d0 CPlayerPed::vf39

/// Resolve the slot pose into `out`: fast copy or blended table-driven solve.
///
/// Fast path (flag bit `0x40` at `this+0xf4` clear and the slot word at
/// `this+0x38` either zero or equal to the word at `this+0x7b4`): copies four
/// words from `src+0x30` (`src` at `this+0x20`) into `out` and returns the
/// last word. No calls.
///
/// Slow path: reads a signed 16-bit index from `this+0x2e` and, twice (with
/// probe words 6 then 3), loads a row pointer from the game table at file
/// `0x1295cd8`, calls the handler in that row's vtable slot `+0x38`, and
/// feeds each answer to the resolver (callee 2), which yields a float triple
/// at `+0x30`. The first triple is scaled by constant B (file `0xfe8858`),
/// the second by constant A (file `0xfe881c`), and the sums are stored to
/// `out[0..3]`; `out[3]` is the frame slot at `[esp+0x3c]`, which the
/// original never stores (defined as +0.0 by the contract's `stack_fill`).
/// Note the first triple's z is stored with one push outstanding, so its
/// `[esp+0x2c]` lands on the same slot the blend later reads as `[esp+0x28]`.
/// Returns `out`.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_009e81d0(this: u32, out: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xf4;
        const FLAG_SLOW: u8 = 0x40;
        const SLOT_OFF: u32 = 0x38;
        const MATCH_OFF: u32 = 0x7b4;
        const SRC_OFF: u32 = 0x20;
        const IDX_OFF: u32 = 0x2e;
        const POSE_TABLE: u32 = 0x1295cd8;
        const VT_SLOT: u32 = 0x38;
        const TRIPLE_OFF: u32 = 0x30;
        const SCALE_A: u32 = 0xfe881c;
        const SCALE_B: u32 = 0xfe8858;
        const PROBE_FIRST: u32 = 6;
        const PROBE_SECOND: u32 = 3;
        type Hook = extern "thiscall" fn(u32, u32) -> u32;
        let slow_flag = ((this + FLAG_OFF) as *const u8).read() & FLAG_SLOW != 0;
        let slot = ((this + SLOT_OFF) as *const u32).read_unaligned();
        let matched =
            slot == 0 || slot == ((this + MATCH_OFF) as *const u32).read_unaligned();
        if !slow_flag && matched {
            let src = ((this + SRC_OFF) as *const u32).read_unaligned();
            let w0 = ((src + TRIPLE_OFF) as *const u32).read_unaligned();
            let w1 = ((src + TRIPLE_OFF + 4) as *const u32).read_unaligned();
            let w2 = ((src + TRIPLE_OFF + 8) as *const u32).read_unaligned();
            let w3 = ((src + TRIPLE_OFF + 12) as *const u32).read_unaligned();
            (out as *mut u32).write_unaligned(w0);
            ((out + 4) as *mut u32).write_unaligned(w1);
            ((out + 8) as *mut u32).write_unaligned(w2);
            ((out + 12) as *mut u32).write_unaligned(w3);
            return w3;
        }
        let table = lf_checker_rt::relocated(POSE_TABLE);
        let idx1 = ((this + IDX_OFF) as *const i16).read_unaligned() as i32;
        let e1 =
            (table.wrapping_add(idx1.wrapping_mul(4) as u32) as *const u32).read_unaligned();
        let v1 = (e1 as *const u32).read_unaligned();
        let h1: Hook =
            core::mem::transmute(((v1 + VT_SLOT) as *const u32).read_unaligned() as usize);
        let p1 = h1(e1, PROBE_FIRST);
        let r1: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, p1);
        let t1x = ((r1 + TRIPLE_OFF) as *const f32).read_unaligned();
        let t1y = ((r1 + TRIPLE_OFF + 4) as *const f32).read_unaligned();
        let t1z = ((r1 + TRIPLE_OFF + 8) as *const f32).read_unaligned();
        let idx2 = ((this + IDX_OFF) as *const i16).read_unaligned() as i32;
        let e2 =
            (table.wrapping_add(idx2.wrapping_mul(4) as u32) as *const u32).read_unaligned();
        let v2 = (e2 as *const u32).read_unaligned();
        let h2: Hook =
            core::mem::transmute(((v2 + VT_SLOT) as *const u32).read_unaligned() as usize);
        let p2 = h2(e2, PROBE_SECOND);
        let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, p2);
        let t2x = ((r2 + TRIPLE_OFF) as *const f32).read_unaligned();
        let t2y = ((r2 + TRIPLE_OFF + 4) as *const f32).read_unaligned();
        let t2z = ((r2 + TRIPLE_OFF + 8) as *const f32).read_unaligned();
        let a = lf_checker_rt::global::<f32>(SCALE_A).read_unaligned();
        let b = lf_checker_rt::global::<f32>(SCALE_B).read_unaligned();
        let bb = core::hint::black_box;
        ((out) as *mut f32).write_unaligned(bb(t1x) * bb(b) + bb(t2x) * bb(a));
        ((out + 4) as *mut f32).write_unaligned(bb(t1y) * bb(b) + bb(t2y) * bb(a));
        ((out + 8) as *mut f32).write_unaligned(bb(t1z) * bb(b) + bb(t2z) * bb(a));
        ((out + 12) as *mut f32).write_unaligned(0.0f32);
        out
    }
});
