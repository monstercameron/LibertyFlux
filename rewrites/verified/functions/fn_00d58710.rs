// original: 0x00d58710 CCamRadar::vf1
/// Attach a fresh radar child to this camera: allocate it from the manager,
/// link it in, initialise it through its virtual slot and mark it.
///
/// Fetches a token from the shared pool `POOL` (intercepted callee 1), then
/// asks the manager at `[this+0x114]` for a child of kind 5 (intercepted
/// callee 2, arguments 5, 0, token). It sets bit `CHILD_BIT` (0x20) in
/// `[this+0x13c]`, stores the child at `[this+0x128]`, sets bit `READY_BIT`
/// (4) in the child's `+0x13c`, hands the global `ARG` to the child's setup
/// routine (intercepted callee 3), runs the child's virtual slot at `+4`
/// (intercepted callee 4) and clears the child's byte at `+0x15d`. Returns
/// the slot's answer with its low byte forced to 1.
///
/// Original: thiscall, no stack arguments; `(an instruction of the original)` over the last answer.
lf_checker_rt::export!(thiscall, rw_00d58710 (this: u32) -> u32 {
    unsafe {
        const POOL: u32 = 0x0103E498;
        const MANAGER: u32 = 0x114;
        const FLAGS: u32 = 0x13c;
        const CHILD: u32 = 0x128;
        const KIND: u32 = 5;
        const CHILD_BIT: u8 = 0x20;
        const READY_BIT: u8 = 4;
        const HOOK_SLOT: u32 = 4;
        const TAG: u32 = 0x15d;
        const ARG_GLOBAL: u32 = 0x0118D808;
        const TOKEN_CALLEE: u32 = 1;
        const SPAWN_CALLEE: u32 = 2;
        const SETUP_CALLEE: u32 = 3;
        let token: u32 = lf_checker_rt::callee_thiscall!(
            TOKEN_CALLEE, u32, lf_checker_rt::relocated(POOL));
        let mgr = ((this + MANAGER) as *const u32).read_unaligned();
        let child: u32 = lf_checker_rt::callee_thiscall!(SPAWN_CALLEE, u32, mgr, KIND, 0, token);
        let flags = (this + FLAGS) as *mut u8;
        flags.write(flags.read() | CHILD_BIT);
        ((this + CHILD) as *mut u32).write_unaligned(child);
        let cflags = (child + FLAGS) as *mut u8;
        cflags.write(cflags.read() | READY_BIT);
        let arg = lf_checker_rt::global::<u32>(ARG_GLOBAL).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP_CALLEE, u32, child, arg);
        let vtable = (child as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + HOOK_SLOT) as *const u32).read_unaligned() as usize);
        let ans = hook(child);
        ((child + TAG) as *mut u8).write(0);
        (ans & 0xFFFFFF00) | 1
    }
});
