// original: 0x00c7a6b0 ctask_move_between_points_dtor

/// Destroy a scenario task, releasing the handle at `+0x2c`.
/// Installs `VTABLE`, then when the handle is non-zero passes it to the
/// release helper callee (cdecl, one word) and nulls it. Tail-calls the
/// base destructor and returns its result.
/// Original: 0x00c7a6b0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7a6b0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5B9C;
        const OFF_HANDLE: u32 = 0x2c;
        const RELEASE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let held = rd32(this.wrapping_add(OFF_HANDLE));
        if held != 0 {
            lf_checker_rt::callee_cdecl!(RELEASE, u32, held);
            wr32(this.wrapping_add(OFF_HANDLE), 0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
