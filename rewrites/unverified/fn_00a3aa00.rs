// original: 0x00a3aa00 CEventVehicleDamageWeapon::vf21 (cloner)

/// Clone this damage event into a freshly allocated event object.
///
/// Allocates through the global allocator (id 1, thiscall/0 on the
/// allocator global); a null answer ends the call with 0. Otherwise fills
/// the new object through the field copier (id 2, thiscall/6) with the
/// three position words, the float at `+0x24`, and the addresses of the
/// `+0x30` and `+0x40` blocks, stamps the event vtable word, and returns
/// the new object. Thiscall, no stack arguments, returns EAX.
lf_checker_rt::export!(thiscall, rw_00a3aa00(obj: u32) -> u32 {
    unsafe {
        const ALLOCATOR: u32 = 0x0166_69C8;
        const VTABLE: u32 = 0x00E9_CA74;
        const ALLOC: u32 = 1;
        const FILL: u32 = 2;
        let mgr = core::ptr::read(lf_checker_rt::global::<u32>(ALLOCATOR));
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
        if fresh == 0 {
            return 0;
        }
        let p0 = core::ptr::read_unaligned((obj + 0x18) as *const u32);
        let p1 = core::ptr::read_unaligned((obj + 0x1C) as *const u32);
        let p2 = core::ptr::read_unaligned((obj + 0x20) as *const u32);
        let f3 = core::ptr::read_unaligned((obj + 0x24) as *const u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FILL, u32, fresh, p0, p1, p2, f3, obj.wrapping_add(0x30), obj.wrapping_add(0x40)
        );
        core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(VTABLE));
        fresh
    }
});
