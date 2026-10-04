// original: 0x00986030 audStaticRadioEmitter::vf1
/// Original 0x00986030 `audStaticRadioEmitter::vf1`: copy the emitter origin.
///
/// Copies the four dwords at +0x10 (a position vector) to the caller's buffer.
/// Void in the original; the checker does not compare a return value.
export!(thiscall, rw_00986030(this_: u32, out: u32) -> u32 {
    unsafe {
        for i in 0..4u32 {
            let w = ((this_ + 0x10 + i * 4) as *const u32).read();
            ((out + i * 4) as *mut u32).write(w);
        }
    }
    0
});
