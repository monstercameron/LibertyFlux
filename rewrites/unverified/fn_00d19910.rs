// original: 0x00d19910 task_slots_query_or (proposed)

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
/// function's own arguments unchanged.
///
/// Answers combine by bitwise OR of the low byte of each answer, starting from 0. Counts compare signed (`jle` skips).
///
/// Original: 0x00d19910 (thiscall, 2 stack words).
lf_checker_rt::export!(thiscall, rw_00d19910(this: u32, a1: u32, a2: u32) -> u32 {
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

        let mut acc = 0u8;
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
                        let r = lf_checker_rt::callee_thiscall!(
                            CALLEE[k as usize], u32, obj, a1, a2);
                        acc |= (r & 0xff) as u8;
                    }
                    i += 1;
                }
            }
            k += 1;
        }
        acc as u32
    }
});
