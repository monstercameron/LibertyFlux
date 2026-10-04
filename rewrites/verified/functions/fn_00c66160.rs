// original: 0x00c66160 cutscene_same_tag_as_current
/// True (1) when the current record exists and its tag halfword at 0x2e
/// equals this object's tag halfword, else 0.
export!(thiscall, rw_c66160(this: u32) -> u32 {
    let current = callee_thiscall!(1, u32, this);
    if current == 0 {
        return 0;
    }
    let got = unsafe { ((current as *const u8).byte_add(0x2e) as *const u16).read() };
    let want = unsafe { ((this as *const u8).byte_add(0x2e) as *const u16).read() };
    (got == want) as u32
});
