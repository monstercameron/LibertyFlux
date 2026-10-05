// original: 0x00c230b0 cam_request_mode3 (proposed)
/// Forward to the shared camera-request worker with the mode-3 constant
/// block (kind 3, flags 1,0,0). Entry ECX is discarded.
///
/// Original: 0x00c230b0 (cdecl, three stack words; second is a float).
lf_checker_rt::export!(cdecl, rw_00c230b0(target: u32, blend: u32, extra: u32) -> u32 {
    unsafe {
        const KIND: u32 = 3;
        const F0: u32 = 1;
        const F1: u32 = 0;
        const F2: u32 = 0;
        lf_checker_rt::callee_cdecl!(1, u32, target, blend, KIND, F0, F1, F2, extra)
    }
});
