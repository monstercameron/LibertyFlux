// original: 0x00c68320 flag_select_word_or_dword
// Return one of two globals depending on a flag call: the u16 at the word
// global (zero-extended) when the flag is set, else the dword global.
export!(cdecl, rw_00c68320() -> u32 {
    unsafe {
        let on: u32 = callee_cdecl!(1, u32,);
        if on & 0xff != 0 {
            *(global::<u16>(0x169e0e4)) as u32
        } else {
            *global::<u32>(0x12fa3f8)
        }
    }
});
