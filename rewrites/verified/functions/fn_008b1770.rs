// original: 0x008B1770 rage::audBiquadFilterEffectPc::vf0 (merged symbol)

/// Configure the biquad's active channel mask and clear its state buffers.
///
/// `count` (argument 0) selects how many entries of the channel table at file
/// VA `0xe7cc08` are tested against the mode byte at argument 1 `+0xe`:
/// `this+0xc1` counts the entries whose bit is set in the mode. `this+0xc0`
/// keeps the low byte of `count`, `this+0xc2` the mode byte. Four state
/// buffers at `this+0x48/0x60/0x78/0x90`, each `counted * 4` bytes, are
/// zeroed through the fill routine (`0xdf9cc0`, cdecl: destination, 0, size),
/// then the coefficient setup (`0x8b1270`, thiscall on `this`, no arguments)
/// runs. Returns 1 in `al`. Original is thiscall with two stack words
/// (the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_008B1770(this: u32, count: u32, desc: u32) -> u32 {
    const FILL: u32 = 1;
    const SETUP: u32 = 2;
    const TABLE_FILE_VA: u32 = 0x00e7_cc08;
    const COUNT_LO: u32 = 0xc0;
    const MATCHED: u32 = 0xc1;
    const MODE: u32 = 0xc2;
    const BUFS: [u32; 4] = [0x48, 0x60, 0x78, 0x90];
    unsafe {
        ((this + COUNT_LO) as *mut u8).write(count as u8);
        let mode = ((desc + 0xe) as *const u8).read();
        ((this + MODE) as *mut u8).write(mode);
        ((this + MATCHED) as *mut u8).write(0);
        if count != 0 {
            let table = lf_checker_rt::relocated(TABLE_FILE_VA);
            let mut i = 0u32;
            while i < count {
                let bit = ((table + i * 4) as *const u32).read_unaligned();
                // `(an instruction of the original)` uses only the low 5 bits of the table byte.
                if mode as u32 & (1u32 << (bit & 31)) != 0 {
                    let m = (this + MATCHED) as *mut u8;
                    m.write(m.read().wrapping_add(1));
                }
                i += 1;
            }
        }
        let n = ((this + MATCHED) as *const u8).read() as u32;
        for b in BUFS {
            lf_checker_rt::callee_cdecl!(FILL, u32, this + b, 0u32, n * 4);
        }
        lf_checker_rt::callee_thiscall!(SETUP, u32, this);
        1
    }
});
