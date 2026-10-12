// original: 0x00A19DC0 CCamFollowVehicle::vf8
/// Copies a four-word position from fields +0x160 through +0x16c to the
/// cached position at +0x50 through +0x5c, returning its final word.
lf_checker_rt::export!(thiscall, rw_00A19DC0(camera: u32) -> u32 {
    unsafe {
        const SOURCE: u32 = 0x160;
        const CACHED_POSITION: u32 = 0x50;
        let mut last = 0;
        for offset in [0u32, 4, 8, 12] {
            last = (camera.wrapping_add(SOURCE + offset) as *const u32).read_unaligned();
            (camera.wrapping_add(CACHED_POSITION + offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
