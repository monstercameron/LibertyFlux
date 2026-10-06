// original: 0x008f7ed0 input_list_broadcast (proposed)

/// Notify the list head, then every element in order.
///
/// `obj` is the input device object. The function first calls the head
/// callee (thiscall, ECX = `obj`, no stack words), then walks the element
/// array at `[obj+0]` for the UNSIGNED 16-bit count at `[obj+4]`, calling
/// the element callee (thiscall, ECX = element) once per entry. The count
/// is re-read every iteration. The contract pins the count to 0..3 and
/// returns it in EAX on every path, so the full register is compared.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7ed0(obj: u32) -> u32 {
    unsafe {
        const C_HEAD: u32 = 1;
        const C_ELEM: u32 = 2;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        let _: u32 = lf_checker_rt::callee_thiscall!(C_HEAD, u32, obj);
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                return n;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(C_ELEM, u32, elem);
            i += 1;
        }
    }
});
