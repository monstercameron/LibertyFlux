// original: 0x00d8c6b0 audio_floats_to_fixed_bytes
/// Packs six scaled floats into signed bytes and copies three tail dwords.
///
/// The floats at `src+0/4/8/0x10/0x14/0x18` (note the gap: `src+0xC` is
/// skipped) are each multiplied by the 127.0 scale and truncated to a byte
/// stored at `this[0xC..0x12]`; then the dwords at `src+0x30/0x34/0x38`
/// are copied to `this+0/4/8`. Returns the last dword.
export!(thiscall, rw_00d8c6b0(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let scale = *global::<f32>(0x00FE_8BC4);
        // Source offsets deliberately skip 0xC.
        let offs = [0usize, 4, 8, 0x10, 0x14, 0x18];
        let mut i = 0;
        while i < 6 {
            let v = *(src.add(offs[i]) as *const f32);
            *this.add(0xC + i) = cvtt_ss2si(v * scale) as u8;
            i += 1;
        }
        *(this as *mut u32) = *(src.add(0x30) as *const u32);
        *(this.add(4) as *mut u32) = *(src.add(0x34) as *const u32);
        let tail = *(src.add(0x38) as *const u32);
        *(this.add(8) as *mut u32) = tail;
        tail
    }
});
