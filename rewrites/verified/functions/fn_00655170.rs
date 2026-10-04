// original: 0x00655170 rage::ptxSprite::vf31

/// Virtual method 31 of `rage::ptxSprite`: run one keyed lookup on the member
/// object at `this + 0x678`, retrying once with a fallback key.
///
/// `this` is the sprite object (never dereferenced here, only offset into a
/// member pointer) and `key` selects the lookup. When `key` is null the
/// original tail-jumps into the lookup with a default key; otherwise it calls
/// the lookup with `key` and, only when the low byte of the answer is zero,
/// calls it again with a fallback key. The answer of the last call made is
/// returned. The two fixed keys are addresses of data in the program image.
///
/// Original: 0x00655170 (thiscall, one stack word). The callee is thiscall
/// with one stack word; its low answer byte decides the retry.
lf_checker_rt::export!(thiscall, rw_00655170(this: u32, key: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 0x678;
        const DEFAULT_KEY: u32 = 0xf9a39c;
        const RETRY_KEY: u32 = 0xf9a3ac;
        const CALLEE: u32 = 1;

        let member = this.wrapping_add(MEMBER_OFF);
        if key == 0 {
            // The original ends this path in a tail jump; the rewrite issues
            // the same call through the intercepted callee and returns it.
            lf_checker_rt::callee_thiscall!(
                CALLEE,
                u32,
                member,
                lf_checker_rt::relocated(DEFAULT_KEY)
            )
        } else {
            let first = lf_checker_rt::callee_thiscall!(CALLEE, u32, member, key);
            if first & 0xff == 0 {
                lf_checker_rt::callee_thiscall!(
                    CALLEE,
                    u32,
                    member,
                    lf_checker_rt::relocated(RETRY_KEY)
                )
            } else {
                first
            }
        }
    }
});
