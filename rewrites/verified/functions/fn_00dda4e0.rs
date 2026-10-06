// original: 0x00dda4e0 UIBasicClip::vf124

/// Set the flag byte on the clip and on its first N parts.
///
/// `this` is the clip object and `flag` is one stack word whose low byte is
/// stored at `FLAG (+0xcb)` of the clip itself. The counter at table slot
/// `COUNT (+0x1d4)` is then called with `this` in ECX; while the unsigned
/// index `i` (starting at 0) is below the returned count, the element
/// `array[i]` (from the table at `this + PARTS (+0x1d4)`) gets the same byte
/// at its `+0xcb`, and the counter is called again. The loop re-reads the
/// count every round, so a count that grows runs longer. The last count
/// read stays in EAX as the result.
///
/// The bound is compared unsigned (`jb`). Counts large enough to separate a
/// signed from an unsigned bound would need billions of loop rounds, so no
/// trial can take them; the contract cycles counts 0 to 4, which cover the
/// empty loop and short loops.
///
/// Original: thiscall, one stack word, callee pops it (the callee pops 4 bytes), word
/// result in EAX.
lf_checker_rt::export!(thiscall, rw_00dda4e0(this: u32, flag: u32) -> u32 {
    const FLAG: u32 = 0xcb;
    const PARTS: u32 = 0x1d4;
    const COUNT: u32 = 0x1d4;
    unsafe {
        let b = (flag & 0xff) as u8;
        ((this + FLAG) as *mut u8).write_unaligned(b);
        let table = (this as *const u32).read_unaligned();
        let counter = ((table + COUNT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(counter as usize);
        let mut count = f(this);
        if count != 0 {
            let mut i: u32 = 0;
            while i < count {
                let array = ((this + PARTS) as *const u32).read_unaligned();
                let elem = ((array + i * 4) as *const u32).read_unaligned();
                ((elem + FLAG) as *mut u8).write_unaligned(b);
                i += 1;
                let table = (this as *const u32).read_unaligned();
                let counter = ((table + COUNT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(counter as usize);
                count = f(this);
            }
        }
        count
    }
});
