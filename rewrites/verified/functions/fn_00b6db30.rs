// original: 0x00B6DB30 task_dtor_o38_global_dispatch
/// Destroy a task: stamp the vtable, and while the payload handle is set
/// release it, re-dispatch it through the shared four-argument helper, and
/// forward the answer to the sink, then run the base destructor.
export!(thiscall, rw_00b6db30(this: *mut u8) -> u32 {
    unsafe {
        let payload = *((this.add(0x38)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1F64);
        if payload != 0 {
            callee_cdecl!(1, u32, payload);
            let helper = *(global::<u32>(0x12E22A4));
            let ans = callee_thiscall!(2, u32, helper, payload, 1, 0, 2);
            let sink = *(global::<u32>(0x12BD0C4));
            callee_thiscall!(3, u32, sink, ans);
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
