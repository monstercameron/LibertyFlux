// original: 0x0097b2c0 audio_voice_read_transform
/// Read a voice transform into an output matrix.
///
/// Zeroes the output, then follows a three-link chain from the voice object,
/// returning early with zero when any link is null. Otherwise resolves a
/// table entry through two helper calls, has the reader call fill four words
/// at an offset into a frame block, and copies those four words to the output.
///
/// The contract skips the reader call's ECX address and snapshots its four
/// input words (zeros); the four output words past them are scripted. The
/// The first call's five stack words are all compared by value.
export!(thiscall, rw_0097b2c0(this: u32, out: u32) -> u32 {
    unsafe {
        let slot = out as *mut u32;
        *slot = 0;
        *slot.add(1) = 0;
        *slot.add(2) = 0;
        let link1 = *((this + 0x120) as *const u32);
        if link1 == 0 {
            return 0;
        }
        let link2 = *((link1 + 0x7b4) as *const u32);
        if link2 == 0 {
            return 0;
        }
        let link3 = *((link2 + 4) as *const u32);
        if link3 == 0 {
            return 0;
        }
        let key = *((link3 + 0xc) as *const u32);
        let table = callee_thiscall!(1, u32, this, key, 0, relocated(0x1152630), relocated(0x1152784), 0);
        let entry = callee_cdecl!(2, u32, *((this + 0x120) as *const u32),
                                  *((this + 0x70) as *const u32));
        let row = entry.wrapping_shl(6)
            .wrapping_add(*((table + 0x84) as *const u32));
        let blk = [0u32; 16];
        let answer = callee_thiscall!(3, u32, blk.as_ptr() as u32, row);
        *slot = blk[12];
        *slot.add(1) = blk[13];
        *slot.add(2) = blk[14];
        *slot.add(3) = blk[15];
        answer
    }
});
