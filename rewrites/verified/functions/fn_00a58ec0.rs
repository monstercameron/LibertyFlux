// original: 0x00A58EC0 bound_item_init_axes

/// Initializes an axes bound through five ordered helper calls. The stack
/// argument points to the source record; its pointer field at +4 leads to a
/// nested record whose word at +12 is passed to the conditional bound test.
/// A nonzero low byte from that test enables the registration call. The
/// final helper's full EAX value is returned.
///
/// The object pointer is in ECX and the source record pointer is on the
/// stack. Helper calls own any deeper object mutation.
lf_checker_rt::export!(thiscall, rw_00A58EC0(this: u32, source_record: u32) -> u32 {
    const NESTED_POINTER_FIELD: u32 = 0x04;
    const NESTED_INDEX_FIELD: u32 = 0x0c;
    const ITEM_VIEW: u32 = 0x10;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let nested_record = unsafe { read_u32(source_record.wrapping_add(NESTED_POINTER_FIELD)) };
    let nested_index = unsafe { read_u32(nested_record.wrapping_add(NESTED_INDEX_FIELD)) };
    let item_view = source_record.wrapping_add(ITEM_VIEW);

    lf_checker_rt::callee_thiscall!(1, u32, this);
    lf_checker_rt::callee_thiscall!(2, u32, this, item_view);

    let should_register = lf_checker_rt::callee_thiscall!(3, u32, this, nested_index, source_record) & 0xff != 0;
    if should_register {
        lf_checker_rt::callee_thiscall!(4, u32, this, item_view, 0);
    }

    lf_checker_rt::callee_thiscall!(5, u32, this)
});
