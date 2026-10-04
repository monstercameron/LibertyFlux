// original: 0x0099e350 lookup_store_pair
/// Look up a record by key and copy two fields to the caller's out-pointers.
///
/// Calls the lookup helper (stdcall/1, stubbed by the checker); on a null
/// answer returns 0 with nothing stored, otherwise stores the dwords at +4
/// and +8 and returns 1.
export!(stdcall, rw_0099e350(key: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        let lookup: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let rec = lookup(key);
        if rec == 0 {
            return 0;
        }
        *(out1 as *mut u32) = *((rec.wrapping_add(4)) as *const u32);
        *(out2 as *mut u32) = *((rec.wrapping_add(8)) as *const u32);
        1
    }
});
