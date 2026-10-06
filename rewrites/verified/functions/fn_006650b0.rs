// original: 0x006650B0 sn_init_task_params (proposed)

/// Initialise a network task's parameter block from 23 caller words, then
/// attach it to the session queue.
///
/// `this` is the task, `edx` a fallback session, and the 23 stack words
/// carry the parameters; word 4 (`esi`) is a queue link (null or a small
/// heap object whose first word is a state). When the link is non-null and
/// its state is 1, the whole initialisation is skipped and control goes to
/// the attach step.
///
/// Otherwise: when the task's session word (`+0x60`) is 0 it takes `edx`;
/// a context pair is formed from the session's words at `+0xd8`/`+0xdc`
/// when the session state (`+0x50`, SIGNED) is 2 or 3, else (0, 0) (the
/// original zeroes them through a vector register). The format callee
/// (cdecl, first megabyte) is called with the task's inline buffer
/// (`+0x6d`), a fixed image address, and the pair. Then the 20 parameter
/// words, two 64-bit values and three 16-bit values are stored to their
/// slots (`+0x9c`, `+0x75c`, `+0x760`, `+0x764`, `+0xa0`..`+0xac`,
/// `+0xb0`, `+0xb8`, `+0xc0`, `+0xc4`, `+0xc8`, `+0xcc`, `+0xd0`, `+0xd4`,
/// `+0xd8`, `+0xdc`, `+0xe0`, `+0xe4`), and the bind callee (thiscall on
/// the task with the link) runs; a nonzero low byte in its answer returns
/// immediately with that answer's upper bytes and low byte set to 1.
///
/// Attach step: a null link or a link state of 2 returns the bind
/// answer's upper bytes with low byte 0. Otherwise the link is exchanged
/// to 1 (interlocked), its second word cleared, and a
/// compare-exchange of 2 for 1 issued; when that answers 1 the second
/// word becomes -1. The return is the compare-exchange answer's upper
/// bytes with low byte 0.
///
/// Calling convention: the original takes its stack words caller-cleaned
/// (bare return), which no Rust declaration combines with register
/// arguments, so this rewrite is fastcall (callee pops 92). The checker
/// verifies identical register and stack inputs, memory, calls and
/// result; only the stack adjustment differs (see narrowed).
///
/// Original: 0x006650B0 (ecx=this, edx=session, 23 stack words).
lf_checker_rt::export!(fastcall, rw_006650B0(
    this: u32, edx: u32,
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32,
    a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32, a15: u32,
    a16: u32, a17: u32, a18: u32, a19: u32, a20: u32, a21: u32, a22: u32,
) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const STATE: u32 = 0x50;
        const FMT_CONST: u32 = 0x00F9_C370;
        const FMT_ID: u32 = 1;
        const BIND_ID: u32 = 2;
        const EXCH_ID: u32 = 3;
        const CAS_ID: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let esi = a4;
        let bl = 0u32; // The original zeroes bl at entry and never changes it.
        let mut ans2 = 0u32;
        if esi == 0 || rd32(esi) != 1 {
            if rd32(this.wrapping_add(CTX)) == 0 {
                wr32(this.wrapping_add(CTX), edx);
            }
            let ctx = rd32(this.wrapping_add(CTX));
            let (lo, hi) = if ctx != 0 {
                let st = rd32(ctx.wrapping_add(STATE)) as i32;
                if st >= 2 && st <= 3 {
                    (rd32(ctx.wrapping_add(0xd8)), rd32(ctx.wrapping_add(0xdc)))
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            };
            let _: u32 = lf_checker_rt::callee_cdecl!(
                FMT_ID,
                u32,
                this.wrapping_add(0x6d),
                lf_checker_rt::relocated(FMT_CONST),
                lo,
                hi
            );
            wr32(this.wrapping_add(0x9c), a0);
            wr32(this.wrapping_add(0x75c), a1);
            wr32(this.wrapping_add(0x760), a2);
            wr32(this.wrapping_add(0x764), a3);
            wr32(this.wrapping_add(0xa0), a5);
            wr32(this.wrapping_add(0xa4), a6);
            wr32(this.wrapping_add(0xa8), a7);
            wr32(this.wrapping_add(0xac), a8);
            wr32(this.wrapping_add(0xb0), a9);
            wr32(this.wrapping_add(0xb0).wrapping_add(4), a10);
            wr32(this.wrapping_add(0xc0), a13);
            wr32(this.wrapping_add(0xb8), a11);
            wr32(this.wrapping_add(0xb8).wrapping_add(4), a12);
            wr32(this.wrapping_add(0xc4), a14);
            wr16(this.wrapping_add(0xc8), a15 as u16);
            wr32(this.wrapping_add(0xcc), a16);
            wr16(this.wrapping_add(0xd0), a17 as u16);
            wr32(this.wrapping_add(0xd4), a18);
            wr16(this.wrapping_add(0xd8), a19 as u16);
            wr32(this.wrapping_add(0xdc), a20);
            wr32(this.wrapping_add(0xe0), a21);
            wr32(this.wrapping_add(0xe4), a22);
            ans2 = lf_checker_rt::callee_thiscall!(BIND_ID, u32, this, esi);
            if (ans2 as u8) != 0 {
                return (ans2 & 0xFFFF_FF00) | 1;
            }
        }
        if esi == 0 || rd32(esi) == 2 {
            return (ans2 & 0xFFFF_FF00) | bl;
        }
        let _: u32 = lf_checker_rt::callee_stdcall!(EXCH_ID, u32, esi, 1);
        wr32(esi.wrapping_add(4), 0);
        let ans4: u32 = lf_checker_rt::callee_stdcall!(CAS_ID, u32, esi, 2, 1);
        if ans4 == 1 {
            wr32(esi.wrapping_add(4), 0xFFFF_FFFF);
        }
        (ans4 & 0xFFFF_FF00) | bl
    }
});
