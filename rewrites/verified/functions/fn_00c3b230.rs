// original: 0x00c3b230 CTrain::vf42
/// Refresh this car through helper id 1, then flag a computed slot.
///
/// Calls helper id 1 (thiscall, this; answer ignored). Loads the word at
/// `this+0x38`, reads the word at its `+0x08`, adds the dword found 0x34
/// bytes past the global pointer, and sets bit 3 of the byte at the
/// resulting address. Returns the global pointer. The contract scripts
/// the two addends so the address always lands in heap.
///
/// Original: 0x00c3b230 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3b230(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x38;
        const WORD_OFF: u32 = 8;
        const PTR: u32 = 0x18b896c;
        const DWORD_OFF: u32 = 0x34;
        const BIT: u8 = 8;
        const HELPER: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let o = ((this + LINK) as *const u32).read_unaligned();
        let w = ((o + WORD_OFF) as *const u16).read_unaligned() as u32;
        let g = lf_checker_rt::global::<u32>(PTR).read_unaligned();
        let d = ((g + DWORD_OFF) as *const u32).read_unaligned();
        let slot = w.wrapping_add(d);
        let b = (slot as *const u8).read();
        (slot as *mut u8).write(b | BIT);
        g
    }
});
