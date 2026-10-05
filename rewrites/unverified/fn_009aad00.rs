// original: 0x009AAD00 audio_voice_clear_by_id (proposed)

/// Voice-slot clear by id: drops every voice slot whose owner id equals `id.
///
/// The object `this` holds three indexed slots plus three fixed slots. Each
/// indexed slot `k` (0..3) has an index byte at `this + 0x368 + k` selecting
/// a 96-byte record; the owner dword sits at a per-slot base offset from the
/// record start (`0x0c`, `0x12c`, `0x24c`) with a flag byte 4 below it. The
/// fixed slots are owner dwords at `this + 0x370 / 0x3d0 / 0x430` with flag
/// bytes 4 below each. Every owner dword equal to `id` is zeroed and its
/// flag byte set to 3 (`FLAG_FREE`). Returns nothing.
/// Original: 0x009AAD00 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009AAD00(this: u32, id: u32) -> u32 {
    unsafe {
        const INDEX_BASE: u32 = 0x368;
        const RECORD_SIZE: u32 = 96;
        const FLAG_FREE: u8 = 3;
        const INDEXED_BASES: [u32; 3] = [0x0c, 0x12c, 0x24c];
        const FIXED_OWNERS: [u32; 3] = [0x370, 0x3d0, 0x430];

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut k: u32 = 0;
        while k < 3 {
            let rec = (rd8(this.wrapping_add(INDEX_BASE).wrapping_add(k)) as u32)
                .wrapping_mul(RECORD_SIZE);
            let owner = this.wrapping_add(rec).wrapping_add(INDEXED_BASES[k as usize]);
            if rd32(owner) == id {
                wr32(owner, 0);
                wr8(owner.wrapping_sub(4), FLAG_FREE);
            }
            k += 1;
        }
        let mut f: u32 = 0;
        while f < 3 {
            let owner = this.wrapping_add(FIXED_OWNERS[f as usize]);
            if rd32(owner) == id {
                wr32(owner, 0);
                wr8(owner.wrapping_sub(4), FLAG_FREE);
            }
            f += 1;
        }
        0
    }
});
