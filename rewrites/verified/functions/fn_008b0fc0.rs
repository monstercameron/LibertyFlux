// original: 0x008b0fc0 audio_param_store_17dc
/// Stores one float at `this+0x17dc`.
export!(thiscall, rw_008b0fc0(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x17dc), v);
    }
});
