// original: 0x0088E7B0 rage::audVoicePcAdpcm::vf8
// 0088E7B0 rage::audVoicePcAdpcm::vf8: snapshot 14 dwords of decode state
// into the current slot, optionally resolve the stream position through two
// helpers, then advance the slot index modulo the slot count. Returns the
// quotient of the slot division.
export!(thiscall, rw_0088e7b0(this: *mut u8, extra: u32) -> u32 {
    unsafe {
        let idx = *(this.add(0x118) as *const u32);
        let slot = (idx << 6) as usize;
        let src = *(this.add(0x14) as *const u32) as *const u32;
        let dst = this.add(0x90 + slot) as *mut u32;
        for i in 0..14usize {
            *dst.add(i) = *src.add(i);
        }
        *(this.add(0xC8 + slot) as *mut u32) = 0;
        if extra != 0 && (*this.add(0x8C) & 0x10) != 0 {
            let owner = *(this.add(0x10) as *const u32);
            let ans1 = callee_thiscall!(1, u32, owner, extra);
            let table = *(*((owner + 0x7C) as *const u32) as *const u32);
            let x = *((table.wrapping_add(ans1.wrapping_mul(8))) as *const u32);
            let scaled = x.wrapping_mul(2) >> 2;
            let ans2 = callee_cdecl!(2, u32, extra, *(this.add(0xC) as *const u32));
            *(this.add(0x128) as *mut u32) = ans2;
            let half = ans2 >> 1;
            let bucket = if half & 0x7FF == 0 {
                half >> 11
            } else {
                (half >> 11).wrapping_add(1)
            };
            let entry = ((bucket as u64 * 3 + *(this.add(0x134) as *const u32) as u64) & 0xFFFF_FFFF) as u32;
            *(this.add(0x14E) as *mut u16) = *(entry as *const u16);
            *this.add(0x150) = *((entry.wrapping_add(2)) as *const u8);
            *(this.add(0xC8 + slot) as *mut u32) =
                (bucket << 11).wrapping_sub(scaled);
        }
        let head = *((src as *const u8).add(0xC) as *const u32);
        let base = *(this.add(0xC8 + slot) as *const u32);
        *(this.add(0xCC + slot) as *mut u32) = head.wrapping_sub(base);
        let div = *(this.add(0x120) as *const u32);
        let n = idx.wrapping_add(1);
        *(this.add(0x118) as *mut u32) = n % div;
        n / div
    }
});
