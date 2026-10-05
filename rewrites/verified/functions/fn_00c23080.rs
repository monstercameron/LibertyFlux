// original: 0x00c23080 cam_request_mode2 (proposed)
/// Forward to the shared camera-request worker with the mode-2 constant
/// block (kind 2, flags 1,0,0). Entry ECX is discarded.
///
/// Original: 0x00c23080 (cdecl, three stack words; second is a float).
lf_checker_rt::export!(cdecl, rw_00c23080(target: u32, blend: u32, extra: u32) -> u32 {
    unsafe {
        const KIND: u32 = 2;
        const F0: u32 = 1;
        const F1: u32 = 0;
        const F2: u32 = 0;
        lf_checker_rt::callee_cdecl!(1, u32, target, blend, KIND, F0, F1, F2, extra)
    }
});
