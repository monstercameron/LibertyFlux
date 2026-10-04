// original: 0x008b0f40 audio_param_store_172c
/// Stores one float at `this+0x172c`.
export!(thiscall, rw_008b0f40(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x172c), v);
    }
});
