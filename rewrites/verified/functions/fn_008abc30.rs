// original: 0x008ABC30 audio_pair_store_14
/// Store a pair of values at offsets 0x14 and 0x18.
export!(thiscall, rw_008ABC30(obj: *mut u8, a: f32, b: f32) -> () {
    unsafe {
        *(obj.add(0x14) as *mut f32) = a;
        *(obj.add(0x18) as *mut f32) = b;
    }
});
