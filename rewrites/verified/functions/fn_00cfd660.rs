// original: 0x00CFD660 CTaskComplexNewUseCover::~CTaskComplexNewUseCover__deleting

/// Scalar deleting destructor for `CTaskComplexNewUseCover`.
///
/// This is the compiler-generated `operator delete` path behind virtual
/// slot 0: run the class destructor, then free the object only when the
/// caller asked for deletion.
///
/// Calling convention: thiscall. `this` arrives in ECX, one flag word on
/// the stack; callee pops the flag word (the callee pops 4 bytes). Returns `this` in EAX
/// on every path.
///
/// Behaviour: call the class destructor (thiscall, ECX = `this`, no stack
/// arguments; intercepted as callee 0 by the checker). Then test bit 0 of
/// the flag word's low byte: when set, load the allocator context from the
/// global slot and call the release routine (thiscall, ECX = context,
/// one stack argument = `this`; callee 1). When clear, skip the release.
/// The destructor's and release's answers are ignored; the class
/// destructor always runs, even on the non-deleting path.
///
/// Edge cases: only the low bit of the low flag byte is tested, so e.g.
/// `0x100` does not delete while `0x101` does. `this` itself is never
/// dereferenced here; a null `this` still reaches the destructor.
lf_checker_rt::export!(thiscall, rw_00cfd660(this: u32, flags: u32) -> u32 {
    unsafe {
        /// Checker callee ids from this function's contract.
        const DTOR: u32 = 0;
        const RELEASE: u32 = 1;
        /// File VA of the global holding the release routine's context.
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        /// Flag bit selecting the deleting path.
        const DELETE_THIS: u32 = 1;

        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & DELETE_THIS != 0 {
            let allocator = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, allocator, this);
        }
        this
    }
});
