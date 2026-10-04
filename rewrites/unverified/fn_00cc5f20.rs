// original: 0x00cc5f20 chain_nth_next
/// Follow the intrusive `next` link (`+4`) `n` times from the head held at
/// `this[0]` and return the node reached (the head itself when `n` is 0).
export!(thiscall, rw_00cc5f20(this: *const u8, n: u32) -> u32 {
    unsafe {
        let mut node = *(this as *const u32);
        for _ in 0..n {
            node = *(((node) as *const u8).add(4) as *const u32);
        }
        node
    }
});
