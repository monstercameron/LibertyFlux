// original: 0x00873b70 crmtManagerPriority::vf1
// Adopt a new priority source: store it, let it attach, then forward the
// weight to this manager's own update hook. (thiscall/2)
export!(thiscall, rw_00873b70(this_ptr: u32, source: u32, weight: u32) -> () {
    unsafe {
        (this_ptr as *mut u32).add(1).write(source);
        let source_vt = (source as *const u32).read();
        let attach: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((source_vt + 4) as *const u32).read() as usize);
        attach(source);
        let own_vt = (this_ptr as *const u32).read();
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((own_vt + 0x14) as *const u32).read() as usize);
        update(this_ptr, weight);
    }
});
