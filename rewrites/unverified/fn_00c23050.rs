// original: 0x00c23050 cam_request_mode1 (proposed)
/// Forward to the shared camera-request worker with the mode-1 constant
/// block (kind 1, flags 0,1,1). Entry ECX is discarded.
///
/// Original: 0x00c23050 (cdecl, three stack words; second is a float).
lf_checker_rt::export!(cdecl, rw_00c23050(target: u32, blend: u32, extra: u32) -> u32 {
    unsafe {
        const KIND: u32 = 1;
        const F0: u32 = 0;
        const F1: u32 = 1;
        const F2: u32 = 1;
        lf_checker_rt::callee_cdecl!(1, u32, target, blend, KIND, F0, F1, F2, extra)
    }
});
