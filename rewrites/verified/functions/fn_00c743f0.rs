// original: 0x00c743f0 CPedModelInfo::vf8

/// Release the optional resources at `+0x12c` and `+0x128`, clearing each
/// field after its resource cleanup and allocator free calls. Then invoke the
/// base-model cleanup as the function's tail operation and return its EAX
/// value.
lf_checker_rt::export!(thiscall, rw_00c743f0(this: u32) -> u32 {
    const CLEAN_FIRST: u32 = 1;
    const FREE_FIRST: u32 = 2;
    const CLEAN_SECOND: u32 = 3;
    const FREE_SECOND: u32 = 4;
    const BASE_CLEANUP: u32 = 5;

    unsafe {
        let first = ((this.wrapping_add(0x12c)) as *const u32).read_unaligned();
        if first != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CLEAN_FIRST, u32, first);
            let _ = lf_checker_rt::callee_cdecl!(FREE_FIRST, u32, first);
            ((this.wrapping_add(0x12c)) as *mut u32).write_unaligned(0);
        }

        let second = ((this.wrapping_add(0x128)) as *const u32).read_unaligned();
        if second != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CLEAN_SECOND, u32, second);
            let _ = lf_checker_rt::callee_cdecl!(FREE_SECOND, u32, second);
            ((this.wrapping_add(0x128)) as *mut u32).write_unaligned(0);
        }
    }

    lf_checker_rt::callee_thiscall!(BASE_CLEANUP, u32, this)
});
