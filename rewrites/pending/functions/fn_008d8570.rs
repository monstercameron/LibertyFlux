// original: 0x008d8570 NativeImpl_REQUEST_INTERIOR_MODELS_2
/// Forward one argument to slot 0x18 of the global streaming object's vtable.
///
/// The original loads the object pointer from its global slot, loads the
/// vtable from the object, and tail-calls the slot as a thiscall with the
/// object in ECX. Both sides land on the same checker-planted stub.
export!(cdecl, rw_008d8570(a: u32) -> u32 {
    unsafe {
        /// Global slot holding the streaming object pointer (file VA).
        const OBJ_SLOT: u32 = 0x01305348;
        /// Vtable slot index called (byte offset 0x18).
        const SLOT: usize = 0x18 / 4;
        let obj = global::<u32>(OBJ_SLOT).read();
        let vtbl = (obj as *const u32).read() as *const u32;
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(vtbl.add(SLOT).read() as usize);
        f(obj, a)
    }
});
