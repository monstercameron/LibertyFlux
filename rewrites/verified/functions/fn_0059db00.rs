// original: 0x0059db00 poll_triple_state
// True when any of three global state words matches its active value:
// word1 == 1, word2 != -1 (signed), or word3 == 0x12.
export!(cdecl, rw_0059DB00() -> u32 {
    let w1 = unsafe { *global::<u32>(0x11F7060) };
    if w1 == 1 {
        return 1;
    }
    let w2 = unsafe { *global::<u32>(0x12088B4) };
    if w2 != 0xFFFFFFFF {
        return 1;
    }
    let w3 = unsafe { *global::<u32>(0x1037720) };
    u32::from(w3 == 0x12)
});
