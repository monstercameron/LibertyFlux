// original: 0x0063B270 rage::ptxRulePropList::vf0

/// Run the ptxRulePropList base destructor first. When the low bit of
/// `cleanup_flags` is set and `this` is non-null, call the TLS-managed cleanup
/// method with the manager in ECX and this object as its one stack argument.
/// Return the original object pointer. The flag word is tested only for bit 0.
lf_checker_rt::export!(thiscall, rw_0063b270(this: *mut u8, cleanup_flags: u32) -> *mut u8 {
    unsafe {
        const BASE_DESTRUCTOR: u32 = 2;
        const TLS_CLEANUP: u32 = 1;
        const TLS_MANAGER_OFFSET: usize = 8;
        const VTABLE_SLOT: usize = 3;

        let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this as u32);
        if cleanup_flags & 1 != 0 && !this.is_null() {
            let tls_block = lf_checker_rt::tls_slot(0) as usize as *const u8;
            let manager = (tls_block.add(TLS_MANAGER_OFFSET) as *const u32).read_unaligned();
            let table_address = (manager as usize as *const u32).read_unaligned();
            let table = table_address as usize as *const u32;
            let method_address = table.add(VTABLE_SLOT).read_unaligned();
            let cleanup: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(method_address as usize);
            let _ = cleanup(manager, this as u32);
        }
    }
    this
});
