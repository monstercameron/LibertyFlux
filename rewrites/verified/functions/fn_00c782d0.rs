// original: 0x00C782D0 rage::pgDictionary<rage::phBound>::vf1

/// Invoke virtual slot `0xb0` on each stored bound object. The dictionary
/// uses a 16-bit count at `+0x1c` and a pointer array at `+0x18`; each child
/// begins with its vtable pointer. The method takes no stack arguments and
/// leaves the zero-extended child count in EAX after the loop.
lf_checker_rt::export!(thiscall, rw_pg_dictionary_phbound_vf1(this: u32) -> u32 {
    unsafe {
        const CHILDREN_PTR: u32 = 0x18;
        const CHILD_COUNT: u32 = 0x1c;
        const VTABLE_PTR: u32 = 0;
        const VISIT_SLOT: u32 = 0xb0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        let count = unsafe { read_u16(this + CHILD_COUNT) } as u32;
        let children = unsafe { read_u32(this + CHILDREN_PTR) };
        for index in 0..count {
            let child = unsafe { read_u32(children.wrapping_add(index.wrapping_mul(4))) };
            let vtable = unsafe { read_u32(child + VTABLE_PTR) };
            let visit = unsafe { read_u32(vtable + VISIT_SLOT) };
            let callback: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(visit as usize)
            };
            let _ = callback(child);
        }
        count
    }
});
