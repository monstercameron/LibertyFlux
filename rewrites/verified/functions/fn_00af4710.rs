// original: 0x00AF4710 drawable_acquire_guarded (proposed)

/// Lock, acquire a slot object (retrying once after a reset), use it, unlock.
///
/// Locks the stream mutex global and tries the acquire callee; when it
/// answers null the reset callee runs and the acquire is tried once more,
/// and a second null clears the mutex and returns null. Otherwise the use
/// callee runs with (slot, `tag`); when `owner` is non-null and bits 6..9
/// of its word at 0x28 read 1 or 5, the bind callee also runs with (slot,
/// `owner`). Clears the mutex and returns the slot.
///
/// Original: 0x00AF4710 (thiscall, two stack words, five direct callees).
lf_checker_rt::export!(thiscall, rw_00af4710(this: u32, owner: u32, tag: u32) -> u32 {
    unsafe {
        const LOCK_CALLEE: u32 = 1;
        const ACQUIRE_CALLEE: u32 = 2;
        const RESET_CALLEE: u32 = 3;
        const USE_CALLEE: u32 = 4;
        const BIND_CALLEE: u32 = 5;
        const MUTEX: u32 = 0x015F8B40;
        const REGISTRY_OBJ: u32 = 0x01173750;
        const KIND_OFF: u32 = 0x28;
        lf_checker_rt::callee_cdecl!(LOCK_CALLEE, u32, lf_checker_rt::relocated(MUTEX));
        let mut slot = lf_checker_rt::callee_thiscall!(ACQUIRE_CALLEE, u32, this);
        if slot == 0 {
            lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, lf_checker_rt::relocated(REGISTRY_OBJ));
            slot = lf_checker_rt::callee_thiscall!(ACQUIRE_CALLEE, u32, this);
            if slot == 0 {
                (lf_checker_rt::global::<u32>(MUTEX)).write_unaligned(0);
                return 0;
            }
        }
        lf_checker_rt::callee_thiscall!(USE_CALLEE, u32, slot, tag);
        if owner != 0 {
            let kind = (((owner.wrapping_add(KIND_OFF)) as *const u32).read_unaligned() >> 6) & 0xF;
            if kind == 1 || kind == 5 {
                lf_checker_rt::callee_thiscall!(BIND_CALLEE, u32, slot, owner);
            }
        }
        (lf_checker_rt::global::<u32>(MUTEX)).write_unaligned(0);
        slot
    }
});
