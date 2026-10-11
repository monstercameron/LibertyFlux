// original: 0x00C78820 rage::pgDictionary<rage::phBound>::vf2

/// Visit every stored bound object through virtual slot `0xb4`, passing the
/// range table as the sole stack argument. Then search the table's signed
/// entry count at `+0x600`; each entry has an unsigned half-open pointer
/// interval in arrays at `+0` and `+0x200`, with a byte adjustment at
/// `+0x400`. When the current value at `this+8` is inside an interval, add
/// its adjustment and store the translated pointer. Return the translated
/// pointer when a range matches, the positive entry count when none matches,
/// or zero when the signed count is not positive.
lf_checker_rt::export!(thiscall, rw_pg_dictionary_phbound_vf2(this: u32, ranges: u32) -> u32 {
    unsafe {
        const CHILDREN_PTR: u32 = 0x18;
        const CHILD_COUNT: u32 = 0x1c;
        const CURRENT_VALUE: u32 = 0x08;
        const VTABLE_PTR: u32 = 0;
        const TRANSLATE_SLOT: u32 = 0xb4;
        const RANGE_STARTS: u32 = 0;
        const RANGE_ENDS: u32 = 0x200;
        const RANGE_ADJUSTMENTS: u32 = 0x400;
        const RANGE_COUNT: u32 = 0x600;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let child_count = unsafe { read_u16(this + CHILD_COUNT) } as u32;
        let children = unsafe { read_u32(this + CHILDREN_PTR) };
        for index in 0..child_count {
            let child = unsafe { read_u32(children.wrapping_add(index.wrapping_mul(4))) };
            let vtable = unsafe { read_u32(child + VTABLE_PTR) };
            let visit = unsafe { read_u32(vtable + TRANSLATE_SLOT) };
            let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(visit as usize)
            };
            let _ = callback(child, ranges);
        }

        let signed_range_count = unsafe { read_u32(ranges + RANGE_COUNT) as i32 };
        if signed_range_count <= 0 {
            return 0;
        }

        let mut result = 0u32;
        let current = unsafe { read_u32(this + CURRENT_VALUE) };
        for index in 0..signed_range_count as u32 {
            let displacement = index.wrapping_mul(4);
            let start = unsafe { read_u32(ranges + RANGE_STARTS + displacement) };
            let end = unsafe { read_u32(ranges + RANGE_ENDS + displacement) };
            if current >= start && current < end {
                let adjustment = unsafe {
                    read_u32(ranges + RANGE_ADJUSTMENTS + displacement)
                };
                result = current.wrapping_add(adjustment);
                unsafe { write_u32(this + CURRENT_VALUE, result) };
                break;
            }
            result = index + 1;
        }
        result
    }
});
