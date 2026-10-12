// original: 0x0059CC30 CNetBlenderBoat::~CNetBlenderBoat

/// Return false only when the repeated virtual query yields a ready boat.
///
/// The object vtable slot at `+0x1C` is called once, then a second time when
/// the first result is nonnull, and a third time when the second result's
/// unsigned word at `+8` equals seven. The final result is ready only when its
/// byte at `+0x1C0` is nonzero. The method is a no-argument thiscall and
/// returns its boolean in `AL`.
lf_checker_rt::export!(thiscall, rw_0059cc30(this: u32) -> u32 {
    const VTABLE_POINTER: u32 = 0;
    const QUERY_SLOT: u32 = 0x1C;
    const QUERY_KIND: u32 = 7;
    const QUERY_KIND_OFFSET: u32 = 8;
    const READY_BYTE: u32 = 0x1C0;
    const TRUE: u32 = 1;

    unsafe {
        let first_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let first_slot = (first_vtable.wrapping_add(QUERY_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(first_slot);
        let first_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        if first_result == 0 {
            return TRUE;
        }

        let second_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let second_slot = (second_vtable.wrapping_add(QUERY_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(second_slot);
        let second_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        let query_kind = (second_result.wrapping_add(QUERY_KIND_OFFSET) as *const u32).read_unaligned();
        if query_kind != QUERY_KIND {
            return TRUE;
        }

        let third_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let third_slot = (third_vtable.wrapping_add(QUERY_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(third_slot);
        let third_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        let ready = (third_result.wrapping_add(READY_BYTE) as *const u8).read_unaligned();
        u32::from(ready == 0)
    }
});
