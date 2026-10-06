// original: 0x009DB6C0 CBaseModelInfo::vf0

/// Deleting destructor: stamp the vtable, optionally run the array form, free on request.
///
/// `this` arrives in ECX, `flags` is the one stack word (thiscall, callee pops
/// 4 bytes); the return value is `this` (scalar path) or the array base (vector path).
/// When bit 1 of `flags` is clear (scalar), the vtable `VTABLE` is written to
/// `[this]`. When bit 1 is set (vector), `count` (a signed 32-bit value) is read
/// from `[this - COUNT_OFF]` and the vtable is stamped on each of the `count`
/// elements of `STRIDE` bytes ending at `[this]`; the loop is signed
/// (`dec; js/js` shape: `count - 1 < 0` runs zero iterations).
/// When bit 0 of `flags` is set the memory is released through operator delete
/// (cdecl, one pointer argument; called with `this` on the scalar path and the
/// array base on the vector path). Edge cases: `count <= 0` (including negative
/// counts) stamps nothing on the vector path; a clear bit 0 frees nothing.
lf_checker_rt::export!(thiscall, rw_009db6c0(this: u32, flags: u32) -> u32 {
    const COUNT_OFF: u32 = 0x10;
    const STRIDE: u32 = 0x60;
    const VTABLE: u32 = 0xe97b9c;
    const OP_DELETE: u32 = 1;
    unsafe {
        let vtable = lf_checker_rt::relocated(VTABLE);
        if flags & 2 == 0 {
            (this as *mut u32).write_unaligned(vtable);
            if flags & 1 != 0 {
                lf_checker_rt::callee_cdecl!(OP_DELETE, u32, this);
            }
            return this;
        }
        let base = this.wrapping_sub(COUNT_OFF);
        let count = (base as *const u32).read_unaligned();
        let mut elem = this.wrapping_add(count.wrapping_mul(STRIDE));
        let mut left = count.wrapping_sub(1) as i32;
        if left >= 0 {
            loop {
                left = left.wrapping_sub(1);
                elem = elem.wrapping_sub(STRIDE);
                (elem as *mut u32).write_unaligned(vtable);
                if left < 0 {
                    break;
                }
            }
        }
        if flags & 1 != 0 {
            lf_checker_rt::callee_cdecl!(OP_DELETE, u32, base);
        }
        base
    }
});
