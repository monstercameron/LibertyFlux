// original: 0x00c07490 stream_value_remove (proposed)

/// Remove the first entry equal to the value from the list.
///
/// `this` points to the list (`ITEMS` the entry array, `LEN` its 16-bit
/// length). The first entry comparing equal to `value` is handed to the
/// release helper (callee 1) unless it is null, the entries after it shift
/// down one slot, and the length shrinks by one. When nothing matches, the
/// end-of-array pointer is returned; when the list is empty the incoming
/// `eax` passes through unchanged (the contract pins it, so the value is
/// fixed). A removal returns 0xFFFF.
///
/// Original: 0x00c07490 (thiscall, one stack word; helper is cdecl, 1 arg).
lf_checker_rt::export!(thiscall, rw_00c07490(this: u32, value: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x08;
        const LEN: u32 = 0x0c;
        const FREE: u32 = 1;
        const EMPTY_RET: u32 = 0x12345678;
        const REMOVED_RET: u32 = 0xffff;
        let len = (this.wrapping_add(LEN) as *const u16).read_unaligned() as u32;
        if (len as i32) <= 0 {
            return EMPTY_RET;
        }
        let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < len {
            if (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned() == value {
                break;
            }
            i += 1;
        }
        if i >= len {
            return base.wrapping_add(len.wrapping_mul(4));
        }
        let entry = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
        if entry != 0 {
            let _r: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, entry);
        }
        let mut j = i;
        while (j as i32) < (len as i32).wrapping_sub(1) {
            let dest = base.wrapping_add(j.wrapping_mul(4));
            let v = (dest.wrapping_add(4) as *const u32).read_unaligned();
            (dest as *mut u32).write_unaligned(v);
            j += 1;
        }
        (this.wrapping_add(LEN) as *mut u16).write_unaligned(len.wrapping_sub(1) as u16);
        REMOVED_RET
    }
});
