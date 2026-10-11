// original: 0x00A55FF0 id_list_store

/// Store up to the fixed capacity of a vehicle query ID list.
///
/// `this` is the owner object, `entries` points to 32-bit IDs, and `requested`
/// is a signed element count. The stored signed count is the lesser of the
/// request and the named capacity. Nonpositive counts leave the destination
/// array unchanged; positive counts copy that many words. The return value is
/// the number copied. The owner layout is inherently 32-bit and the original
/// uses thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_id_list_store(this: u32, entries: u32, requested: i32) -> u32 {
    const COUNT_OFFSET: u32 = 0x22e0;
    const ENTRIES_OFFSET: u32 = 0x1fe0;
    const CAPACITY: i32 = 64;

    let stored_count = if requested < CAPACITY { requested } else { CAPACITY };
    unsafe {
        ((this.wrapping_add(COUNT_OFFSET)) as *mut i32).write_unaligned(stored_count);
        let mut copied = 0i32;
        while copied < stored_count {
            let value = ((entries.wrapping_add((copied as u32).wrapping_mul(4))) as *const u32).read_unaligned();
            ((this.wrapping_add(ENTRIES_OFFSET).wrapping_add((copied as u32).wrapping_mul(4))) as *mut u32).write_unaligned(value);
            copied += 1;
        }
        copied as u32
    }
});
