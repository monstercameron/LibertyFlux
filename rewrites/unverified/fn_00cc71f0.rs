// original: 0x00cc71f0 items_apply_matching
/// Walk the item iterator and invoke the apply helper with the threshold
/// value on every item whose key field (`+0x10`) equals `key`.
/// Returns 0.
export!(cdecl, rw_00cc71f0(obj: u32, key: u32, threshold_bits: u32) -> u32 {
    unsafe {
        let mut item: u32 = callee_thiscall!(1, u32, obj, 0, 2);
        while item != 0 {
            if *(((item) as *const u8).add(0x10) as *const u32) == key {
                let _: u32 = callee_thiscall!(2, u32, item, threshold_bits);
            }
            item = callee_thiscall!(3, u32, obj, 0, 2);
        }
        0
    }
});
