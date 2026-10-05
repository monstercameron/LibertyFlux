// original: 0x00c24260 cam_target_guard (proposed)
/// Guard for the camera-target worker: return 0 for a null target, else
/// tail-call the worker with the target. Only the low byte is a real
/// result on the null path (upper bytes keep entry garbage there), so the
/// contract compares `al`.
///
/// Original: 0x00c24260 (cdecl, one stack word; `al` result).
lf_checker_rt::export!(cdecl, rw_00c24260(target: u32) -> u32 {
    unsafe {
        if target == 0 {
            0
        } else {
            lf_checker_rt::callee_cdecl!(1, u32, target)
        }
    }
});
