// original: 0x00e40e60 row_key_store
// row key store.
// Stores value at base+index*0x2b0+0x10. Returns base.
export!(stdcall, rw_00e40e60(base: u32, index: u32, value: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        const KEY_OFF: u32 = 0x10;
        let slot = base
            .wrapping_add(index.wrapping_mul(STRIDE))
            .wrapping_add(KEY_OFF);
        *(slot as *mut u32) = value;
        base
    }
});
