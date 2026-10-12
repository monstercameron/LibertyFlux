// original: 0x00a0b4b0 pool_object_attach (proposed)
/// Attach an object to its tracker, or detach it, by kind.
///
/// Bits 6..9 of the word at `item + 0x28` select the path: kind 4 resolves
/// `arg` through the object pool (null fails), checks the pair, maps it
/// through the mapper (-1 fails), attaches with (arg, mapped, -1) and
/// commits, answering the commit. Kinds 2 and 3 drive the slot at
/// item+0x80's +4 entry with (link, 0) where link is the word at item+0x20
/// plus 0x30 or, when null, item+0x10, then enable and arm the slot's
/// tracker and run the item slot at +0x18, answering that. Any other kind
/// answers the kind itself. Stdcall, three words.
lf_checker_rt::export!(stdcall, rw_00a0b4b0(item: u32, arg: u32, extra: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const SLOT_BASE: u32 = 0x80;
        const LOOKUP: u32 = 0;
        const PAIR_OK: u32 = 1;
        const MAP: u32 = 2;
        const ATTACH: u32 = 3;
        const SLOT_RUN: u32 = 4;
        const ENABLE: u32 = 5;
        const ARM: u32 = 6;
        const ITEM_RUN: u32 = 7;
        const COMMIT: u32 = 8;
        let w = ((item + 0x28) as *const u32).read_unaligned();
        let kind = (w >> 6) & 0x0f;
        if kind == 4 {
            let pool = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
            let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, extra);
            if obj == 0 {
                return 0;
            }
            let ok: u32 = lf_checker_rt::callee_cdecl!(PAIR_OK, u32, item, 1u32);
            if ok == 0 {
                return 0;
            }
            let mapped: u32 = lf_checker_rt::callee_thiscall!(MAP, u32, obj, arg);
            if mapped == 0xffff_ffff {
                return 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH, u32, item, extra, mapped, 0xffff_ffff);
            return lf_checker_rt::callee_cdecl!(COMMIT, u32, item, 1u32);
        }
        if kind == 2 || kind == 3 {
            let link_at = ((item + 0x20) as *const u32).read_unaligned();
            let link = if link_at == 0 { item + 0x10 } else { link_at + 0x30 };
            let slot = item + SLOT_BASE;
            let vtab = (slot as *const u32).read_unaligned();
            let run = ((vtab + 4) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(run as usize);
            let _: u32 = f(slot, link, 0u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(ENABLE, u32, slot, arg, 1u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(ARM, u32, slot, arg);
            let vtab2 = (item as *const u32).read_unaligned();
            let run2 = ((vtab2 + 0x18) as *const u32).read_unaligned();
            let g: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(run2 as usize);
            return g(item);
        }
        kind
    }
});
