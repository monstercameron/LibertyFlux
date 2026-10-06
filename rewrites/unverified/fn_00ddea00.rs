// original: 0x00DDEA00 UITextField select and notify by reference
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Select the child for `mode`, then call its virtual slot `+0x208` with a
/// pointer to `key`'s stack slot, returning the callee's answer. The pointer's
/// value differs per side (skipped in the proof); the word it points to is
/// snapshotted at call time and compared.
/// Original: thiscall, two stack words, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDEA00(this: u32, key: u32, mode: u32) -> u32 {
    unsafe {
        const SELECT: u32 = 1;
        const SLOT: u32 = 0x208;
        let child = lf_checker_rt::callee_thiscall!(SELECT, u32, this, mode);
        let vt = ((child) as *const u32).read_unaligned();
        let slot = ((vt + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let mut key_slot = key;
        f(child, (&mut key_slot as *mut u32) as u32)
    }
});
