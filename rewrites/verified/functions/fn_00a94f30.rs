// original: 0x00a94f30 fiStreamingDevice::vf6 (symbols)

/// Device virtual slot 6: close the kind channel, then post a 64-bit span.
///
/// The kind byte (`table[arg0*3*8+4]`, table reached through `this+0x04`)
/// times 160 selects a channel in a global array. A set flag at `this+0x0c`
/// first closes the channel's current object (slot-0x80 virtual, callee 1,
/// thiscall/0). Then the channel's 64-bit cursor (`+0x08`:`+0x0c`) plus the
/// (second, third) argument pair is posted with the (fourth, fifth)
/// arguments to the channel's span sink (slot-0x18 virtual, callee 2,
/// thiscall/5).
///
/// Returns the sink's answer. Thiscall: object in ecx, five stack words,
/// callee pops 20.
lf_checker_rt::export!(thiscall, rw_00a94f30(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x04;
        const FLAG_OFF: u32 = 0x0c;
        const KIND_OFF: u32 = 0x04;
        const CHANNEL_BASE: u32 = 0x012fb3b8;
        const CHANNEL_STRIDE: u32 = 160;
        const CUR_LO: u32 = 0x08;
        const CUR_HI: u32 = 0x0c;
        const OBJ_OFF: u32 = 0x90;
        const SINK_ARG: u32 = 0x98;
        let elem = ((this + TABLE_PTR) as *const u32).read_unaligned();
        let elem = (elem as *const u32).read_unaligned()
            + a0.wrapping_mul(3).wrapping_mul(8);
        let kind = ((elem + KIND_OFF) as *const u8).read() as u32;
        let ch = lf_checker_rt::relocated(CHANNEL_BASE) + kind.wrapping_mul(CHANNEL_STRIDE);
        if (((this + FLAG_OFF) as *const u8).read()) != 0 {
            let obj = ((ch + OBJ_OFF) as *const u32).read_unaligned();
            let vt = (obj as *const u32).read_unaligned();
            let t = ((vt + 0x80) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(t as usize);
            f(obj);
        }
        let lo = ((ch + CUR_LO) as *const u32).read_unaligned().wrapping_add(a1);
        let carry = (lo < a1) as u32;
        let hi = ((ch + CUR_HI) as *const u32)
            .read_unaligned()
            .wrapping_add(a2)
            .wrapping_add(carry);
        let obj = ((ch + OBJ_OFF) as *const u32).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let t = ((vt + 0x18) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(t as usize);
        f(obj, ((ch + SINK_ARG) as *const u32).read_unaligned(), lo, hi, a3, a4)
    }
});
