// original: 0x0089c3b0 rage::audSpeechSound::vf5

/// Deleting destructor of `rage::audSpeechSound` (virtual slot 5).
///
/// Runs the object destructor (a thiscall taking no stack arguments,
/// intercepted as callee 1), then, when the low bit of `flags` is set and
/// `this` is non-null, releases the object through the sound pool at
/// `SOUND_POOL`: the pool handle is the object's bank byte at `+0x40`
/// (`BANK_OFF`). Returns the object pointer unchanged.
///
/// Original: 0x0089c3b0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0089c3b0(this: u32, flags: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SOUND_POOL: u32 = 0x0115d8a0;
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & 1 != 0 && this != 0 {
            let bank = ((this as *const u8).add(BANK_OFF as usize)).read();
            lf_checker_rt::callee_thiscall!(FREE, u32, lf_checker_rt::relocated(SOUND_POOL), this, bank as u32);
        }
        this
    }
});
