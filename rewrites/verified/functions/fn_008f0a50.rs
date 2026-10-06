// original: 0x008f0a50 reset_control_tables (proposed)

/// Reset every control binding table to its resting state.
///
/// `this` is the controls manager. Four binding groups are processed in turn;
/// each group has an entry count, a pointer array and a kind array, plus a
/// static id table embedded below (copied from the original's read-only data,
/// which never changes at run time):
///
/// | group | count  | pointers | kinds  | ids       |
/// |-------|--------|----------|--------|-----------|
/// | 0     | `+0x0` | `+0x528` | `+0x8`  | T0, 49 ids |
/// | 1     | `+0x7b8`| `+0xce0` | `+0x7c0`| T1, 6 ids  |
/// | 2     | `+0xf70`| `+0x1498`| `+0xf78`| T2, 43 ids |
/// | 3     | `+0x1ee0`| `+0x2408`| `+0x1ee8`| T3, 4 ids  |
///
/// For each id `v`, the slot is the 16-byte record at `this+0x2698+16*v`.
/// Groups 0-2 scan the pointer array for every index holding that slot (the
/// count is re-read after each fill, so an aliasing fill would change the
/// scan; the kind is `kinds[i]-1` as an UNSIGNED compare against 14). Group 3
/// always fills the slot itself; its kind comes from the found index, or from
/// `+0x1ee4` (the word just before its kind array) when the slot is absent.
///
/// A kind of 1, 2, 9, 11, 14 or 15 selects the analog fill; any other kind
/// selects it only for the ids the group treats as analog anyway (group 0:
/// 12-15; group 2: 30-33; groups 1 and 3: none). The analog fill writes the
/// word 0x8080 at slot `+6` and, unless the buffer pointer at slot `+12` is
/// null, fills 0x200 bytes with a 0x80 byte plus the shared tick counter per
/// 8-byte chunk. The digital fill writes 0 there instead, with a 0x00 byte.
/// The buffer pointer is re-read for every chunk, as the original does.
///
/// Returns the accumulator left by group 3 (last kind selector or last buffer
/// pointer seen). Original: 0x008f0a50 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008f0a50(this: u32) -> u32 {
    unsafe {
        const TICK: u32 = 0x011735b4;
        const SLOTS: u32 = 0x2698;
        const ENTRY_FLAG: u32 = 6;
        const ENTRY_BUF: u32 = 12;
        const ANALOG_FLAG: u16 = 0x8080;
        const ANALOG_BYTE: u8 = 0x80;
        const DIGITAL_FLAG: u16 = 0;
        const DIGITAL_BYTE: u8 = 0;
        const FILL_LEN: u32 = 0x200;
        const KIND_MAP: [u32; 15] = [0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0];
        const T0: [u32; 49] = [
            0, 89, 88, 90, 91, 93, 94, 87, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
            13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 184,
            86, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
        ];
        const T1: [u32; 6] = [59, 60, 61, 85, 62, 63];
        const T2: [u32; 43] = [
            89, 88, 110, 111, 92, 112, 113, 136, 114, 30, 31, 32, 33, 34, 35, 36, 37,
            38, 39, 40, 41, 42, 43, 44, 45, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
            58, 141, 142, 143, 144, 183, 185, 186,
        ];
        const T3: [u32; 4] = [145, 146, 147, 148];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        /// Fill one slot's buffer; returns the last buffer pointer seen (the
        /// original leaves it in eax), or the incoming `eax` when no fill ran.
        #[inline(always)]
        unsafe fn fill_slot(entry: u32, flag: u16, byte: u8, tick: u32, eax: u32) -> u32 {
            unsafe {
                let first = rd32(entry.wrapping_add(ENTRY_BUF));
                wr16(entry.wrapping_add(ENTRY_FLAG), flag);
                if first == 0 {
                    return eax;
                }
                let mut out = eax;
                let mut d: u32 = 0;
                while d < FILL_LEN {
                    d = d.wrapping_add(8);
                    let b1 = rd32(entry.wrapping_add(ENTRY_BUF));
                    (b1.wrapping_add(d).wrapping_sub(8) as *mut u8).write(byte);
                    let b2 = rd32(entry.wrapping_add(ENTRY_BUF));
                    wr32(b2.wrapping_add(d).wrapping_sub(4), tick);
                    out = b2;
                }
                out
            }
        }
        #[inline(always)]
        unsafe fn analog(entry: u32, tick: u32, eax: u32) -> u32 {
            unsafe { fill_slot(entry, ANALOG_FLAG, ANALOG_BYTE, tick, eax) }
        }
        #[inline(always)]
        unsafe fn digital(entry: u32, tick: u32, eax: u32) -> u32 {
            unsafe { fill_slot(entry, DIGITAL_FLAG, DIGITAL_BYTE, tick, eax) }
        }
        /// True when `kind_minus_1` (unsigned) selects the analog fill, or the
        /// id is in this group's analog set. `lo..=hi` is that set, or `1..=0`
        /// (empty) for groups without one.
        #[inline(always)]
        fn use_analog(kind_minus_1: u32, v: u32, lo: u32, hi: u32) -> bool {
            if kind_minus_1 <= 14 && KIND_MAP[kind_minus_1 as usize] == 0 {
                return true;
            }
            lo <= v && v <= hi
        }

        let tick: u32 = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        // Groups 0-2: fill every array index holding the slot.
        const GROUPS: [(u32, u32, u32, u32, u32); 3] = [
            (0x0, 0x528, 0x8, 12, 15),
            (0x7b8, 0xce0, 0x7c0, 1, 0),
            (0xf70, 0x1498, 0xf78, 30, 33),
        ];
        let mut g = 0;
        while g < 3 {
            let (cnt_off, ptr_off, kind_off, lo, hi) = GROUPS[g];
            let mut ti = 0;
            let n = [T0.len(), T1.len(), T2.len()][g];
            while ti < n {
                let v = [T0[ti % T0.len()], T1[ti % T1.len()], T2[ti % T2.len()]][g];
                let slot = this.wrapping_add(SLOTS).wrapping_add(v.wrapping_mul(16));
                let mut eax: i32 = -1;
                loop {
                    let count = rd32(this.wrapping_add(cnt_off)) as i32;
                    eax = eax.wrapping_add(1);
                    let mut found = false;
                    loop {
                        if eax >= count {
                            break;
                        }
                        let p = rd32(
                            this
                                .wrapping_add(ptr_off)
                                .wrapping_add((eax as u32).wrapping_mul(4)),
                        );
                        if p == slot {
                            found = true;
                            break;
                        }
                        eax = eax.wrapping_add(1);
                    }
                    if !found {
                        break;
                    }
                    if eax < 0 {
                        break;
                    }
                    let entry = rd32(
                        this
                            .wrapping_add(ptr_off)
                            .wrapping_add((eax as u32).wrapping_mul(4)),
                    );
                    let kindm1 = rd32(
                        this
                            .wrapping_add(kind_off)
                            .wrapping_add((eax as u32).wrapping_mul(4)),
                    )
                    .wrapping_sub(1);
                    if use_analog(kindm1, v, lo, hi) {
                        analog(entry, tick, 0);
                    } else {
                        digital(entry, tick, 0);
                    }
                }
                ti += 1;
            }
            g += 1;
        }
        // Group 3: always fill the slot; the kind comes from the found index
        // or from the word before the kind array when absent.
        let mut eax_out: u32 = 0;
        for v in T3 {
            let slot = this.wrapping_add(SLOTS).wrapping_add(v.wrapping_mul(16));
            let count = rd32(this.wrapping_add(0x1ee0)) as i32;
            let mut idx: i32 = -1;
            if count > 0 {
                let mut j: i32 = 0;
                while j < count {
                    let p = rd32(
                        this
                            .wrapping_add(0x2408)
                            .wrapping_add((j as u32).wrapping_mul(4)),
                    );
                    if p == slot {
                        idx = j;
                        break;
                    }
                    j = j.wrapping_add(1);
                }
            }
            let kindm1 = rd32(
                this
                    .wrapping_add(0x1ee8)
                    .wrapping_add((idx as u32).wrapping_mul(4)),
            )
            .wrapping_sub(1);
            if kindm1 > 14 {
                eax_out = kindm1;
                eax_out = digital(slot, tick, eax_out);
            } else {
                let m = KIND_MAP[kindm1 as usize];
                eax_out = m;
                if m == 0 {
                    eax_out = analog(slot, tick, eax_out);
                } else {
                    eax_out = digital(slot, tick, eax_out);
                }
            }
        }
        eax_out
    }
});
