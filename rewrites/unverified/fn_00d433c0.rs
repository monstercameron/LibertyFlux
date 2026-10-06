// original: 0x00d433c0 task_weapon_flag_check (proposed)

/// Decide whether a ped task may run, from a mode-gated flag word filled by
/// the task row's own routine and the ped's current weapon type.
///
/// `task` points to the task object (signed row-selector word at `+0x2e`,
/// grade byte at `+0x1e2`, extra dword at `+0x1bc`, weapon manager at
/// `+0x2b0`). `index` points to a selector holding two 16-bit counts
/// (`+0x4`, `+0x0c`) and a row-pointer table (`+0x08`). `mode` selects which
/// flag bits are required. `out_flags` receives one flag word from the row's
/// fill routine. `mask`, when nonzero, must share a bit with the flags.
///
/// The row is `rows[min(count_b - 1, trunc(0.0 / count_a))]`, where the
/// minimum is a SIGNED comparison: a zero `count_a` divides 0.0 by 0.0,
/// truncates the NaN to `i32::MIN` (what x86 `cvttss2si` yields for NaN),
/// and the scaled address wraps back to row 0. The row's fill routine
/// (virtual slot `+0x14`, called thiscall with the negated scaled count as
/// a float and `out_flags`) stores the flag word; a status byte from a
/// second callee gates one mode.
///
/// Gate order: the task's table row must lack one high flag bit (bit 18
/// when the row's own bit 1 is set, else bit 22); the mode's bits must be
/// present (`mode` is compared for equality only: 4 needs bit 3, 2 needs
/// bit 1 or (a zero status byte with bit 4), 3 needs bit 2, 0 needs bit 0
/// or bits 28-29, 1 needs bit 0 or bit 29 with bit 28 clear, 5 needs bit 5,
/// any other mode passes without a test); a set bit 30 additionally
/// requires a grade of at least 2 and a nonzero extra dword. The weapon
/// manager is then queried twice through the same pointer (a null first
/// answer means "no weapon", otherwise the type at `+0x18` of the second
/// answer is used); each set flag bit among 0x80 (needs no weapon) then
/// 0x100, 0x1000, 0x200000, 0x2000, 0x200, 0x400 (needing weapon types 7,
/// 10, 11, 12, 13, 14) ORs its match into the answer, and a set bit 0x800
/// needs an already-set answer or weapon type 16. With none of those bits
/// set the answer is true. Returns 1 or 0 in the low byte; the original
/// leaves the upper three bytes of `eax` holding whatever the last loaded
/// value left there, which the proof does not compare.
///
/// The floating-point work is exact for every input the selector allows:
/// 0.0 divided by a small count is +0.0 (or NaN for a zero count), and the
/// scaled count converted back to a float is always 0 or a small negative,
/// so the fill routine always sees -0.0 on a non-faulting call.
///
/// Original: 0x00d433c0 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00d433c0(task: u32, index: u32, mode: u32, out_flags: u32, mask: u32) -> u32 {
    unsafe {
        const COUNT_A: u32 = 0x04;
        const ROW_TABLE: u32 = 0x08;
        const COUNT_B: u32 = 0x0c;
        const ROW_THIS: u32 = 0x04;
        const FILL_SLOT: u32 = 0x14;
        const TASK_ROW_SEL: u32 = 0x2e;
        const ROW_GATE: u32 = 0x120;
        const TASK_GRADE: u32 = 0x1e2;
        const TASK_EXTRA: u32 = 0x1bc;
        const TASK_WEAPON_MGR: u32 = 0x2b0;
        const WEAPON_TYPE: u32 = 0x18;
        const ROW_TABLE_BASE: u32 = 0x0129_5CD8;
        const SIGN_MASK_ADDR: u32 = 0x00FE_8FA0;
        const CALLEE_STATUS: u32 = 2;
        const CALLEE_WEAPON_MGR: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        wr32(out_flags, 0);
        let count_a = rd16(index + COUNT_A);
        let count_b = rd16(index + COUNT_B);
        // Truncated 0.0 / count_a: 0, or i32::MIN when count_a is 0, exactly
        // what cvttss2si produces (Rust `as` would give 0 for NaN instead).
        let avg = fdiv(0.0f32, count_a as f32);
        let avg_trunc: i32 = if avg.is_nan() { i32::MIN } else { avg as i32 };
        // Signed minimum (cmovl): with count_a == 0 the wrapped address
        // below lands back on row 0.
        let mut row_idx: i32 = (count_b as i32).wrapping_sub(1);
        if avg_trunc < row_idx {
            row_idx = avg_trunc;
        }
        let scaled = (count_a as i32).wrapping_mul(row_idx);
        let rows = rd32(index + ROW_TABLE);
        let row = rd32(rows.wrapping_add((row_idx as u32).wrapping_mul(4)));
        let this = rd32(row + ROW_THIS);
        let sign = lf_checker_rt::global::<u32>(SIGN_MASK_ADDR).read();
        let amount = f32::from_bits((scaled as f32).to_bits() ^ sign);
        let fill: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + FILL_SLOT) as usize);
        let _ = fill(this, amount.to_bits(), out_flags);
        let status: u8 = lf_checker_rt::callee_cdecl!(CALLEE_STATUS, u32) as u8;

        let flags = rd32(out_flags);
        let tab_idx = rd16(task + TASK_ROW_SEL) as i16 as i32;
        let tab_entry =
            lf_checker_rt::global::<u32>(ROW_TABLE_BASE.wrapping_add((tab_idx.wrapping_mul(4)) as u32))
                .read();
        let gate = rd32(tab_entry.wrapping_add(ROW_GATE));
        if (gate & 2) != 0 {
            if (flags & 0x40000) != 0 {
                return 0;
            }
        } else if (flags & 0x400000) != 0 {
            return 0;
        }
        // Mode dispatch: every comparison on `mode` is equality (or a zero
        // test), so signedness plays no part here.
        let mode_ok = match mode {
            4 => (flags & 0x8) != 0,
            2 => {
                if (flags & 0x2) != 0 {
                    true
                } else if status != 0 {
                    return 0;
                } else {
                    (flags & 0x10) != 0
                }
            }
            3 => (flags & 0x4) != 0,
            0 => (flags & 0x1) != 0 || (flags & 0x3000_0000) != 0,
            1 => {
                if (flags & 0x1) == 0 && (flags & 0x2000_0000) == 0 {
                    return 0;
                }
                (flags & 0x1000_0000) == 0
            }
            5 => (flags & 0x20) != 0,
            _ => true,
        };
        if !mode_ok {
            return 0;
        }
        if (flags & 0x4000_0000) != 0 {
            if (rd8(task + TASK_GRADE) & 0x0f) < 2 {
                return 0;
            }
            if rd32(task + TASK_EXTRA) == 0 {
                return 0;
            }
        }
        if mask != 0 && (mask & flags) == 0 {
            return 0;
        }
        let mgr = task.wrapping_add(TASK_WEAPON_MGR);
        let first = lf_checker_rt::callee_thiscall!(CALLEE_WEAPON_MGR, u32, mgr);
        let weapon: u32 = if first == 0 {
            0
        } else {
            let second = lf_checker_rt::callee_thiscall!(CALLEE_WEAPON_MGR, u32, mgr);
            rd32(second.wrapping_add(WEAPON_TYPE))
        };
        // Flag-driven weapon match. `matched`/`seen` are the original's al/ah.
        let mut matched = false;
        let mut seen = false;
        if (flags & 0x80) != 0 {
            seen = true;
            matched = weapon == 0;
        }
        for (bit, want) in [
            (0x100u32, 7u32),
            (0x1000, 0x0a),
            (0x200000, 0x0b),
            (0x2000, 0x0c),
            (0x200, 0x0d),
            (0x400, 0x0e),
        ] {
            if (flags & bit) != 0 {
                seen = true;
                matched = matched || weapon == want;
            }
        }
        if (flags & 0x800) != 0 {
            if matched {
                return 1;
            }
            return u32::from(weapon == 0x10);
        }
        if !seen {
            return 1;
        }
        u32::from(matched)
    }
});
