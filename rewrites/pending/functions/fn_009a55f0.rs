// original: 0x009a55f0 script_audio_fill_record
/// Original 0x009a55f0 (unnamed): fill a script-audio record.
///
/// Sets record `idx` (stride 0x70): stores `tag` at +0xc, copies the two
/// NUL-terminated strings into the fields at +0x10 and +0x50, stores `extra`
/// at +0x70 and the low byte of `mode` at +0x78. Returns `extra` with its
/// low byte replaced by the mode byte.
export!(thiscall, rw_009a55f0(
    this_: u32,
    idx: u32,
    tag: u32,
    s1: u32,
    s2: u32,
    extra: u32,
    mode: u32,
) -> u32 {
    unsafe {
        let rec = this_.wrapping_add(idx.wrapping_mul(0x70));
        ((rec + 0xc) as *mut u32).write(tag);
        let mut s = s1;
        let mut d = rec.wrapping_add(0x10).wrapping_sub(s1);
        loop {
            let b = (s as *const u8).read();
            (d.wrapping_add(s) as *mut u8).write(b);
            s = s.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        s = s2;
        d = rec.wrapping_add(0x50).wrapping_sub(s2);
        loop {
            let b = (s as *const u8).read();
            (d.wrapping_add(s) as *mut u8).write(b);
            s = s.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        ((rec + 0x70) as *mut u32).write(extra);
        ((rec + 0x78) as *mut u8).write((mode & 0xff) as u8);
        (extra & !0xff) | (mode & 0xff)
    }
});
