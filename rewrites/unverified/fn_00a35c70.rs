// original: 0x00a35c70 vehicle_subobj_init (proposed)

/// Initialise a vehicle sub-object's status block: eight words and one byte
/// at fixed offsets from the object pointer.
///
/// Writes zero to every slot except `+0x84` (set to all-ones, "no index")
/// and `+0x8c` (the float 1.0). Returns the object pointer. Thiscall, no
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00a35c70(obj: u32) -> u32 {
    unsafe {
        const NO_INDEX: u32 = 0xFFFF_FFFF;
        const ONE_BITS: u32 = 0x3F80_0000;
        let base = obj as *mut u8;
        core::ptr::write_unaligned(base.add(0x80) as *mut u32, 0);
        core::ptr::write_unaligned(base.add(0x84) as *mut u32, NO_INDEX);
        core::ptr::write_unaligned(base.add(0x88) as *mut u32, 0);
        core::ptr::write_unaligned(base.add(0x8c) as *mut u32, ONE_BITS);
        core::ptr::write(base.add(0x90), 0u8);
        core::ptr::write_unaligned(base.add(0x94) as *mut u32, 0);
        core::ptr::write_unaligned(base.add(0x98) as *mut u32, 0);
        core::ptr::write_unaligned(base.add(0x9c) as *mut u32, 0);
        core::ptr::write_unaligned(base.add(0xa0) as *mut u32, 0);
        obj
    }
});
