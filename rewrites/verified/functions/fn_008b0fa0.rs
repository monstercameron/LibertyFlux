// original: 0x008b0fa0 audio_param_store_1748
/// Stores one float at `this+0x1748`.
export!(thiscall, rw_008b0fa0(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x1748), v);
    }
});
