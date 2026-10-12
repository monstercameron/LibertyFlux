// original: 0x00DE7730 UIMontageEditorLayout::UIMontageEditorLayout_2

/// Install the relocated montage-editor-layout vtable, call the clip probe,
/// then tail-forward `this` to the frame child-destruction method. The probe's
/// result is ignored; the tail callee's full `EAX` becomes the result. This is
/// a zero-argument `thiscall` method.
lf_checker_rt::export!(thiscall, rw_00DE7730(this: u32) -> u32 {
    const LAYOUT_VTABLE: u32 = 0x00EFEFCC;
    const CALLEE_CLIP_PROBE: u32 = 1;
    const CALLEE_DESTROY_CHILDREN: u32 = 2;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(LAYOUT_VTABLE));
    }
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_CLIP_PROBE, u32, this);
    lf_checker_rt::callee_thiscall!(CALLEE_DESTROY_CHILDREN, u32, this)
});
