// original: 0x009BB290 CCamScriptInstruction_SnapShotCam::vf2

/// Snapshot the camera named by this instruction into the selected camera.
///
/// The source key is at `this + 0x0C` and the destination key at `+0x08`.
/// The source lookup happens first; its result is forwarded as the sole stack
/// argument to the snapshot helper, while the destination lookup result is
/// the helper's this pointer. The helper result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_009bb290(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const LOOKUP: u32 = 1;
        const SNAPSHOT: u32 = 2;
        let manager = lf_checker_rt::relocated(MANAGER);
        let source_key = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
        let destination_key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let source = lf_checker_rt::callee_thiscall!(LOOKUP, u32, manager, source_key);
        let destination = lf_checker_rt::callee_thiscall!(LOOKUP, u32, manager, destination_key);
        lf_checker_rt::callee_thiscall!(SNAPSHOT, u32, destination, source)
    }
});
