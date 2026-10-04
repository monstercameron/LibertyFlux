// original: 0x008D2270 EnterMode3FromMode2
/// If the mode word at offset 0x4e8 is 2, advances it to 3 and emits the two
/// configuration calls; otherwise leaves everything untouched.
export!(thiscall, rw_008D2270(this: u32) -> () {
    unsafe {
        let mode = &mut *((this + 0x4e8) as *mut u32);
        if *mode == 2 {
            *mode = 3;
            callee_cdecl!(0, u32, 0x105, 0x3f80_0000);
            callee_cdecl!(1, u32, 1);
        }
    }
});
