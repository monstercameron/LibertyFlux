// original: 0x008f7b90 input_list_notify_each (proposed)

/// Notify every element through its virtual slot, then re-arm the device.
///
/// `obj` is the input device object. The function walks the element array
/// at `[obj+0]` for the UNSIGNED 16-bit count at `[obj+4]`, calling each
/// element's virtual slot at vtable `+4` (thiscall, ECX = element, no stack
/// words) through the fabricated vtable exactly like the original. It then
/// calls the reset callee (ECX = `obj`), the key-event callee
/// (ECX = `obj`, stack words 0 and 0x3E8), and, when the global device
/// pointer is non-null, zeroes its dword at `+0x944`. The global pointer is
/// returned in EAX.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7b90(obj: u32) -> u32 {
    unsafe {
        const C_VIRT: u32 = 1;
        const C_RESET: u32 = 2;
        const C_KEYEV: u32 = 3;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        const VT_SLOT: u32 = 0x4;
        const KEY_P1: u32 = 0;
        const KEY_P2: u32 = 0x3e8;
        const G_DEV: u32 = 0x1601088;
        const DEV_CLEAR: u32 = 0x944;
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                break;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            let vt = (elem as *const u32).read_unaligned();
            let target = ((vt + VT_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = f(elem);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_RESET, u32, obj);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_KEYEV, u32, obj, KEY_P1, KEY_P2);
        let dev = (lf_checker_rt::global::<u32>(G_DEV)).read_unaligned();
        if dev != 0 {
            ((dev + DEV_CLEAR) as *mut u32).write_unaligned(0);
        }
        dev
    }
});
