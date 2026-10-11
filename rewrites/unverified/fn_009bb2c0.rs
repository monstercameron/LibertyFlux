// original: 0x009BB2C0 CCamScriptInstruction_UnAttachCam::vf2

/// Detach a camera from its attached-object reference.
///
/// The camera key is at `this + 0x08`. If lookup succeeds, the handler keeps
/// only bits 0 through 2 and 7 in the camera flag byte at `+0x264`. A non-null
/// reference at `+0x240` is released with a pointer to that field; the field
/// is then cleared. EAX is the release helper result when called, otherwise
/// it is the camera lookup result.
lf_checker_rt::export!(thiscall, rw_009bb2c0(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const CAMERA_REFERENCE: u32 = 0x240;
        const CAMERA_FLAGS: u32 = 0x264;
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
        let reference_slot = camera.wrapping_add(CAMERA_REFERENCE);
        let reference = (reference_slot as *const u32).read_unaligned();
        if reference != 0 {
            let result = lf_checker_rt::callee_thiscall!(RELEASE, u32, reference, reference_slot);
            (reference_slot as *mut u32).write_unaligned(0);
            return result;
        }
        camera
    }
});
