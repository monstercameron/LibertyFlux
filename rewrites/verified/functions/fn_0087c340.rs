// original: 0x0087c340 rage::crmtNodeBlend::vf5
/// Report a blend node's state as three records through a trace sink.
///
/// Runs intercepted callee 1 (thiscall/1) over (this, ctx), widens the
/// floats at `+0x20` and `+0x24` to doubles with exact cvtps2pd semantics
/// (the original converts two lanes but stores only the low double of
/// each, so the upper-lane input is unobserved), and reports three records
/// through intercepted callees 2/3/4 (cdecl/6, cdecl/4, cdecl/3): ctx, a
/// relocated record id, then the payload: both doubles, the words at
/// `+0x28`/`+0x2c`, and the zero-extended byte at `+0x30`. Computes no
/// return value.
///
/// Original: thiscall/1, four direct calls, float widening only.
export!(thiscall, rw_0087c340(this: u32, ctx: u32) -> u32 {
    /// Record id for the doubles payload (file VA; relocated at load).
    const REC_DOUBLES: u32 = 0x00FC8914;
    /// Record id for the words payload (file VA; relocated at load).
    const REC_WORDS: u32 = 0x00FC88EC;
    /// Record id for the byte payload (file VA; relocated at load).
    const REC_BYTE: u32 = 0x00FC88D0;
    unsafe {
        callee_thiscall!(1, u32, this, ctx);
        let w20 = ((this + 0x20) as *const f32).read_unaligned();
        let w24 = ((this + 0x24) as *const f32).read_unaligned();
        let d20 = f64::from(w20).to_bits();
        let d24 = f64::from(w24).to_bits();
        callee_cdecl!(2, u32, ctx, relocated(REC_DOUBLES),
            d20 as u32, (d20 >> 32) as u32, d24 as u32, (d24 >> 32) as u32);
        let p28 = ((this + 0x28) as *const u32).read_unaligned();
        let p32 = ((this + 0x2C) as *const u32).read_unaligned();
        callee_cdecl!(3, u32, ctx, relocated(REC_WORDS), p28, p32);
        let b = ((this + 0x30) as *const u8).read() as u32;
        callee_cdecl!(4, u32, ctx, relocated(REC_BYTE), b);
        0
    }
});
