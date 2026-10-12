// original: 0x00c87b00 audio_entry_scan_and_dispatch

/// Scan the requested slots of an audio entry list and dispatch work for
/// occupied entries or eligible empty pairs. The request object supplies a
/// slot count, a pointer to 16-bit entry indices, and a pointer to a bitmap.
/// The other pointer supplies an `(x, y)` point. `this` supplies the entry
/// table, a loop selector, and a threshold; its linked object supplies the
/// entry-count check, virtual-call table, and 32-byte gate rows.
///
/// A null link in the request chain, or a mismatch between the linked
/// object's count and half the request count, returns zero. Otherwise the
/// selector chooses a half-open range of request slots. Nonzero entry
/// indices may update an entry flag, then pass seven ordered squared-distance
/// gates before dispatching the entry and clearing its slot. Zero indices
/// can dispatch only at even positions when the request flags, linked entry
/// word, and bitmap bits allow it. That path makes two virtual calls and one
/// of three distinct mixer calls based on the gate row and following slot.
/// The function returns one after the selected range is scanned.
///
/// The three mixer call shapes include words read from the defined zero
/// stack fill, so the rewrite passes those words as zero. All callee calls in
/// the proof are intercepted according to their call site. Scalar arithmetic
/// explicitly forwards the destination NaN when both operands are NaNs;
/// each threshold test is ordered `distance >= threshold`, matching the
/// original's unordered branch behavior.
lf_checker_rt::export!(thiscall, rw_00c87b00(
    this: u32,
    request: u32,
    point: u32,
    selector: u32,
) -> u32 {
    unsafe {
        const ENTRY_TABLE: u32 = 0xF30;
        const ENTRY_BACK: u32 = 0x60;
        const ENTRY_STRIDE: u32 = 96;
        const FLAG_OFF: u32 = 0x55;
        const FLAG_CHECK: u8 = 2;
        const FLAG_DONE: u8 = 4;
        const OBJ_GLOBAL: u32 = 0x1683290;
        const CALLEE_FLAG: u32 = 1;
        const CALLEE_DISTANCE: u32 = 2;
        const CALLEE_VCALL_FIRST: u32 = 3;
        const CALLEE_VCALL_SECOND: u32 = 4;
        const CALLEE_MIX_A: u32 = 5;
        const CALLEE_MIX_B: u32 = 6;
        const CALLEE_MIX_C: u32 = 7;
        const MAGIC_SELECTOR: u32 = 0x7A7A7A7A;
        const THIRD: u32 = 0x3EAAAAAB;

        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u32 {
            unsafe { (address as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        fn is_nan_bits(bits: u32) -> bool {
            bits & 0x7F80_0000 == 0x7F80_0000 && bits & 0x007F_FFFF != 0
        }
        #[inline(never)]
        fn fadd(left: u32, right: u32) -> u32 {
            if is_nan_bits(left) && is_nan_bits(right) {
                left | 0x0040_0000
            } else {
                unsafe {
                    let a = core::arch::x86::_mm_set_ss(f32::from_bits(left));
                    let b = core::arch::x86::_mm_set_ss(f32::from_bits(right));
                    let mut out = 0.0f32;
                    core::arch::x86::_mm_store_ss(&mut out, core::arch::x86::_mm_add_ss(a, b));
                    out.to_bits()
                }
            }
        }
        #[inline(never)]
        fn fsub(left: u32, right: u32) -> u32 {
            if is_nan_bits(left) && is_nan_bits(right) {
                left | 0x0040_0000
            } else {
                unsafe {
                    let a = core::arch::x86::_mm_set_ss(f32::from_bits(left));
                    let b = core::arch::x86::_mm_set_ss(f32::from_bits(right));
                    let mut out = 0.0f32;
                    core::arch::x86::_mm_store_ss(&mut out, core::arch::x86::_mm_sub_ss(a, b));
                    out.to_bits()
                }
            }
        }
        #[inline(never)]
        fn fmul(left: u32, right: u32) -> u32 {
            if is_nan_bits(left) && is_nan_bits(right) {
                left | 0x0040_0000
            } else {
                unsafe {
                    let a = core::arch::x86::_mm_set_ss(f32::from_bits(left));
                    let b = core::arch::x86::_mm_set_ss(f32::from_bits(right));
                    let mut out = 0.0f32;
                    core::arch::x86::_mm_store_ss(&mut out, core::arch::x86::_mm_mul_ss(a, b));
                    out.to_bits()
                }
            }
        }
        #[inline(always)]
        fn ordered_ge(left: u32, right: u32) -> bool {
            f32::from_bits(left) >= f32::from_bits(right)
        }

        let request_chain = rd32(request.wrapping_add(0x44));
        if request_chain == 0 {
            return 0;
        }
        let second_chain = rd32(request_chain.wrapping_add(0x38));
        if second_chain == 0 {
            return 0;
        }
        let third_chain = rd32(second_chain.wrapping_add(4));
        let linked_object = rd32(third_chain.wrapping_add(0x0C));
        if linked_object == 0 {
            return 0;
        }

        let slot_count = rd16(request.wrapping_add(0x54));
        let half_count = slot_count >> 1;
        if rd32(linked_object.wrapping_add(0xCC)) != half_count {
            return 0;
        }

        let quarter_count = slot_count >> 3;
        let (mut index, end) = if quarter_count == 0 || selector == MAGIC_SELECTOR {
            (0, slot_count)
        } else {
            let start = quarter_count.wrapping_mul(selector);
            if selector == 7 {
                (start, slot_count)
            } else {
                (start, quarter_count.wrapping_mul(selector.wrapping_add(1)))
            }
        };
        let entry_table = rd32(this.wrapping_add(ENTRY_TABLE));
        let word_table = rd32(request.wrapping_add(0x50));

        while (index as i32) < (end as i32) {
            let entry_index = rd16(word_table.wrapping_add(index.wrapping_mul(2)));
            if entry_index != 0 {
                let entry = entry_table
                    .wrapping_sub(ENTRY_BACK)
                    .wrapping_add(entry_index.wrapping_mul(ENTRY_STRIDE));
                let flags = rd8(entry.wrapping_add(FLAG_OFF));
                if flags & FLAG_CHECK != 0 && flags & FLAG_DONE == 0 {
                    let answer = lf_checker_rt::callee_thiscall!(
                        CALLEE_FLAG,
                        u32,
                        lf_checker_rt::relocated(OBJ_GLOBAL),
                        entry
                    );
                    if answer != 0 {
                        wr8(entry.wrapping_add(FLAG_OFF), flags | FLAG_DONE);
                    }
                }

                let x = rd32(point);
                let y = rd32(point.wrapping_add(4));
                let dy0 = fsub(y, rd32(entry.wrapping_add(4)));
                let dx0 = fsub(x, rd32(entry));
                let dy1 = fsub(y, rd32(entry.wrapping_add(0x14)));
                let dy2 = fsub(y, rd32(entry.wrapping_add(0x24)));
                let dy3 = fsub(y, rd32(entry.wrapping_add(0x34)));
                let dx1 = fsub(x, rd32(entry.wrapping_add(0x10)));
                let dx2 = fsub(x, rd32(entry.wrapping_add(0x20)));
                let dx3 = fsub(x, rd32(entry.wrapping_add(0x30)));

                let avg_y01 = fmul(fadd(dy1, dy0), THIRD);
                let avg_x21 = fmul(fadd(dx2, dx1), THIRD);
                let avg_y21 = fmul(fadd(dy2, dy1), THIRD);
                let avg_y20 = fmul(fadd(dy2, dy0), THIRD);
                let avg_x10 = fmul(fadd(dx1, dx0), THIRD);
                let avg_x20 = fmul(fadd(dx2, dx0), THIRD);
                let threshold = rd32(this.wrapping_add(0x20));

                let distances = [
                    fadd(fmul(dx0, dx0), fmul(dy0, dy0)),
                    fadd(fmul(dx1, dx1), fmul(dy1, dy1)),
                    fadd(fmul(dx2, dx2), fmul(dy2, dy2)),
                    fadd(fmul(dx3, dx3), fmul(dy3, dy3)),
                    fadd(fmul(avg_x10, avg_x10), fmul(avg_y01, avg_y01)),
                    fadd(fmul(avg_x21, avg_x21), fmul(avg_y21, avg_y21)),
                    fadd(fmul(avg_x20, avg_x20), fmul(avg_y20, avg_y20)),
                ];
                if distances.iter().all(|distance| ordered_ge(*distance, threshold)) {
                    let _ = lf_checker_rt::callee_thiscall!(
                        CALLEE_DISTANCE,
                        u32,
                        entry
                    );
                    wr16(word_table.wrapping_add(index.wrapping_mul(2)), 0);
                }
            } else if index & 1 == 0
                && rd16(this.wrapping_add(4)) != 0
                && rd16(
                    entry_table
                        .wrapping_add(rd16(this.wrapping_add(4)).wrapping_mul(ENTRY_STRIDE))
                        .wrapping_sub(0x0A),
                ) != 0
            {
                let bit_table = rd32(request.wrapping_add(0x48));
                let bit_pair = (index >> 1).wrapping_mul(2);
                let first_bit = 1u32 << (bit_pair & 31);
                let second_index = bit_pair.wrapping_add(1);
                let second_bit = 1u32 << (second_index & 31);
                let first_set = rd32(bit_table.wrapping_add((bit_pair >> 5).wrapping_mul(4)))
                    & first_bit
                    != 0;
                let second_set = rd32(bit_table.wrapping_add((second_index >> 5).wrapping_mul(4)))
                    & second_bit
                    != 0;
                if first_set || second_set {
                    let gate_base = rd32(linked_object.wrapping_add(0x8C));
                    let group = index >> 1;
                    let gate_row = gate_base.wrapping_add(group.wrapping_mul(32));
                    let first_answer = lf_checker_rt::callee_thiscall!(
                        CALLEE_VCALL_FIRST,
                        u32,
                        linked_object,
                        group
                    );
                    let second_answer = lf_checker_rt::callee_thiscall!(
                        CALLEE_VCALL_SECOND,
                        u32,
                        linked_object,
                        first_answer
                    );
                    let low_flag = u32::from(first_set);
                    let high_flag = u32::from(second_set);
                    if rd16(gate_row.wrapping_add(0x16)) == 0 {
                        let _ = lf_checker_rt::callee_thiscall!(
                            CALLEE_MIX_A,
                            u32,
                            this,
                            index,
                            request,
                            linked_object,
                            point,
                            gate_row,
                            0u32,
                            1u32,
                            2u32,
                            low_flag,
                            high_flag,
                            second_answer
                        );
                    } else {
                        let _ = lf_checker_rt::callee_thiscall!(
                            CALLEE_MIX_B,
                            u32,
                            this,
                            index,
                            request,
                            linked_object,
                            point,
                            gate_row,
                            0u32,
                            1u32,
                            2u32,
                            low_flag,
                            high_flag,
                            second_answer
                        );
                        if rd16(word_table.wrapping_add(index.wrapping_mul(2)).wrapping_add(2)) == 0 {
                            let _ = lf_checker_rt::callee_thiscall!(
                                CALLEE_MIX_C,
                                u32,
                            this,
                                index.wrapping_add(1),
                                request,
                            linked_object,
                                point,
                                gate_row,
                                0u32,
                                2u32,
                                3u32,
                                low_flag,
                                high_flag,
                                second_answer
                            );
                        }
                    }
                }
            }
            index = index.wrapping_add(1);
        }
        1
    }
});
