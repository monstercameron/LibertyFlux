// original: 0x005e6d50 DESTROY_MOBILE_PHONE
/// Script native `DESTROY_MOBILE_PHONE`.
///
/// Takes no script arguments. Runs an engine worker, then fetches an object
/// through a second engine call and reads a field at offset 0x228. When the
/// field is non-null it clears a flag byte at offset 0x486 of that object
/// and returns the field; when null it takes the original's null path,
/// which stores a zero byte to absolute address 0x416 (a fault, compared
/// by the checker through the termination channel) and returns 0.
export!(cdecl, rw_005e6d50(ctx: *const u8) -> u32 {
    unsafe {
        let _ = ctx;
        callee_cdecl!(1, u32,);
        let obj = callee_cdecl!(2, u32, 0u32);
        let inner = *((obj + 0x228) as *const u32);
        if inner == 0 {
            *(0x416u32 as *mut u8) = 0;
            0
        } else {
            *((inner + 0x486) as *mut u8) = 0;
            inner
        }
    }
});
