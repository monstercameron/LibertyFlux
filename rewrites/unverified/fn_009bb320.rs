// original: 0x009BB320 CCamScriptInstruction_UnInheritRollCam::vf2

/// Clear the camera's inherited-roll reference.
///
/// The camera key is at `this + 0x08`. A non-null reference in the camera's
/// `+0x248` slot is released with the slot address, after which that slot is
/// cleared. EAX is the release result when released, otherwise the camera
/// lookup result; a missing camera returns zero.
lf_checker_rt::export!(thiscall, rw_009bb320(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const ROLL_REFERENCE: u32 = 0x248;
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
        let slot = camera.wrapping_add(ROLL_REFERENCE);
        let reference = (slot as *const u32).read_unaligned();
        if reference != 0 {
            let result = lf_checker_rt::callee_thiscall!(RELEASE, u32, reference, slot);
            (slot as *mut u32).write_unaligned(0);
            result
        } else {
            camera
        }
    }
});
