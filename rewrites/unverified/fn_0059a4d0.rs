// original: 0x0059A4D0 leaderboard_update_init
///
/// Initialise a leaderboard update record.
///
/// Zeroes the header words at +0..+8 and +0x10, builds the sub-object at
/// +0x18 through the first helper, stamps -1 into the words at +0x58 and
/// +0x5C, zeroes the 16 bytes at +0x60..+0x6F, then runs the second helper
/// twice over the sub-object: once after re-zeroing the word at +0x68 and
/// once after storing 0x80000000 at +0x88 with -1 at +0x8C and +0x90.
/// Finally writes the count 4 at +0x10 and zeroes the four bytes at +0x0C.
/// Both helpers keep the object pointer in ECX and take no stack arguments;
/// their return values are ignored. Returns the object pointer.
///
/// thiscall: the record pointer arrives in ECX; no stack arguments.
lf_checker_rt::export!(thiscall, rw_0059a4d0(obj: u32) -> u32 {
    const SUB_AT: u32 = 0x18;
    const COUNT_AT: u32 = 0x10;
    const COUNT: u32 = 4;
    const BYTES_AT: u32 = 0x0C;
    const SENTINEL_A_AT: u32 = 0x58;
    const SENTINEL_B_AT: u32 = 0x5C;
    const ZERO_QWORDS_AT: [u32; 2] = [0x60, 0x68];
    const REZERO_AT: u32 = 0x68;
    const DISCR_AT: u32 = 0x88;
    const DISCR_B_AT: u32 = 0x8C;
    const DISCR_C_AT: u32 = 0x90;
    const BUILD_SUB: u32 = 1;
    const CONFIGURE_SUB: u32 = 2;
    
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u64(address: u32, value: u64) {
        unsafe { (address as *mut u64).write_unaligned(value) }
    }

    unsafe {
        write_u32(obj, 0);
        write_u32(obj.wrapping_add(4), 0);
        write_u32(obj.wrapping_add(8), 0);
        write_u32(obj.wrapping_add(COUNT_AT), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(BUILD_SUB, u32, obj.wrapping_add(SUB_AT));
        write_u32(obj.wrapping_add(SENTINEL_A_AT), 0xFFFF_FFFF);
        write_u32(obj.wrapping_add(SENTINEL_B_AT), 0xFFFF_FFFF);
        for at in ZERO_QWORDS_AT {
            write_u64(obj.wrapping_add(at), 0);
        }
        write_u32(obj.wrapping_add(REZERO_AT), 0);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(CONFIGURE_SUB, u32, obj.wrapping_add(SUB_AT));
        write_u32(obj.wrapping_add(DISCR_AT), 0x8000_0000);
        write_u32(obj.wrapping_add(DISCR_B_AT), 0xFFFF_FFFF);
        write_u32(obj.wrapping_add(DISCR_C_AT), 0xFFFF_FFFF);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(CONFIGURE_SUB, u32, obj.wrapping_add(SUB_AT));
        write_u32(obj.wrapping_add(COUNT_AT), COUNT);
        let mut i = 0u32;
        while i < COUNT {
            ((obj.wrapping_add(BYTES_AT).wrapping_add(i)) as *mut u8).write_unaligned(0);
            i += 1;
        }
        obj
    }
});
