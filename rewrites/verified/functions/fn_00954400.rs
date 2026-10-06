
// original: 0x00954400 disk_free_space_check (proposed)
/// Report whether the free disk space exceeds a required 64-bit size.
///
/// Takes the requirement as (`req_lo`, `req_hi`). After two setup
/// callees (id 1; id 2 fills the path buffer, an image-pointer format
/// plus a global drive selector), it zeroes three local 64-bit slots
/// and calls GetDiskFreeSpaceExA (id 3, via import) for the path,
/// receiving free-available, total and total-free. Returns 1 when the
/// call succeeds and free-available is STRICTLY above the requirement
/// (UNSIGNED 64-bit comparison, high word first), else 0. The original
/// guards its frame with the security cookie (`COOKIE`): it passes the
/// cookie value to the check callee (id 4) on both exits; this rewrite
/// passes the value directly instead of re-deriving it through its own
/// stack pointer. Original is cdecl/2, returns AL.
lf_checker_rt::export!(cdecl, rw_00954400(req_lo: u32, req_hi: u32) -> u32 {
    const COOKIE: u32 = 0x01057FB4;
    const NAME_A: u32 = 0x00E899E0;
    const NAME_FMT: u32 = 0x00E899E8;
    const DRIVE_SEL: u32 = 0x011695D8;
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(NAME_A), 0);
        let mut path = [0u32; 4];
        lf_checker_rt::callee_cdecl!(
            2,
            u32,
            path.as_mut_ptr() as u32,
            lf_checker_rt::relocated(NAME_FMT),
            lf_checker_rt::relocated(DRIVE_SEL)
        );
        let mut free_avail = [0u32; 2];
        let mut total = [0u32; 2];
        let mut total_free = [0u32; 2];
        let ok: u32 = {
            let f: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(3) as usize);
            f(
                path.as_mut_ptr() as u32,
                free_avail.as_mut_ptr() as u32,
                total.as_mut_ptr() as u32,
                total_free.as_mut_ptr() as u32,
            )
        };
        let cookie = lf_checker_rt::global::<u32>(COOKIE).read();
        if ok == 0 {
            lf_checker_rt::callee_thiscall!(4, u32, cookie);
            return 0;
        }
        let (fa_lo, fa_hi) = (free_avail[0], free_avail[1]);
        if fa_hi < req_hi || (fa_hi == req_hi && fa_lo <= req_lo) {
            lf_checker_rt::callee_thiscall!(4, u32, cookie);
            return 0;
        }
        lf_checker_rt::callee_thiscall!(4, u32, cookie);
        1
    }
});
