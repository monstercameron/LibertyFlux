// original: 0x00873ed0 crmt_inner_record_or_null
// Resolve the linked record behind this manager, or null when either link
// is missing. (thiscall/0)
export!(thiscall, rw_00873ed0(this_ptr: u32) -> u32 {
    unsafe {
        let mid = (this_ptr as *const u32).add(2).read();
        if mid == 0 {
            return 0;
        }
        (mid as *const u32).add(2).read()
    }
});
