// original: 0x008b0ec0 audio_param_store_174c
/// Stores one float at `this+0x174c`.
export!(thiscall, rw_008b0ec0(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x174c), v);
    }
});
