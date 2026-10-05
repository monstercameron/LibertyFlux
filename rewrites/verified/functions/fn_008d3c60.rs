// original: 0x008D3C60 submit_objects_init
/// Creates the shared submit objects and publishes them to globals.
///
/// Builds a 16-byte descriptor through a construct/configure/finish
/// call chain, then resolves the factory object through a global,
/// creates the two shared submit objects through its vtable, wires per
/// object state (a linked slot, two probed capacities, a parent link),
/// optionally notifies a global listener around four numbered channel
/// lookups, and finishes three named registrations. Global slots that
/// read null skip their listener call. Returns 1 in the low byte.
lf_checker_rt::export!(cdecl, rb109_fn7() -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_NEW: u32 = 1; // descriptor construct (thiscall/0, fills it)
    const CAL_CFG_A: u32 = 2; // descriptor step A (thiscall/3: 1, 6, 0)
    const CAL_CFG_B: u32 = 3; // descriptor step B (thiscall/2: 1, 9)
    const CAL_FIN: u32 = 4; // descriptor finish (thiscall/3: 0, 1, 5)
    const CAL_OPEN: u32 = 5; // open (cdecl/5: descriptor, 0, 0, 0, 0)
    const CAL_MAP: u32 = 6; // map (cdecl/4: 0x600, descriptor, 0, 0)
    const CAL_MKOBJ: u32 = 7; // make primary object, vtable +0x4 (thiscall/3)
    const CAL_MKEP: u32 = 8; // make entry object, vtable +0x8 (thiscall/0)
    const CAL_CFG9: u32 = 9; // configure primary, vtable +0xC (thiscall/8)
    const CAL_LINK: u32 = 10; // link slot (thiscall/1: 1)
    const CAL_PROBE: u32 = 11; // capacity probe (cdecl/1: 4)
    const CAL_PARENT: u32 = 12; // set parent (thiscall/1: primary)
    const CAL_NOTIFY: u32 = 13; // listener notify (cdecl/2, via global)
    const CAL_CHAN: u32 = 14; // channel lookup (thiscall/2)
    const CAL_REG: u32 = 15; // named registration (thiscall/2)

    // Globals (file VAs; resolved through the worker's image base).
    const G_FACTORY: u32 = 0x017F5A04; // factory object (fabricated)
    const G_LISTENER: u32 = 0x017F597C; // listener func (stub addr or 0)
    const G_PRIMARY: u32 = 0x011736C8; // primary object slot
    const G_ENTRY: u32 = 0x011736C4; // entry object slot
    const G_HANDLE: u32 = 0x011736EC; // open handle slot
    const G_MAPPED: u32 = 0x011736F0; // mapped handle slot
    const G_CH0: u32 = 0x011736D8; // channel slots
    const G_CH1: u32 = 0x011736CC;
    const G_CH2: u32 = 0x011736D0;
    const G_CH3: u32 = 0x011736D4;
    const G_REG0: u32 = 0x011736DC; // registration slots
    const G_REG1: u32 = 0x011736E0;
    const G_REG2: u32 = 0x011736E4;

    // Data keys (file VAs; all relocated immediates in the original).
    const K_OBJ: u32 = 0x00E80F04;
    const K_CFG: u32 = 0x00E80F0C;
    const K_CH0: u32 = 0x00E80F14;
    const K_CH1: u32 = 0x00E80F20;
    const K_CH2: u32 = 0x00E80F2C;
    const K_CH3: u32 = 0x00E80F3C;
    const K_RG0: u32 = 0x00E80F4C;
    const K_RG1: u32 = 0x00E80F58;
    const K_RG2: u32 = 0x00E80F68;

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }

    /// Call a planted vtable slot exactly like the original: load the
    /// slot and call through it. Both sides land on the same stub.
    #[inline(always)]
    unsafe fn vcall3(object: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, a0, a1, a2)
    }
    #[inline(always)]
    unsafe fn vcall0(object: u32, slot: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object)
    }
    #[inline(always)]
    unsafe fn vcall8(
        object: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32,
        a4: u32, a5: u32, a6: u32, a7: u32,
    ) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, a0, a1, a2, a3, a4, a5, a6, a7)
    }

    unsafe {
        let r = lf_checker_rt::relocated;
        let mut desc = [0u32; 4];
        let dp = desc.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_NEW, u32, dp);
        lf_checker_rt::callee_thiscall!(CAL_CFG_A, u32, dp, 1, 6, 0);
        lf_checker_rt::callee_thiscall!(CAL_CFG_B, u32, dp, 1, 9);
        lf_checker_rt::callee_thiscall!(CAL_FIN, u32, dp, 0, 1, 5);
        let h = lf_checker_rt::callee_cdecl!(CAL_OPEN, u32, dp, 0, 0, 0, 0);
        *(lf_checker_rt::global::<u32>(G_HANDLE)) = h;
        let m = lf_checker_rt::callee_cdecl!(CAL_MAP, u32, 0x600, dp, 0, 0);
        *(lf_checker_rt::global::<u32>(G_MAPPED)) = m;
        let factory = *(lf_checker_rt::global::<u32>(G_FACTORY));
        let primary = vcall3(factory, 0x4, r(K_OBJ), 0, 0);
        *(lf_checker_rt::global::<u32>(G_PRIMARY)) = primary;
        let entry = vcall0(factory, 0x8);
        *(lf_checker_rt::global::<u32>(G_ENTRY)) = entry;
        vcall8(primary, 0xC, r(K_CFG), 0, 0, 0, 0, 0, 0, 0);
        let slot = lf_checker_rt::callee_thiscall!(CAL_LINK, u32, entry.wrapping_add(8), 1);
        *((entry.wrapping_add(8)) as *mut u32) = slot;
        *((entry.wrapping_add(0xE)) as *mut u16) = 1;
        let c0 = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32, 4);
        *((entry.wrapping_add(0x40)) as *mut u32) = c0;
        *((entry.wrapping_add(0x46)) as *mut u16) = 1;
        let c1 = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32, 4);
        *((entry.wrapping_add(0x48)) as *mut u32) = c1;
        *((entry.wrapping_add(0x4E)) as *mut u16) = 1;
        lf_checker_rt::callee_thiscall!(CAL_PARENT, u32, entry, primary);
        // Listener + channel lookups. The listener address is re-read per
        // channel; a null slot skips its call. The fourth lookup takes the
        // last answer, or the channel key itself when the call was skipped
        // (the key is still sitting in ECX on that path).
        let listener = *(lf_checker_rt::global::<u32>(G_LISTENER));
        // Each channel lookup takes the notify answer, or the channel key
        // itself when the notify was skipped (the key is still in EAX/ECX
        // on that path).
        let a0 = if listener != 0 {
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, primary, r(K_CH0))
        } else {
            r(K_CH0)
        };
        let g0 = lf_checker_rt::callee_thiscall!(CAL_CHAN, u32, load(primary, 0x18), a0, 1);
        *(lf_checker_rt::global::<u32>(G_CH0)) = g0;
        let a1 = if listener != 0 {
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, primary, r(K_CH1))
        } else {
            r(K_CH1)
        };
        let g1 = lf_checker_rt::callee_thiscall!(CAL_CHAN, u32, load(primary, 0x18), a1, 1);
        *(lf_checker_rt::global::<u32>(G_CH1)) = g1;
        let a2 = if listener != 0 {
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, primary, r(K_CH2))
        } else {
            r(K_CH2)
        };
        let g2 = lf_checker_rt::callee_thiscall!(CAL_CHAN, u32, load(primary, 0x18), a2, 1);
        *(lf_checker_rt::global::<u32>(G_CH2)) = g2;
        let tag = if listener != 0 {
            lf_checker_rt::callee_cdecl!(CAL_NOTIFY, u32, primary, r(K_CH3))
        } else {
            r(K_CH3)
        };
        let g3 = lf_checker_rt::callee_thiscall!(CAL_CHAN, u32, load(primary, 0x18), tag, 1);
        *(lf_checker_rt::global::<u32>(G_CH3)) = g3;
        let r0 = lf_checker_rt::callee_thiscall!(CAL_REG, u32, entry, r(K_RG0), 1);
        *(lf_checker_rt::global::<u32>(G_REG0)) = r0;
        let r1 = lf_checker_rt::callee_thiscall!(CAL_REG, u32, entry, r(K_RG1), 1);
        *(lf_checker_rt::global::<u32>(G_REG1)) = r1;
        let r2 = lf_checker_rt::callee_thiscall!(CAL_REG, u32, entry, r(K_RG2), 1);
        *(lf_checker_rt::global::<u32>(G_REG2)) = r2;
        // `(an instruction of the original)` with bl=1: only the low byte is set.
        (r2 & 0xFFFF_FF00) | 1
    }
});

