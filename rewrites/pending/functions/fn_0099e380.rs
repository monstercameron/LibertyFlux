// original: 0x0099e380 lookup_store_single
/// Look up a record by key and copy one field to the caller's out-pointer.
///
/// Single-field variant of the paired lookup: calls the lookup helper
/// (stdcall/1, stubbed by the checker) and, unless it answers null, stores
/// the dword at +4 and returns 1.
export!(stdcall, rw_0099e380(key: u32, out: u32) -> u32 {
    unsafe {
        let lookup: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let rec = lookup(key);
        if rec == 0 {
            return 0;
        }
        *(out as *mut u32) = *((rec.wrapping_add(4)) as *const u32);
        1
    }
});
