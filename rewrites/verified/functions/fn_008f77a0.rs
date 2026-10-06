// original: 0x008f77a0 input_list_collect_flagged (proposed)

/// Collect the indices of flagged elements that accept, then sort them.
///
/// `obj` is the input device object. The free callee (cdecl, the old array
/// word at `[obj+8]`) runs first and both list fields (`[obj+8]`, `[obj+0xC]`)
/// are zeroed. Each of the UNSIGNED 16-bit count at `[obj+4]` elements whose
/// flag byte at `+0x558` has bit 0 set has its virtual slot at vtable `+0xC`
/// called (thiscall, ECX = element, no stack words; only AL of the answer
/// matters); when AL is nonzero the append callee (thiscall, ECX = `obj+8`,
/// one stack word 0x10) yields a slot where the element index is stored.
/// Finally the sort callee runs (cdecl: the zeroed `[obj+8]`, the count,
/// the constant 4, and the comparator address). EAX holds the zeroed count
/// word at return.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f77a0(obj: u32) -> u32 {
    unsafe {
        const C_FREE: u32 = 1;
        const C_VIRT: u32 = 2;
        const C_APPEND: u32 = 3;
        const C_SORT: u32 = 4;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        const LIST_OFF: u32 = 0x8;
        const NITEMS_OFF: u32 = 0xc;
        const FLAG_OFF: u32 = 0x558;
        const VT_SLOT: u32 = 0xc;
        const APPEND_ARG: u32 = 0x10;
        const SORT_ARG: u32 = 4;
        const COMPARE_FN: u32 = 0x8f7d90;
        let old: u32 = ((obj + LIST_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(C_FREE, u32, old);
        ((obj + LIST_OFF) as *mut u32).write_unaligned(0);
        ((obj + NITEMS_OFF) as *mut u32).write_unaligned(0);
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                break;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            if ((elem + FLAG_OFF) as *const u8).read() & 1 != 0 {
                let vt = (elem as *const u32).read_unaligned();
                let target = ((vt + VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                if f(elem) & 0xff != 0 {
                    let slot: u32 = lf_checker_rt::callee_thiscall!(
                        C_APPEND, u32, obj.wrapping_add(LIST_OFF), APPEND_ARG);
                    (slot as *mut u32).write_unaligned(i);
                }
            }
            i += 1;
        }
        let n = ((obj + NITEMS_OFF) as *const u16).read_unaligned() as u32;
        let b = ((obj + LIST_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            C_SORT, u32, b, n, SORT_ARG, lf_checker_rt::relocated(COMPARE_FN));
        ((obj + NITEMS_OFF) as *const u16).read_unaligned() as u32
    }
});
