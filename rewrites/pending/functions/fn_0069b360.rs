// original: 0x0069b360 rage::crAnimChannelStaticFloat::vf4
/// Copy the constant at `this+8` to `out`; the frame argument is ignored.
/// Returns `out`.
export!(thiscall, rw_0069b360(this: u32, _f: u32, out: u32) -> u32 {
    unsafe {
        *(out as *mut u32) = *((this + 8) as *const u32);
        out
    }
});
