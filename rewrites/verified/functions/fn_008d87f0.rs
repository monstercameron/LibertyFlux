// original: 0x008d87f0 file_vcall_87f0
/// Forward two arguments to slot 0x38 of the global object's vtable.
///
/// Same shape as the slot-0x18 forwarder nearby: object pointer from the
/// global slot, vtable from the object, thiscall through the slot.
export!(cdecl, rw_008d87f0(a: u32, b: u32) -> u32 {
    unsafe {
        /// Global slot holding the object pointer (file VA).
        const OBJ_SLOT: u32 = 0x01305348;
        /// Vtable slot index called (byte offset 0x38).
        const SLOT: usize = 0x38 / 4;
        let obj = global::<u32>(OBJ_SLOT).read();
        let vtbl = (obj as *const u32).read() as *const u32;
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(vtbl.add(SLOT).read() as usize);
        f(obj, a, b)
    }
});
