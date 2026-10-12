// original: 0x00DB2440 UITexture::UITexture_2

/// Install the UITexture virtual table, release any existing texture
/// resource, and tail-forward construction to the shared UI layout helper.
/// The release call and the final helper call remain observable as separate
/// scripted calls; the final helper supplies the returned EAX value.
lf_checker_rt::export!(thiscall, rw_00db2440(this: u32) -> u32 {
    const UI_TEXTURE_VTABLE_FILE_VA: u32 = 0x00EF21F4;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(UI_TEXTURE_VTABLE_FILE_VA));
    }
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    lf_checker_rt::callee_thiscall!(2, u32, this)
});
