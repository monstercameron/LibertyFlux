// original: 0x00C6E850 anim_find_entry

/// Look up an animation entry by group and id. Returns 0 at once when either index is -1. Otherwise stores the group's name pointer (table + index*0x58 + 0x20) through out_name, scans the group's count entries at stride 12 for the id, and on a hit stores the matching value through out_val and returns 1, else returns 0. The count comparison is signed. Original: stdcall, four stack words.
lf_checker_rt::export!(stdcall, rw_c6e850(group: u32, id: u32, out_name: u32, out_val: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        const TABLE_GVA: u32 = 0x016DD640;
                const REC: u32 = 0x58;
                if group == u32::MAX || id == u32::MAX {
                    return 0;
                }
                let base = rd32(lf_checker_rt::relocated(TABLE_GVA));
                let rec = base.wrapping_add(group.wrapping_mul(REC));
                wr32(out_name, rec.wrapping_add(0x20));
                let count = rd32(rec.wrapping_add(0x38)) as i32;
                if count <= 0 {
                    return 0;
                }
                let entries = rd32(rec.wrapping_add(0x44));
                let mut i = 0i32;
                while i < count {
                    if rd32(entries.wrapping_add((i as u32).wrapping_mul(0xc))) == id {
                        let values = rd32(rec.wrapping_add(0x3c));
                        wr32(out_val, rd32(values.wrapping_add((i as u32).wrapping_mul(4))));
                        return 1;
                    }
                    i += 1;
                }
                0
        }
});
