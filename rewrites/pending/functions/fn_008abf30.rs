// original: 0x008ABF30 rage::audBiquadFilterEffect::vf5
/// Advance one tick: rotate a coefficient row into the slot selected by
/// the step counter, then hand control to the linked stage through its
/// slot-5 entry point, or report the new slot when unlinked.
export!(thiscall, rw_008ABF30(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        let step = *(obj.add(0x30) as *const u32);
        let slot = step.wrapping_add(1) % 3;
        let src = step.wrapping_add(5).wrapping_mul(24);
        let dst = slot.wrapping_mul(24).wrapping_add(0x78);
        for i in 0..3u32 {
            let v = core::ptr::read_unaligned(
                (obj as *const u8).add(src.wrapping_add(i * 8) as usize)
                    as *const u64,
            );
            core::ptr::write_unaligned(
                (obj as *mut u8).add(dst.wrapping_add(i * 8) as usize)
                    as *mut u64,
                v,
            );
        }
        let next = *(obj.add(8) as *const u32);
        *(obj.add(0x30) as *mut u32) = slot;
        if next == 0 {
            return slot.wrapping_mul(3);
        }
        let vtable = *(next as *const u32);
        let target = *((vtable as *const u8).add(0x14) as *const u32);
        let advance: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        advance(next)
    }
});
