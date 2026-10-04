// original: 0x00a323a0 entity_link_splice
/// Intrusive list splice: inserts `new` after `ent` in the doubly linked
/// list threaded through offsets 0x44 (previous) and 0x48 (next), and
/// returns the previous next link (the value the original leaves in EAX).
export!(thiscall, rw_00a323a0(ent: *mut u8, new: *mut u8) -> u32 {
    unsafe {
        let old_next = *(ent.add(0x48) as *const u32);
        *(new.add(0x48) as *mut u32) = old_next;
        *((old_next as *mut u8).add(0x44) as *mut u32) = new as u32;
        *(new.add(0x44) as *mut u32) = ent as u32;
        *(ent.add(0x48) as *mut u32) = new as u32;
        old_next
    }
});
