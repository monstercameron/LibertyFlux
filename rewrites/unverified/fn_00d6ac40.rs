// original: 0x00d6ac40 FRONTEND_MENU_MONTAGE_PRESS_MT
/// Handle a montage press: sample, gate, log, tail-forward (original
/// 0x00D6AC40, thiscall/0, ends in a tail call).
///
/// Reads the mode dword from the global at file VA 0x01037720, stores
/// callee 1's answer into the global at file VA 0x011F70E4, and notifies
/// with 0xf (callee 2). When the mode is nonzero it advances the selector
/// (callee 3 with 2) and clears the byte at the double-indirected slot
/// (`this+0x1c`, `+8`, `+0x20`). Then it logs the fixed token (callee 4 on
/// the constant object), runs the shared step (callee 5) and tail-calls the
/// member forwarder (callee 6) on `this+4`. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6ac40(this_ptr: u32) -> u32 {
    unsafe {
        const MODE_GLOBAL: u32 = 0x01037720;
        const SAMPLE_GLOBAL: u32 = 0x011F70E4;
        const LOG_OBJ: u32 = 0x01176888;
        const LOG_TOKEN: u32 = 0x00EEAC14;
        const MEMBER_OFF: u32 = 4;
        const LINK0_OFF: u32 = 0x1c;
        const LINK1_OFF: u32 = 8;
        const FLAG_OFF: u32 = 0x20;
        let mode = (lf_checker_rt::relocated(MODE_GLOBAL) as *const u32)
            .read_unaligned();
        let sample: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        (lf_checker_rt::relocated(SAMPLE_GLOBAL) as *mut u32)
            .write_unaligned(sample);
        lf_checker_rt::callee_cdecl!(2, u32, 0xf);
        if mode != 0 {
            lf_checker_rt::callee_thiscall!(3, u32, this_ptr, 2);
            let mid =
                ((this_ptr + LINK0_OFF) as *const u32).read_unaligned();
            let slot =
                ((mid + LINK1_OFF) as *const u32).read_unaligned();
            ((slot + FLAG_OFF) as *mut u8).write(0);
        }
        lf_checker_rt::callee_thiscall!(4, u32,
            lf_checker_rt::relocated(LOG_OBJ),
            lf_checker_rt::relocated(LOG_TOKEN));
        lf_checker_rt::callee_thiscall!(5, u32, this_ptr);
        let member =
            ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(6, u32, member);
        0
    }
});
