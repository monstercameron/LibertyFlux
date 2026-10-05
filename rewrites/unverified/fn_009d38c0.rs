// original: 0x009D38C0 guarded_table_slot1_a (proposed)
//
/// Guarded table lookup: reads a flag byte and either faults or returns a table slot.
///
/// `S` is the table descriptor read from a global: base pointer at `+0x00`,
/// flag-byte bias at `+0x04`, row stride at `+0x0c`. The flag byte at
/// `arg + bias` is tested for bit `0x80`; when set the original dereferences a null pointer (a fault).
/// Otherwise the dword at `base + stride * arg + 4` is returned.
/// The multiply is a 32-bit wrapping multiply (signedness immaterial to the
/// low 32 bits); the flag test is a single bit test. Cdecl, one stack argument.
lf_checker_rt::export!(cdecl, rw_009D38C0(arg: u32) -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x016EC774;
        const BASE: u32 = 0x00;
        const BIAS: u32 = 0x04;
        const STRIDE: u32 = 0x0c;
        const FLAG_BIT: u8 = 0x80;
        const SLOT: u32 = 4;
        #[inline(always)]
        unsafe fn rd(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        let s = lf_checker_rt::global::<u32>(GLOBAL).read();
        let base = rd(s + BASE);
        let bias = rd(s + BIAS);
        let stride = rd(s + STRIDE);
        let flag = ((arg.wrapping_add(bias)) as *const u8).read();
        if flag & FLAG_BIT != 0 {
            // Original reads through a null pointer here; reproduce the fault.
            core::ptr::read_volatile(0 as *const u32)
        } else {
            rd(base.wrapping_add(stride.wrapping_mul(arg)).wrapping_add(SLOT))
        }
    }
});
