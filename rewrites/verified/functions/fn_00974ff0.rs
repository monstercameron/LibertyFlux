// original: 0x00974ff0 audio_resolve_and_publish (proposed)

/// Resolve a key through the global directory, then publish the row.
///
/// A zero key returns at once. Otherwise the resolver callee runs on the
/// fixed directory object with (key, aux, out-pointer), where the
/// out-pointer aims at the original's own incoming key slot and receives one
/// word (only its low byte is read back); the row publisher then runs with
/// (resolver result, byte). Equality compare on the key.
/// Original: 0x00974FF0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00974ff0(this: u32, key: u32, aux: u32) -> u32 {
    unsafe {
        const DIR: u32 = 0x1165880;
        const RESOLVE: u32 = 1;
        const PUBLISH: u32 = 2;
        if key == 0 {
            return 0;
        }
        // The original clears the low byte of its incoming key slot (the
        // out-byte) before the call; the snapshot observes this value.
        let mut slot: u32 = key & 0xFFFFFF00;
        let r = lf_checker_rt::callee_thiscall!(
            RESOLVE,
            u32,
            lf_checker_rt::relocated(DIR),
            key,
            aux,
            &mut slot as *mut u32 as u32
        );
        let b = (slot & 0xFF) as u32;
        lf_checker_rt::callee_thiscall!(PUBLISH, u32, this, r, b);
        0
    }
});
