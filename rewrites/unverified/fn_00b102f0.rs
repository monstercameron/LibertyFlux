// original: 0x00B102F0 heli_attach_physics_resource

/// Forward one attachment to the resource object's registration method. The
/// inline field at `this + 0xb0` must be non-null and the signed count at
/// `this + 0xb8` must be positive. A non-null array at `this + 0x20` selects
/// its storage plus 0x30; otherwise the inline storage at `this + 0x10` is
/// used. The callee receives the resource object in ECX, then the count,
/// storage pointer and a fixed mode value of one. This method is thiscall.
lf_checker_rt::export!(thiscall, rw_00b102f0(this: u32) -> u32 {
    const RESOURCE_SLOT: u32 = 0xB0;
    const ITEM_COUNT: u32 = 0xB8;
    const ARRAY_FIELD: u32 = 0x20;
    const INLINE_STORAGE: u32 = 0x10;
    const ARRAY_STORAGE: u32 = 0x30;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE_SLOT));
        if resource == 0 {
            return 0;
        }
        let count = read_u32(this.wrapping_add(ITEM_COUNT));
        if (count as i32) <= 0 {
            return 0;
        }
        let array = read_u32(this.wrapping_add(ARRAY_FIELD));
        let storage = if array != 0 {
            array.wrapping_add(ARRAY_STORAGE)
        } else {
            this.wrapping_add(INLINE_STORAGE)
        };
        let _ = lf_checker_rt::callee_thiscall!(1, u32, resource, count, storage, 1u32);
        0
    }
});
