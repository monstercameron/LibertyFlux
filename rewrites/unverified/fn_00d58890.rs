// original: 0x00d58890 ccam_view_seq_spawn_pair
/// Spawn a linked pair of sequence children: a primary of kind 0x22 tagged
/// with the first two arguments, then a secondary tagged with the third.
///
/// Asks the manager at `[this+0x114]` for a primary of kind `KIND`
/// (intercepted callee 1, arguments `KIND`, 0, `this`), prepares it
/// (intercepted callee 2), links it to this sequence (intercepted callee 3,
/// argument: the primary) and runs its virtual slot at `+4` (intercepted
/// callee 4). It stores the first argument at primary `+0x144` and the
/// second at `+0x148`, sets bits `LINK_BITS` (0xc) in primary `+0x13c`, then
/// asks the manager for a secondary (callee 1 again, arguments: third
/// argument, 0, primary), runs its slot at `+4` (intercepted callee 5), sets
/// its link bits and returns it.
///
/// Original: thiscall, three stack arguments, returns the secondary in `eax`.
lf_checker_rt::export!(thiscall, rw_00d58890 (this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x114;
        const KIND: u32 = 0x22;
        const FLAGS: u32 = 0x13c;
        const TAG0: u32 = 0x144;
        const TAG1: u32 = 0x148;
        const HOOK_SLOT: u32 = 4;
        const LINK_BITS: u8 = 0x0c;
        const SPAWN: u32 = 1;
        const PREPARE: u32 = 2;
        const LINK: u32 = 3;
        let mgr = ((this + MANAGER) as *const u32).read_unaligned();
        let primary: u32 = lf_checker_rt::callee_thiscall!(SPAWN, u32, mgr, KIND, 0, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, primary);
        let _: u32 = lf_checker_rt::callee_thiscall!(LINK, u32, this, primary);
        let vtable = (primary as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + HOOK_SLOT) as *const u32).read_unaligned() as usize);
        let _: u32 = hook(primary);
        ((primary + TAG0) as *mut u32).write_unaligned(a0);
        ((primary + TAG1) as *mut u32).write_unaligned(a1);
        let pflags = (primary + FLAGS) as *mut u8;
        pflags.write(pflags.read() | LINK_BITS);
        let secondary: u32 = lf_checker_rt::callee_thiscall!(SPAWN, u32, mgr, a2, 0, primary);
        let vtable2 = (secondary as *const u32).read_unaligned();
        let hook2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable2 + HOOK_SLOT) as *const u32).read_unaligned() as usize);
        let _: u32 = hook2(secondary);
        let sflags = (secondary + FLAGS) as *mut u8;
        sflags.write(sflags.read() | LINK_BITS);
        secondary
    }
});
