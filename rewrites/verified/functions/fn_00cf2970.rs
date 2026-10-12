// original: 0x00CF2970 phMaterialMgrGta::~phMaterialMgrGta

/// Destroy the material array, its table subobject, and the manager itself.
///
/// The destructor installs the relocated manager vtable, reads the array
/// pointer at `0x14` and the 16-bit element count at `0x1A`, calls each
/// element's slot-zero destructor with a null flag, frees the array when the
/// count is nonzero, destroys the table at `this+8`, then tail-delegates to
/// the final base cleanup. Elements are spaced `0x3C` bytes apart.
lf_checker_rt::export!(thiscall, rw_fn_00cf2970(this: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00ed_e9c4;
    const ELEMENT_DESTRUCTOR: u32 = 1;
    const FREE_ARRAY: u32 = 2;
    const TABLE_DESTRUCTOR: u32 = 3;
    const TAIL_CLEANUP: u32 = 4;
    const ARRAY_FIELD: u32 = 0x14;
    const COUNT_FIELD: u32 = 0x1a;
    const ELEMENT_STRIDE: u32 = 0x3c;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
    }
    let array = unsafe {
        (this.wrapping_add(ARRAY_FIELD) as *const u32).read_unaligned()
    };
    let count = unsafe {
        (this.wrapping_add(COUNT_FIELD) as *const u16).read_unaligned()
    };

    for index in 0..u32::from(count) {
        let element = array.wrapping_add(index.wrapping_mul(ELEMENT_STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(ELEMENT_DESTRUCTOR, u32, element, 0);
    }
    if count != 0 {
        let _ = lf_checker_rt::callee_cdecl!(FREE_ARRAY, u32, array);
    }
    let _ = lf_checker_rt::callee_thiscall!(TABLE_DESTRUCTOR, u32, this.wrapping_add(8));
    lf_checker_rt::callee_thiscall!(TAIL_CLEANUP, u32, this)
});
