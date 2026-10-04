// original: 0x00873a30 crmt_node_zero_and_set_vtable
// Null-checked zeroing initializer: publish the scratch vtable, clear the
// 32 eight-byte records starting at offset 0x20, then publish the final
// vtable. (cdecl/1)
export!(cdecl, rw_00873a30(obj: u32) -> () {
    if obj == 0 {
        return;
    }
    unsafe {
        const RECORDS: usize = 32;
        const FIRST_WORD: usize = 8; // offset 0x20 in words
        let base = obj as *mut u32;
        base.write(relocated(0x00FE8024));
        for i in 0..RECORDS {
            base.add(FIRST_WORD + i * 2).write(0);
            base.add(FIRST_WORD + i * 2 + 1).write(0);
        }
        base.write(relocated(0x00FE7FE8));
    }
});
