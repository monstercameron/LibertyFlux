// original: 0x0065B0D0 shader_register_named_pass (proposed)

/// Create a pass through the registry, hash its name, bind and append it.
///
/// Calls the registry (object at the global `REGISTRY`, slot `+4`, thiscall:
/// `arg + 4`, 0, 0) and returns null when it answers null. Otherwise hashes
/// the NUL-terminated name at `arg + 0x44` (empty names skip the hash) with
/// the hash callee (thiscall on the name, no stack arguments); a hash of
/// exactly -1 (equality: any other negative still indexes) or an empty name
/// leaves the entry null, otherwise the entry is the global `TABLE` plus
/// `hash * 28`. Then runs the binder (slot `+0x0c` of the created pass,
/// thiscall: `arg + 4`, 0, 0, entry, 0, 0, 0, 0) and returns null when its
/// low byte is zero. On success appends the pass through the append callee
/// (thiscall on `this + 0x1c`: the word surviving the binder call in the
/// object register, skipped in the proof as harness scratch) and stores the
/// pass in the appended slot. Returns the pass (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0065b0d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x17F5A04;
        const TABLE: u32 = 0x17F59C8;
        const CALLEE_CREATE: u32 = 1;
        const CALLEE_HASH: u32 = 2;
        const CALLEE_BIND: u32 = 3;
        const CALLEE_APPEND: u32 = 4;
        let reg = *lf_checker_rt::global::<u32>(REGISTRY);
        let rvt = (reg as *const u32).read_unaligned();
        let rtgt = ((rvt + 4) as *const u32).read_unaligned();
        let create: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rtgt as usize);
        let name = arg.wrapping_add(4);
        let pass = create(reg, name, 0, 0);
        if pass == 0 {
            return 0;
        }
        let strp = arg.wrapping_add(0x44);
        let mut len = 0u32;
        while ((strp + len) as *const u8).read_unaligned() != 0 {
            len += 1;
        }
        let mut entry = 0u32;
        if len != 0 {
            let hash: u32 = lf_checker_rt::callee_thiscall!(CALLEE_HASH, u32, strp);
            if hash != 0xFFFFFFFF {
                let table = *lf_checker_rt::global::<u32>(TABLE);
                entry = table.wrapping_add(hash.wrapping_mul(7).wrapping_mul(4));
            }
        }
        let pvt = (pass as *const u32).read_unaligned();
        let btgt = ((pvt + 0x0c) as *const u32).read_unaligned();
        let bind: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(btgt as usize);
        let bans = bind(pass, name, 0, 0, entry, 0, 0, 0, 0);
        if bans as u8 == 0 {
            return 0;
        }
        let slot: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_APPEND, u32, this.wrapping_add(0x1c), 0);
        (slot as *mut u32).write_unaligned(pass);
        pass
    }
});
