// original: 0x00AE8E90 schedule_gate_check (proposed)

/// Decide whether a scheduled object may run in the current slot.
///
/// Arguments (cdecl): `obj` is the scheduled object (class selected by the
/// sign-extended kind word at `+0x2E` through `CLASS_TABLE`); `out` is an
/// optional pointer cleared when the object is retired through its virtual
/// slot `+0x44`; `retire` (low byte) selects retiring through the virtual
/// slot versus setting the pending bit; `audit` (low byte) enables a second
/// slot-membership test. Returns 1 when the object may run, else 0 (only the
/// low byte is the result; the upper bytes are the original's leftovers).
///
/// The class row's virtual slot `+0x14` yields the slot descriptor: a bit
/// mask at `+0`, option flags at `+3`, and a follow-up class index at `+4`.
/// The hour selector starts from `HOUR_BASE` unless `HOUR_OVER` is not -1,
/// and likewise the minute selector from `MIN_BASE`/`MIN_OVER`. When option
/// bit 0 is set, the minute selector advances by a hash of the deepest
/// ancestor pointer (`+0x4C` chain of depth two, `>> 7`, six bits) with
/// base-60 rollover into the hour selector (hours past 0x17 wrap to zero);
/// otherwise, when the object has uncovered bits (`~+0xC & +0x8`), it is
/// offered to callee 2 first.
/// With auditing on, the hour selector advances once more when minutes pass
/// 0x37 (hours past 0x17 wrap to zero).
/// The mask bit for the hour selector then selects the outcome: when clear,
/// state bits 18 and 16 both set together with mask bit 24 clear rejects;
/// otherwise bit 18 is cleared and the follow-up class (unless -1) must
/// have its `+0x40` bit 3 set or the object is rejected. When the mask bit
/// is set the same three tests accept instead, and rejection sets bit 18.
/// An accepted object with retiring on is retired through its virtual slot
/// `+0x44` (clearing `*out` when nonzero); with retiring off the pending
/// bit 0x20000000 is set (`+0x8` only on the mask-hit path, `+0x8` and
/// `+0xC` otherwise). With auditing on, an accepted object is finally
/// rejected unless the mask lacks the audited hour bit.
/// Original: 0x00AE8E90 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00AE8E90(obj: u32, out: u32, retire: u32, audit: u32) -> u32 {
    unsafe {
        const CLASS_TABLE: u32 = 0x01295CD8;
        const HOUR_OVER: u32 = 0x01295854;
        const HOUR_BASE: u32 = 0x01295848;
        const MIN_OVER: u32 = 0x01295858;
        const MIN_BASE: u32 = 0x0129584C;
        const KIND: u32 = 0x2E;
        const PARENT: u32 = 0x4C;
        const SEEN: u32 = 0x0C;
        const BITS: u32 = 0x08;
        const STATE: u32 = 0x28;
        const VT_SLOT: u32 = 0x44;
        const CLASS_SLOT: u32 = 0x14;
        const CLASS_FLAGS: u32 = 0x40;
        const PENDING: u32 = 0x2000_0000;
        const DIV_MAGIC: u64 = 0x8888_8889;

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

        let table = lf_checker_rt::relocated(CLASS_TABLE);
        let idx = ((obj + KIND) as *const i16).read_unaligned() as i32;
        let class = (table as *const u32).offset(idx as isize).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(class) + CLASS_SLOT) as usize);
        let desc = slot(class);
        let go = lf_checker_rt::global::<u32>;
        let mut hour = go(HOUR_BASE).read_unaligned();
        let ho = go(HOUR_OVER).read_unaligned();
        if ho != 0xFFFF_FFFF {
            hour = ho;
        }
        let mut minute = go(MIN_BASE).read_unaligned();
        let mo = go(MIN_OVER).read_unaligned();
        if mo != 0xFFFF_FFFF {
            minute = mo;
        }
        let follow = rd32(desc + 4);
        if rd8(desc + 3) & 1 != 0 {
            let mut sel = obj;
            let p = rd32(sel + PARENT);
            if p != 0 {
                sel = p;
            }
            let g = rd32(sel + PARENT);
            if g != 0 {
                sel = g;
            }
            minute = minute.wrapping_add((sel >> 7) & 0x3F);
            if minute > 0x3C {
                let n = minute.wrapping_sub(0x3D);
                let q = ((((n as u64 * DIV_MAGIC) >> 32) as u32) >> 5).wrapping_add(1);
                hour = hour.wrapping_add(q);
                minute = minute.wrapping_sub(q.wrapping_mul(60));
            }
            if hour > 0x17 {
                hour = 0;
            }
        } else if rd32(obj + BITS) & !rd32(obj + SEEN) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, obj);
        }
        let mut audited = hour;
        if (audit & 0xFF) != 0 {
            if minute > 0x37 {
                audited = audited.wrapping_add(1);
            }
            if audited > 0x17 {
                audited = 0;
            }
        }
        let mask = rd32(desc);
        let state = rd32(obj + STATE);
        if mask & 1u32.wrapping_shl(hour) == 0 {
            if (state >> 0x12) & 1 != 0 && (state >> 0x10) & 1 != 0 && (mask >> 0x18) & 1 == 0 {
                return 0;
            }
            wr32(obj + STATE, state & 0xFFF_BFFFF);
            if follow != 0xFFFF_FFFF {
                let fc = (table as *const u32).offset(follow as i32 as isize).read_unaligned();
                if rd8(fc + CLASS_FLAGS) & 8 == 0 {
                    return 0;
                }
            }
            if (retire & 0xFF) != 0 {
                let drop_fn: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
                drop_fn(obj);
                if out != 0 {
                    wr32(out, 0);
                }
            } else {
                wr32(obj + BITS, rd32(obj + BITS) | PENDING);
                wr32(obj + SEEN, rd32(obj + SEEN) | PENDING);
            }
            if (audit & 0xFF) != 0 && mask & 1u32.wrapping_shl(audited) != 0 {
                return 0;
            }
            1
        } else if (state >> 0x12) & 1 != 0
            || (state >> 0x10) & 1 == 0
            || (mask >> 0x18) & 1 != 0
        {
            wr32(obj + STATE, state | 0x0004_0000);
            0
        } else {
            if (retire & 0xFF) != 0 {
                let drop_fn: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
                drop_fn(obj);
                if out != 0 {
                    wr32(out, 0);
                }
            } else {
                wr32(obj + BITS, rd32(obj + BITS) | PENDING);
            }
            1
        }
    }
});
