// original: 0x00A19E90 CCamFollowVehicle::set_script_pos
/// Marks script position as present, copies its four raw words into the
/// camera's script-position fields, and returns the fourth word.
lf_checker_rt::export!(thiscall, rw_00A19E90(camera: u32, source: u32) -> u32 {
    unsafe {
        const HAS_SCRIPT_POSITION: u32 = 0x310;
        const SCRIPT_POSITION: u32 = 0x300;
        (camera.wrapping_add(HAS_SCRIPT_POSITION) as *mut u8).write(1);
        let mut last = 0;
        for offset in [0u32, 4, 8, 12] {
            last = (source.wrapping_add(offset) as *const u32).read_unaligned();
            (camera.wrapping_add(SCRIPT_POSITION + offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
