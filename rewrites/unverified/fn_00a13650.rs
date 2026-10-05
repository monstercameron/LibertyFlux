// original: 0x00a13650 spawn_eligibility_check (proposed)
/// Decide whether a spawner may run, through staged checks and fallbacks.
///
/// Returns 0 (low byte) for a null `spawner`. Otherwise fetches the context
/// and, when it exists, carries flag bit 2 at `+0x26c` and links back to
/// `spawner` at `+0xb30`, queries it with 0x419: a non-null answer whose
/// word at `+0x44` is below 6 (flag set) or 5 (flag clear) returns 1. Then,
/// when the global mode is 2 and the identity at `spawner + 0x2e` matches
/// one of two globals, queries the registry: a non-null answer whose bytes
/// at `+0x28fe`/`+0x28fc` xor above 0x7f returns 1. Finally returns 1 when
/// the override byte is set or the first counter exceeds the second, else 0.
/// Cdecl, two stack arguments.
export!(cdecl, rw_00a13650(spawner: u32, flag: u32) -> u32 {
    unsafe {
        const CTX: u32 = 1;
        const QUERY: u32 = 2;
        const REGISTRY: u32 = 3;
        const TAG_OFF: u32 = 0x26c;
        const TAG_BIT: u8 = 4;
        const BACK_OFF: u32 = 0xb30;
        const VAT_OFF: u32 = 0x224;
        const VAT_BIAS: u32 = 0x44;
        const QUERY_ARG: u32 = 0x419;
        const COUNT_OFF: u32 = 0x44;
        const ID_OFF: u32 = 0x2e;
        const MODE: u32 = 0x011d6fd4;
        const WANT_A: u32 = 0x012fa008;
        const WANT_B: u32 = 0x012fa248;
        const XOR_HI: u32 = 0x28fe;
        const XOR_LO: u32 = 0x28fc;
        const OVERRIDE: u32 = 0x012bd191;
        const COUNT_A: u32 = 0x012bd19c;
        const COUNT_B: u32 = 0x011735b4;
        if spawner == 0 {
            return 0;
        }
        let ctx = callee_cdecl!(CTX, u32,);
        if ctx != 0
            && ((ctx + TAG_OFF) as *const u8).read() & TAG_BIT != 0
            && ((ctx + BACK_OFF) as *const u32).read_unaligned() == spawner
        {
            let vat = ((ctx + VAT_OFF) as *const u32)
                .read_unaligned()
                .wrapping_add(VAT_BIAS);
            let ans = callee_thiscall!(QUERY, u32, vat, QUERY_ARG);
            let lim = if (flag & 0xff) != 0 { 6u32 } else { 5u32 };
            if ans != 0 && ((ans + COUNT_OFF) as *const u32).read_unaligned() < lim {
                return 1;
            }
        }
        if *global::<u32>(MODE) == 2 {
            let id =
                ((spawner + ID_OFF) as *const u16).read_unaligned() as i16 as i32 as u32;
            if id == *global::<u32>(WANT_A) || id == *global::<u32>(WANT_B) {
                let reg = callee_cdecl!(REGISTRY, u32, 0, 0);
                if reg != 0 {
                    let x = ((reg + XOR_HI) as *const u8).read()
                        ^ ((reg + XOR_LO) as *const u8).read();
                    if x > 0x7f {
                        return 1;
                    }
                }
            }
        }
        if *global::<u8>(OVERRIDE) != 0 {
            return 1;
        }
        if *global::<u32>(COUNT_A) > *global::<u32>(COUNT_B) {
            return 1;
        }
        0
    }
});
