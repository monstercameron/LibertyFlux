// original: 0x008920d0 aud_sound_relink
/// Relinks a sound from the looked-up parameter block and updates it.
///
/// Resolves the target from the byte at 4 (0xff faults on a null dereference,
/// exactly like the original) and the table index byte, copies the dword at
/// 0xdc into this object's word at 0x80, derives one flag bit from the byte
/// at 0xe8 and the index word at 0xe4, feeds the word to the index callee and
/// finally updates this object with the callee answer, the flag bit and zero.
/// Returns the update answer.
export!(thiscall, rw_008920d0(this: *mut u8) -> u32 {
    unsafe {
        let b = *(this.add(4));
        let idx = *(this.add(0x40)) as u32;
        let stride = *global::<u32>(0x115d968);
        let table = *global::<u32>(0x115d988);
        let entry = *((table
            .wrapping_add(idx.wrapping_mul(0x6f40))
            .wrapping_add(0x6f14)) as *const u32);
        let target = if b == 0xff {
            0
        } else {
            stride.wrapping_mul(b as u32).wrapping_add(entry)
        };
        *(this.add(0x80) as *mut u32) = *((target + 0xdc) as *const u32);
        let flag = ((*((target + 0xe8) as *const u8) >> 2) & 1) as u32;
        // movzx loads the word, but the later cwde re-sign-extends from AX.
        let w = *((target + 0xe4) as *const i16) as i32 as u32;
        // The original passes its own pushed-ECX slot (this pointer with the
        // low byte replaced by the flag) as the middle argument.
        let mixed = (this as u32 & !0xff) | flag;
        let v: u32 = callee_cdecl!(1, u32, w);
        callee_thiscall!(2, u32, this as u32, v, mixed, 0)
    }
});
