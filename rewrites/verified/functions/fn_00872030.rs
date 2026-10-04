// original: 0x00872030 crmt_request_copy
//! Copy a motion-request state block (0x50 bytes with gaps) from `src` to
//! `dst` and return `dst`. Bytes 0-11 go byte-wise, then a word at 0x0C, a
//! dword at 0x10, dwords at 0x20/0x24/0x28 and qwords at
//! 0x30/0x38/0x40/0x48; the gaps (0x0E, 0x14-0x1F, 0x2C-0x2F) are left alone.
export!(thiscall, rw_00872030(dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let mut i = 0usize;
        while i < 12 {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
        *(dst.add(0x0C) as *mut u16) = *(src.add(0x0C) as *const u16);
        *(dst.add(0x10) as *mut u32) = *(src.add(0x10) as *const u32);
        *(dst.add(0x20) as *mut u32) = *(src.add(0x20) as *const u32);
        *(dst.add(0x24) as *mut u32) = *(src.add(0x24) as *const u32);
        *(dst.add(0x28) as *mut u32) = *(src.add(0x28) as *const u32);
        for off in [0x30usize, 0x38, 0x40, 0x48] {
            (dst.add(off) as *mut u64)
                .write_unaligned((src.add(off) as *const u64).read_unaligned());
        }
        dst as u32
    }
});
