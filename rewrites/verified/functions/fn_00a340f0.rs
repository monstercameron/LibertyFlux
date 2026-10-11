// original: 0x00A340F0 CEntity::apply_state_timing

/// Looks up a value through the entity system's global context and forwards
/// it with the remaining two arguments to the state-timing helper. A zero
/// lookup key instead sets the entity state byte at +0x41 to 3 and returns
/// zero. The lookup helper consumes one stack word; the other two words stay
/// live for the state-timing call. This is a thiscall method with three
/// incoming stack words.
lf_checker_rt::export!(thiscall, rw_00a340f0(this: u32, lookup_key: u32, mode: u32, limit: u32) -> u32 {
    const ENTITY_STATE: u32 = 0x41;
    const GLOBAL_CONTEXT: u32 = 0x012f_b214;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        if lookup_key == 0 {
            write_u8(this + ENTITY_STATE, 3);
            return 0;
        }

        let context_slot = lf_checker_rt::global::<u32>(GLOBAL_CONTEXT) as u32;
        let context = read_u32(context_slot);
        let looked_up = lf_checker_rt::callee_thiscall!(1, u32, context, lookup_key);
        lf_checker_rt::callee_thiscall!(2, u32, this, looked_up, mode, limit)
    }
});
