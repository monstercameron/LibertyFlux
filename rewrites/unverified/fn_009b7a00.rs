// original: 0x009B7A00 input_pad_state_is_2 (proposed)

/// Reports whether either of two input slots holds state 2. Reads the pad-table base from its global, queries slot one then slot two through the state callee, and returns 1 in AL on the first answer equal to 2, else 0.
///
/// Original: 0x009B7A00 (cdecl).
lf_checker_rt::export!(cdecl, rw_009B7A00() -> u32 {
    const STATE: u32 = 2;
    const SLOT_ONE: u32 = 0x424;
    const SLOT_TWO: u32 = 0x448;
    const TABLE_GLOBAL_VA: u32 = 0x0118_D804;

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe {
        let table = lf_checker_rt::global::<u32>(TABLE_GLOBAL_VA).read_unaligned();
        let first = lf_checker_rt::callee_thiscall!(1, u32, table + SLOT_ONE);
        if first == STATE {
            return 1;
        }
        let second = lf_checker_rt::callee_thiscall!(2, u32, table + SLOT_TWO);
        if second == STATE {
            return 1;
        }
        0
    }
});
