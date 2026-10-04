// original: 0x00b951a0 START_KILL_FRENZY
/// Native handler `START_KILL_FRENZY`: forwards eight script words plus a booleanised ninth argument built in its own stack slot.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
#[no_mangle]
pub extern "cdecl" fn rn24_start_kill_frenzy(ctx: u32) {
        // Quirk: as with the single-argument version, the ninth argument is
        // the booleanised script word fused into the low byte of this
        // handler's own incoming stack slot value.
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let flag = (unsafe { args.add(8).read() } != 0) as u32;
        let ninth = (ctx & 0xFFFF_FF00) | flag;
        lf_rn24_rt::callee_cdecl!(1, (), unsafe { args.read() }, unsafe { args.add(1).read() }, unsafe { args.add(2).read() }, unsafe { args.add(3).read() }, unsafe { args.add(4).read() }, unsafe { args.add(5).read() }, unsafe { args.add(6).read() }, unsafe { args.add(7).read() }, ninth);
}
