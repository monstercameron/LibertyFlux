// original: 0x00c3ecf0 train_fill_carriages_loop (proposed)
/// Push each of `count` carriage descriptors through two helpers.
///
/// `this` (ECX) is the train, `array` points at the descriptor array,
/// `idx` selects the first carriage slot (`slot = this + idx*0x30`),
/// `count` is how many descriptors to push. Marks `this+0x27c` with 1.
/// For each k in 0..count (nothing when count <= 0), loads eight words
/// from the k-th 0x30-byte record at `r = array+k*0x30` and passes them
/// as raw bits (arg0..arg7 are the words at r+0x04, r+0x08, r+0x14,
/// r+0x0c, r+0x24, r+0x00, r+0x28, r+0x10) to filler id 1 (thiscall/8,
/// this = slot+k*0x30), then calls sealer id 2 (thiscall/0, same this).
/// Returns nothing meaningful.
///
/// Original: 0x00c3ecf0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c3ecf0(this: u32, array: u32, idx: u32, count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x30;
        const MARK: u32 = 0x27c;
        const FILL: u32 = 1;
        const SEAL: u32 = 2;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        let mut slot = this.wrapping_add(idx.wrapping_mul(STRIDE));
        ((this + MARK) as *mut u8).write(1);
        if (count as i32) <= 0 {
            return 0;
        }
        let mut rec = array.wrapping_add(0x28);
        let mut left = count as i32;
        loop {
            // Stack words top-first: the original pushes placeholders and
            // overwrites them, then reserves two more words below.
            let w0 = rd(rec.wrapping_sub(0x24));
            let w1 = rd(rec.wrapping_sub(0x20));
            let w2 = rd(rec.wrapping_sub(0x14));
            let w3 = rd(rec.wrapping_sub(0x1c));
            let w4 = rd(rec.wrapping_sub(4));
            let w5 = rd(rec.wrapping_sub(0x28));
            let w6 = rd(rec);
            let w7 = rd(rec.wrapping_sub(0x18));
            let _: u32 = lf_checker_rt::callee_thiscall!(FILL, u32, slot, w0, w1, w2, w3, w4, w5, w6, w7);
            let _: u32 = lf_checker_rt::callee_thiscall!(SEAL, u32, slot);
            slot = slot.wrapping_add(STRIDE);
            rec = rec.wrapping_add(STRIDE);
            left -= 1;
            if left == 0 {
                break;
            }
        }
        0
    }
});
