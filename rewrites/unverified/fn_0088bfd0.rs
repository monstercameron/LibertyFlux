// original: 0x0088bfd0 rage::audVoiceDSound::vf1
/// Build one voice's sound buffer and wire it to its channel.
///
/// `this` points to the voice object, `arg0` an initial position. After
/// zeroing the buffer fields, an opener helper runs (thiscall), a format
/// block at `+0xec` is registered (three arguments), and the source words
/// at `[+0x14]` are copied (`+0xd0`, `+0xbc`) with a divisor from
/// `[[+0x10]+0x28]` (`+0xe8`). Unless the source count at `+0x14` is `-1`,
/// flag bit 2 is set, the count word at `+0xa0` becomes twice the count,
/// and a scaler helper answers into `+0xb0`.
///
/// A format descriptor is then built on the frame (two flag words, the
/// source rate, doubled rate, constants, the length word `+0xc0`, and a
/// 16-byte id copied from the image) and handed with an out-pointer to the
/// device object (a fixed image word) through its slot at `+0xc`. When flag
/// bit `0x10` is set the tail runs next, else the fresh channel at `+0x90`
/// is locked through slot `+0x2c` (eight arguments, two frame out-words),
/// failures released through slot `+0x8`, and the length drained: an empty
/// count word pushes one sink call (three arguments) plus a conditional
/// fill call, while a set count word divides out a quantum (`+0xb8`), a
/// repeat count (`+0xa8`), a leftover (`+0xb4`) and a quantum count
/// (`+0xc4`), then loops the sink while a 16-bit cursor stays below
/// `+0xa8 minus +0xc4`. The tail unlocks through slot `+0x4c` (five
/// arguments, skipped on the flag-bit path), reports through slot `+0x34`
/// (two arguments), opens through slot `+0x0` (three arguments, one a fixed
/// image address), and posts through slot `+0x3c` with `-10000`, whose
/// answer is returned.
///
/// The fill call's second site never fires: the compared words are always
/// equal there. The length word and the incoming slot it overwrites are
/// observed through the later call arguments (see the contract).
///
/// Original: 0x008bfd0 (thiscall, one stack word; true size 835).
lf_checker_rt::export!(thiscall, rw_0088bfd0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const CHAN_OBJ: u32 = 0x90;
        const CHAN_AUX: u32 = 0x94;
        const FLAG_BYTE: u32 = 0x8c;
        const HAS_COUNT: u8 = 2;
        const QUICK_PATH: u8 = 0x10;
        const COUNT_WORD: u32 = 0xa0;
        const SRC_PTR: u32 = 0x14;
        const FMT_PTR: u32 = 0x10;
        const FMT_DIV: u32 = 0x28;
        const LEN_WORD: u32 = 0xc0;
        const BC_WORD: u32 = 0xbc;
        const QUANT_WORD: u32 = 0xb8;
        const REP_WORD: u32 = 0xa8;
        const LEFT_WORD: u32 = 0xb4;
        const QCOUNT_WORD: u32 = 0xc4;
        const D0_WORD: u32 = 0xd0;
        const MODE_BYTE: u32 = 0x9c;
        const MIN_RATE: u32 = 0x5dc0;
        const QUICK_LEN: u32 = 0x20000;
        const POST_CODE: u32 = 0xffff_d8f0;
        const CAL_OPEN: u32 = 1;
        const CAL_REG: u32 = 2;
        const CAL_SCALE: u32 = 3;
        const CAL_SINK: u32 = 7;
        // Device slot +0xc is id 4; channel slots +0x2c/+0x8/+0x4c/+0x34/
        // +0x0/+0x3c are ids 5/6/8/9/10/11, reached through the planted
        // tables exactly like the original.
        const DEV_WORD: u32 = 0x0115_a444;
        const ID_Q1_LO: u32 = 0x00f1_40e4;
        const ID_Q1_HI: u32 = 0x00f1_40e8;
        const ID_Q2_LO: u32 = 0x00f1_40ec;
        const ID_Q2_HI: u32 = 0x00f1_40f0;
        const OPEN_BLOB: u32 = 0x00f1_3cc0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }

        wr32(this.wrapping_add(CHAN_OBJ), 0);
        wr32(this.wrapping_add(CHAN_AUX), 0);
        lf_checker_rt::callee_thiscall!(CAL_OPEN, u32, this, arg0);
        let fb = this.wrapping_add(FLAG_BYTE);
        (fb as *mut u8).write((fb as *const u8).read() & !HAS_COUNT);
        lf_checker_rt::callee_cdecl!(CAL_REG, u32, this.wrapping_add(0xec), 0u32, 0x80u32);
        for off in [0xa0u32, 0xa4, 0xa8, 0xd8, 0xdc, 0xe0, 0xe4, 0xc8, 0xcc] {
            wr32(this.wrapping_add(off), 0);
        }
        (this.wrapping_add(0x9c) as *mut u16).write_unaligned(0);
        let src = rd32(this.wrapping_add(SRC_PTR));
        wr32(this.wrapping_add(D0_WORD), rd32(src));
        wr32(this.wrapping_add(BC_WORD), rd32(src.wrapping_add(0xc)));
        let fmt = rd32(this.wrapping_add(FMT_PTR));
        wr32(this.wrapping_add(0xe8), rd32(fmt.wrapping_add(FMT_DIV)));
        let scount = rd32(src.wrapping_add(0x14));
        if scount != 0xffff_ffff {
            (fb as *mut u8).write((fb as *const u8).read() | HAS_COUNT);
            wr32(this.wrapping_add(COUNT_WORD), scount.wrapping_add(scount));
            let srate = rd16(src.wrapping_add(0x18)) as u32;
            let scaled = lf_checker_rt::callee_cdecl!(CAL_SCALE, u32, scount, srate);
            wr32(this.wrapping_add(0xb0), scaled);
            wr32(this.wrapping_add(0xa4), rd32(this.wrapping_add(BC_WORD)));
        }
        let flags = (fb as *const u8).read();
        let c0val = if flags & QUICK_PATH != 0 {
            wr32(this.wrapping_add(BC_WORD), QUICK_LEN);
            wr32(this.wrapping_add(LEN_WORD), QUICK_LEN);
            QUICK_LEN
        } else if flags & HAS_COUNT != 0 && rd32(this.wrapping_add(COUNT_WORD)) != 0 {
            let a4 = rd32(this.wrapping_add(0xa4));
            let a0 = rd32(this.wrapping_add(COUNT_WORD));
            let ecx = a4.wrapping_sub(a0);
            let num = if a4 < MIN_RATE { MIN_RATE } else { a4 };
            let q = num / ecx;
            wr32(this.wrapping_add(QUANT_WORD), ecx);
            let rep = q.wrapping_add(1);
            wr32(this.wrapping_add(REP_WORD), rep);
            let len = rep.wrapping_mul(ecx);
            wr32(this.wrapping_add(LEN_WORD), len);
            len
        } else {
            let len = rd32(this.wrapping_add(BC_WORD));
            wr32(this.wrapping_add(LEN_WORD), len);
            len
        };
        // Format descriptor: same relative layout as the original's frame
        // block; the pointer below aims at word 5, matching the snapshots.
        let mut desc = [0u32; 14];
        desc[0] = 0x0001_0001;
        let srate = rd16(src.wrapping_add(0x18)) as u32;
        desc[1] = srate;
        desc[2] = srate.wrapping_add(srate);
        desc[3] = 0x0010_0002;
        desc[4] = 0;
        desc[5] = 0x24;
        desc[6] = 0x80b0;
        desc[7] = c0val;
        desc[8] = 0;
        desc[9] = desc.as_ptr() as u32;
        desc[10] = g32(ID_Q1_LO);
        desc[11] = g32(ID_Q1_HI);
        desc[12] = g32(ID_Q2_LO);
        desc[13] = g32(ID_Q2_HI);
        wr32(this.wrapping_add(0xd4), srate);
        let gobj = g32(DEV_WORD);
        let gvt = rd32(gobj);
        let mkdev: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(gvt.wrapping_add(0x0c)) as usize);
        mkdev(
            gobj,
            desc.as_ptr().wrapping_add(5) as u32,
            this.wrapping_add(CHAN_OBJ),
            0,
        );
        let mut tail_ebp = 0u32;
        if flags & QUICK_PATH == 0 {
            let obj = rd32(this.wrapping_add(CHAN_OBJ));
            let vt = rd32(obj);
            let mut arg0w = arg0;
            let mut out40 = 0u32;
            let lock: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0x2c)) as usize);
            let lr = lock(
                obj,
                0,
                rd32(this.wrapping_add(BC_WORD)),
                &mut arg0w as *mut u32 as u32,
                &mut out40 as *mut u32 as u32,
                0,
                0,
                2,
            );
            if (lr as i32) < 0 {
                let fail: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(0x08)) as usize);
                fail(obj);
            }
            let a0 = rd32(this.wrapping_add(COUNT_WORD));
            if a0 == 0 {
                let d0 = rd32(this.wrapping_add(D0_WORD));
                let sc = rd32(rd32(this.wrapping_add(SRC_PTR)).wrapping_add(0xc));
                lf_checker_rt::callee_cdecl!(CAL_SINK, u32, arg0w, d0, sc);
                let bc = rd32(this.wrapping_add(BC_WORD));
                if sc < bc {
                    lf_checker_rt::callee_cdecl!(CAL_REG, u32, arg0w.wrapping_add(sc), 0u32, bc.wrapping_sub(sc));
                }
                (this.wrapping_add(MODE_BYTE) as *mut u8).write(0);
            } else {
                let c0 = rd32(this.wrapping_add(LEN_WORD));
                let d0 = rd32(this.wrapping_add(D0_WORD));
                tail_ebp = c0.wrapping_sub(a0);
                lf_checker_rt::callee_cdecl!(CAL_SINK, u32, arg0w.wrapping_add(tail_ebp), d0, a0);
                let b8 = rd32(this.wrapping_add(QUANT_WORD));
                let dx16 = (a0 / b8).wrapping_add(1) & 0xffff;
                wr32(this.wrapping_add(LEFT_WORD), c0.wrapping_sub(dx16.wrapping_mul(b8)));
                wr32(this.wrapping_add(QCOUNT_WORD), dx16);
                let diff = rd32(this.wrapping_add(REP_WORD)).wrapping_sub(dx16);
                if diff != 0 {
                    let mut edx = 0u32;
                    let mut si = 0u16;
                    loop {
                        let cc = rd32(this.wrapping_add(QUANT_WORD));
                        let aa = rd32(this.wrapping_add(D0_WORD)).wrapping_add(a0);
                        edx = edx.wrapping_mul(cc).wrapping_add(arg0w);
                        lf_checker_rt::callee_cdecl!(CAL_SINK, u32, edx, aa, cc);
                        si = si.wrapping_add(1);
                        edx = si as u32;
                        if edx >= diff {
                            break;
                        }
                    }
                }
                (this.wrapping_add(MODE_BYTE) as *mut u8).write(1);
            }
            let unlock: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0x4c)) as usize);
            unlock(obj, arg0w, out40, 0, 0);
        } else {
            (this.wrapping_add(MODE_BYTE) as *mut u8).write(0);
        }
        let obj = rd32(this.wrapping_add(CHAN_OBJ));
        let vt = rd32(obj);
        let report: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(0x34)) as usize);
        report(obj, tail_ebp);
        let open: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt) as usize);
        open(obj, lf_checker_rt::relocated(OPEN_BLOB), this.wrapping_add(CHAN_AUX));
        let post: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(0x3c)) as usize);
        post(obj, POST_CODE)
    }
});
