// original: 0x00b4feb0 ped_spawn_init (proposed)

/// Initialise a freshly spawned ped from its definition and model choice.
///
/// `this` is the ped, `a` the definition index and `b` the model choice.
/// The definition is registered (callee 1, thiscall on `this` with `a`)
/// while flag bit 5 at `+0x24` is set, then validated (callee 2, thiscall).
/// When the active byte at `+0x11C` is set and validation passed, two
/// blend pairs are attached: each resolves its driver twice through virtual
/// slot `+0xD0` and registers the pair (callees 4 and 5, cdecl with the
/// ped, the second driver adjusted by `+0x80` and the first). The skeleton
/// is then built (callees 6, 7 with the dereferenced word at `+0x34` or 0,
/// 8 with a zero kind, 9 thiscall on the block at `+0x350` with the ped).
/// The model id is `b` unless `b` is -1, when it is read from the table at
/// `MODEL_TABLE` indexed by `a` (word at `+0x7C` of that row): when the
/// override byte (`OVERRIDE`) is set and `b` is -1 the id is first offered
/// to the override hook (callee 10, cdecl; its answer is stored but then
/// replaced), then always resolved (callee 11, cdecl) and stored at
/// `+0x124`. Returns the resolver's answer.
///
/// Original: 0x00b4feb0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b4feb0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const REGISTER: u32 = 1;
        const VALIDATE: u32 = 2;
        const DRIVER_SLOT: u32 = 0xd0;
        const ATTACH_A: u32 = 4;
        const ATTACH_B: u32 = 5;
        const BUILD: u32 = 6;
        const LINK: u32 = 7;
        const KIND: u32 = 8;
        const PLACE: u32 = 9;
        const OVERRIDE_HOOK: u32 = 10;
        const RESOLVE: u32 = 11;
        const FLAGS: u32 = 0x24;
        const ACTIVE: u32 = 0x11c;
        const RIG: u32 = 0x34;
        const BLOCK: u32 = 0x350;
        const MODEL: u32 = 0x124;
        const MODEL_TABLE: u32 = 0x01295cd8;
        const MODEL_OFF: u32 = 0x7c;
        const OVERRIDE: u32 = 0x012b41c5;
        const NO_CHOICE: u32 = 0xffff_ffff;
        let flags = (this + FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() | 0x20);
        lf_checker_rt::callee_thiscall!(REGISTER, u32, this, a);
        let ok = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this);
        if ((this + ACTIVE) as *const u8).read() != 0 && ok & 0xff != 0 {
            let vt = (this as *const u32).read_unaligned();
            let driver: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + DRIVER_SLOT) as *const u32).read_unaligned() as usize,
            );
            let d1 = driver(this);
            let d2 = driver(this);
            lf_checker_rt::callee_cdecl!(ATTACH_A, u32, this, d2.wrapping_add(0x80), d1);
            let d3 = driver(this);
            let d4 = driver(this);
            lf_checker_rt::callee_cdecl!(ATTACH_B, u32, this, d4.wrapping_add(0x80), d3);
        }
        lf_checker_rt::callee_thiscall!(BUILD, u32, this);
        let rig = ((this + RIG) as *const u32).read_unaligned();
        let link = if rig == 0 {
            0
        } else {
            (rig as *const u32).read_unaligned()
        };
        lf_checker_rt::callee_thiscall!(LINK, u32, this, link);
        lf_checker_rt::callee_thiscall!(KIND, u32, this, 0);
        lf_checker_rt::callee_thiscall!(PLACE, u32, this + BLOCK, this);
        let id = if b == NO_CHOICE {
            let row = ((a.wrapping_mul(4)).wrapping_add(lf_checker_rt::relocated(MODEL_TABLE)))
                as *const u32;
            let tab = row.read_unaligned();
            ((tab + MODEL_OFF) as *const u32).read_unaligned()
        } else {
            b
        };
        if lf_checker_rt::global::<u8>(OVERRIDE).read() != 0 && b == NO_CHOICE {
            let over = lf_checker_rt::callee_cdecl!(OVERRIDE_HOOK, u32, id);
            ((this + MODEL) as *mut u32).write_unaligned(over);
        }
        let out = lf_checker_rt::callee_cdecl!(RESOLVE, u32, id);
        ((this + MODEL) as *mut u32).write_unaligned(id);
        out
    }
});
