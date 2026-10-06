// original: 0x008f8590 input_pack_binding (proposed)

/// Pack four binding bytes into one dword of the binding table.
///
/// `idx` selects the dword in the global binding table, and `b1..b4` are
/// single bytes passed as stack words (only the low byte of each is read).
/// The stored dword is `(b4:b1 << 16) | ((b2:b3) & 0xFFFFFF)`: the first
/// pair forms the high half, the second pair the low 24 bits with the top
/// byte always zero. The contract pins `idx` to 0..3. EAX holds `b3`
/// (zero-extended) at return, so the contract compares the full register.
///
/// Cdecl: five stack words `(idx, b1, b2, b3, b4)`, caller cleans.
lf_checker_rt::export!(cdecl, rw_008f8590(idx: u32, b1: u32, b2: u32, b3: u32, b4: u32) -> u32 {
    unsafe {
        const G_BIND_TABLE: u32 = 0x118dd98;
        let hi = ((b4 & 0xff) << 8) | (b1 & 0xff);
        let lo = (((b2 & 0xff) << 8) | (b3 & 0xff)) & 0x00ff_ffff;
        let packed = (hi << 16) | lo;
        (lf_checker_rt::global::<u32>(G_BIND_TABLE).add(idx as usize)).write_unaligned(packed);
        b3 & 0xff
    }
});
