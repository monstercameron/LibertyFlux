// original: 0x00afdfd0 poll_binding_state
/// Poll one of two binding testers selected by the top bits of a mode byte
/// and report whether its answer equals 2. Only AL is meaningful; the upper
/// bits of EAX keep their incoming value, which this rewrite reproduces.
export!(cdecl, rw_00afdfd0(a: u32, b: u32) -> u32 {
    unsafe {
        let sel = *((b + 5) as *const u8) >> 6;
        if sel == 0 {
            return b & 0xFFFFFF00;
        }
        let bit = ((*((a + 0x1f) as *const u8) >> 5) & 1) as u32;
        let answer = if sel == 1 {
            callee_cdecl!(1, u32, bit, 0x9c4)
        } else {
            callee_cdecl!(2, u32, bit, 0x9c4)
        };
        if answer == 2 {
            (answer & 0xFFFFFF00) | 1
        } else {
            answer & 0xFFFFFF00
        }
    }
});
