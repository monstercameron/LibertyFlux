// original: 0x00e70240 two_stage_init_1a01cb0
/// Run the first init step on the object at 0x01A01CB0, then tail-call the second.
///
/// Calls the first callee (thiscall/0) with the constant object pointer,
/// then forwards the same pointer to the second callee and returns its
/// answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00e70240() -> u32 {
    unsafe {
        const OBJ: u32 = 0x01A01CB0;
        let obj = relocated(OBJ);
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        lf_checker_rt::callee_thiscall!(2, u32, obj)
    }
});
