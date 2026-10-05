// original: 0x0088F7C0 rage::audSound::vf5

/// Rebuild this sound through the second constructor, then unregister it
/// when the free flag is set.
///
/// `this` points to the sound. The second constructor (callee 1) runs
/// first with the sound as `this`. When bit 0 of `free` is set (and the
/// sound pointer is non-null, which it always is), the sound is passed to
/// the registry's unregister entry (callee 2) together with its voice id
/// byte at `+0x40`. Returns `this`.
///
/// Original: 0x0088F7C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088F7C0(this: u32, free: u32) -> u32 {
    unsafe {
        const VOICE_ID: u32 = 0x40;
        const REGISTRY: u32 = 0x0115_d8a0;
        const REBUILD: u32 = 1;
        const UNREGISTER: u32 = 2;

        lf_checker_rt::callee_thiscall!(REBUILD, u32, this);
        if free & 1 != 0 && this != 0 {
            let vid = ((this + VOICE_ID) as *const u8).read() as u32;
            lf_checker_rt::callee_thiscall!(
                UNREGISTER,
                u32,
                lf_checker_rt::relocated(REGISTRY),
                this,
                vid
            );
        }
        this
    }
});
