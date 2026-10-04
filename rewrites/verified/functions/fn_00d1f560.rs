// original: 0x00d1f560 CTaskComplexSlideIntoCover::vf18
// Returns 0. When this task's flag byte is set and the given object's
// pointer slot is live, measures the squared distance between two stored
// points and, if it exceeds a global threshold, notifies the object
// through its reset entry. Always returns 0.
export!(thiscall, rw_00d1f560(this_ptr: u32, obj: u32) -> u32 {
    unsafe {
        if *((this_ptr.wrapping_add(0x3C)) as *const u8) == 0 {
            return 0;
        }
        if *((obj.wrapping_add(0xD68)) as *const u32) == 0 {
            return 0;
        }
        let anchor = *((obj.wrapping_add(0x20)) as *const u32);
        let ax = f32::from_bits(*((anchor.wrapping_add(0x30)) as *const u32));
        let ay = f32::from_bits(*((anchor.wrapping_add(0x34)) as *const u32));
        let tx = f32::from_bits(*((this_ptr.wrapping_add(0x20)) as *const u32));
        let ty = f32::from_bits(*((this_ptr.wrapping_add(0x24)) as *const u32));
        let dx = ax - tx;
        let dy = ay - ty;
        let dist_sq = dx * dx + dy * dy;
        let thresh = f32::from_bits(*(global::<u32>(0x00FE87E4)));
        if dist_sq > thresh {
            callee_thiscall!(1, u32, obj);
        }
        0
    }
});
