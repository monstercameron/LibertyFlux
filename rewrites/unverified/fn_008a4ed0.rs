// original: 0x008A4ED0 rage::audIfSound::~audIfSound__deleting

/// Destroy an if-sound node, freeing it when asked.
///
/// Runs the reparenting destructor, then when bit 0 of `flags` is set and
/// the object pointer is non-null releases it through the audio heap with
/// its pool byte. Returns the object pointer. Thiscall, object in ECX,
/// flags popped from the stack.
lf_checker_rt::export!(thiscall, rw_008A4ED0(this: u32, flags: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0115_D8A0;
        const DELETE: u32 = 1;
        const POOL: u32 = 0x40;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & DELETE != 0 && this != 0 {
            let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                2,
                u32,
                lf_checker_rt::relocated(HEAP),
                this,
                u32::from(pool)
            );
        }
        this
    }
});
