// original: 0x00e6d770 screen_aspect_init_03
/// Store the default screen aspect ratio (width over height) into its global.
///
/// Nullary init stub, called indirectly through the settings callback table:
/// divides the read-only width constant by the read-only height constant
/// (file values 1024.0 / 768.0, scripted per trial by the checker) and stores
/// the quotient. The original also leaves the quotient in `xmm0`; the rewrite
/// returns `()` because 32-bit Rust cannot return `f32` in `xmm0`, and the
/// stored global carries the full observable behaviour.
export!(cdecl, rw_00e6d770() -> () {
    unsafe {
        let width = *global::<f32>(0x01057678);
        let height = *global::<f32>(0x0105767C);
        *global::<f32>(0x017A6594) = width / height;
    }
});
