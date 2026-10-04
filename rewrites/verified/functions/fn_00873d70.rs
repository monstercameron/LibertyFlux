// original: 0x00873d70 crmt_indexed_float_store
// Indexed float store into the linked record's table at
// `table + 0x20 + index * 8`. The value passes through as raw bits.
// (thiscall/2)
export!(thiscall, rw_00873d70(this_ptr: u32, index: u32, value_bits: u32) -> () {
    unsafe {
        let mid = (this_ptr as *const u32).add(2).read();
        if mid == 0 {
            return;
        }
        let table = (mid as *const u32).add(2).read();
        if table == 0 {
            return;
        }
        let slot = table.wrapping_add(0x20).wrapping_add(index.wrapping_mul(8));
        (slot as *mut u32).write(value_bits);
    }
});
