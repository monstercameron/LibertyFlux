// original: 0x0059DBF0 timing_block_init (proposed)

/// Initialise a mainloop timing state block: zero every field of the
/// header (bytes `0x00..0x58`, with three constants), construct three
/// embedded sub-objects through their intercepted constructors, publish an
/// interlocked slot, and zero the counter and configuration regions.
///
/// `state` points to a caller-provided block of at least `0x452` bytes.
/// Sub-object constructors run for the blocks at `+0x60`, `+0x384` and
/// `+0x3D0` (thiscall, no stack arguments, results ignored); the
/// interlocked exchange publishes value `0` to the slot at `+0x3B8` and its
/// previous value is ignored. Constants: `+0x04 = 1`, `+0x34 = 0x0B` (11),
/// `+0x44 = 3`, `+0x39C = 1`, and the two bytes at `+0x3CC = 0x01, 0x01`.
/// Untouched holes keep the caller's bytes: `0x5C..0x5F`, `0x330..0x337`,
/// `0x382..0x383`, `0x3A1..0x3A3`, `0x3A9..0x3AB`, `0x3B8..0x3BB` (written
/// only by the real exchange, which is intercepted), `0x419..0x43B`, and
/// everything from `0x452` on. No field is read before it is written.
///
/// Original: 0x0059DBF0 (thiscall, no stack arguments, returns `state`).
/// No floating point, no thread-local state, no indirection.
lf_checker_rt::export!(thiscall, rw_0059DBF0(state: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const SUB_A: u32 = 0x60;
        const SUB_B: u32 = 0x384;
        const SUB_C: u32 = 0x3D0;
        const XCHG_SLOT: u32 = 0x3B8;
        wr32(state.wrapping_add(0x0), 0x0);
        wr32(state.wrapping_add(0x4), 0x1);
        wr32(state.wrapping_add(0x8), 0x0);
        wr32(state.wrapping_add(0xC), 0x0);
        wr32(state.wrapping_add(0x10), 0x0);
        wr32(state.wrapping_add(0x14), 0x0);
        wr32(state.wrapping_add(0x18), 0x0);
        wr32(state.wrapping_add(0x1C), 0x0);
        wr32(state.wrapping_add(0x20), 0x0);
        wr32(state.wrapping_add(0x24), 0x0);
        wr32(state.wrapping_add(0x28), 0x0);
        wr32(state.wrapping_add(0x2C), 0x0);
        wr32(state.wrapping_add(0x30), 0x0);
        wr32(state.wrapping_add(0x34), 0xB);
        wr32(state.wrapping_add(0x38), 0x0);
        wr32(state.wrapping_add(0x3C), 0x0);
        wr32(state.wrapping_add(0x40), 0x0);
        wr32(state.wrapping_add(0x44), 0x3);
        wr32(state.wrapping_add(0x48), 0x0);
        wr32(state.wrapping_add(0x4C), 0x0);
        wr32(state.wrapping_add(0x50), 0x0);
        wr32(state.wrapping_add(0x54), 0x0);
        wr32(state.wrapping_add(0x58), 0x0);
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, state.wrapping_add(SUB_A));
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, state.wrapping_add(SUB_B));
        wr8(state.wrapping_add(0x3C0), 0x0);
        wr32(state.wrapping_add(0x3C4), 0x0);
        wr32(state.wrapping_add(0x3C8), 0x0);
        wr32(state.wrapping_add(0x3CC), 0x0);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, state.wrapping_add(SUB_C));
        wr32(state.wrapping_add(0x404), 0x0);
        wr32(state.wrapping_add(0x408), 0x0);
        wr8(state.wrapping_add(0x40C), 0x0);
        wr32(state.wrapping_add(0x410), 0x0);
        wr32(state.wrapping_add(0x414), 0x0);
        wr8(state.wrapping_add(0x418), 0x0);
        wr32(state.wrapping_add(0x43C), 0x0);
        wr32(state.wrapping_add(0x440), 0x0);
        wr32(state.wrapping_add(0x444), 0x0);
        wr32(state.wrapping_add(0x448), 0x0);
        wr32(state.wrapping_add(0x44C), 0x0);
        wr16(state.wrapping_add(0x450), 0x0);
        wr32(state.wrapping_add(0x310), 0x0);
        wr32(state.wrapping_add(0x314), 0x0);
        wr32(state.wrapping_add(0x318), 0x0);
        wr32(state.wrapping_add(0x31C), 0x0);
        wr32(state.wrapping_add(0x320), 0x0);
        wr32(state.wrapping_add(0x324), 0x0);
        wr32(state.wrapping_add(0x328), 0x0);
        wr32(state.wrapping_add(0x32C), 0x0);
        wr32(state.wrapping_add(0x338), 0x0);
        wr32(state.wrapping_add(0x33C), 0x0);
        wr32(state.wrapping_add(0x340), 0x0);
        wr32(state.wrapping_add(0x344), 0x0);
        wr32(state.wrapping_add(0x348), 0x0);
        wr32(state.wrapping_add(0x34C), 0x0);
        wr32(state.wrapping_add(0x350), 0x0);
        wr32(state.wrapping_add(0x354), 0x0);
        wr16(state.wrapping_add(0x358), 0x0);
        wr32(state.wrapping_add(0x35C), 0x0);
        wr32(state.wrapping_add(0x360), 0x0);
        wr32(state.wrapping_add(0x364), 0x0);
        wr32(state.wrapping_add(0x368), 0x0);
        wr32(state.wrapping_add(0x36C), 0x0);
        wr32(state.wrapping_add(0x370), 0x0);
        wr32(state.wrapping_add(0x374), 0x0);
        wr32(state.wrapping_add(0x378), 0x0);
        wr32(state.wrapping_add(0x37C), 0x0);
        wr16(state.wrapping_add(0x380), 0x0);
        wr32(state.wrapping_add(0x384), 0x0);
        wr32(state.wrapping_add(0x388), 0x0);
        wr32(state.wrapping_add(0x38C), 0x0);
        wr32(state.wrapping_add(0x390), 0x0);
        wr16(state.wrapping_add(0x394), 0x0);
        wr32(state.wrapping_add(0x396), 0x0);
        wr32(state.wrapping_add(0x39C), 0x1);
        wr8(state.wrapping_add(0x3A0), 0x0);
        wr32(state.wrapping_add(0x3A4), 0x0);
        wr8(state.wrapping_add(0x3A8), 0x0);
        wr32(state.wrapping_add(0x3AC), 0x0);
        wr32(state.wrapping_add(0x3B0), 0x0);
        wr32(state.wrapping_add(0x3B4), 0x0);
        wr16(state.wrapping_add(0x3CC), 0x101);
        wr8(state.wrapping_add(0x3CE), 0x0);
        let _: u32 = lf_checker_rt::callee_stdcall!(4, u32, state.wrapping_add(XCHG_SLOT), 0);
        wr32(state.wrapping_add(0x3BC), 0x0);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, state.wrapping_add(SUB_C));
        state
    }
});
