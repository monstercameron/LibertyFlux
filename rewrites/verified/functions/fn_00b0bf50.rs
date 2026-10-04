// original: 0x00b0bf50 net_slot_init_row (proposed)

/// Initialise one row of the network slot table from the given fields.
///
/// `index` selects the row: rows are `ROW_STRIDE` bytes wide starting at the
/// table base. Each row holds an id word (`+0x42`), a key word (`+0x40`), two
/// coordinate triples, several flag bytes and an owner word (`+0x04`).
///
/// Behaviour: when the row is already active (flag byte `+0x44` non-zero) and
/// its id word already equals `id`, nothing happens. Otherwise the row is
/// reset through two helper calls, the flag cleared, and every field filled
/// in: id, key, the two triples from `pos_a`/`pos_b`, the activation flag
/// from `flags0`, the merged flag byte `+0x45` (low bit from the prepare
/// callee's answer, bit 3 from `flags28`, shifted up three, over the kept
/// bits of the old value), the owner word and the merged bit 0 of `+0x46`.
/// A non-zero owner is registered with the owner callee. Rows activated with
/// the special flag value go through the keyed-lookup callee pair; an extra
/// setup block runs when `extra` is non-zero (five-argument open call, then
/// one two-argument and three three-argument configuration calls).
///
/// Original: 0x00b0bf50 (cdecl, twelve stack words).
export!(cdecl, rw_00b0bf50(flags0: u32, index: u32, id: u32, pos_a: u32, pos_b: u32, key: u32, commit: u32, alt_owner: u32, flags28: u32, flags2c: u32, owner: u32, extra: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x1615660;
        const ROW_STRIDE: u32 = 80;
        const OFF_OWNER: u32 = 0x04;
        const OFF_HANDLE: u32 = 0x10;
        const OFF_POS_A: u32 = 0x18;
        const OFF_POS_B: u32 = 0x30;
        const OFF_ALT: u32 = 0x0c;
        const OFF_KEY: u32 = 0x40;
        const OFF_ID: u32 = 0x42;
        const OFF_ACTIVE: u32 = 0x44;
        const OFF_MERGE: u32 = 0x45;
        const OFF_BIT: u32 = 0x46;
        const SPECIAL_FLAG: u8 = 0x17;
        const MERGE_KEEP: u8 = 0xb7;
        const LOOKUP_FMT: u32 = 0xeaa924;
        const C_RESET: u32 = 1;
        const C_CLEAR: u32 = 2;
        const C_COMMIT: u32 = 3;
        const C_PREPARE: u32 = 4;
        const C_OWNER: u32 = 5;
        const C_FINISH: u32 = 6;
        const C_LOOKUP: u32 = 7;
        const C_EMIT: u32 = 8;
        const C_NOTIFY: u32 = 9;
        const C_OPEN: u32 = 10;
        const C_CONFIG: u32 = 11;
        const C_PAIR: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let row = lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(ROW_STRIDE));
        if ((row + OFF_ACTIVE) as *const u8).read() != 0 {
            let cur = ((row + OFF_ID) as *const u16).read_unaligned() as u32;
            if cur == id {
                return 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(C_RESET, u32, row);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_CLEAR, u32, row, 0);
        let m = (row + OFF_MERGE) as *mut u8;
        m.write(m.read() & 0xfe);
        ((row + OFF_ID) as *mut u16).write_unaligned(id as u16);
        ((row + OFF_KEY) as *mut u16).write_unaligned(key as u16);
        wr32(row + OFF_POS_A, rd32(pos_a));
        wr32(row + OFF_POS_A + 4, rd32(pos_a + 4));
        wr32(row + OFF_POS_A + 8, rd32(pos_a + 8));
        wr32(row + OFF_POS_B, rd32(pos_b));
        wr32(row + OFF_POS_B + 4, rd32(pos_b + 4));
        wr32(row + OFF_POS_B + 8, rd32(pos_b + 8));
        wr32(row + OFF_POS_B + 12, rd32(pos_b + 12));
        let _: u32 = lf_checker_rt::callee_thiscall!(C_COMMIT, u32, row, commit);
        ((row + OFF_ACTIVE) as *mut u8).write(flags0 as u8);
        let prep: u32 = lf_checker_rt::callee_thiscall!(C_PREPARE, u32, row);
        let acc = ((prep & 1) | ((flags28 & 1) << 3)) << 3;
        let kept = ((row + OFF_MERGE) as *const u8).read() & MERGE_KEEP;
        ((row + OFF_MERGE) as *mut u8).write((acc as u8) | kept);
        wr32(row + OFF_OWNER, owner);
        let b = (row + OFF_BIT) as *mut u8;
        b.write(b.read() ^ ((b.read() ^ (flags2c as u8)) & 1));
        if owner != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(C_OWNER, u32, row + OFF_OWNER);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_FINISH, u32, row);
        if ((row + OFF_ACTIVE) as *const u8).read() == SPECIAL_FLAG {
            let mut slot: u32 = 0;
            let found: u32 = lf_checker_rt::callee_thiscall!(
                C_LOOKUP, u32, row, (&mut slot as *mut u32) as u32);
            let h: u32 = lf_checker_rt::callee_cdecl!(
                C_EMIT, u32, found, lf_checker_rt::relocated(LOOKUP_FMT), index);
            wr32(row + OFF_HANDLE, h);
        }
        if alt_owner != 0 {
            wr32(row + OFF_ALT, alt_owner);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(C_NOTIFY, u32, index);
        if (extra as u8) != 0 {
            let h: u32 = lf_checker_rt::callee_cdecl!(C_OPEN, u32, 1, 6, index, 4, 0);
            wr32(row + OFF_HANDLE, h);
            let g1 = (lf_checker_rt::global::<u32>(0x1615634) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(C_CONFIG, u32, 3, h, g1);
            let f = (lf_checker_rt::global::<u32>(0x10400cc) as *const u32).read_unaligned();
            core::hint::black_box(f);
            let g2 = (lf_checker_rt::global::<u32>(0x161562c) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(C_PAIR, u32, 0, h);
            let _: u32 = lf_checker_rt::callee_cdecl!(C_CONFIG, u32, 1, h, g2);
            let g3 = (lf_checker_rt::global::<u32>(0x1615638) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(C_CONFIG, u32, 5, h, g3);
            let g4 = (lf_checker_rt::global::<u32>(0x1615630) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(C_CONFIG, u32, 0x0f, h, g4);
            let ob = (row + OFF_BIT) as *mut u8;
            ob.write(ob.read() | 4);
        }
        0
    }
});
