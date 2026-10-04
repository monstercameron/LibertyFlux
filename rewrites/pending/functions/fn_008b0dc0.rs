// original: 0x008b0dc0 audio_meter_store_group3
/// Copies four words into the meter slot group at `this+0x1774`.
export!(thiscall, rw_008b0dc0(this: u32, src: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x1774), ld32(src));
        st32(this.wrapping_add(0x1778), ld32(src.wrapping_add(4)));
        st32(this.wrapping_add(0x177c), ld32(src.wrapping_add(8)));
        st32(this.wrapping_add(0x1780), ld32(src.wrapping_add(12)));
    }
});
