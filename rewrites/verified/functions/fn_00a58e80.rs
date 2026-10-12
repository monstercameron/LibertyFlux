// original: 0x00A58E80 bound_item_init_merge_box

/// Initializes a merge-box bound through five ordered helper calls. The
/// second stack argument is the source item; the first is forwarded with it
/// to the conditional bound test. A nonzero low byte from that test enables
/// one additional registration call. The final helper's full EAX value is
/// the function result.
///
/// The object pointer is in ECX and two 32-bit arguments are on the stack.
/// The helpers own the underlying object writes; this function only derives
/// the source item's +16 view and sequences the calls.
lf_checker_rt::export!(thiscall, rw_00A58E80(this: u32, first_input: u32, source_item: u32) -> u32 {
    const ITEM_VIEW: u32 = 0x10;

    lf_checker_rt::callee_thiscall!(1, u32, this);

    let item_view = source_item.wrapping_add(ITEM_VIEW);
    lf_checker_rt::callee_thiscall!(2, u32, this, item_view);

    let should_register = lf_checker_rt::callee_thiscall!(3, u32, this, first_input, source_item) & 0xff != 0;
    if should_register {
        lf_checker_rt::callee_thiscall!(4, u32, this, item_view, 0);
    }

    lf_checker_rt::callee_thiscall!(5, u32, this)
});
