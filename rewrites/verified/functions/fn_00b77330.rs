// original: 0x00b77330 anim_channel_drive (proposed)

/// Drive one animation channel from two float arguments.
///
/// Loads the owner at `this+0x964`; null means nothing to do. Else resolves
/// the channel (thiscall on the owner, one stack word 0); a null channel
/// also ends the call. Otherwise zeroes the channel (thiscall, one stack
/// word 0.0f), adds the first stack word to `this+0xbb8` and writes the sum
/// through the set callee (thiscall on the channel), then writes the second
/// stack word through the commit callee (thiscall on the channel). The float
/// add runs in the original's operand order. Returns the commit answer, or
/// the resolve answer when the channel is null, or the caller's eax when
/// the owner is null (the contract pins entry eax for that path).
///
/// Original: 0x00b77330 (thiscall, two stack words, both floats).
lf_checker_rt::export!(thiscall, rw_00b77330(this: u32, add_f: u32, set_f: u32) -> u32 {
    unsafe {
        const OWNER_OFF: u32 = 0x964;
        const BASE_OFF: u32 = 0xbb8;
        const RESOLVE: u32 = 1;
        const ZERO: u32 = 2;
        const SET: u32 = 3;
        const COMMIT: u32 = 4;
        let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
        if owner == 0 {
            return 0;
        }
        let chan: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, owner, 0);
        if chan == 0 {
            return chan;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(ZERO, u32, chan, 0);
        let base = f32::from_bits(((this + BASE_OFF) as *const u32).read_unaligned());
        let inc = f32::from_bits(add_f);
        let sum = core::hint::black_box(base) + core::hint::black_box(inc);
        let _: u32 = lf_checker_rt::callee_thiscall!(SET, u32, chan, sum.to_bits());
        lf_checker_rt::callee_thiscall!(COMMIT, u32, chan, set_f)
    }
});
