// original: 0x00890700 audio_chain_tail_tag
/// Return the tag byte of the node at the end of this object's chain.
///
/// Resolves the chain tail through the helper the checker stubs out, then
/// reads its tag byte at +4. A null tail yields 0xff.
export!(thiscall, rw_00890700(this: u32) -> u32 {
    let tail: u32 = callee_thiscall!(1, u32, this);
    if tail == 0 {
        0xff
    } else {
        unsafe {
            let tag = ((tail + 4) as *const u8).read();
            (tail & !0xff) | (tag as u32)
        }
    }
});
