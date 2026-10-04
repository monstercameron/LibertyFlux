// original: 0x00C67210 cutscene_player_assemble (proposed)
//
// thiscall (ecx = this, no stack arguments). Builds the actor's player
// rig: allocates four parts through callee 1 (cdecl, sizes 0x74/0x90/0xa8/
// 0x40), constructs the non-null ones through callees 2/3/4/8 (thiscall,
// no arguments) and stores the results at +0x294/+0x298/+0x29c/+0x290,
// links them through callees 5 (thiscall, one argument, callee-popped),
// 6 and 7 (cdecl, three arguments each), attaches part 4 through callee 9
// (thiscall, this), then resolves a live handle through the hook pair at
// object slots +0xa0/+0xe0 (falling back to +0x100, running callee 12 when
// null) and sets bit 1 of the mode byte at +0xf4.
//
// When gate callee 13 answers nonzero, a work block runs: the u16 key at
// +0x2e (sign-extended) selects a row of pointer table T (0x01295CD8)
// whose word at +0x58 (sign-extended) is repaired through callee 14 when
// -1; when the word at +0xd4 is zero, callee 15 runs on a fixed object and
// predicate callee 16 decides a three-callee sequence (17, 18, 19); then a
// lookup pair (20, 21) feeds callee 22 unless it answers null. When the
// gate answers zero, callee 22 takes the pointer at [this+0x34] (or null)
// instead. Callee 23 always runs next.
//
// When gate 13 answers zero a second time, a tail block runs: an 8-byte
// frame object is set up through callee 24, hook 25 (slot +0x34 of the
// table row) answers an index into the flag bytes of the directory at
// global 0x015F8BA4 (a set 0x80 bit selects address 0 and faults on the
// read below, on both sides alike), otherwise stride*index+base selects a
// record whose word at +0xc ends the block when -1 or flows through
// callees 26 (cdecl), 27 (thiscall on the frame object), 25 again, 28
// (cdecl) and 29 (thiscall on a second frame object, two arguments: the
// value and the literal 0 the original pushes before hook 25 and leaves
// for callee 29 to pop) into +0x310, and callee 30 tears the frame object
// down. The two frame addresses are skipped in the call comparison and
// their (untouched, zero-filled) contents are snapshotted instead.
//
// Returns the second gate answer when the tail is skipped, else callee 30's
// answer.
lf_checker_rt::export!(thiscall, rw_00C67210(this: u32) -> u32 {
    unsafe {
        const PART0: u32 = 0x294;
        const PART1: u32 = 0x298;
        const PART2: u32 = 0x29c;
        const PART3: u32 = 0x290;
        const STAMP: u32 = 0x314;
        const LINK: u32 = 0x34;
        const STORED: u32 = 0x100;
        const MODE: u32 = 0xf4;
        const MODE_BIT: u8 = 2;
        const KEY: u32 = 0x2e;
        const ROW_WORD: u32 = 0x58;
        const READY: u32 = 0xd4;
        const PICK: u32 = 0x310;
        const PTRS: u32 = 0x01295CD8;
        const DIR: u32 = 0x015F8BA4;
        const FIXED_OBJ: u32 = 0x011737D0; // file VA, relocated below
        const REPAIR_ARG: u32 = 0x00ECB92C; // file VA, relocated below
        const SEQ_ARG: u32 = 0x00ECB934; // file VA, relocated below
        const FLAG_SKIP: u8 = 0x80;
        const SLOT_WORD: u32 = 0xc;
        const NEW: u32 = 1;
        const CTOR0: u32 = 2;
        const CTOR1: u32 = 3;
        const CTOR2: u32 = 4;
        const LINKFIRST: u32 = 5;
        const LINKA: u32 = 6;
        const LINKB: u32 = 7;
        const CTOR3: u32 = 8;
        const ATTACH: u32 = 9;
        const HOOK_B: u32 = 10;
        const HOOK_C: u32 = 11;
        const HOOK_D: u32 = 12;
        const GATE: u32 = 13;
        const REPAIR: u32 = 14;
        const PRIME: u32 = 15;
        const PRED: u32 = 16;
        const SEQ0: u32 = 17;
        const SEQ1: u32 = 18;
        const SEQ2: u32 = 19;
        const LOOKUP: u32 = 20;
        const RESOLVE: u32 = 21;
        const FEED: u32 = 22;
        const COMMIT: u32 = 23;
        const FRAME_CTOR: u32 = 24;
        const ROWHOOK: u32 = 25;
        const MAP: u32 = 26;
        const USE: u32 = 27;
        const CONVERT: u32 = 28;
        const FINISH: u32 = 29;
        const FRAME_DTOR: u32 = 30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn sx16(a: u32) -> u32 {
            unsafe { (a as *const i16).read_unaligned() as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(this + STAMP, 0);
        let m0 = lf_checker_rt::callee_cdecl!(NEW, u32, 0x74);
        let p0 = if m0 != 0 {
            lf_checker_rt::callee_thiscall!(CTOR0, u32, m0)
        } else {
            0
        };
        wr32(this + PART0, p0);
        let m1 = lf_checker_rt::callee_cdecl!(NEW, u32, 0x90);
        let p1 = if m1 != 0 {
            lf_checker_rt::callee_thiscall!(CTOR1, u32, m1)
        } else {
            0
        };
        wr32(this + PART1, p1);
        let m2 = lf_checker_rt::callee_cdecl!(NEW, u32, 0xa8);
        let p2 = if m2 != 0 {
            lf_checker_rt::callee_thiscall!(CTOR2, u32, m2)
        } else {
            0
        };
        wr32(this + PART2, p2);
        let link = rd32(this + LINK);
        let first = rd32(rd32(link) + SLOT_WORD);
        lf_checker_rt::callee_thiscall!(LINKFIRST, u32, p2, first);
        lf_checker_rt::callee_cdecl!(LINKA, u32, this, p1, p0);
        lf_checker_rt::callee_cdecl!(LINKB, u32, this, p1, p0);
        let m3 = lf_checker_rt::callee_cdecl!(NEW, u32, 0x40);
        let p3 = if m3 != 0 {
            lf_checker_rt::callee_thiscall!(CTOR3, u32, m3)
        } else {
            0
        };
        wr32(this + PART3, p3);
        lf_checker_rt::callee_thiscall!(ATTACH, u32, p3, this);

        let hook_b: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + 0xa0) as usize);
        let mut handle = hook_b(this);
        if handle == 0 {
            handle = rd32(this + STORED);
        } else {
            let rec = hook_b(this);
            let hook_c: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(rec) + 0xe0) as usize);
            handle = hook_c(rec);
        }
        if handle == 0 {
            lf_checker_rt::callee_thiscall!(HOOK_D, u32, this);
        }
        wr8(this + MODE, rd8(this + MODE) | MODE_BIT);

        let g1 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if (g1 as u8) != 0 {
            let t = rd32(lf_checker_rt::relocated(PTRS).wrapping_add(
                sx16(this + KEY).wrapping_mul(4),
            ));
            let mut edi = sx16(t + ROW_WORD);
            if edi == 0xFFFF_FFFF {
                lf_checker_rt::callee_thiscall!(REPAIR, u32, t, lf_checker_rt::relocated(REPAIR_ARG));
                edi = sx16(t + ROW_WORD);
            }
            if rd32(this + READY) == 0 {
                lf_checker_rt::callee_thiscall!(PRIME, u32, lf_checker_rt::relocated(FIXED_OBJ), 0);
                let c = lf_checker_rt::callee_cdecl!(PRED, u32, edi);
                if (c as u8) == 0 {
                    lf_checker_rt::callee_cdecl!(SEQ0, u32,);
                    lf_checker_rt::callee_cdecl!(SEQ1, u32, edi, lf_checker_rt::relocated(SEQ_ARG));
                    lf_checker_rt::callee_cdecl!(SEQ2, u32,);
                }
            }
            let k = lf_checker_rt::callee_cdecl!(RESOLVE, u32, lf_checker_rt::callee_cdecl!(LOOKUP, u32, p0, 1));
            if k != 0 {
                lf_checker_rt::callee_thiscall!(FEED, u32, this, k);
            }
        } else {
            let e = rd32(this + LINK);
            let a = if e != 0 { rd32(e) } else { 0 };
            lf_checker_rt::callee_thiscall!(FEED, u32, this, a);
        }
        lf_checker_rt::callee_thiscall!(COMMIT, u32, this, 0);

        let g2 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if (g2 as u8) != 0 {
            return g2;
        }
        let row = rd32(lf_checker_rt::relocated(PTRS).wrapping_add(
            sx16(this + KEY).wrapping_mul(4),
        ));
        let mut frame = [0u32; 2];
        let frame_ptr = frame.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(FRAME_CTOR, u32, frame_ptr);
        let rowhook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(row) + 0x34) as usize);
        let b = rowhook(row);
        let dir = rd32(lf_checker_rt::relocated(DIR));
        let flags = rd32(dir + 4);
        let slot = if rd8(flags.wrapping_add(b)) & FLAG_SKIP != 0 {
            0
        } else {
            rd32(dir + SLOT_WORD).wrapping_mul(b).wrapping_add(rd32(dir))
        };
        let w = rd32(slot + SLOT_WORD);
        if w == 0xFFFF_FFFF {
            return lf_checker_rt::callee_thiscall!(FRAME_DTOR, u32, frame_ptr);
        }
        let m = lf_checker_rt::callee_cdecl!(MAP, u32, w);
        lf_checker_rt::callee_thiscall!(USE, u32, frame_ptr, m);
        let n2 = rowhook(row);
        let s = lf_checker_rt::callee_cdecl!(CONVERT, u32, n2);
        let mut frame2 = [0u32; 2];
        let v = lf_checker_rt::callee_thiscall!(FINISH, u32, frame2.as_mut_ptr() as u32, s, 0);
        wr32(this + PICK, v);
        lf_checker_rt::callee_thiscall!(FRAME_DTOR, u32, frame_ptr)
    }
});
