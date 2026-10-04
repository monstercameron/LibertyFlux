// original: 0x00953710 refresh_slot_or_sentinel
/// Read the object's cached slot, refreshing it through the loader when spent.
///
/// A null object yields 0xfffffffe. A fresh slot (non-negative narrow value
/// below 0x818) is returned as is; any other settled value yields 0xffffffff
/// unless it is the spent marker 0xffffffff with a nonzero refresh flag, in
/// which case the loader is called, the fresh value stored and returned.
export!(cdecl, rw_00953710(obj: u32, flags: u32) -> u32 {
    if obj == 0 {
        return 0xFFFFFFFE;
    }
    unsafe {
        let slot = (obj.wrapping_add(0x64)) as *mut u32;
        let settled = *slot;
        let narrow = (settled & 0xFFFF) as u16 as i16;
        if narrow >= 0 && narrow < 0x818 {
            return settled;
        }
        if settled != 0xFFFFFFFF {
            return 0xFFFFFFFF;
        }
        if flags & 0xFF == 0 {
            return 0xFFFFFFFF;
        }
        let fresh: u32 = callee_cdecl!(0, u32, obj);
        *slot = fresh;
        fresh
    }
});
