// original: 0x00c23790 cam_request_full (proposed)
/// Forward `(a1, a2, a3, a4, a5)` to the full request worker as
/// `(a1, a2, P, 0, a3, a4, 0, 0, a5, 0)` where P points at two zeroed frame
/// words. Entry ECX is discarded (pushed only as scratch, then
/// overwritten). Returns the worker's answer.
///
/// Original: 0x00c23790 (cdecl, five stack words; fourth is a float).
lf_checker_rt::export!(cdecl, rw_00c23790(a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        let frame: [u32; 3] = [0, 0, 0];
        let p = &frame as *const u32 as u32;
        lf_checker_rt::callee_cdecl!(1, u32, a1, a2, p, 0, a3, a4, 0, 0, a5, 0)
    }
});
