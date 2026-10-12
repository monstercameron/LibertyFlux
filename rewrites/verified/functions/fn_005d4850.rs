// original: 0x005D4850 CViewportPrimaryOrtho::vf2

/// Release the inherited viewport resource, then initialise the fade channel
/// embedded at `this + 0x424` and mark the primary orthographic viewport as
/// pending. The first call is the base viewport's second virtual method. The
/// fade setup call receives nine stack words; its fifth word points to a
/// one-word local containing the opaque colour marker. The helper's return
/// value remains in EAX and is the return value of this method.
///
/// Original convention: thiscall with `this` in ECX and no stack arguments.
/// The pending marker is the byte at `this + 0x560`.
lf_checker_rt::export!(thiscall, rw_005d4850(this: u32) -> u32 {
    unsafe {
        const BASE_RELEASE: u32 = 1;
        const FADE_SETUP: u32 = 2;
        const FADE_CHANNEL_OFFSET: u32 = 0x424;
        const PENDING_BYTE_OFFSET: u32 = 0x560;
        const FADE_SPAN: u32 = 0x50;
        const LOW_BOUND_BITS: u32 = 0x3eaa_aa9f;
        const HIGH_BOUND_BITS: u32 = 0x3f2a_aa9f;
        const OPAQUE_COLOUR_MARKER: u32 = 0xff00_0000;

        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_RELEASE, u32, this);

        let mut colour_marker_slot = OPAQUE_COLOUR_MARKER;
        let fade_result: u32 = lf_checker_rt::callee_thiscall!(
            FADE_SETUP,
            u32,
            this.wrapping_add(FADE_CHANNEL_OFFSET),
            0,
            FADE_SPAN,
            0,
            0,
            &mut colour_marker_slot as *mut u32 as u32,
            LOW_BOUND_BITS,
            HIGH_BOUND_BITS,
            0,
            0,
        );

        (this.wrapping_add(PENDING_BYTE_OFFSET) as *mut u8).write(1);
        fade_result
    }
});
