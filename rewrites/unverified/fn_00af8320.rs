// original: 0x00AF8320 veh_pick_subobject (proposed)

/// Pick the linked subobject to use for this holder.
///
/// `this` points to a single pointer. When it is null the result is null.
/// Otherwise bits 6..9 of the word at `inner + 0x28` select: value 2 returns
/// the inner pointer itself; value 3 returns the pointer at `inner + 0xB30`,
/// but only when bit 2 of the byte at `inner + 0x26C` is set; anything else
/// returns null.
///
/// Original: 0x00AF8320 (thiscall, no stack arguments, pointer in EAX).
lf_checker_rt::export!(thiscall, rw_00AF8320(this: u32) -> u32 {
    unsafe {
        const KIND_WORD: u32 = 0x28;
        const GUARD_BYTE: u32 = 0x26C;
        const ALT_PTR: u32 = 0xB30;
        let inner = (this as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let kind = (((inner + KIND_WORD) as *const u32).read_unaligned() >> 6) & 0xF;
        if kind == 2 {
            return inner;
        }
        if kind != 3 {
            return 0;
        }
        if ((inner + GUARD_BYTE) as *const u8).read() & 4 == 0 {
            return 0;
        }
        ((inner + ALT_PTR) as *const u32).read_unaligned()
    }
});
