// original: 0x008ABAD0 audio_pair_store
/// Store a pair of values and mark the object active.
export!(thiscall, rw_008ABAD0(obj: *mut u8, a: f32, b: f32) -> () {
    unsafe {
        let flags = obj.add(0x18);
        let mode = *flags;
        *(obj as *mut f32) = a;
        *(obj.add(4) as *mut f32) = b;
        *flags = (mode & 0xFB) | 3;
    }
});
