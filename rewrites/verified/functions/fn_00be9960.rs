// original: 0x00be9960 task_gate_by_kind (proposed)

/// Gate a task object `this` and apply its kind-selected operation with `arg`.
///
/// Callee 1 must approve `(this, arg)` (a zero low byte returns its answer
/// as is). Flag bit 3 of `this+0x4f` then resolves the selector at
/// `this+0x18` through callee 2 (result unused). The selector dispatches:
/// 0x4e looks an object up from `arg`'s target (index byte at `+4`,
/// 0xff means none; slot byte at `+0x40` into a strided shared table plus
/// a scaled stride, like the neighbouring dispatcher) and sends it a zero
/// word via callee 5; 0x73, when the shared mode is 3 or 4, scans two
/// shared slots for `this+0x0c` and for each hit prepares an output block
/// through callee 3 and reports the slot's value plus the block to the
/// fixed shared object via callee 4; anything else skips both.
///
/// The tail sends callee 6 the triple (kept word, enable bit, `this+0x30`)
/// with `arg`'s target as `this`, where the enable bit is bit 4 of
/// `this+0x4e` and the kept word is callee 2's answer when it ran, else
/// zero (the original keeps it in a stack slot the scan's float scratch
/// never touches).
///
/// Returns 1 over the high bytes of callee 6's answer (the original keeps
/// the callee's eax and sets only its low byte).
///
/// Original: 0x00be9960 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00be9960(this: u32, arg: u32) -> u32 {
    unsafe {
        const G_MODE: u32 = 0x011f70cc;
        const G_SCAN: u32 = 0x01682f40;
        const G_STRIDE: u32 = 0x0115d968;
        const G_OTAB: u32 = 0x0115d988;
        const FIXED_OBJ: u32 = 0x01231800;
        const SEL_LOOKUP: u32 = 0x4e;
        const SEL_SCAN: u32 = 0x73;
        const SLOT_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const NO_OBJECT: u8 = 0xff;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(x: u32) -> u8 {
            unsafe { (x as *const u8).read() }
        }

        let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, arg);
        if gate & 0xff == 0 {
            return gate;
        }
        let kept = if rd8(this + 0x4f) & 8 != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, rd32(this + 0x18), 0u32)
        } else {
            0
        };
        let sel = rd32(this + 0x18);
        if sel == SEL_LOOKUP {
            let inner = rd32(arg);
            let ix = rd8(inner + 4);
            let obj = if ix == NO_OBJECT {
                0
            } else {
                let stride = rd32(lf_checker_rt::relocated(G_STRIDE));
                let base = rd32(lf_checker_rt::relocated(G_OTAB));
                let slot = rd8(inner + 0x40);
                let entry = rd32(
                    base
                        .wrapping_add((slot as u32).wrapping_mul(SLOT_STRIDE))
                        .wrapping_add(TABLE_BIAS),
                );
                stride.wrapping_mul(ix as u32).wrapping_add(entry)
            };
            lf_checker_rt::callee_thiscall!(5, u32, obj, 0u32);
        } else if sel == SEL_SCAN {
            let mode = rd32(lf_checker_rt::relocated(G_MODE));
            if mode == 3 || mode == 4 {
                let want = rd32(this + 0x0c);
                let mut slot = 0u32;
                while slot < 2 {
                    let base = lf_checker_rt::relocated(G_SCAN).wrapping_add(slot * 8);
                    if rd32(base.wrapping_sub(4)) == want {
                        let mut out = [0u32; 4];
                        lf_checker_rt::callee_thiscall!(3, u32, out.as_mut_ptr() as u32);
                        lf_checker_rt::callee_thiscall!(
                            4,
                            u32,
                            lf_checker_rt::relocated(FIXED_OBJ),
                            rd32(base),
                            out.as_mut_ptr() as u32,
                            0xffff_ffff,
                            0u32,
                            0u32
                        );
                    }
                    slot += 1;
                }
            }
        }
        let bit = ((rd8(this + 0x4e) >> 4) & 1) as u32;
        let tail_ans: u32 =
            lf_checker_rt::callee_thiscall!(6, u32, rd32(arg), kept, bit, rd32(this + 0x30));
        (tail_ans & 0xffff_ff00) | 1
    }
});
