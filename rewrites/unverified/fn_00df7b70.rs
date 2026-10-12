// original: 0x00DF7B70 UIFileViewer::bind_controller_frame

/// Bind the UI file viewer to its controller frame and forward the final call.
///
/// Look up the primary frame name, call its `+0x1b0` method when found, then
/// query the viewer at vtable slot `+0x1c`. If that predicate is false, look
/// up the alternate frame name, call its `+0x1b0` method and notify the viewer
/// at `+0x1ac`. Finish by forwarding `this` to the frame handoff tail call.
/// The lookup receiver is a relocated image object; both names are image
/// strings. The final EAX value comes from the forwarded thiscall.
lf_checker_rt::export!(thiscall, rw_df7b70(this: u32) -> u32 {
    const LOOKUP_CONTEXT: u32 = 0x01981A4C;
    const PRIMARY_FRAME_NAME: u32 = 0x00F01E8C;
    const ALTERNATE_FRAME_NAME: u32 = 0x00F01EA0;

    let primary_frame = lf_checker_rt::callee_thiscall!(
        1, u32,
        lf_checker_rt::relocated(LOOKUP_CONTEXT),
        lf_checker_rt::relocated(PRIMARY_FRAME_NAME)
    );
    if primary_frame != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, primary_frame);
    }

    let uses_primary = lf_checker_rt::callee_thiscall!(3, u32, this) as u8 != 0;
    if !uses_primary {
        let alternate_frame = lf_checker_rt::callee_thiscall!(
            4, u32,
            lf_checker_rt::relocated(LOOKUP_CONTEXT),
            lf_checker_rt::relocated(ALTERNATE_FRAME_NAME)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, alternate_frame);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this);
    }

    lf_checker_rt::callee_thiscall!(6, u32, this)
});
