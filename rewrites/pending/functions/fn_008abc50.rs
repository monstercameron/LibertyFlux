// original: 0x008ABC50 audio_pair_store_24
/// Store a pair of values at offsets 0x24 and 0x28.
export!(thiscall, rw_008ABC50(obj: *mut u8, a: f32, b: f32) -> () {
    unsafe {
        *(obj.add(0x24) as *mut f32) = a;
        *(obj.add(0x28) as *mut f32) = b;
    }
});
