// original: 0x00b1c7b0 rebuild_channel (proposed)

/// Rebuilds the object's channel through a handle fetched fresh.
///
/// Thiscall with no stack arguments. Sets flag bit 0x40 at +0x41, fetches
/// a handle from the provider (cdecl of the constant 1), runs the
/// handle's opener on it (thiscall, no stack arguments), then runs two
/// object-local passes (thiscall on this, no stack arguments). Clears the
/// flag bit and zeroes the byte at +0x40. Returns the second pass result.
lf_checker_rt::export!(thiscall, rw_00b1c7b0(this: u32) -> u32 {
    unsafe {
        const REBUILDING: u8 = 0x40;
        let flag = (this + 0x41) as *mut u8;
        flag.write(flag.read() | REBUILDING);
        let handle: u32 = lf_checker_rt::callee_cdecl!(1, u32, 1);
        lf_checker_rt::callee_thiscall!(2, u32, handle);
        lf_checker_rt::callee_thiscall!(3, u32, this);
        let result: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
        flag.write(flag.read() & !REBUILDING);
        ((this + 0x40) as *mut u8).write(0);
        result
    }
});
