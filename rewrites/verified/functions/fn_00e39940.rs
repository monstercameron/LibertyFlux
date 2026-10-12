// original: 0x00E39940 rage::audEntity::acquire_slot_sound

/// Resolve a sound slot for this audio entity and write the resolved handle
/// through `out_slot`. The 16-bit field at `this + 4` rejects the empty-slot
/// sentinel; a null output pointer also skips the lookup, though the original
/// still attempts the final store and therefore faults on that input. On a
/// valid request, the helper receives the owner pointer from the shared audio
/// context followed by the first key, request kind, entity pointer and a zero
/// option. This is a 32-bit thiscall method with three stack arguments.
lf_checker_rt::export!(thiscall, rw_00e39940(this: u32, first_key: u32, request_kind: u32, out_slot: u32) -> u32 {
    unsafe {
        const SLOT_STATE_OFFSET: u32 = 0x04;
        const EMPTY_SLOT: u16 = 0xFFFF;
        const AUDIO_OWNER_GLOBAL: u32 = 0x0115_F848;

        let output = out_slot as *mut u32;
        let unavailable = (this.wrapping_add(SLOT_STATE_OFFSET) as *const u16).read_unaligned() == EMPTY_SLOT;
        if unavailable || out_slot == 0 {
            output.write_unaligned(0);
            return EMPTY_SLOT as u32;
        }

        let audio_owner = lf_checker_rt::global::<u32>(AUDIO_OWNER_GLOBAL).read_unaligned();
        let resolved = lf_checker_rt::callee_thiscall!(1, u32, audio_owner, first_key, request_kind, this, 0);
        output.write_unaligned(resolved);
        resolved
    }
});
