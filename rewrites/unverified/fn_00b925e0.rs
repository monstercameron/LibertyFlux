// original: 0x00b925e0 script_classify_null_token (proposed)

/// Returns 1 unless a name matches the first initialised null token.
///
/// On first use initialises two 8-byte token slots from constant strings
/// through `INIT` and raises `FLAG`. A null pointer, an empty string or a
/// lone newline yields 1 at once. Otherwise hashes the string through
/// `HASH`, compares against the second slot through `CMP`, returning 1 on a
/// nonzero low byte, and otherwise returns whether the comparison against
/// the first slot is nonzero. Only the low return byte is meaningful (upper
/// bytes are residue), so the contract compares `al` only.
///
/// Original: 0x00B925E0 (cdecl, one stack word, flag in al).
lf_checker_rt::export!(cdecl, rw_00b925e0(s: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const HASH: u32 = 2;
        const CMP: u32 = 3;
        const FLAG: u32 = 0x0167EFC8;
        const TOK_A: u32 = 0x0167EFCC;
        const TOK_B: u32 = 0x0167EFDC;
        const STR_A: u32 = 0x00EB5714;
        const STR_B: u32 = 0x00EB571C;
        if lf_checker_rt::global::<u8>(FLAG).read() == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                INIT, u32, lf_checker_rt::relocated(TOK_A), lf_checker_rt::relocated(STR_A), 8
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                INIT, u32, lf_checker_rt::relocated(TOK_B), lf_checker_rt::relocated(STR_B), 8
            );
            lf_checker_rt::global::<u8>(FLAG).write(1);
        }
        if s == 0 {
            return 1;
        }
        let w = (s as *const u16).read_unaligned();
        if w == 0x000A || w == 0 {
            return 1;
        }
        let h: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, s);
        let r: u32 = lf_checker_rt::callee_cdecl!(
            CMP, u32, s, lf_checker_rt::relocated(TOK_B), h & 0xFFFF
        );
        if r & 0xFF != 0 {
            return 1;
        }
        let h2: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, s);
        let r2: u32 = lf_checker_rt::callee_cdecl!(
            CMP, u32, s, lf_checker_rt::relocated(TOK_A), h2 & 0xFFFF
        );
        u32::from((r2 & 0xFF) != 0)
    }
});
