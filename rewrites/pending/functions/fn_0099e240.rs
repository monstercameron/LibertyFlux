// original: 0x0099e240 audio_entry_search
/// Search a 16-byte-entry table for the first entry matching a key.
///
/// The owning object (passed in ECX) holds a descriptor pointer at +0x94,
/// falling back to +0x60 when that is null. The descriptor is used only when
/// its tag byte at +0x3b is 4 and its version word at +6 is 2. A seed taken
/// from the descriptor (+0x3c, sign-extended) is resolved through two lookup
/// calls into an entry-group pointer; eight bytes past that pointer lies the
/// entry array, whose length is the dword at group+4 divided by 16. When the
/// group holds more than one entry and the owner's sorted flag (+0x209) is
/// clear, a nested sort call orders the array and the flag is set. The scan
/// then walks the entries for the key kept at owner+index*4+0x70: an entry
/// matches when its first dword equals the key and a saved limit is at least
/// its key at +0xc. On the first entry whose index also exceeds the best
/// index kept at owner+index*4+0x80, the best index is updated and the
/// entry's address is returned; otherwise the result is null.
lf_rb80_rt::export!(thiscall, rw_0099e240(this: u32, index: u32) -> u32 {
    let owner = this;
    let rd32 = |addr: u32| unsafe { (addr as *const u32).read_unaligned() };
    let mut desc = rd32(owner.wrapping_add(0x94));
    if desc == 0 {
        desc = rd32(owner.wrapping_add(0x60));
        if desc == 0 {
            return 0;
        }
    }
    if unsafe { (desc.wrapping_add(0x3b) as *const u8).read_unaligned() } != 4 {
        return 0;
    }
    if unsafe { (desc.wrapping_add(6) as *const u16).read_unaligned() } != 2 {
        return 0;
    }
    let seed = unsafe { (desc.wrapping_add(0x3c) as *const i16).read_unaligned() } as i32 as u32;
    let token = lf_rb80_rt::callee_cdecl!(1, u32, seed);
    let handle = lf_rb80_rt::callee_thiscall!(2, u32, desc);
    let group = lf_rb80_rt::callee_cdecl!(3, u32, token, handle, 0x7deef8b7u32);
    if group == 0 {
        return 0;
    }
    let entries = group.wrapping_add(8);
    let probe = lf_rb80_rt::callee_thiscall!(4, u32, desc);
    let saved = lf_rb80_rt::callee_thiscall!(5, u32, probe, 0);
    let count = rd32(group.wrapping_add(4)) >> 4;
    if count > 1 && unsafe { (owner.wrapping_add(0x209) as *const u8).read_unaligned() } == 0 {
        let end = entries.wrapping_add(count << 4);
        lf_rb80_rt::callee_cdecl!(6, u32, entries, end, 0);
        unsafe { (owner.wrapping_add(0x209) as *mut u8).write_unaligned(1) };
    }
    if count == 0 {
        return 0;
    }
    let key_addr = owner.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x70);
    let best_addr = owner.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x80);
    let key = rd32(key_addr);
    let best = rd32(best_addr) as i32;
    let mut entry = entries;
    let mut i: u32 = 0;
    while i < count {
        if rd32(entry) == key && saved >= rd32(entry.wrapping_add(12)) && (i as i32) > best {
            unsafe { (best_addr as *mut u32).write_unaligned(i) };
            return entries.wrapping_add(i << 4);
        }
        entry = entry.wrapping_add(16);
        i += 1;
    }
    0
});
