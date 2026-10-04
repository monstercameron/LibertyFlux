// original: 0x008b0e90 audio_meter_store_group2
/// Copies four words into the meter slot group at `this+0x1764`.
export!(thiscall, rw_008b0e90(this: u32, src: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x1764), ld32(src));
        st32(this.wrapping_add(0x1768), ld32(src.wrapping_add(4)));
        st32(this.wrapping_add(0x176c), ld32(src.wrapping_add(8)));
        st32(this.wrapping_add(0x1770), ld32(src.wrapping_add(12)));
    }
});
