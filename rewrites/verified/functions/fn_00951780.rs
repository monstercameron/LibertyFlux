// original: 0x00951780 object_refresh_and_bind
/// Refresh a source object, then bind a freshly resolved one.
///
/// Takes two object pointers and a flag word (only its low byte is
/// read). When the source is non-null and the flag byte is non-zero, a
/// refresh runs: if the dword at +0x20 is zero a scripted reset helper
/// runs and then a scripted link helper joins the source's +0x10
/// block to that dword; a scripted scope helper takes an identity
/// matrix plus the dword, a scripted triple (1, the dword at peer +8,
/// the source) runs, and two virtual calls fire on the source (slot
/// +0x18 with no arguments, slot +0 with argument 1). A global context
/// then resolves through two scripted helpers; a null at either stage
/// returns 0 (or the null). The resolved object runs a scripted touch
/// helper, then a virtual slot +0x38 call with the word at peer +0xc
/// unless the flag byte is zero and the word at +0x2e is not -1. The
/// tail registers the triple (1, peer +8, object), stores that dword
/// at +0x64 and a scripted kind answer at +0x68, sets bit 6 of the
/// dword at +0x24 from whether the byte at peer +0xf is non-zero,
/// sets or clears bit 16 from whether the byte at peer +0xe reaches
/// 2, sets byte 2 at +0x41, runs a scripted hook, and returns the
/// object.
export!(cdecl, rw_00951780(src: u32, peer: u32, flag: u32) -> u32 {
    unsafe {
        const G_CTX: u32 = 0x12BD0E8;
        const ID_RESET: u32 = 0;
        const ID_LINK: u32 = 1;
        const ID_SCOPE: u32 = 2;
        const ID_TRIPLE: u32 = 3;
        const ID_V18: u32 = 4;
        const ID_V0: u32 = 5;
        const ID_RESOLVE: u32 = 6;
        const ID_OPEN: u32 = 7;
        const ID_TOUCH: u32 = 8;
        const ID_V38: u32 = 9;
        const ID_REG: u32 = 10;
        const ID_V4: u32 = 11;
        const ID_KIND: u32 = 12;
        const ID_HOOK: u32 = 13;
        const ONE_BITS: u32 = 0x3F800000;

        // Identity matrix the original builds on its frame: three
        // 4-word rows (the fourth word of each row is an uninitialised
        // pad slot, zero under the contract's stack fill). Both
        // matrix-taking helpers receive a pointer to it.
        let mat = [
            ONE_BITS, 0, 0, 0,
            0, ONE_BITS, 0, 0,
            0, 0, ONE_BITS, 0,
        ];
        let pmat = mat.as_ptr() as u32;
        let flag_b = (flag & 0xFF) as u8;
        if src != 0 && flag_b != 0 {
            let slot = *((src.wrapping_add(0x20)) as *const u32);
            if slot == 0 {
                let _: u32 = callee_thiscall!(ID_RESET, u32, src);
                let slot_now = *((src.wrapping_add(0x20)) as *const u32);
                let _: u32 =
                    callee_thiscall!(ID_LINK, u32, src.wrapping_add(0x10), slot_now);
            }
            let slot_again = *((src.wrapping_add(0x20)) as *const u32);
            let _: u32 = callee_thiscall!(ID_SCOPE, u32, pmat, slot_again);
            let peer8 = *((peer.wrapping_add(8)) as *const u32);
            let _: u32 = callee_cdecl!(ID_TRIPLE, u32, 1, peer8, src);
            let vt = *(src as *const u32);
            let f18 = *((vt.wrapping_add(0x18)) as *const u32);
            let c18: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(f18 as usize);
            let _: u32 = c18(src);
            let vt2 = *(src as *const u32);
            let f0 = *(vt2 as *const u32);
            let c0: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(f0 as usize);
            let _: u32 = c0(src, 1);
        }
        let ctx = *(global::<u32>(G_CTX));
        let resolved: u32 = callee_thiscall!(ID_RESOLVE, u32, ctx);
        if resolved == 0 {
            return 0;
        }
        let opened: u32 = callee_thiscall!(ID_OPEN, u32, resolved);
        if opened == 0 {
            return opened;
        }
        let _: u32 = callee_thiscall!(ID_TOUCH, u32, opened);
        let do_v38 = if flag_b != 0 {
            true
        } else {
            core::ptr::read_unaligned((opened.wrapping_add(0x2E)) as *const u16)
                == 0xFFFF
        };
        if do_v38 {
            let wv = core::ptr::read_unaligned((peer.wrapping_add(0xC)) as *const u16)
                as u32;
            let vt = *(opened as *const u32);
            let f38 = *((vt.wrapping_add(0x38)) as *const u32);
            let c38: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(f38 as usize);
            let _: u32 = c38(opened, wv);
        }
        let peer8 = *((peer.wrapping_add(8)) as *const u32);
        let _: u32 = callee_cdecl!(ID_REG, u32, 1, peer8, opened);
        let vt = *(opened as *const u32);
        let f4 = *((vt.wrapping_add(4)) as *const u32);
        let c4: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(f4 as usize);
        let _: u32 = c4(opened, pmat, 0, 0);
        *((opened.wrapping_add(0x64)) as *mut u32) = peer8;
        let kind: u32 = callee_cdecl!(ID_KIND, u32, peer8);
        *((opened.wrapping_add(0x68)) as *mut u32) = kind;
        let fb = *((peer.wrapping_add(0xF)) as *const u8);
        let bit6 = if fb != 0 { 0x40u32 } else { 0 };
        let mut w = *((opened.wrapping_add(0x24)) as *const u32);
        w ^= (w ^ bit6) & 0x40;
        let eb = *((peer.wrapping_add(0xE)) as *const u8);
        if eb < 2 {
            w &= 0xFFFEFFFF;
        } else {
            w |= 0x10000;
        }
        *((opened.wrapping_add(0x24)) as *mut u32) = w;
        *((opened.wrapping_add(0x41)) as *mut u8) = 2;
        let _: u32 = callee_cdecl!(ID_HOOK, u32, opened, 0);
        opened
    }
});
