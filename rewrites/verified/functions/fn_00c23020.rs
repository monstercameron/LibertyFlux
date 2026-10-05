// original: 0x00c23020 cam_request_mode0 (proposed)
/// Forward `(target, blend, extra)` to the shared camera-request worker with
/// the mode-0 constant block (kind 4, flags 0,0,0). Entry ECX is discarded.
/// Returns the worker's result.
///
/// Original: 0x00c23020 (cdecl, three stack words; second is a float).
lf_checker_rt::export!(cdecl, rw_00c23020(target: u32, blend: u32, extra: u32) -> u32 {
    unsafe {
        const KIND: u32 = 4;
        const F0: u32 = 0;
        const F1: u32 = 0;
        const F2: u32 = 0;
        lf_checker_rt::callee_cdecl!(1, u32, target, blend, KIND, F0, F1, F2, extra)
    }
});
