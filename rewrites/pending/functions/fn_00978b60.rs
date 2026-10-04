// original: 0x00978B60 audio_voice_setup
/// Set up one voice slot from a parameter block.
///
/// Copies the id word, two float parameters and the flags word into the
/// destination slot, resolves the bank key through the bank lookup (callee 1)
/// and then either settles the slot from the resolved bank record and the
/// owner object, or falls back to the legacy resolver (callee 2).
///
/// Returns the settled stamp word, or the legacy resolver's answer on the
/// early path. thiscall(this, dst, src).
export!(thiscall, rw_s103_978b60(this: *mut u8, dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let rd = |p: *const u8, off: usize| *(((p as usize) + off) as *const u32);
        let wr = |p: *mut u8, off: usize, v: u32| *(((p as usize) + off) as *mut u32) = v;
        wr(dst, 0x00, rd(src, 0x10));
        wr(dst, 0x04, rd(src, 0x14));
        wr(dst, 0x08, rd(src, 0x18));
        wr(dst, 0x0C, rd(src, 0x1C));
        let tag = *(((src as usize) + 0x52) as *const u16) as u32;
        let key = rd(src, 0);
        if key != 0 {
            wr(dst, 0x20, callee_cdecl!(1, u32, key));
        } else {
            wr(dst, 0x20, 0);
        }
        wr(dst, 0x50, tag);
        wr(dst, 0x54, 0);
        *dst.add(0x71) = 0;
        wr(dst, 0x58, 0);
        // Finish a slot whose selector is already stored: record the owner
        // field, fill a zero selector from the per-owner table, stamp it.
        let settle = |dst: *mut u8, owner_field: u32| {
            wr(dst, 0x48, owner_field);
            if rd(dst, 0x58) == 0 {
                let table = *global::<u32>(0x1231290);
                wr(dst, 0x58, *(((table as usize) + ((owner_field & 0xFF) as usize) * 4) as *const u32));
            }
            let stamp = *global::<u32>(0x11735B4);
            wr(dst, 0x6C, stamp);
            stamp
        };
        let resolved = rd(dst, 0x20);
        if resolved != 0 && (rd(resolved as usize as *const u8, 0x28) & 0x3C0) == 0x80 {
            let sel = if rd(resolved as usize as *const u8, 0x1304) == 1 {
                *global::<u32>(0x12202D0)
            } else {
                0
            };
            wr(dst, 0x58, sel);
            settle(dst, rd(this, 0xFA44))
        } else {
            let v = callee_thiscall!(2, u32, this as usize as u32, key, tag);
            wr(dst, 0x58, v);
            if v == 0 {
                settle(dst, rd(src, 0x48))
            } else {
                wr(dst, 0x48, 0);
                v
            }
        }
    }
});
