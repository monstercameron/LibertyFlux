// original: 0x00b8c890 GET_FIRST_N_CHARACTERS_OF_STRING
/// Native handler `GET_FIRST_N_CHARACTERS_OF_STRING`: forwards two script words to the engine and stores the full answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_get_first_n_characters_of_string(ctx: u32) {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let answer: u32 = lf_rn24_rt::callee_cdecl!(1, u32, unsafe { args.read() }, unsafe { args.add(1).read() });
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        unsafe { slot.write(answer) };
}
