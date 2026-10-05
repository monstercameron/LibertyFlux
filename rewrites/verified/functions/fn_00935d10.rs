// original: 0x00935D10 net_obj_update_a (proposed)

/// Periodic update of a network object: fast acknowledgement path plus a
/// slow classifier-driven state machine.
///
/// `this`+0xD84 is a counter: unless it is -1 it increments when `kind` is
/// 5 and resets to 0 otherwise. When the fast flag is set, `seq` matches
/// the object's last sequence and `level` exceeds its threshold, callee 1
/// classifies `level` and callee 2 is asked about the code; the answer's
/// low byte decides between returning 1 and 0.
///
/// Otherwise the slow path converts the count global to float, scales the
/// rate global by it, and asks callee 3 (five float words) for a double
/// that is truncated to an integer exactly like the original's truncate
/// control-word round trip (out of range or NaN yields 0). `kind` 9, or a
/// bounded `param` with a fresh `seq`, takes the resync branch (callee 4,
/// code 0x19, returns 1). Other kinds dispatch on a jump table: 6 and 7
/// refresh one flag each unless set recently (callee 4, codes 0xB/0xC), 8
/// always refreshes (code 0x16), 5 recycles the object once the counter
/// passes 30 (code 0x17), and 0 and 1 rebuild the object's message through
/// callee 5 (this batch's 0x936840) after time-window checks; anything else
/// returns 0. The full eax residue is mirrored, including the caller's
/// answer bytes above the 0/1 in al.
///
/// Original: 0x00935D10 (thiscall, six stack words; the sixth is only
/// forwarded as the rebuild call's last argument). Float order is the
/// original's throughout.
lf_checker_rt::export!(thiscall, rw_00935D10(
    this: u32,
    kind: u32,
    level: u32,
    seq: u32,
    param: u32,
    rate: u32,
    tail: u32,
) -> u32 {
    unsafe {
        const CNT: u32 = 0xD84;
        const LAST_SEQ: u32 = 0xD88;
        const STAMP: u32 = 0xD90;
        const SYNC_SEQ: u32 = 0xD98;
        const FLAG_A: u32 = 0xD80;
        const FLAG_B: u32 = 0xD81;
        const G_FAST: u32 = 0x0117_6BC8;
        const G_COUNT: u32 = 0x0103_6EB4;
        const G_SCALE: u32 = 0x0103_6EB8;
        const G_TIME: u32 = 0x0117_35B4;
        const G_BASE: u32 = 0x0117_6BCC;
        const G_C4: u32 = 0x0103_6EC4;
        const G_C8: u32 = 0x0103_6EC8;
        const G_BC: u32 = 0x0103_6EBC;
        const G_CC: u32 = 0x0103_6ECC;
        const THRESH: u32 = 0x00FE_8628;
        const SENDER: u32 = 0x0117_6888;
        const CAL_CLASSIFY: u32 = 1;
        const CAL_ASK: u32 = 2;
        const CAL_ESTIMATE: u32 = 3;
        const CAL_NOTIFY: u32 = 4;
        const CAL_REBUILD: u32 = 5;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Low 32 bits of the original's truncate-mode `fistp qword`: the
        /// truncated value when representable, else the indefinite's low
        /// word (0), which NaN, infinities and out-of-range doubles take.
        #[inline(always)]
        fn fistp_low32(x: f64) -> u32 {
            const TWO63: f64 = 9.223372036854776e18;
            if x.is_nan() || x >= TWO63 || x < -TWO63 {
                0
            } else {
                (x as i64) as u32
            }
        }

        let g = lf_checker_rt::relocated;
        // Counter update.
        let cnt = rd32(this.wrapping_add(CNT));
        if cnt != 0xFFFF_FFFF {
            if kind == 5 {
                wr32(this.wrapping_add(CNT), cnt.wrapping_add(1));
            } else {
                wr32(this.wrapping_add(CNT), 0);
            }
        }
        // Fast path.
        if rd8(g(G_FAST)) != 0 {
            let mut eax = seq;
            if seq != rd32(this.wrapping_add(LAST_SEQ)) {
                return eax & 0xFFFF_FF00;
            }
            let lv = f32::from_bits(level);
            let th = f32::from_bits(rd32(g(THRESH)));
            // comiss/jbe: below, equal or unordered all skip.
            if !(lv > th) {
                return eax & 0xFFFF_FF00;
            }
            let mut slot = 0xFFFF_FFFFu32;
            let _: u32 = lf_checker_rt::callee_stdcall!(
                CAL_CLASSIFY,
                u32,
                level,
                &mut slot as *mut u32 as u32
            );
            let ans: u32 = lf_checker_rt::callee_thiscall!(CAL_ASK, u32, g(SENDER), slot);
            eax = ans;
            if (ans as u8) == 0 {
                return eax & 0xFFFF_FF00;
            }
            return (eax & 0xFFFF_FF00) | 1;
        }
        // Slow path: count to float, scaled rate, estimate call.
        let count = rd32(g(G_COUNT));
        let scaled = mul(
            f32::from_bits(rd32(g(G_SCALE))),
            (count as f64) as f32,
        );
        let est: f64 = lf_checker_rt::callee_cdecl!(
            CAL_ESTIMATE,
            f64,
            ((count as f64) as f32).to_bits(),
            scaled.to_bits(),
            0u32,
            0x4170_0000u32,
            rate
        );
        let bound = fistp_low32(est);
        let mut eax = kind;
        // Resync branch.
        if kind == 9 {
            wr32(this.wrapping_add(SYNC_SEQ), seq);
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0x19u32);
            return (eax & 0xFFFF_FF00) | 1;
        }
        if param != 0
            && param <= bound
            && rd32(this.wrapping_add(SYNC_SEQ)) != seq
            && rd32(this.wrapping_add(LAST_SEQ)) == seq
        {
            wr32(this.wrapping_add(SYNC_SEQ), seq);
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0x19u32);
            return (eax & 0xFFFF_FF00) | 1;
        }
        let now = rd32(g(G_TIME));
        let base = rd32(g(G_BASE));
        // Returns dl with the eax residue above it.
        macro_rules! ret_dl {
            ($dl:expr) => {
                return (eax & 0xFFFF_FF00) | ($dl as u32)
            };
        }
        if kind > 8 {
            ret_dl!(0);
        }
        match kind {
            6 | 7 => {
                let flag = if kind == 6 { FLAG_A } else { FLAG_B };
                let code = if kind == 6 { 0x0Bu32 } else { 0x0Cu32 };
                if rd8(this.wrapping_add(flag)) != 0 {
                    eax = rd32(this.wrapping_add(STAMP)).wrapping_add(0x2710);
                    if eax >= now {
                        ret_dl!(0);
                    }
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, code);
                wr8(this.wrapping_add(flag), 1);
                wr32(this.wrapping_add(LAST_SEQ), 0xFFFF_FFFF);
                eax = rd32(g(G_TIME));
                wr32(this.wrapping_add(STAMP), eax);
                wr32(this.wrapping_add(CNT), 0);
                ret_dl!(1);
            }
            8 => {
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0x16u32);
                eax = rd32(g(G_TIME));
                wr32(this.wrapping_add(STAMP), eax);
                ret_dl!(1);
            }
            5 => {
                eax = rd32(this.wrapping_add(CNT));
                if (eax as i32) <= 0x1E {
                    ret_dl!(0);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0x17u32);
                wr32(this.wrapping_add(CNT), 0xFFFF_FFFF);
                wr32(this.wrapping_add(LAST_SEQ), 0xFFFF_FFFF);
                eax = rd32(g(G_TIME));
                wr32(this.wrapping_add(STAMP), eax);
                ret_dl!(1);
            }
            0 | 1 => {
                if param <= rd32(g(G_C4)) {
                    ret_dl!(0);
                }
                eax = rd32(g(G_C8)).wrapping_add(base);
                if eax >= now {
                    ret_dl!(0);
                }
                if param >= rd32(g(G_BC)) {
                    eax = rd32(g(G_CC)).wrapping_add(base);
                    if eax >= now {
                        ret_dl!(0);
                    }
                }
                if seq == rd32(this.wrapping_add(LAST_SEQ)) {
                    ret_dl!(0);
                }
                let first = if kind == 0 { 0u32 } else { 1u32 };
                let ans: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_REBUILD,
                    u32,
                    this,
                    first,
                    level,
                    param,
                    tail
                );
                wr32(this.wrapping_add(LAST_SEQ), seq);
                wr32(this.wrapping_add(CNT), 0);
                if kind == 0 {
                    eax = rd32(g(G_TIME));
                    wr32(this.wrapping_add(STAMP), eax);
                } else {
                    let t = rd32(g(G_TIME));
                    wr32(this.wrapping_add(STAMP), t);
                    eax = ans;
                }
                ret_dl!(1);
            }
            _ => {
                ret_dl!(0);
            }
        }
    }
});
