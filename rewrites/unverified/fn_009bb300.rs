// original: 0x009BB300 CCamScriptInstruction_UnAttachCamFromViewport::vf2

/// Detach the named camera from its viewport through the camera helper.
///
/// The camera key is at `this + 0x08`. A successful manager lookup is tail
/// forwarded as the helper's this pointer; a missing camera returns zero.
/// The tail helper result is preserved as the method's EAX result.
lf_checker_rt::export!(thiscall, rw_009bb300(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const LOOKUP: u32 = 1;
        const DETACH_FROM_VIEWPORT: u32 = 2;
        let key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let camera = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            key,
        );
        if camera == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(DETACH_FROM_VIEWPORT, u32, camera)
        }
    }
});
