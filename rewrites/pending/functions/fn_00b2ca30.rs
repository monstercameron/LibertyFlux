// original: 0x00b2ca30 garage_record_copy
// 0xB2CA30 garage_record_copy (thiscall/1).
//
// Copies a record field by field from src into dst: three tag bytes land at
// the front, sixteen dwords follow at +4, the dword at +0x40 is copied in
// place (src+0x3c is skipped), and two more tag bytes close at +0x44.
export!(thiscall, rw_00b2ca30(dst: *mut u8, src: *const u8) -> () {
    unsafe {
        *dst = *src.add(0x48);
        *dst.add(1) = *src.add(0x49);
        *dst.add(2) = *src.add(0x4C);
        let mut word = 0usize;
        while word < 16 {
            *(dst.add(4 + word * 4) as *mut u32) = *(src.add(word * 4) as *const u32);
            word += 1;
        }
        *(dst.add(0x40) as *mut u32) = *(src.add(0x40) as *const u32);
        *dst.add(0x44) = *src.add(0x4A);
        *dst.add(0x45) = *src.add(0x4B);
    }
});
