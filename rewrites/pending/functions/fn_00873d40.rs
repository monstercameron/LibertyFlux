// original: 0x00873d40 crmt_push_value_and_notify
// Push a value through the linked record's setter hook, then notify this
// manager's own hook with the same value. No links, no work. (thiscall/1)
export!(thiscall, rw_00873d40(this_ptr: u32, value: u32) -> () {
    unsafe {
        let mid = (this_ptr as *const u32).add(2).read();
        if mid == 0 {
            return;
        }
        let table = (mid as *const u32).add(2).read();
        if table == 0 {
            return;
        }
        callee_thiscall!(1, u32, table, value);
        let own_vt = (this_ptr as *const u32).read();
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((own_vt + 0x1c) as *const u32).read() as usize);
        notify(this_ptr, value);
    }
});
