// original: 0x008b0ee0 audio_meter_store_group1
/// Copies four words into the meter slot group at `this+0x1754`.
export!(thiscall, rw_008b0ee0(this: u32, src: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x1754), ld32(src));
        st32(this.wrapping_add(0x1758), ld32(src.wrapping_add(4)));
        st32(this.wrapping_add(0x175c), ld32(src.wrapping_add(8)));
        st32(this.wrapping_add(0x1760), ld32(src.wrapping_add(12)));
    }
});
