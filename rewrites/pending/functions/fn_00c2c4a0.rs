// original: 0x00c2c4a0 audEntityRadioEmitter::vf1
/// Copies a 4-word vector to the output, or zeroes 3 words.
///
/// When the inner object exists, copies 4 words from its linked vector
/// (or the inline vector when the link is null) and returns the last
/// word. When null, writes 3 zero words and returns the output pointer.
export!(thiscall, rw_00c2c4a0(this: *const u8, out: *mut u32) -> u32 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner == 0 {
            *out = 0;
            *out.add(1) = 0;
            *out.add(2) = 0;
            return out as u32;
        }
        let link = *(inner.wrapping_add(0x20) as *const u32);
        let src = if link != 0 {
            link.wrapping_add(0x30)
        } else {
            inner.wrapping_add(0x10)
        };
        let w0 = *(src as *const u32);
        let w1 = *((src.wrapping_add(4)) as *const u32);
        let w2 = *((src.wrapping_add(8)) as *const u32);
        let w3 = *((src.wrapping_add(12)) as *const u32);
        *out = w0;
        *out.add(1) = w1;
        *out.add(2) = w2;
        *out.add(3) = w3;
        w3
    }
});
