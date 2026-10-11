// original: 0x005F4D00 rage::fragInst::vf24

/// Copy the manager's current fragment value into the associated object when present.
///
/// `this` stores the associated-object pointer at byte offset `0x64`; the target
/// value slot is at `0x0c`. The relocated manager global points to a record at
/// offset `0x2f0`, whose value word is at `0x08`. A null associated-object
/// pointer leaves memory unchanged. This is a thiscall method that discards four
/// stack arguments. Field names stay descriptive because the merged layout gives
/// these words offsets but no stronger semantics.
lf_checker_rt::export!(thiscall, rw_005f4d00(
    this: u32,
    _arg0: u32,
    _arg1: u32,
    _arg2: u32,
    _arg3: u32,
) -> () {
    const ASSOCIATED_OBJECT: u32 = 0x64;
    const ASSOCIATED_VALUE: u32 = 0x0c;
    const MANAGER_GLOBAL: u32 = 0x018B_7448;
    const MANAGER_TYPE: u32 = 0x2f0;
    const TYPE_VALUE: u32 = 0x08;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let associated = read_u32(this.wrapping_add(ASSOCIATED_OBJECT));
        if associated != 0 {
            let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL));
            let type_record = read_u32(manager.wrapping_add(MANAGER_TYPE));
            let value = read_u32(type_record.wrapping_add(TYPE_VALUE));
            write_u32(associated.wrapping_add(ASSOCIATED_VALUE), value);
        }
    }
});
