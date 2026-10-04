// original: 0x00aff950 teardown_and_notify
/// Tear down the object when its busy word is set, then notify the manager
/// through its slot-2 handler.
export!(cdecl, rw_00aff950(obj: u32) -> u32 {
    unsafe {
        if *((obj + 0x70) as *const u32) != 0 {
            callee_cdecl!(1, u32, obj, 0);
        }
        let manager = *global::<u32>(0x166d9fc);
        let table = *(manager as *const u32) as *const u32;
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*table.add(2));
        notify(manager, obj);
        0
    }
});
