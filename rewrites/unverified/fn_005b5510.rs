// original: 0x005B5510 menu_present_apply (proposed)

/// Drive one front-end present/apply pass from two mode flags and scripted getters.
///
/// Reads two mode bytes (`MODE_FLAG_A`, `MODE_FLAG_B`) and selects a mode word:
/// 2 when the first flag equals `MODE_VALUE_DIRECT`, else 7 when the second flag
/// is clear, else 2. It then runs a fixed getter sequence: a one-word probe
/// (callee 0), two indexed reads (callee 1, indexes 8 and 11) of which the
/// second returns a pointer the function dereferences for one float word, a
/// four-word setup call (callee 2) taking (0, mode, two frame pointers), and
/// three colour reads (callee 3, indexes 0x42, 0x3e, 0x3b), each returning a
/// pointer the function dereferences for one word.
///
/// The gathered words feed a seven-argument apply call (callee 4) together with
/// two globals (a dword and a float, passed by value) and the address of a
/// third global. After a two-word ping (callee 5) a status word is fetched
/// (callee 6) and compared, SIGNED, against -0x5c for equality and then with a
/// signed greater-or-equal against a 16-bit bound loaded from `BOUND_TABLE +
/// index * 24` (the index is a global; the bound word is zero-extended, so it
/// is always non-negative as a signed value). Either match selects the fallback
/// selector 0, otherwise the status word itself is the selector. The tail is
/// identical on both paths: a two-word select (callee 7) of (0, selector), a
/// two-word commit (callee 9) of its answer, a three-word select (callee 8) of
/// (0, selector, 1), and a two-word final commit (callee 10) of its answer.
///
/// The float word from the second indexed read is stored to a frame slot that
/// ends up as the fifth stack word of the setup call; the contract declares the
/// setup callee with five arguments so that word is compared (the real callee
/// takes four; the fifth word is a spill the original happens to place there).
/// All frame-pointer arguments are skipped in the comparison and their contents
/// are unobserved: the stubs write nothing through them and the caller never
/// reads them back. Takes no arguments, returns nothing (cdecl, 0 args).
lf_checker_rt::export!(cdecl, rw_005B5510() -> u32 {
    unsafe {
        const MODE_FLAG_A: u32 = 0x116C250;
        const MODE_FLAG_B: u32 = 0x116C253;
        const MODE_VALUE_DIRECT: u8 = 0x6A;
        const MODE_DIRECT: u32 = 2;
        const MODE_ALT: u32 = 7;
        const G_FLOAT: u32 = 0x1161864;
        const G_DWORD_A: u32 = 0x1161850;
        const G_ADDR_IMM: u32 = 0x116185C;
        const G_INDEX: u32 = 0x1160C0C;
        const BOUND_TABLE: u32 = 0x19D33A4;
        const BOUND_STRIDE: u32 = 24;
        const STATUS_FALLBACK: u32 = 0xFFFF_FFA4; // -0x5c, compared SIGNED
        const C_PROBE: u32 = 0;
        const C_INDEXED: u32 = 1;
        const C_SETUP: u32 = 2;
        const C_COLOUR: u32 = 3;
        const C_APPLY: u32 = 4;
        const C_PING: u32 = 5;
        const C_STATUS: u32 = 6;
        const C_SELECT2: u32 = 7;
        const C_SELECT3: u32 = 8;
        const C_COMMIT: u32 = 9;
        const C_FINAL: u32 = 10;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let flag_a = rd8(lf_checker_rt::relocated(MODE_FLAG_A));
        let flag_b = rd8(lf_checker_rt::relocated(MODE_FLAG_B));
        let mode = if flag_a == MODE_VALUE_DIRECT {
            MODE_DIRECT
        } else if flag_b == 0 {
            MODE_ALT
        } else {
            MODE_DIRECT
        };

        // Frame buffers. Their addresses are skipped in the comparison; the
        // stubs write nothing through them and nothing reads them back.
        let mut buf_probe = [0u32; 2];
        let mut buf_idx = [0u32; 2];
        let mut buf_setup = [0u32; 2];
        let mut buf_colour = [0u32; 2];
        let p_probe = (&mut buf_probe as *mut u32) as u32;
        let p_idx = (&mut buf_idx as *mut u32) as u32;
        let p_setup = (&mut buf_setup as *mut u32) as u32;
        let p_colour = (&mut buf_colour as *mut u32) as u32;

        lf_checker_rt::callee_cdecl!(C_PROBE, u32, p_probe);
        lf_checker_rt::callee_cdecl!(C_INDEXED, u32, p_idx, 8);
        let float_ptr = lf_checker_rt::callee_cdecl!(C_INDEXED, u32, p_idx, 0x0B);
        let float_bits = rd32(float_ptr);
        lf_checker_rt::callee_cdecl!(C_SETUP, u32, 0, mode, p_setup, p_idx, float_bits);

        let c0 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_idx, 0x42);
        let v0 = rd32(c0);
        let c1 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_setup, 0x3E);
        let v1 = rd32(c1);
        let c2 = lf_checker_rt::callee_cdecl!(C_COLOUR, u32, p_colour, 0x3B);
        let v2 = rd32(c2);

        let g_float = rd32(lf_checker_rt::relocated(G_FLOAT));
        let g_dword = rd32(lf_checker_rt::relocated(G_DWORD_A));
        let index = rd32(lf_checker_rt::relocated(G_INDEX));
        lf_checker_rt::callee_cdecl!(
            C_APPLY, u32, index, g_dword,
            lf_checker_rt::relocated(G_ADDR_IMM),
            g_float, v2, v1, v0
        );

        lf_checker_rt::callee_cdecl!(C_PING, u32, 0, 1);
        let status = lf_checker_rt::callee_cdecl!(C_STATUS, u32, index);
        let bound = rd16(
            lf_checker_rt::relocated(BOUND_TABLE).wrapping_add(index.wrapping_mul(BOUND_STRIDE)),
        );
        // Both comparisons are SIGNED (je after cmp against -0x5c; jge after
        // cmp against the zero-extended bound).
        let fallback =
            status == STATUS_FALLBACK || (status as i32) >= (bound as i32);
        let sel = if fallback { 0 } else { status };

        let t = lf_checker_rt::callee_cdecl!(C_SELECT2, u32, 0, sel);
        lf_checker_rt::callee_cdecl!(C_COMMIT, u32, 0, t);
        let u = lf_checker_rt::callee_cdecl!(C_SELECT3, u32, 0, sel, 1);
        lf_checker_rt::callee_cdecl!(C_FINAL, u32, 0, u);
    }
    0
});
