// original: 0x005E16A0 proposed_ped_phase_record_initialize
/// Initialize a 12-byte phase record from its existing 14-bit tag, the shared
/// counter, and one caller-supplied word. The record stores a relocated vtable
/// at offset `0x00`, the mixed tag at `0x04`, and the argument at `0x08`.
/// The counter is one relocated 32-bit global word and increments once. The
/// thiscall method returns the record pointer and consumes one stack word.
lf_checker_rt::export!(thiscall, rw_005e16a0(this: u32, value: u32) -> u32 {
    const COUNTER_FILE_VA: u32 = 0x0103_27a0;
    const PHASE_VTABLE_FILE_VA: u32 = 0x00e7_e080;
    const TAG_MASK: u32 = 0x3fff;
    const TAG_OFFSET: u32 = 4;
    const VALUE_OFFSET: u32 = 8;

    let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
    let old_counter = unsafe { counter.read_unaligned() };
    let old_tag = unsafe { ((this + TAG_OFFSET) as *const u32).read_unaligned() };
    let mixed_bits = (old_tag ^ old_counter) & TAG_MASK;
    unsafe {
        ((this + TAG_OFFSET) as *mut u32).write_unaligned(old_tag ^ mixed_bits);
        ((this + VALUE_OFFSET) as *mut u32).write_unaligned(value);
        counter.write_unaligned(old_counter.wrapping_add(1));
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PHASE_VTABLE_FILE_VA));
    }
    this
});
