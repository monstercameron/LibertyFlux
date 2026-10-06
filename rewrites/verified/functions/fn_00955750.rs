
// original: 0x00955750 handle_open_buffered (proposed)
/// Open a buffered handle for a record, creating or reusing it.
///
/// Returns 0 when `handle` is null or the open callee (id 1, passed the
/// relocated name `OPEN_NAME` with `handle` and `len`) answers null.
/// Otherwise checksums `len - 4` (wrapping) bytes at `handle + ID_END`
/// (4) through id 2: when the checksum differs from the id dword at
/// `handle + 0`, the create path allocates a handle (id 3, eight fixed
/// arguments) into `out`, returns 0 for a null handle, and otherwise
/// queries a word through id 4 (index `QUERY_IDX` (0x42), frame buffer),
/// passes it to id 5 with the handle re-read from `out`, releases id 6
/// and returns 1. When the checksum matches, the reuse path releases id
/// 7, re-resolves id 8, stores that handle into `out` and returns
/// whether it is non-null. Original is cdecl/3 (verified against its
/// caller, which passes three arguments and reads `out` back), returns
/// AL.
lf_checker_rt::export!(cdecl, rw_00955750(handle: u32, len: u32, out: u32) -> u32 {
    const OPEN_NAME: u32 = 0x00E8AB20;
    const ID_END: u32 = 4;
    const QUERY_IDX: u32 = 0x42;
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        if handle == 0 {
            return 0;
        }
        let ctx = lf_checker_rt::callee_cdecl!(1, u32, handle, len, lf_checker_rt::relocated(OPEN_NAME), 0);
        if ctx == 0 {
            return 0;
        }
        let id = rd32(handle);
        let sum =
            lf_checker_rt::callee_cdecl!(2, u32, handle.wrapping_add(ID_END), len.wrapping_sub(4));
        if id == sum {
            lf_checker_rt::callee_thiscall!(7, u32, ctx, 4);
            let h = lf_checker_rt::callee_cdecl!(8, u32, ctx, 0);
            wr32(out, h);
            (h != 0) as u32
        } else {
            let h = lf_checker_rt::callee_cdecl!(3, u32, 0x1C8, 0x100, 1, 1, 0, 0, 0, 0);
            wr32(out, h);
            if h == 0 {
                lf_checker_rt::callee_thiscall!(6, u32, ctx);
                return 0;
            }
            let mut buf = [0u32; 1];
            let p = lf_checker_rt::callee_cdecl!(4, u32, buf.as_mut_ptr() as u32, QUERY_IDX);
            let w = rd32(p);
            let h2 = rd32(out);
            lf_checker_rt::callee_thiscall!(5, u32, h2, w);
            lf_checker_rt::callee_thiscall!(6, u32, ctx);
            1
        }
    }
});
