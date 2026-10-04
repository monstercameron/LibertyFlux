// original: 0x0069a700 rage::crAnimChannelStaticFloat::vf9
/// Copy the channel's constant value at `this+8` to `out`. Only `this` and
/// `out` are used. Returns `out`.
export!(thiscall, rw_0069a700(this: u32, _a: u32, _b: u32, out: u32) -> u32 {
    unsafe {
        *(out as *mut u32) = *((this + 8) as *const u32);
        out
    }
});
