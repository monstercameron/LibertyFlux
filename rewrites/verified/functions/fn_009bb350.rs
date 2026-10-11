// original: 0x009BB350 CCamScriptInstruction_UnPointCam::vf2

/// Remove the camera's point-at target and its associated roll reference.
///
/// The camera key is at `this + 0x08`. When found, the handler retains flag
/// bits 0 through 2 and 7 at `+0x264`, releases a non-null reference in
/// `+0x244` with the address of that slot, and clears both `+0x244` and
/// `+0x24C`. EAX is the release result when a reference was released, or the
/// camera lookup result otherwise; a missing camera returns zero.
lf_checker_rt::export!(thiscall, rw_009bb350(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const POINT_REFERENCE: u32 = 0x244;
        const CAMERA_FLAGS: u32 = 0x264;
        const POINT_DISTANCE: u32 = 0x24c;
        const KEEP_FLAGS: u8 = 0x87;
        const LOOKUP: u32 = 1;
        const RELEASE: u32 = 2;

        let key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let camera = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            key,
        );
        if camera == 0 {
            return 0;
        }
        let flags = (camera.wrapping_add(CAMERA_FLAGS) as *mut u8).read();
        (camera.wrapping_add(CAMERA_FLAGS) as *mut u8).write(flags & KEEP_FLAGS);
        let slot = camera.wrapping_add(POINT_REFERENCE);
        let reference = (slot as *const u32).read_unaligned();
        let result = if reference != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, reference, slot)
        } else {
            camera
        };
        (slot as *mut u32).write_unaligned(0);
        (camera.wrapping_add(POINT_DISTANCE) as *mut u32).write_unaligned(0);
        result
    }
});
