// original: 0x00a0a330 cleanup_record_resolve (proposed)
/// Resolve a cleanup descriptor to a live handle and store it at `out`.
///
/// Clears `out` first. An empty descriptor (word at +0x10 zero) and kind 3
/// both succeed at once with a null handle. Kinds 1, 2 and 4 resolve the
/// word at +4 through pools A, B and C; a null object fails. Otherwise the
/// resolver runs on (object, slot, &rec): a null record, a record without
/// flag 0x73, or (when the type table says the kind needs it) a record
/// without the word at +0x38 fails. Else the handle fetched through pool D
/// is stored at `out`. Returns 1 on success, 0 on failure (low byte only).
/// Cdecl with callee cleanup of two words.
lf_checker_rt::export!(stdcall, rw_00a0a330(entry: u32, out: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x012e22a4;
        const POOL_B: u32 = 0x018b6f1c;
        const POOL_C: u32 = 0x01632c60;
        const POOL_D: u32 = 0x012fb214;
        const KIND_TABLE: u32 = 0x01295cd8;
        const LOOKUP: u32 = 0;
        const RESOLVE: u32 = 1;
        const HANDLE: u32 = 2;
        (out as *mut u32).write_unaligned(0);
        let slot = ((entry + 0x10) as *const u32).read_unaligned();
        if slot == 0 {
            return 1;
        }
        let kind = (entry as *const u8).read();
        let pool_va = match kind {
            1 => POOL_A,
            2 => POOL_B,
            4 => POOL_C,
            3 => return 1,
            _ => return 0,
        };
        let pool = (lf_checker_rt::global::<u32>(pool_va) as *const u32).read_unaligned();
        let key = ((entry + 4) as *const u32).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, key);
        if obj == 0 {
            return 0;
        }
        let mut rec = 0u32;
        lf_checker_rt::callee_cdecl!(RESOLVE, u32, obj, slot, &mut rec as *mut u32 as u32);
        if rec == 0 {
            return 0;
        }
        if ((rec + 0x73) as *const u8).read() == 0 {
            return 0;
        }
        let idx = ((rec + 0x2e) as *const i16).read_unaligned() as i32;
        let tab = (lf_checker_rt::global::<u32>(KIND_TABLE) as *const u32)
            .offset(idx as isize)
            .read_unaligned();
        let need = ((tab + 0x52) as *const u16).read_unaligned();
        if need != 0xffff {
            if ((rec + 0x38) as *const u32).read_unaligned() == 0 {
                return 0;
            }
        }
        let poold = (lf_checker_rt::global::<u32>(POOL_D) as *const u32).read_unaligned();
        let h = lf_checker_rt::callee_thiscall!(HANDLE, u32, poold, rec);
        (out as *mut u32).write_unaligned(h);
        1
    }
});
