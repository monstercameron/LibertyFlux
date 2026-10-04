// original: 0x00b42ed0 unregister_slot_entry
/// Unregister an object from its slot entry.
///
/// Follows the object's table link: when the linked record is of the live
/// kind, scans the global entry array for this object's index and, on a
/// hit, clears the object's registered flag and decrements both the entry
/// and global use counts. Returns 1 on a hit, else 0 (low byte defined).
export!(cdecl, rw_b42ed0(obj: u32) -> u32 {
    unsafe {
        let idx = (obj as *const u16).byte_add(0x2e).read() as i16 as i32;
        let tab = relocated(0x1295CD8);
        let ent = (tab.wrapping_add((idx as u32).wrapping_mul(4)) as *const u32).read();
        if (ent as *const u32).byte_add(0x6c).read() != 2 {
            return 0;
        }
        let count = (global::<i32>(0x16B9D70)).read();
        if count <= 0 {
            return 0;
        }
        let mut e = relocated(0x16B9C30);
        let mut i: i32 = 0;
        while i < count {
            if (e as *const i32).read() == idx {
                let flag = (obj as *mut u8).byte_add(0xf1c);
                flag.write(flag.read() & 0xbf);
                let use_slot = (e as *mut u32).byte_add(4);
                use_slot.write(use_slot.read().wrapping_sub(1));
                let total = global::<u32>(0x16B8F88);
                total.write(total.read().wrapping_sub(1));
                return 1;
            }
            i = i.wrapping_add(1);
            e = e.wrapping_add(0x14);
        }
        0
    }
});
