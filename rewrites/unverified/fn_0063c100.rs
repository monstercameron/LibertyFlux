// original: 0x0063C100 rage::ptxModel::vf6

/// Forward this call through the second entry in the callback table stored
/// at model offset `0x174`. The table-slot address is the receiver observed
/// by the original call; pass it in ECX and return the selected method's
/// 32-bit result.
lf_checker_rt::export!(thiscall, rw_0063c100(this: *mut u8) -> u32 {
    unsafe {
        const CALLBACK_TABLE: usize = 0x174;
        const VTABLE_SLOT: usize = 1;
        let table_address = (this.add(CALLBACK_TABLE) as *const u32).read_unaligned();
        let table = table_address as usize as *const u32;
        let method_address = table.add(VTABLE_SLOT).read_unaligned();
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(method_address as usize);
        method(this.add(CALLBACK_TABLE) as u32)
    }
});
