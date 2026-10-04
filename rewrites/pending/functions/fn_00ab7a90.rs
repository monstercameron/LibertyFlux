// original: 0x00ab7a90 object_table_resize
/// Resizes the object table (node links at +0x8c) to `new_count` buckets.

export!(thiscall, rw_00ab7a90(this: *mut u8, new_count_w: u32) -> u32 {
    unsafe { rehash_table(this, new_count_w & 0xFFFF, 0x8C) };
    0 // unchecked: original returns entry garbage when disabled
});

fn rehash_table(this: *mut u8, new_count: u32, next_off: usize) {
    unsafe {
        if *this.add(0xB) == 0 {
            return;
        }
        let newb = callee_cdecl!(1, u32, new_count * 4) as *mut u32;
        for i in 0..new_count {
            *newb.add(i as usize) = 0;
        }
        let old_count = *(this.add(4) as *const u16) as u32;
        let mut i = 0u32;
        while i < old_count {
            let base = *(this as *const u32) as *const u32;
            let mut c = *base.add(i as usize);
            while c != 0 {
                let idx = (*(c as *const u32) % new_count) as usize;
                let next = *((c + next_off as u32) as *const u32);
                *((c + next_off as u32) as *mut u32) = *newb.add(idx);
                *newb.add(idx) = c;
                c = next;
            }
            i += 1;
        }
        let old = *(this as *const u32);
        *(this.add(4) as *mut u16) = new_count as u16;
        let _: u32 = callee_cdecl!(2, u32, old);
        *(this as *mut u32) = newb as u32;
    }
}
