// original: 0x00a7e190 linked_ctor_b
/// Linking constructor: base-construct, attach the peer object through the
/// link helper when non-null, then resolve the code word from the peer's
/// inner pointer (zero when any link in the chain is null). Returns the object.
export!(thiscall, rw_00a7e190(this: *mut u8, peer: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let slot = (this.add(0x14)) as *mut u32;
        st32(this, 0, relocated(0xEA10A4));
        *slot = peer;
        if peer != 0 {
            callee_thiscall!(2, u32, peer, slot as u32);
        }
        let cur = *slot;
        let mut code: u16 = 0;
        if cur != 0 {
            let inner = ld32(cur as *const u8, 0x6C);
            if inner != 0 {
                code = callee_thiscall!(3, u32, inner) as u16;
            }
        }
        st16(this, 0x18, code);
        this as u32
    }
});
