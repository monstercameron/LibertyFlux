// original: 0x00be0020 UnknownTaskHelper1
/// Ask the task child-list helper to process `item` using the list at `this + 0x14`. A nonzero low-byte helper result is returned as true. If the helper rejects the item, a null item returns false; otherwise the item's first vtable entry is called with the item in ECX and the word 1 on the stack, then false is returned. The list policy and virtual target are callees and are scripted by the contract. Calling convention: thiscall with one stack pointer argument; the result is an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00be0020(this: u32, item: u32) -> u32 {
    const TASK_ARRAY_OFFSET: u32 = 0x14;
    const FIND_OR_INSERT: u32 = 1;
    let inserted = lf_checker_rt::callee_thiscall!(FIND_OR_INSERT, u32, this.wrapping_add(TASK_ARRAY_OFFSET), item);
    if inserted & 0xFF != 0 {
        1
    } else if item == 0 {
        0
    } else {
        let item_vtable = unsafe { (item as *const u32).read_unaligned() };
        let dispose_target = unsafe { (item_vtable as *const u32).read_unaligned() };
        let dispose: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(dispose_target as usize) };
        let _ = dispose(item, 1);
        0
    }
});
