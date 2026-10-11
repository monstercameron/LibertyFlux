// original: 0x00c86160 fn_00c86160_task_record_available

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Check whether an indexed task row exists and is not marked with the high
/// flag bit. The all-ones index is a sentinel; the global manager supplies the
/// record base, flag-byte base and signed-independent row stride.
const TASK_MANAGER_GLOBAL: u32 = 0x016FB694;
const TASK_FLAGS_MASK: u8 = 0x80;
const TASK_MANAGER_RECORDS_OFFSET: u32 = 0;
const TASK_MANAGER_FLAGS_OFFSET: u32 = 4;
const TASK_MANAGER_STRIDE_OFFSET: u32 = 0x0C;

unsafe fn task_record_available(index: u32, invert_flag_test: bool) -> u8 {
    if index == u32::MAX {
        return 0;
    }
    let manager = unsafe { read_u32(lf_checker_rt::relocated(TASK_MANAGER_GLOBAL)) };
    let flags_base = unsafe { read_u32(manager + TASK_MANAGER_FLAGS_OFFSET) };
    let flag = unsafe { (flags_base.wrapping_add(index) as *const u8).read_unaligned() };
    if if invert_flag_test { flag & TASK_FLAGS_MASK == 0 } else { flag & TASK_FLAGS_MASK != 0 } {
        return 0;
    }
    let record_base = unsafe { read_u32(manager + TASK_MANAGER_RECORDS_OFFSET) };
    let stride = unsafe { read_u32(manager + TASK_MANAGER_STRIDE_OFFSET) };
    let record = record_base.wrapping_add(index.wrapping_mul(stride));
    if record == 0 || unsafe { read_u32(record) } == 0 { 0 } else { 1 }
}

lf_checker_rt::export!(cdecl, rw_00c86160_task_record_available(index: u32) -> u8 {
    unsafe { task_record_available(index, false) }
});
