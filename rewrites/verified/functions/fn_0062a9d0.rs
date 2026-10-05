// original: 0x0062A9D0 streaming_object_init (proposed)

/// Initialise a streaming object: query its helper, then zero the state.
///
/// Calls the helper (patched callee) with `this` still in the accumulator
/// slot, reads two words through the returned pointer (observed only through
/// fault parity: the values are discarded), stamps the class vtable pointer
/// at `+0x80` and zeroes the flag words, the 64-bit lanes and the tail slots.
/// Returns `this` (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0062a9d0(this: u32) -> u32 {
    unsafe {
        const VTABLE_STREAM_OBJ: u32 = 0xEC5E14;
        const CALLEE_HELPER: u32 = 1;
        let got: u32 = lf_checker_rt::callee_thiscall!(CALLEE_HELPER, u32, this);
        // Dead loads: the original reads two words here and drops them. Kept
        // so a bad helper answer faults on both sides alike.
        let first = (got as *const u32).read_unaligned();
        let second = ((got + 4) as *const u32).read_unaligned();
        core::hint::black_box(first);
        core::hint::black_box(second);
        unsafe fn w32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        unsafe fn w64(base: u32, off: u32) {
            unsafe { ((base + off) as *mut u64).write_unaligned(0) }
        }
        w32(this, 0x3c, 0);
        w32(this, 0x40, 0);
        w32(this, 0x80, lf_checker_rt::relocated(VTABLE_STREAM_OBJ));
        w32(this, 0x78, 0);
        w32(this, 0x68, 0);
        w64(this, 0x0c);
        w64(this, 0x14);
        w64(this, 0x58);
        w64(this, 0x60);
        w64(this, 0x48);
        w64(this, 0x50);
        this
    }
});
