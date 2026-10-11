// original: 0x006692C0 rage::ptxEventEffect::vf4

/// For a nonnull record, read the target object at receiver offset `0x9c`.
/// If that target is nonnull, call its slot at vtable offset `0x14` with a
/// zero first argument and `record + 0xa0` second, returning the helper's EAX.
/// A null record returns zero; a null target returns the nonnull record pointer.
/// The receiver is a 32-bit game object pointer; the second and third stack
/// words are unused. The checker contract plants and intercepts the table slot.
const VF4_TARGET_OBJECT_OFFSET: u32 = 0x9c;
const VF4_TARGET_DATA_OFFSET: u32 = 0x00a0;
const VF4_TARGET_CALL_ID: u32 = 1;

lf_checker_rt::export!(thiscall, rw_006692c0(receiver: u32, record: u32, _arg1: u32, _arg2: u32) -> u32 {
    if record == 0 {
        return 0;
    }
    let target = unsafe {
        ((receiver.wrapping_add(VF4_TARGET_OBJECT_OFFSET)) as *const u32).read_unaligned()
    };
    if target == 0 {
        return record;
    }
    let data = record.wrapping_add(VF4_TARGET_DATA_OFFSET);
    unsafe { lf_checker_rt::callee_thiscall!(VF4_TARGET_CALL_ID, u32, target, 0u32, data) }
});
