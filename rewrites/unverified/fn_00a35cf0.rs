// original: 0x00a35cf0 CEventVehicleDamageWeapon::vf0 (deleting destructor)

/// Destroy this damage event and free it when the flag word asks.
///
/// Stamps the event vtable word, runs the base destructor (id 1,
/// thiscall/0), and when the flag word's bit 0 is set, releases the
/// object through the global releaser (id 2, thiscall/1 on the allocator
/// global). Thiscall/1, callee cleans 4, returns the object.
lf_checker_rt::export!(thiscall, rw_00a35cf0(obj: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E9_CA74;
        const ALLOCATOR: u32 = 0x0166_69C8;
        const BASE_DTOR: u32 = 1;
        const RELEASE: u32 = 2;
        core::ptr::write_unaligned(obj as *mut u32, lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, obj);
        if flags & 1 != 0 {
            let mgr = core::ptr::read(lf_checker_rt::global::<u32>(ALLOCATOR));
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, mgr, obj);
        }
        obj
    }
});
