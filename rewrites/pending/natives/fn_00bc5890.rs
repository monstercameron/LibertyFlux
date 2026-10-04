// original: 0x00bc5890 FORCE_GENERATE_PARKED_CARS_TOO_CLOSE_TO_OTHERS
/// Native handler `FORCE_GENERATE_PARKED_CARS_TOO_CLOSE_TO_OTHERS`: booleanises one script argument into the low byte of its own stack slot and forwards the whole dword.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_force_generate_parked_cars_too_close_to_others(ctx: u32) {
        // Quirk: the original writes the booleanised argument into the low byte
        // of its own incoming stack slot and pushes the whole dword, so the
        // engine sees this handler's address with a replaced low byte.
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let flag = (unsafe { args.read() } != 0) as u32;
        lf_rn24_rt::callee_cdecl!(1, (), (ctx & 0xFFFF_FF00) | flag);
}
