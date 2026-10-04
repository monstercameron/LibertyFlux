// original: 0x00c67130 cutscene_set_pair_and_state2
/// Stores two words at offsets 0x2a0/0x2a4 and forces state word to 2.
export!(thiscall, rw_c67130(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        (this as *mut u32).byte_add(0x2a0).write(first);
        (this as *mut u32).byte_add(0x2a4).write(second);
        (this as *mut u32).byte_add(0x314).write(2);
    }
    second
});
