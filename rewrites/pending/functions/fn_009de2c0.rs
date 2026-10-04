// original: 0x009de2c0 list_contains
// fn_009de2c0: linked-list value search (thiscall/1).
//
// Walks the `{value, next}` chain whose head lives at `this+0` and reports
// whether any node holds `val`. Returns 1 or 0 in the low byte.
export!(thiscall, rw_009de2c0(this: *const u32, val: u32) -> u32 {
    unsafe {
        let mut node = *this;
        while node != 0 {
            if *(node as *const u32) == val {
                return 1;
            }
            node = *((node.wrapping_add(4)) as *const u32);
        }
        0
    }
});
