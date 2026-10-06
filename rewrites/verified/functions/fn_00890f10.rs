// original: 0x00890F10 audsound_notify_slot_change
/// Notifies the resolved slot of a change, detaching the old voice first.
///
/// Resolves the slot pointer: null when the byte at `this+4` is 0xff,
/// otherwise `stride * byte + table[idx * 0x6f40 + 0x6f14]` (`stride`/`table`
/// from the globals, `idx` the byte at `this+0x40`, wrapping unsigned). When
/// the voice object at `this+0x74` is non-null, calls its detach slot
/// (virtual slot +4, thiscall with one stack word: 0) and feeds the answer
/// to the attach callee (thiscall: slot, answer). Then calls the sync callee
/// (thiscall: slot, 1) and returns its answer.
/// Original: 0x00890F10 (thiscall, no stack arguments).
export!(thiscall, rw_00890F10(this: *mut u8) -> u32 {
    unsafe {
        const DETACH: u32 = 1;
        const ATTACH: u32 = 2;
        const SYNC: u32 = 3;
        const SLOT_BYTE: usize = 4;
        const INDEX: usize = 0x40;
        const VOICE: usize = 0x74;
        const ROW: u32 = 0x6f40;
        const COL: u32 = 0x6f14;
        const EMPTY: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d968;
        const TABLE_G: u32 = 0x115d988;
        let b = *this.add(SLOT_BYTE);
        let slot = if b == EMPTY {
            0
        } else {
            let stride = *global::<u32>(STRIDE_G);
            let table = *global::<u32>(TABLE_G);
            let idx = *this.add(INDEX) as u32;
            let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL))
                as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(base)
        };
        let voice = *(this.add(VOICE) as *const u32);
        if voice != 0 {
            let vt = (voice as *const u32).read_unaligned();
            let target = ((vt.wrapping_add(4)) as *const u32).read_unaligned();
            let detach: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let ans = detach(voice, 0);
            let _: u32 = callee_thiscall!(ATTACH, u32, slot, ans);
        }
        callee_thiscall!(SYNC, u32, slot, 1)
    }
});
