// original: 0x00CC7720 phInstGta::~phInstGta__deleting_2

/// Runs the base destructor and, when the low bit of `flags` is set, calls the
/// allocator-context helper with the stored context pointer and this object.
/// The context is a relocated image global at `0x0171c128`; the helper receives
/// it in ECX and the object as its one stack argument. The method returns the
/// original object pointer and removes its one stack word. Both helper effects
/// are supplied by the scripted proof callees.
lf_checker_rt::export!(thiscall, rw_00cc7720(this: u32, flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 1;
    const ALLOCATOR_HELPER: u32 = 2;
    const ALLOCATOR_CONTEXT: u32 = 0x0171_c128;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if flags & 1 != 0 {
        let context = unsafe { lf_checker_rt::global::<u32>(ALLOCATOR_CONTEXT).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(ALLOCATOR_HELPER, u32, context, this);
    }
    this
});
