// original: 0x00a87a70 release_table_drain
/// Drain the four-entry release table at `this + 0x890`.
///
/// Each 16-byte row holds two (pointer, live-flag) pairs. A live pair with
/// a nonzero reference count (word at entry + 0xA) has its count
/// decremented; when the count reaches zero on a type 2 or 4 entry (byte
/// at entry + 8), the entry's vtable slot 0 is invoked (intercepted,
/// thiscall/1 with argument 1). Every live pair's slot is then cleared.
/// Returns nothing checkable (the original's exit EAX is unobservable
/// entry state when no call fires), so the return channel is not compared.
export!(thiscall, rw_00a87a70(this_obj: u32) -> u32 {
    unsafe fn drain_entry(ptr: u32) {
        let count = *((ptr.wrapping_add(0xa)) as *const u16);
        if count == 0 {
            return;
        }
        let left = count.wrapping_sub(1);
        *((ptr.wrapping_add(0xa)) as *mut u16) = left;
        let typ = *((ptr.wrapping_add(8)) as *const u8);
        if left == 0 && (typ == 2 || typ == 4) {
            let vtable = *(ptr as *const u32);
            let target = *(vtable as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(ptr, 1);
        }
    }
    unsafe {
        let mut row = 0u32;
        while row < 4 {
            let slot = this_obj.wrapping_add(0x890).wrapping_add(row.wrapping_mul(0x10));
            let pa = *(slot as *const u32);
            if pa != 0 && *((slot.wrapping_add(4)) as *const u8) != 0 {
                drain_entry(pa);
                *(slot as *mut u32) = 0;
                *((slot.wrapping_add(4)) as *mut u8) = 0;
            }
            let pb = *((slot.wrapping_add(8)) as *const u32);
            if pb != 0 && *((slot.wrapping_add(0xc)) as *const u8) != 0 {
                drain_entry(pb);
                *((slot.wrapping_add(8)) as *mut u32) = 0;
                *((slot.wrapping_add(0xc)) as *mut u8) = 0;
            }
            row += 1;
        }
        0
    }
});
