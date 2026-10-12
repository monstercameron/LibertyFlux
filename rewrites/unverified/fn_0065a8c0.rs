// original: 0x0065a8c0 rage::ProceduralTextureShaderDef::vf0
use lf_checker_rt::{export, relocated, tls_slot};

/// Set the shader definition's vtable, release its counted buffer when the
/// stored count is nonzero, then release the object when the low flag bit is
/// set. ECX is the object pointer; the function pops one stack flag.
export!(thiscall, rw_0065a8c0(this: *mut u8, delete_flag: u32) -> *mut u8 {
    const OBJECT_VTABLE_VA: u32 = 0x00FE_2E3C;
    const BUFFER_OFFSET: usize = 0xC8;
    const BUFFER_COUNT_OFFSET: usize = 0xCE;
    const CONTEXT_MANAGER_OFFSET: usize = 0x08;
    const MANAGER_SLOT: usize = 3;

    unsafe {
        this.cast::<u32>().write_unaligned(relocated(OBJECT_VTABLE_VA));
        let buffer = this.add(BUFFER_OFFSET).cast::<*mut u8>().read_unaligned();
        let count = this.add(BUFFER_COUNT_OFFSET).cast::<u16>().read_unaligned() as u32;
        if count != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this as u32, buffer as u32, count);
        }
        if delete_flag & 1 != 0 {
            let context = tls_slot(0) as *mut u8;
            let manager = context.add(CONTEXT_MANAGER_OFFSET).cast::<*mut u8>().read_unaligned();
            let vtable = manager.cast::<*mut u32>().read_unaligned();
            let _release_entry = vtable.add(MANAGER_SLOT).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, manager as u32, this as u32);
        }
        this
    }
});
