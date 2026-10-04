// original: 0x005d5000 CTaskComplexPlayerSettingsTask::CTaskComplexPlayerSettingsTask
/// Construct a player-settings task: allocate through the manager global,
/// run the base initialiser on the new object, then stamp the tag, vtable
/// and status fields. Returns the new object, or null when allocation fails.
export!(thiscall, rw_005d5000(this_ptr: u32) -> u32 {
    let mgr = unsafe { global::<u32>(0x0167E2A0).read() };
    let obj: u32 = callee_thiscall!(0, u32, mgr);
    if obj == 0 {
        return 0;
    }
    let tag = unsafe { ((this_ptr + 0x20) as *const u32).read() };
    let _: u32 = callee_thiscall!(1, u32, obj);
    unsafe {
        ((obj + 0x20) as *mut u32).write(tag);
        ((obj) as *mut u32).write(relocated(0x00E9F0BC));
        ((obj + 0x14) as *mut u8).write(0);
        ((obj + 0x18) as *mut u32).write(0xFFFF_FFFF);
        ((obj + 0x1C) as *mut u32).write(0xFFFF_FFFF);
    }
    obj
});
