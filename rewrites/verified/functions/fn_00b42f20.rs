// original: 0x00b42f20 register_slot_entry
/// Register an object into its slot entry.
///
/// Like the unregister twin but for objects whose mode field selects the
/// managed kind: on a hit in the global entry array it sets the object's
/// registered flag and increments both use counts. Returns 1 on a hit,
/// else 0 (low byte defined).
export!(cdecl, rw_b42f20(obj: u32) -> u32 {
    unsafe {
        if (obj as *const u32).byte_add(0x28).read() & 0x3c0 != 0x80 {
            return 0;
        }
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
                flag.write(flag.read() | 0x40);
                let use_slot = (e as *mut u32).byte_add(4);
                use_slot.write(use_slot.read().wrapping_add(1));
                let total = global::<u32>(0x16B8F88);
                total.write(total.read().wrapping_add(1));
                return 1;
            }
            i = i.wrapping_add(1);
            e = e.wrapping_add(0x14);
        }
        0
    }
});
