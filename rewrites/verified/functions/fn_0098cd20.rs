// original: 0x0098CD20 audVehicleAudioEntity::horn_flag_check

/// Returns whether the vehicle's cached horn object has either low flag bit
/// set. If the cache is null, asks the object's factory for a horn object,
/// stores that pointer at offset 0x9E0, then tests the byte at object offset
/// 5. The factory is an intercepted thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_horn_flag_check(this: u32) -> u32 {
    const CACHED_HORN: u32 = 0x9E0;
    const HORN_FLAGS: u32 = 5;
    const ACTIVE_FLAGS: u8 = 0x03;

    unsafe {
        let cache = this.wrapping_add(CACHED_HORN) as *mut u32;
        let mut horn = cache.read_unaligned();
        if horn == 0 {
            let factory: unsafe extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(1));
            horn = factory(this);
            cache.write_unaligned(horn);
        }
        if horn == 0 {
            return 0;
        }
        let flags = (horn.wrapping_add(HORN_FLAGS) as *const u8).read();
        u32::from(flags & ACTIVE_FLAGS != 0)
    }
});
