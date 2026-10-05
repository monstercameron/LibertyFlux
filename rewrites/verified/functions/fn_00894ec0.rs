// original: 0x00894ec0 this_adjusting_thunk
/// Sub-object selector: advance past the header, then hand off.
///
/// Takes the object pointer in ECX. Advances it past the object's header
/// to the embedded member, then transfers control to the shared
/// implementation with the member pointer, returning whatever that call
/// answers.
export!(thiscall, rw_00894ec0(this: u32) -> u32 {
    callee_thiscall!(1, u32, this.wrapping_add(0x2D4))
});
