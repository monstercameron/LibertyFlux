// original: 0x0088AD80 audsound_visit_slot_triplets
/// Visits the slot triplets twice, notifying the set and unset members.
///
/// `this+0xc0` is the triplet count, triplets packed three bytes apart from
/// `this+1` (guard byte, value byte, flag byte). The first pass visits every
/// triplet whose guard is not 0xff and whose flag is nonzero; the second
/// pass visits every triplet whose guard is not 0xff and whose flag IS zero.
/// A visit calls pass one's notifier (thiscall: object, index) or pass two's
/// (thiscall: object, no stack words) with the object
/// `value * [this+0xc4] + [base + arg * 0x6f40 + 0x6f10]` (`base` is the
/// dword at `this+0xe8`, wrapping unsigned) and, for pass one, the triplet
/// index. The count compares unsigned and is re-read every iteration (but
/// never written, so it is stable). Returns the zero-extended count (entry
/// EAX untouched when the count is zero, so the contract pins entry EAX to
/// 0 and the rewrite returns the count on all paths).
/// Original: 0x0088AD80 (thiscall, one stack word: arg).
export!(thiscall, rw_0088AD80(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        const NOTIFY_SET: u32 = 1;
        const NOTIFY_UNSET: u32 = 2;
        const COUNT: usize = 0xc0;
        const MULT: usize = 0xc4;
        const BASE: usize = 0xe8;
        const ROW: u32 = 0x6f40;
        const COL: u32 = 0x6f10;
        const SKIP: u8 = 0xff;
        const NTRIPLET: usize = 3;
        let mult = *(this.add(MULT) as *const u32);
        let base = *(this.add(BASE) as *const u32);
        let row = base.wrapping_add(arg.wrapping_mul(ROW)).wrapping_add(COL);
        let entry = (row as *const u32).read_unaligned();
        let count = || *(this.add(COUNT));
        // First pass: guard set, flag nonzero.
        let mut i = 0u32;
        while i < count() as u32 {
            let t = this.add(1 + (i as usize) * NTRIPLET);
            let guard = *t.sub(1);
            let flag = *t.add(1);
            if guard != SKIP && flag != 0 {
                let v = *t as u32;
                let obj = mult.wrapping_mul(v).wrapping_add(entry);
                let _: u32 = callee_thiscall!(NOTIFY_SET, u32, obj, i);
            }
            i = i.wrapping_add(1);
        }
        // Second pass: guard set, flag zero.
        i = 0;
        while i < count() as u32 {
            let t = this.add(1 + (i as usize) * NTRIPLET);
            let guard = *t.sub(1);
            let flag = *t.add(1);
            if guard != SKIP && flag == 0 {
                let v = *t as u32;
                let obj = mult.wrapping_mul(v).wrapping_add(entry);
                let _: u32 = callee_thiscall!(NOTIFY_UNSET, u32, obj);
            }
            i = i.wrapping_add(1);
        }
        count() as u32
    }
});
