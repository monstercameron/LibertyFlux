// original: 0x00d1a0b0 task_slots_query_sum (proposed)

/// Query every active slot of the four task-slot arrays and combine the answers.
///
/// `this` is the owning object. It holds four arrays of slot records with a
/// signed slot count each at `+0x13a0` (`u32` at `+0x13a0 + 4*k` for array
/// `k`): a count at or below zero skips that array. A slot fires only when
/// its flag byte is non-zero (flags live at `+0x13b0`, `+0x13bb`, `+0x13bc`,
/// `+0x13c7` plus the slot index). A firing slot is queried through a
/// `thiscall` callee whose object pointer is the slot record
/// (`this + base + i * stride`, bases `0x90`/`0x820`/`0x8e0`/`0x1280`,
/// strides `0xb0`/`0xc0`/`0xe0`/`0x120`) and whose stack arguments are this
/// function's own arguments unchanged. Before each query the slot record's words at `+0x3a` and `+0x3c` are set from the owner's words at the same offsets.
///
/// Answers combine by wrapping sum of the full 32-bit answers, starting from 0. Counts compare signed (`jle` skips).
///
/// Original: 0x00d1a0b0 (thiscall, 4 stack words).
lf_checker_rt::export!(thiscall, rw_00d1a0b0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const SLOT_COUNTS: u32 = 0x13a0;
        const FLAG_OFFS: [u32; 4] = [0x13b0, 0x13bb, 0x13bc, 0x13c7];
        const REC_BASES: [u32; 4] = [0x90, 0x820, 0x8e0, 0x1280];
        const REC_STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];
        const CALLEE: [u32; 4] = [1, 2, 3, 4];
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        let mut acc = 0u32;
        let mut k = 0u32;
        while k < 4 {
            let n = rd32(this.wrapping_add(SLOT_COUNTS).wrapping_add(k * 4)) as i32;
            if n > 0 {
                let mut i = 0i32;
                while i < n {
                    let idx = i as u32;
                    if rd8(this.wrapping_add(FLAG_OFFS[k as usize]).wrapping_add(idx)) != 0 {
                        let obj = this
                            .wrapping_add(REC_BASES[k as usize])
                            .wrapping_add(idx.wrapping_mul(REC_STRIDES[k as usize]));
                        wr16(obj.wrapping_add(0x3a), rd16(this.wrapping_add(0x3a)));
                        wr16(obj.wrapping_add(0x3c), rd16(this.wrapping_add(0x3c)));
                        let r = lf_checker_rt::callee_thiscall!(
                            CALLEE[k as usize], u32, obj, a1, a2, a3, a4);
                        acc = acc.wrapping_add(r);
                    }
                    i += 1;
                }
            }
            k += 1;
        }
        acc
    }
});
