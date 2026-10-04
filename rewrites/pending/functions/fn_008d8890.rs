// original: 0x008d8890 file_teardown_8890
/// Tear down the file object if present, then tail-call the next stage.
///
/// Calls the fixed-this finalizer, releases the global object through its
/// destructor and the heap free helper when non-null, and returns the
/// tail call's answer. The original ends in a jump; the rewrite makes the
/// same call normally, which the checker observes identically.
export!(cdecl, rw_008d8890() -> u32 {
    unsafe {
        /// Fixed `this` of the finalizer call (file VA).
        const FIRST_THIS: u32 = 0x01173750;
        /// Global slot holding the object pointer (file VA).
        const OBJ_SLOT: u32 = 0x01173748;
        let _: u32 = callee_thiscall!(1, u32, relocated(FIRST_THIS));
        let obj = global::<u32>(OBJ_SLOT).read();
        if obj != 0 {
            let _: u32 = callee_thiscall!(2, u32, obj);
            let _: u32 = callee_cdecl!(3, u32, obj);
        }
        callee_cdecl!(4, u32,)
    }
});
