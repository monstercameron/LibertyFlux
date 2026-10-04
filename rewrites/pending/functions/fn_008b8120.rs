// original: 0x008b8120 InputBackendSensitivityScale
/// Backend table lookup with a positive clamp.
///
/// Reads one word from the backend table at `index` and returns it when it is
/// a positive signed value, otherwise returns 1.
export!(cdecl, rw_008b8120(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01160FE8;
        let addr = relocated(TABLE).wrapping_add(index.wrapping_mul(4));
        let value = (addr as *const u32).read();
        if (value as i32) > 0 { value } else { 1 }
    }
});
