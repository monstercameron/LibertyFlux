// original: 0x008a8bb0 audio_params_store_3f
/// Store three float parameters into the audio object (`this`).
///
/// Writes the three stack arguments to fixed slots. Returns nothing; the
/// original preserves EAX, which is unobservable from safe Rust, so the
/// contract compares no return channel.
export!(thiscall, rw_008a8bb0(this: *mut u8, v0: f32, v1: f32, v2: f32) -> () {
    unsafe {
        *(this.add(0x171c) as *mut f32) = v0;
        *(this.add(0x1718) as *mut f32) = v1;
        *(this.add(0x1720) as *mut f32) = v2;
    }
});
