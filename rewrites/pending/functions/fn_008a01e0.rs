// original: 0x008A01E0 audRetriggeredSound_update (proposed)
/// Audio voice update tick (proposed name `audRetriggeredSound_update`).
///
/// Each call offers the incoming stimulus to the two voice slots of an
/// overlapped sound object, refreshes the cached count and period from the
/// live parameter pointers, and either retires the expired voice (flipping
/// the active side) or re-arms the sounding slots. Returns 1 while the
/// voice stays alive, 0 once it has retired.
///
/// Sound entries live in per-bank tables: the bank byte selects a row and
/// the slot byte selects an entry within it; only entries tagged 2 accept
/// the stimulus here. The period path multiplies the live period by a
/// constant factor and truncates toward zero to an integer.
///
/// Checker notes: the original reuses its incoming stack-argument slot as
/// floating-point scratch, so the contract disables the stack check
/// (callee frame below the incoming ESP is unobserved either way) while
/// ESP adjustment is still verified. Only AL carries the result.
lf_k2_rt::export!(thiscall, rw_008A01E0(this: *mut u8, stim: u32) -> u32 {
    unsafe {
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        // Offer the stimulus to each live slot's entry.
        let mut slot_i = 0u32;
        while slot_i < 2 {
            let slot = *this.add(0x48 + slot_i as usize);
            if slot != 0xFF {
                let entry = audio_entry_008A01E0(table, stride, bank, slot);
                if entry != 0 && *((entry + 6) as *const u16) == 2 {
                    let ok: u32 = lf_k2_rt::callee_thiscall!(1, u32, entry, stim);
                    if (ok & 0xFF) == 0 {
                        lf_k2_rt::callee_thiscall!(2, u32, this as u32, slot_i);
                    }
                }
            }
            slot_i += 1;
        }
        // Refresh cached count and period from the live parameter pointers.
        let count_ptr = *(this.add(0xDC) as *const u32);
        if count_ptr != 0 {
            *(this.add(0xD0) as *mut i32) =
                trunc_f32_i32_008A01E0(*(count_ptr as *const f32));
        }
        let period_ptr = *(this.add(0xD8) as *const u32);
        let mut scratch_hi = stim & 0xFFFFFF00;
        if period_ptr != 0 {
            let prod =
                *(period_ptr as *const f32) * *lf_k2_rt::global::<f32>(0xFE8C58);
            *(this.add(0xCC) as *mut u32) = trunc_f32_i64lo_008A01E0(prod);
            // The float-to-int path leaves its residue in the reused
            // argument slot: product bits above the low word, x87 control
            // word below it (see X87_CW_MID_008A01E0).
            scratch_hi = (prod.to_bits() & 0xFFFF0000)
                | ((X87_CW_MID_008A01E0 as u32) << 8);
        }
        // Expiry test: a voice still within its period goes to re-arm.
        let start = *(this.add(0x84) as *const u32);
        let elapsed = if (start as i32) < 0 {
            0
        } else {
            stim.wrapping_sub(start)
        };
        let age = elapsed.wrapping_sub(*(this.add(0xC8) as *const u32));
        if age < *(this.add(0xCC) as *const u32) {
            return retrigger_armed_008A01E0(
                this, stim, table, stride, bank, scratch_hi,
            );
        }
        // Retire path, unless the voice repeats or is held.
        let count = *(this.add(0xD0) as *const i32);
        if (count != -1 && count <= 1) || (*this.add(0x39) & 8) != 0 {
            return retire_early_008A01E0(
                this, table, stride, bank, stim, scratch_hi,
            );
        }
        if count != -1 {
            let next = count.wrapping_sub(1);
            *(this.add(0xD0) as *mut i32) = next;
            if count_ptr != 0 {
                *(count_ptr as *mut f32) = next as f32;
            }
        }
        let active = *this.add(0xE1);
        if active != 0 {
            if *this.add(0xE0) != 0 {
                let h: u32 = lf_k2_rt::callee_thiscall!(3, u32, this as u32, 1);
                if h != 0 {
                    let h2: u32 =
                        lf_k2_rt::callee_thiscall!(3, u32, this as u32, 1);
                    lf_k2_rt::callee_thiscall!(4, u32, h2);
                }
            }
            lf_k2_rt::callee_thiscall!(5, u32, this as u32, 0);
            let c = *(this.add(0x84) as *const u32);
            *(this.add(0xC8) as *mut u32) = if (c as i32) < 0 {
                0
            } else {
                stim.wrapping_sub(c)
            };
            *this.add(0xE2) = 1;
        } else {
            if *this.add(0xE0) != 0 {
                let h: u32 = lf_k2_rt::callee_thiscall!(3, u32, this as u32, 0);
                if h != 0 {
                    let h2: u32 =
                        lf_k2_rt::callee_thiscall!(3, u32, this as u32, 0);
                    lf_k2_rt::callee_thiscall!(4, u32, h2);
                }
            }
            lf_k2_rt::callee_thiscall!(5, u32, this as u32, 1);
            let c = *(this.add(0x84) as *const u32);
            *(this.add(0xC8) as *mut u32) = if (c as i32) < 0 {
                0
            } else {
                stim.wrapping_sub(c)
            };
            *this.add(0xE3) = 1;
        }
        *this.add(0xE1) = if active == 0 { 1 } else { 0 };
        retrigger_armed_008A01E0(this, stim, table, stride, bank, scratch_hi)
    }
});

/// Middle byte of the trial thread's x87 control word as observed through
/// the original's float scratch slot (Windows/MSVC default control word).
/// Verified empirically: every mismatch against another value fails loudly.
const X87_CW_MID_008A01E0: u8 = 0x03;

/// Look up a sound entry: bank row from the table base, then the slot's
/// entry within the row. Slot 0xFF means empty.
fn audio_entry_008A01E0(table: u32, stride: u32, bank: u8, slot: u8) -> u32 {
    if slot == 0xFF {
        return 0;
    }
    let row = unsafe {
        *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32)
    };
    row.wrapping_add((slot as u32).wrapping_mul(stride))
}

/// Re-arm section: offer the stimulus to each armed slot and clear the
/// arm flag where the retrigger fires. Always reports the voice alive.
fn retrigger_armed_008A01E0(
    this: *mut u8,
    stim: u32,
    table: u32,
    stride: u32,
    bank: u8,
    scratch_hi: u32,
) -> u32 {
    unsafe {
        let mut arm_i = 0u32;
        while arm_i < 2 {
            let slot = *this.add(0x48 + arm_i as usize);
            if slot != 0xFF {
                let entry = audio_entry_008A01E0(table, stride, bank, slot);
                if entry != 0 && *this.add(0xE2 + arm_i as usize) != 0 {
                    let flag = (*this.add(0x39) >> 5) & 1;
                    let key = *(this.add(0x3C) as *const i16) as i32 as u32;
                    let ebx: u32 = lf_k2_rt::callee_cdecl!(6, u32, key);
                    let entry2 = audio_entry_008A01E0(
                        table,
                        stride,
                        bank,
                        *this.add(0x48 + arm_i as usize),
                    );
                    // The original passes its scratch word here: low byte is
                    // the flag, upper bytes are whatever the reused
                    // argument slot holds (incoming stimulus, or the
                    // float-path residue).
                    let flagword = scratch_hi | (flag as u32);
                    let r: u32 =
                        lf_k2_rt::callee_thiscall!(7, u32, entry2, ebx, flagword, 0);
                    if r == 1 {
                        let h: u32 =
                            lf_k2_rt::callee_thiscall!(3, u32, this as u32, arm_i);
                        lf_k2_rt::callee_thiscall!(8, u32, h, stim);
                        *this.add(0xE2 + arm_i as usize) = 0;
                    }
                }
            }
            arm_i += 1;
        }
        1
    }
}

/// Early-retire check: any live slot keeps the voice in the re-arm
/// section, otherwise the voice is done.
fn retire_early_008A01E0(
    this: *mut u8,
    table: u32,
    stride: u32,
    bank: u8,
    stim: u32,
    scratch_hi: u32,
) -> u32 {
    unsafe {
        let mut slot_i = 0u32;
        while slot_i < 2 {
            let slot = *this.add(0x48 + slot_i as usize);
            if slot != 0xFF
                && audio_entry_008A01E0(table, stride, bank, slot) != 0
            {
                return retrigger_armed_008A01E0(
                    this, stim, table, stride, bank, scratch_hi,
                );
            }
            slot_i += 1;
        }
        0
    }
}

/// Truncate f32 toward zero to i32 with x86 cvttss2si overflow semantics:
/// NaN and positive overflow yield the indefinite value instead of
/// Rust's saturating `as` result.
fn trunc_f32_i32_008A01E0(f: f32) -> i32 {
    if f.is_nan() || f >= 2147483648.0 {
        0x80000000u32 as i32
    } else {
        f as i32
    }
}

/// Low dword of an x87 qword float-to-int conversion in chop mode.
/// Only positive overflow differs from Rust's saturating `as`: every
/// other out-of-range or NaN input already yields low dword zero.
fn trunc_f32_i64lo_008A01E0(f: f32) -> u32 {
    if f >= 9223372036854775808.0 {
        0
    } else {
        (f as i64) as u32
    }
}
