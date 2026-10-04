// original: 0x008b0f20 audio_param_store_1730
/// Stores one float at `this+0x1730`.
export!(thiscall, rw_008b0f20(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x1730), v);
    }
});
