// original: 0x00901830 BLIP_DEST

/// Configure the destination-blip slot for a resolved blip index.
///
/// `arg1` carries the request code (low byte gates the setup block),
/// `arg2` is the destination parameter (stored into the slot and selecting
/// both switch arms), `arg3` is the owner word (low word stored as the slot
/// code, full word as the owner), `arg4` is forwarded to callee 4, `arg5`
/// is stored as the slot aux value, and the sixth word is unread. The slot
/// table at `0x118F6F8` is indexed by the resolved index; each slot holds an
/// active byte at `+8`, flags at `+0x20`, and the configured fields.
///
/// Behaviour: resolve the index through callee 1 (`arg1`, 1); -1 returns -1.
/// When the low byte of `arg1` is set, set flag bit 0x40 on the flags word
/// of the resolved slot (or, when that slot is inactive, the slot selected
/// by the global at `0x1034494`), issue the eight flag calls of callee 2
/// with (index, 2/4/0x100/0x200/0x400/8/0x10/0x80), and, while the resolved
/// slot stays active, write mode 2 at `+0x44`, count 0 at `+0x40`, the
/// parameter at `+0x48`, scale 1.0 at `+0x50`, alpha 0xff at `+0x58`, the
/// link word from table `0x118F4F0` (selected by the global at `0x1034490`)
/// at `+0x5c`, and `arg5` at `+0x4c`. The slot code (low word of `arg3`) is
/// always stored at `+2`. The first switch maps `arg2 - 1` (0..7) to one of
/// eight callee-3 constants (`0xE84BB8` … `0xE84C0C`, table order
/// B8+36/B8+48/B8+24/B8+0/B8+12/B8+60/B8+84/B8+72) with `0xE84C0C` as the
/// default, and calls callee 3 with (index, constant). The second switch
/// maps `arg2 - 3`: arms 1, 2, 4 and 6 call callee 4 with (index, `arg4`)
/// and, when the low byte of `arg1` is set and the slot is active, write
/// kind 0x26 at `+0x54`; arm 5 calls callee 4 then, when the low byte is
/// set, callees 5 and 6 with (index, 0x26/0x80); arms 0 and 3 and every other
/// value write kind 0x29 (arm 0) or 0x2a at `+0x54` while active and then,
/// while still active, store the owner (`arg1`) at `+0x24`. Finally callee 7
/// is called with the index, its answer is stored as the handle at `+4`,
/// state at `+0xC` is cleared, and the handle is returned.
///
/// Original: 0x00901830 (cdecl, six stack words, the sixth unread; returns eax).
lf_checker_rt::export!(cdecl, rw_00901830(arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32, _arg6: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x0118_f6f8;
        const LINKS: u32 = 0x0118_f4f0;
        const ALT_SEL: u32 = 0x0103_4494;
        const LINK_SEL: u32 = 0x0103_4490;
        const ACTIVE: u32 = 0x08;
        const FLAGS: u32 = 0x20;
        const FLAG_BIT: u16 = 0x40;
        const CODE: u32 = 0x02;
        const HANDLE: u32 = 0x04;
        const STATE: u32 = 0x0c;
        const OWNER: u32 = 0x24;
        const COUNT: u32 = 0x40;
        const MODE: u32 = 0x44;
        const PARAM: u32 = 0x48;
        const AUX: u32 = 0x4c;
        const SCALE: u32 = 0x50;
        const KIND: u32 = 0x54;
        const ALPHA: u32 = 0x58;
        const LINKW: u32 = 0x5c;
        const ONE: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn slot(i: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(SLOTS).wrapping_add(i.wrapping_mul(4))) }
        }

        let esi = lf_checker_rt::callee_cdecl!(1, u32, arg1, 1);
        if esi == 0xffff_ffff {
            return 0xffff_ffff;
        }
        let ebp = arg2;
        if (arg1 & 0xff) != 0 {
            let mut p = slot(esi);
            if rd8(p.wrapping_add(ACTIVE)) == 0 {
                let alt = rd32(lf_checker_rt::relocated(ALT_SEL));
                p = slot(alt);
            }
            wr16(p.wrapping_add(FLAGS), rd16(p.wrapping_add(FLAGS)) | FLAG_BIT);
            for flag in [2u32, 4, 0x100, 0x200, 0x400, 8, 0x10, 0x80] {
                lf_checker_rt::callee_cdecl!(2, u32, esi, flag);
            }
            let mut q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr32(q.wrapping_add(MODE), 2);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr32(q.wrapping_add(COUNT), 0);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr32(q.wrapping_add(PARAM), ebp);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr32(q.wrapping_add(SCALE), ONE);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr8(q.wrapping_add(ALPHA), 0xff);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                let li = rd32(lf_checker_rt::relocated(LINK_SEL));
                let link = rd32(lf_checker_rt::relocated(LINKS).wrapping_add(li.wrapping_mul(4)));
                wr32(q.wrapping_add(LINKW), link);
            }
            q = slot(esi);
            if rd8(q.wrapping_add(ACTIVE)) != 0 {
                wr32(q.wrapping_add(AUX), arg5);
            }
        }
        wr16(slot(esi).wrapping_add(CODE), (arg3 & 0xffff) as u16);
        // First switch on arg2 - 1, in jump-table order.
        let c: u32 = match ebp.wrapping_sub(1) {
            0 => lf_checker_rt::relocated(0x00e8_4bdc),
            1 => lf_checker_rt::relocated(0x00e8_4be8),
            2 => lf_checker_rt::relocated(0x00e8_4bd0),
            3 => lf_checker_rt::relocated(0x00e8_4bb8),
            4 => lf_checker_rt::relocated(0x00e8_4bc4),
            5 => lf_checker_rt::relocated(0x00e8_4bf4),
            6 => lf_checker_rt::relocated(0x00e8_4c0c),
            7 => lf_checker_rt::relocated(0x00e8_4c00),
            _ => lf_checker_rt::relocated(0x00e8_4c0c),
        };
        lf_checker_rt::callee_cdecl!(3, u32, esi, c);
        // Second switch on arg2 - 3.
        let j = ebp.wrapping_sub(3);
        let mut store_owner = false;
        if j > 6 {
            let s = slot(esi);
            if rd8(s.wrapping_add(ACTIVE)) != 0 {
                wr32(s.wrapping_add(KIND), 0x2a);
            }
            store_owner = true;
        } else if j == 0 {
            let s = slot(esi);
            if rd8(s.wrapping_add(ACTIVE)) != 0 {
                wr32(s.wrapping_add(KIND), 0x29);
            }
            store_owner = true;
        } else if j == 3 {
            let s = slot(esi);
            if rd8(s.wrapping_add(ACTIVE)) != 0 {
                wr32(s.wrapping_add(KIND), 0x2a);
            }
            store_owner = true;
        } else if j == 5 {
            lf_checker_rt::callee_cdecl!(4, u32, esi, arg4);
            if (arg1 & 0xff) != 0 {
                lf_checker_rt::callee_cdecl!(5, u32, esi, 0x26);
                lf_checker_rt::callee_cdecl!(6, u32, esi, 0x80);
            }
        } else {
            lf_checker_rt::callee_cdecl!(4, u32, esi, arg4);
            if (arg1 & 0xff) != 0 {
                let s = slot(esi);
                if rd8(s.wrapping_add(ACTIVE)) != 0 {
                    wr32(s.wrapping_add(KIND), 0x26);
                }
            }
        }
        if store_owner {
            let s = slot(esi);
            if rd8(s.wrapping_add(ACTIVE)) != 0 {
                wr32(s.wrapping_add(OWNER), arg3);
            }
        }
        let v = lf_checker_rt::callee_cdecl!(7, u32, esi);
        wr32(slot(esi).wrapping_add(HANDLE), v);
        wr32(slot(esi).wrapping_add(STATE), 0);
        rd32(slot(esi).wrapping_add(HANDLE))
    }
});
