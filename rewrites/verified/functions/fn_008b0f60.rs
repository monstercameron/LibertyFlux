// original: 0x008b0f60 audio_param_store_1724_1728
/// Stores two floats at `this+0x1724` and `this+0x1728`.
export!(thiscall, rw_008b0f60(this: u32, a: f32, b: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x1724), a);
        wrf(this.wrapping_add(0x1728), b);
    }
});
