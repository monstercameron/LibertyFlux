// original: 0x0065a880 rage::ProceduralTextureRenderTargetDef::vf0
use lf_checker_rt::{export, relocated, tls_slot};

/// Set the render-target definition's vtable and, when the low flag bit is
/// set, call the TLS manager's third virtual slot to release the object.
/// ECX carries the object pointer; one stack flag is popped by the function.
export!(thiscall, rw_0065a880(this: *mut u8, delete_flag: u32) -> *mut u8 {
    const OBJECT_VTABLE_VA: u32 = 0x00FE_2E48;
    const TLS_MANAGER_OFFSET: usize = 0x08;
    const VTABLE_SLOT: usize = 3;

    unsafe {
        this.cast::<u32>().write_unaligned(relocated(OBJECT_VTABLE_VA));
        if delete_flag & 1 != 0 {
            let context = tls_slot(0) as *mut u8;
            let manager = context.add(TLS_MANAGER_OFFSET).cast::<*mut u8>().read_unaligned();
            let vtable = manager.cast::<*mut u32>().read_unaligned();
            let _release_entry = vtable.add(VTABLE_SLOT).read_unaligned();
            lf_checker_rt::callee_thiscall!(1, u32, manager as u32, this as u32);
        }
        this
    }
});
