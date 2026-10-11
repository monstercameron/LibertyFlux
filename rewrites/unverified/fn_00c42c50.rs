// original: 0x00c42c50 CIntermezzoEvent::get_nested_active_cam
/// Follow the active nested-camera links at offset 0x124 twice, then return
/// the next link. Either null intermediate yields zero. The thiscall method
/// has no stack parameters or outgoing calls.
lf_checker_rt::export!(thiscall, rw_00c42c50(this: u32) -> u32 {
    const CAMERA: u32 = 4;
    const NESTED_LINK: u32 = 0x124;
    unsafe {
        let camera = ((this + CAMERA) as *const u32).read_unaligned();
        let first = ((camera + NESTED_LINK) as *const u32).read_unaligned();
        if first == 0 {
            return 0;
        }
        let second = ((first + NESTED_LINK) as *const u32).read_unaligned();
        if second == 0 {
            return 0;
        }
        ((second + NESTED_LINK) as *const u32).read_unaligned()
    }
});
