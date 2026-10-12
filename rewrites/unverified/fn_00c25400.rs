// original: 0x00c25400 CCamInterp::vf2
/// Releases both optional source objects, decrementing the reference byte at offset 0x13B once per field, then clears the two fields and returns true in AL. If both fields alias, the byte is decremented twice.
lf_checker_rt::export!(thiscall, rw_00c25400(this: u32) -> u32 {
    unsafe {
        const SOURCE_A: usize = 0x14c;
        const SOURCE_B: usize = 0x150;
        const REFCOUNT: usize = 0x13b;
        let object = this as usize as *mut u8;
        let first = core::ptr::read_unaligned(object.add(SOURCE_A) as *const u32);
        if first != 0 {
            let count = (first as usize as *mut u8).add(REFCOUNT);
            count.write(count.read().wrapping_sub(1));
            core::ptr::write_unaligned(object.add(SOURCE_A) as *mut u32, 0);
        }
        let second = core::ptr::read_unaligned(object.add(SOURCE_B) as *const u32);
        if second != 0 {
            let count = (second as usize as *mut u8).add(REFCOUNT);
            count.write(count.read().wrapping_sub(1));
            core::ptr::write_unaligned(object.add(SOURCE_B) as *mut u32, 0);
        }
    }
    1
});
