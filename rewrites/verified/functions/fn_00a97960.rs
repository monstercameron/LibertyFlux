// original: 0x00a97960 filemem_find_named_entry

/// Find the indexed entry under a resolved controller and open it.
///
/// `a1` points to the owner, `a2` is the entry index. The resolve callee
/// (vtable slot `+0xa0`) names the target, defaulting to the word at
/// `+0x38` on a null answer; the controller at target `+0x4` dereferenced
/// `+0xc` must be non-null. When the controller's kind byte at `+0x4` is
/// not 0xc, the answer is 1 only for kinds 0xa or 4 with the owner's flag
/// bit 0x4000000 (bit 26) clear. Otherwise the entry at controller
/// `+0x80` indexed by `a2` must be non-null with marker byte 4 at `+0x4`,
/// its level callee (vtable slot `+0x54`) must answer at most 1 as a
/// SIGNED 32-bit value (the original jumps signed: negatives pass), and
/// the open callee (slot `+0x5c`, handed 0) yields a handle the register
/// callee files under the global at `0x01305d30`; a nonzero low byte from
/// the register answers 1. Anything else answers 0.
///
/// Original: 0x00A97960 (stdcall, two stack words; three indirect callees,
/// one direct callee).
lf_checker_rt::export!(stdcall, rw_00a97960(a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Resolve slot in the owner vtable; fallback target, from owner.
        const VT_RESOLVE: u32 = 0xa0;
        const TARGET_OFF: u32 = 0x38;
        /// Controller chain: at target +0x4, dereferenced +0xc.
        const MID_OFF: u32 = 0x4;
        const CTRL_OFF: u32 = 0xc;
        /// Kind byte (at controller +0x4) selecting the path.
        const KIND_OFF: u32 = 0x4;
        const KIND_MAIN: u8 = 0xc;
        const KIND_ALT1: u8 = 0xa;
        const KIND_ALT2: u8 = 4;
        /// Owner flag bit gating the side path.
        const FLAG_OFF: u32 = 0x24;
        const FLAG_BIT: u32 = 0x04000000;
        /// Entry array (at controller +0x80) and marker byte.
        const ARRAY_OFF: u32 = 0x80;
        const MARKER_OFF: u32 = 0x4;
        const MARKER_WANT: u8 = 4;
        /// Level/open slots in the entry vtable; level bound (SIGNED).
        const VT_LEVEL: u32 = 0x54;
        const VT_OPEN: u32 = 0x5c;
        const LEVEL_MAX: i32 = 1;
        /// Global registry the handle is filed under (file VA).
        const REGISTRY: u32 = 0x01305d30;
        const REGISTER: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let slot = rd32(rd32(a1).wrapping_add(VT_RESOLVE));
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut target = resolve(a1);
        if target == 0 {
            target = rd32(a1.wrapping_add(TARGET_OFF));
        }
        let ctrl = rd32(rd32(target.wrapping_add(MID_OFF)).wrapping_add(CTRL_OFF));
        if ctrl == 0 {
            return 0;
        }
        let kind = rd8(ctrl.wrapping_add(KIND_OFF));
        if kind != KIND_MAIN {
            if kind != KIND_ALT1 && kind != KIND_ALT2 {
                return 0;
            }
            if rd32(a1.wrapping_add(FLAG_OFF)) & FLAG_BIT != 0 {
                return 0;
            }
            return 1;
        }
        let ent = rd32(rd32(ctrl.wrapping_add(ARRAY_OFF)).wrapping_add(a2.wrapping_mul(4)));
        if ent == 0 {
            return 0;
        }
        if rd8(ent.wrapping_add(MARKER_OFF)) != MARKER_WANT {
            return 0;
        }
        let lslot = rd32(rd32(ent).wrapping_add(VT_LEVEL));
        let level: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(lslot as usize);
        if (level(ent) as i32) > LEVEL_MAX {
            return 0;
        }
        let oslot = rd32(rd32(ent).wrapping_add(VT_OPEN));
        let open: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(oslot as usize);
        let handle = open(ent, 0);
        let ok: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, lf_checker_rt::relocated(REGISTRY), handle);
        if (ok as u8) == 0 {
            return 0;
        }
        1
    }
});
