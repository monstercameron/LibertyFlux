// original: 0x00AC1630 stream_select_bank_and_tail (proposed)

/// Run the banked shutdown sequence for `bank`, then tail into the pump.
///
/// The original picks a global bank by the low byte of `bank` (cdecl, one
/// word): bank 1 uses the high globals, any other bank the low ones. It
/// calls the stop callee with the bank flag, the detach callee with
/// (0, object, 0, object word `+0xc`), the release callee with (4, 0, cookie),
/// forces the incoming word to 0 and tail-calls the pump. The rewrite calls
/// the pump through the checker and returns its answer.
lf_checker_rt::export!(cdecl, rw_00AC1630(bank: u32) -> u32 {
    unsafe {
        const STOP: u32 = 1;
        const DETACH: u32 = 2;
        const RELEASE: u32 = 3;
        const PUMP: u32 = 4;
        const HI_FLAG: u32 = 0x0103EFCC;
        const HI_OBJ: u32 = 0x0103EFC8;
        const HI_COOKIE: u32 = 0x0103EFB8;
        const LO_FLAG: u32 = 0x0103EFB4;
        const LO_OBJ: u32 = 0x0103EFB0;
        const LO_COOKIE: u32 = 0x0103EFA0;
        let (flag_a, obj_a, cookie_a) = if bank as u8 == 1 {
            (HI_FLAG, HI_OBJ, HI_COOKIE)
        } else {
            (LO_FLAG, LO_OBJ, LO_COOKIE)
        };
        let flag = (lf_checker_rt::relocated(flag_a) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(STOP, u32, flag);
        let obj = (lf_checker_rt::relocated(obj_a) as *const u32).read_unaligned();
        let w = (obj.wrapping_add(0x0c) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(DETACH, u32, 0u32, obj, 0u32, w);
        let cookie = (lf_checker_rt::relocated(cookie_a) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(RELEASE, u32, 4u32, 0u32, cookie);
        lf_checker_rt::callee_cdecl!(PUMP, u32, 0u32)
    }
});
