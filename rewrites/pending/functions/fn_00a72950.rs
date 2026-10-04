// original: 0x00a72950 ranged_gate_with_fallback (proposed)
/// True (1) when an xor-ed state byte falls in a global-selected range,
/// else whatever a fallback probe says.
///
/// `cdecl`, no stack words. Fetches the record (callee 1) and the state
/// (callee 2 on the record); either null returns 0. Picks the range from
/// two global bytes: mode set means [1, g], mode clear means [g, 0xFF].
/// Probes the registry (callee 3) with the table word selected by
/// `(link+3)*3` off `record+0x2b0` (the original borrows its own saved
/// register slot for the range bytes; only the values matter, and those
/// live below the incoming stack, outside the comparison); the probe
/// result latches `armed` when the probe word at `+0xc` is 1. When the
/// byte `state[+0x26fe] ^ state[+0x26fc]` lands in the range the result is
/// 1; otherwise, when disarmed, 0; when armed, the fallback probe
/// (callee 4, thiscall on `state+0x2c08` with 0x78, 0x80, 0xFF) decides.
lf_checker_rt::export!(cdecl, rw_00a72950() -> u32 {
    unsafe {
        const MODE: u32 = 0x012fa6ea;
        const G: u32 = 0x0103ce48;
        const LINK_OFF: u32 = 0x2b0;
        const PROBE_OFF: u32 = 0xc;
        const X_HI: u32 = 0x26fe;
        const X_LO: u32 = 0x26fc;
        const FB_OFF: u32 = 0x2c08;
        let rb = |p: u32| (p as *const u8).read();
        let rec = lf_checker_rt::callee_cdecl!(1, u32,);
        if rec == 0 {
            return 0;
        }
        let st = lf_checker_rt::callee_thiscall!(2, u32, rec);
        if st == 0 {
            return 0;
        }
        let g = rb(lf_checker_rt::relocated(G));
        let (lo, hi) = if rb(lf_checker_rt::relocated(MODE)) != 0 {
            (1u8, g)
        } else {
            (g, 0xffu8)
        };
        let link = ((rec + LINK_OFF) as *const u32).read_unaligned();
        let slot = link.wrapping_add(3).wrapping_mul(3);
        let key =
            ((rec + LINK_OFF + slot.wrapping_mul(4)) as *const u32).read_unaligned();
        let r = lf_checker_rt::callee_cdecl!(3, u32, key);
        let armed = r != 0 && ((r + PROBE_OFF) as *const u32).read_unaligned() == 1;
        let x = rb(st + X_HI) ^ rb(st + X_LO);
        if x >= lo && x <= hi {
            return 1;
        }
        if !armed {
            return 0;
        }
        (lf_checker_rt::callee_thiscall!(4, u32, st.wrapping_add(FB_OFF), 0x78, 0x80, 0xff)
            as u8
            != 0) as u32
    }
});
