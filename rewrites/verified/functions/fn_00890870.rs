// original: 0x00890870 rage::audSound::find_by_name

/// Find a sound by its name: hash the name, then run the slot-0 lookup.
///
/// `this` points to the sound and `name` to its name string. The name hash
/// callee (callee 1, cdecl) hashes `(name, 0)` and the table slot at `+0x00`
/// of the sound's table (callee 2, thiscall) is then called with the sound
/// and the hash; its result is returned. The hash is an unsigned 32-bit
/// value passed through unchanged.
///
/// Original: 0x00890870 (thiscall, one stack word, 2 calls).
lf_checker_rt::export!(thiscall, rw_00890870(this: u32, name: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const FIND_SLOT: u32 = 0x00;
        const HASH_SEED: u32 = 0;

        let vtable = (this as *const u32).read_unaligned();
        let hash: u32 = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, name, HASH_SEED);
        let find: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((vtable + FIND_SLOT) as *const u32).read_unaligned() as usize,
        );
        find(this, hash)
    }
});
