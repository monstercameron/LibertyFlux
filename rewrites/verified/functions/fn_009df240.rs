// original: 0x009df240 bounds_copy_unless_bypassed
// fn_009df240: bounds-copy unless bypassed (stdcall/1).
//
// When the mode word is 0 or above 4 (signed), resets it and copies four
// floats from `src` into the global bounds, returning `src`. Otherwise (modes
// 1..4 or any negative value) leaves everything alone and returns the mode.
export!(stdcall, rw_009df240(src: *const u32) -> u32 {
    unsafe {
        let mode = *global::<u32>(0x12B4170);
        if mode != 0 && (mode as i32) <= 4 {
            return mode;
        }
        *global::<u32>(0x12B4170) = 0;
        let dst = global::<u32>(0x12B4180);
        *dst = *src;
        *dst.add(1) = *src.add(1);
        *dst.add(2) = *src.add(2);
        *dst.add(3) = *src.add(3);
        src as u32
    }
});
