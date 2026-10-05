// original: 0x009D3920 recarray_grow_append (proposed)
//
/// Appends a slot to a record array, regrowing it when full.
///
/// The array holds 16-bit `count` at `this + 0x04` and `capacity` at
/// `this + 0x06` over `0x88`-byte records at `[this]`. When full
/// (`count == capacity`) the capacity grows by `extra` (wrapping 16-bit),
/// a fresh block is allocated (callee 1), the old records are copied over,
/// the old block is freed (callee 2) and the base is swapped. Then the next
/// record address (`base + count * 0x88`) is returned and the count bumped.
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_009D3920(this: u32, extra: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const CAP: u32 = 0x06;
        const STRIDE: u32 = 0x88;
        const WORDS: usize = 0x22;
        const ALLOC: u32 = 1;
        const FREE: u32 = 2;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned();
        let cap = (this.wrapping_add(CAP) as *const u16).read_unaligned();
        if count == cap {
            let newcap = cap.wrapping_add(extra as u16);
            (this.wrapping_add(CAP) as *mut u16).write_unaligned(newcap);
            let p: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, this, newcap as u32);
            let old = rd(this.wrapping_add(RECORDS));
            for i in 0..count {
                let src = old.wrapping_add((i as u32).wrapping_mul(STRIDE));
                let dst = p.wrapping_add((i as u32).wrapping_mul(STRIDE));
                core::ptr::copy_nonoverlapping(
                    src as *const u32, dst as *mut u32, WORDS);
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, old);
            (this.wrapping_add(RECORDS) as *mut u32).write_unaligned(p);
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned();
        let base = rd(this.wrapping_add(RECORDS));
        let slot = base.wrapping_add((count as u32).wrapping_mul(STRIDE));
        (this.wrapping_add(COUNT) as *mut u16)
            .write_unaligned(count.wrapping_add(1));
        slot
    }
});
