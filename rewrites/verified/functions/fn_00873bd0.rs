// original: 0x00873bd0 crmt_forward_query_or_null
// Forward to the linked record's query hook, or report null when either
// link is missing. The original ends in a tail jump; the call below
// forwards the same receiver and result. (thiscall/0)
export!(thiscall, rw_00873bd0(this_ptr: u32) -> u32 {
    unsafe {
        let mid = (this_ptr as *const u32).add(2).read();
        if mid == 0 {
            return 0;
        }
        let table = (mid as *const u32).add(2).read();
        if table == 0 {
            return 0;
        }
        let vt = (table as *const u32).read();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + 0x24) as *const u32).read() as usize);
        query(table)
    }
});
