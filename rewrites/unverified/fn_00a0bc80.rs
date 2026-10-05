// original: 0x00a0bc80 camera_pose_fetch (proposed)
/// Fetch the current camera pose block and copy its position to `dst`.
///
/// Asks the camera manager for slot 0; a null manager means a null answer.
/// Otherwise the pose handle at manager+0x228 selects the pose (a null
/// handle faults on both sides, exactly like the original), the status word
/// at pose+0x384 is the answer, and when `dst` is non-null the four words
/// at pose+0x30..0x3c are copied there. Cdecl, one word.
lf_checker_rt::export!(cdecl, rw_00a0bc80(dst: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0;
        const POSE_HANDLE: u32 = 0x228;
        const POSE_BASE: u32 = 0x70;
        const STATUS_OFF: u32 = 0x384;
        const POS_OFF: u32 = 0x30;
        let mgr: u32 = lf_checker_rt::callee_cdecl!(MANAGER, u32, 0u32);
        if mgr == 0 {
            return 0;
        }
        let inner = ((mgr + POSE_HANDLE) as *const u32).read_unaligned();
        let pose = if inner == 0 { 0 } else { inner + POSE_BASE };
        let status = ((pose + STATUS_OFF) as *const u32).read_unaligned();
        if dst != 0 {
            for i in 0..4u32 {
                let w = ((pose + POS_OFF + i * 4) as *const u32).read_unaligned();
                ((dst + i * 4) as *mut u32).write_unaligned(w);
            }
        }
        status
    }
});
