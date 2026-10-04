// original: 0x00be2190 CTaskComplexUseEffect::vf20 (symbols)

/// Refresh this task's effect use, notifying on a kind change.
///
/// Without a current effect handle (`this + HANDLE_OFF`, 0x1c, null) this
/// delegates at once to the fallback operation in this object's vtable slot
/// `FALLBACK_SLOT` (0x4c) with the incoming argument, returning its answer.
/// Otherwise the resolve callee maps the handle and argument to an effect
/// object; a null answer, or a null owner (`this + OWNER_OFF`, 0x8), returns
/// the answer as is.
///
/// With both present, each is asked its kind through its third virtual
/// (`KIND_SLOT`, 0x0c): matching kinds return the effect unchanged, while a
/// mismatch runs the notify callee with (argument, 1, 0) and the owner as
/// object (its answer discarded) before returning the effect.
///
/// Original: 0x00be2190 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be2190(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x1c;
        const OWNER_OFF: u32 = 0x08;
        const KIND_SLOT: u32 = 0x0c;
        const FALLBACK_SLOT: u32 = 0x4c;
        const RESOLVE: u32 = 1;
        const NOTIFY: u32 = 4;
        // The kind queries and the fallback run through their objects' own
        // vtables (stub ids 2, 3 and 5 in the contract), exactly like the
        // original: no ctable use for them.
        let handle = (this.wrapping_add(HANDLE_OFF) as *const u32).read_unaligned();
        if handle == 0 {
            let vtable = (this as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(FALLBACK_SLOT) as *const u32).read_unaligned();
            let fallback: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            return fallback(this, arg);
        }
        let effect = lf_checker_rt::callee_thiscall!(RESOLVE, u32, handle, arg);
        if effect == 0 {
            return 0;
        }
        let owner = (this.wrapping_add(OWNER_OFF) as *const u32).read_unaligned();
        if owner == 0 {
            return effect;
        }
        let evtable = (effect as *const u32).read_unaligned();
        let eslot = (evtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let ekind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(eslot as usize);
        let ovtable = (owner as *const u32).read_unaligned();
        let oslot = (ovtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let okind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(oslot as usize);
        if ekind(effect) == okind(owner) {
            return effect;
        }
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, owner, arg, 1, 0);
        effect
    }
});
