// original: 0x0065A590 rage::fragHeapAllocator::vf15
use lf_checker_rt::export;

/// Return one when the allocator's global state word is zero, and zero
/// otherwise. The ECX object value is unused; the result is returned in EAX.
export!(thiscall, rw_0065a590(_this: u32) -> u32 {
    const ALLOCATOR_STATE: u32 = 0x017E_D8D8;
    let state = unsafe { lf_checker_rt::global::<u32>(ALLOCATOR_STATE).read_unaligned() };
    u32::from(state == 0)
});
