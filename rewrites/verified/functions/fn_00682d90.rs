// original: 0x00682D90 record_list_insert

/// Insert a record into this object's lists at a helper-chosen index.
///
/// `this` holds a main pointer array at `+0x20` with a 16-bit count at
/// `+0x24`, a sub-object array at `+0x14` with a 16-bit length at `+0x18`,
/// and a word at `+0x10` cleared on success. `a0` is the record: a key byte
/// at `+0`, a key word at `+2`, and a per-sub value array at `+8`.
///
/// A lookup helper (thiscall over `this` with the key byte, key word and a
/// stack out-slot) decides: a nonzero answer returns at once with the
/// answer's upper bytes and a zero low byte. Otherwise the out-slot's index
/// steers two insertion phases. First the main array: its count is
/// decremented (wrapping from 0 to 0xFFFF, which then faults on the wild
/// rotate read, on both sides alike), entries above the index shift one slot
/// up while the count stays SIGNED-greater than the index, the count is
/// restored, and the record lands at the index. Then, for each of the
/// length sub-objects (each an array at `+0` with a 16-bit count at `+4`
/// that goes through the same decrement/shift/restore/insert with the
/// record's per-sub value), after notifying the sub-object through a helper
/// that receives the previous phase's leftover in ECX (the main loop's
/// evolved array cursor for the first sub, each earlier sub's inserted
/// value after that, since the insert reloads ECX with the stored word).
/// A flag byte at
/// `+7` (bit 0) selects an extra preparation helper up front. Success returns
/// 1 in the low byte with the upper bytes of the last sub-array pointer (the
/// helper answers are clobbered by later loads), or plain 1 when no sub ran.
///
/// The original spills nothing into its incoming stack; the out-slot starts
/// uninitialised, so the contract fills scratch stack with zero (the
/// snapshot then observes a defined pre-value) and skips the frame pointer
/// while scripting the index. Very-negative indexes would hang the signed
/// shift guard on both sides, so the contract's most-negative index is -1.
///
/// Original: 0x00682D90 (thiscall, one stack word; callee ids 1-4).
lf_checker_rt::export!(thiscall, rw_00682D90(this: u32, a0: u32) -> u32 {
    unsafe {
        const SUBS_OFF: u32 = 0x14;
        const SUBS_LEN: u32 = 0x18;
        const ARR_OFF: u32 = 0x20;
        const ARR_COUNT: u32 = 0x24;
        const FLAG_OFF: u32 = 7;
        const DONE_OFF: u32 = 0x10;
        const KEY_WORD: u32 = 2;
        const VAL_ARR: u32 = 8;
        const SUB_ARR: u32 = 0;
        const SUB_COUNT: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        /// One decrement/shift/restore/insert round. Returns the leftover
        /// array cursor the original keeps in ECX. `decremented` is the
        /// count AFTER the wrap-around decrement; the shift guard compares
        /// it SIGNED against the index. Only the main array's loop evolves
        /// the cursor this way; each sub-array's loop instead walks EDX and
        /// leaves ECX at the sub-array base (see the caller).
        #[inline(always)]
        unsafe fn insert_round(arr: u32, count_slot: u32, decremented: u32, idx: u32, value: u32) -> u32 {
            unsafe {
                let mut cursor = arr;
                let mut rest = decremented;
                if (decremented as i32) > (idx as i32) {
                    loop {
                        cursor = arr.wrapping_add(rest.wrapping_mul(4));
                        let w = rd32(cursor.wrapping_sub(4));
                        wr32(cursor, w);
                        rest = rest.wrapping_sub(1);
                        if !((rest as i32) > (idx as i32)) {
                            break;
                        }
                    }
                }
                wr16(count_slot, decremented.wrapping_add(1));
                wr32(arr.wrapping_add(idx.wrapping_mul(4)), value);
                cursor
            }
        }

        let key_byte = unsafe { (a0 as *const u8).read() } as u32;
        let mut idx: u32 = 0;
        let found: u32 = lf_checker_rt::callee_thiscall!(
            1, u32,
            this,
            key_byte,
            rd16(a0 + KEY_WORD),
            &mut idx as *mut u32 as u32
        );
        if found != 0 {
            return found & 0xffff_ff00;
        }
        if unsafe { ((this + FLAG_OFF) as *const u8).read() } & 1 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
        }
        let _: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, this + ARR_OFF, 1);
        let arr = rd32(this + ARR_OFF);
        let old = rd16(this + ARR_COUNT);
        wr16(this + ARR_COUNT, old.wrapping_sub(1));
        let new = rd16(this + ARR_COUNT);
        // Rotate read (self-copy unless the count wrapped to 0xFFFF).
        let w = rd32(arr.wrapping_add(new.wrapping_mul(4)));
        wr32(arr.wrapping_add(old.wrapping_mul(4)).wrapping_sub(4), w);
        let mut cursor = insert_round(arr, this + ARR_COUNT, new, idx, a0);
        let nsubs = rd16(this + SUBS_LEN);
        // Upper return bytes: the last sub-array pointer when the loop runs
        // (the helper answers are clobbered by later loads), else the length.
        let mut tail: u32 = nsubs;
        if (nsubs as i32) > 0 {
            let subarr = rd32(this + SUBS_OFF);
            let vals = rd32(a0 + VAL_ARR);
            let mut i = 0u32;
            while (i as i32) < (nsubs as i32) {
                let sub = rd32(subarr + i.wrapping_mul(4));
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, sub, cursor);
                let sc = rd32(sub + SUB_ARR);
                let scold = rd16(sub + SUB_COUNT);
                wr16(sub + SUB_COUNT, scold.wrapping_sub(1));
                let scnew = rd16(sub + SUB_COUNT);
                let w = rd32(sc.wrapping_add(scnew.wrapping_mul(4)));
                wr32(sc.wrapping_add(scold.wrapping_mul(4)).wrapping_sub(4), w);
                let tmp = rd32(vals.wrapping_add(i.wrapping_mul(4)));
                insert_round(sc, sub + SUB_COUNT, scnew, idx, tmp);
                // The insert reloads ECX with the stored value, which is the
                // next iteration's pushed argument.
                cursor = tmp;
                tail = sc;
                i = i.wrapping_add(1);
            }
        }
        wr32(this + DONE_OFF, 0);
        (tail & 0xffff_ff00) | 1
    }
});
