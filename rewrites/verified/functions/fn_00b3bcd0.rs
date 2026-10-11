// original: 0x00B3BCD0 apply_object_updates_and_dispatch
/// Apply an object's two virtual update operations when its enable byte is
/// nonzero, then dispatch one global manager operation according to the
/// object's mode bits. The first virtual method receives two constant words
/// and the object's signed 16-bit adjustment; all helper calls are observable
/// through the checker's call recorder.
lf_checker_rt::export!(cdecl, rw_00b3bcd0(object: u32, enabled: u32) -> () {
    unsafe {
        const OBJECT_VTABLE: u32 = 0;
        const OBJECT_ADJUSTMENT: u32 = 0x2e;
        const OBJECT_MODE: u32 = 0x28;
        const GLOBAL_MANAGER: u32 = 0x0166d9f4;
        const VTABLE_SLOT_FIRST: u32 = 0x0c;
        const VTABLE_SLOT_SECOND: u32 = 0x18;
        const VTABLE_SLOT_UPDATE: u32 = 0xd0;
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

        if enabled as u8 != 0 {
            let vtable = read_u32(object.wrapping_add(OBJECT_VTABLE));
            let target = read_u32(vtable.wrapping_add(VTABLE_SLOT_UPDATE));
            let update: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let signed_adjustment = i32::from(read_u16(object.wrapping_add(OBJECT_ADJUSTMENT)) as i16) as u32;
            let first_value = update(object, 1, 1, signed_adjustment);
            let _ = lf_checker_rt::callee_cdecl!(2, u32, first_value);
            let signed_adjustment = i32::from(read_u16(object.wrapping_add(OBJECT_ADJUSTMENT)) as i16) as u32;
            let second_value = update(object, 1, 0, signed_adjustment);
            let _ = lf_checker_rt::callee_cdecl!(2, u32, second_value);
        }

        let manager = lf_checker_rt::global::<u32>(GLOBAL_MANAGER).read_unaligned();
        let manager_vtable = read_u32(manager);
        let mode = read_u32(object.wrapping_add(OBJECT_MODE)) & 0x3c0;
        let slot = if mode == 0xc0 { VTABLE_SLOT_SECOND } else { VTABLE_SLOT_FIRST };
        let target = read_u32(manager_vtable.wrapping_add(slot));
        // The final virtual operation takes the current object as its one
        // stack argument; the manager itself is carried in ECX.
        let dispatch: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
        let _ = dispatch(manager, object);
    }
});
