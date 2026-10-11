// original: 0x00be0050 task_child_list_insert_or_notify
/// Scan the fixed 16-entry child array beginning at `this + 4`. The first zero entry receives `item`, and the function returns true, including when `item` itself is null. If all entries are occupied, a non-null `item` is sent to its vtable's first method with argument 1; the function then returns false. All array words are read in order, and only the chosen empty word is written. Calling convention: thiscall with one caller-supplied stack pointer argument; the result is an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00be0050(this: u32, item: u32) -> u32 {
    const FIRST_CHILD_OFFSET: u32 = 0x04;
    const CHILD_CAPACITY: u32 = 16;
    const DISPOSE_CHILD: u32 = 1;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let write_word = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };
    let mut first_empty = CHILD_CAPACITY;
    for index in 0..CHILD_CAPACITY {
        let slot_address = this.wrapping_add(FIRST_CHILD_OFFSET).wrapping_add(index * 4);
        if read_word(slot_address) == 0 {
            first_empty = index;
            break;
        }
    }
    if first_empty < CHILD_CAPACITY {
        let slot_address = this.wrapping_add(FIRST_CHILD_OFFSET).wrapping_add(first_empty * 4);
        write_word(slot_address, item);
        1
    } else {
        if item != 0 {
            let item_vtable = read_word(item);
            let dispose_target = read_word(item_vtable);
            let dispose: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(dispose_target as usize) };
            let _ = dispose(item, 1);
        }
        0
    }
});
