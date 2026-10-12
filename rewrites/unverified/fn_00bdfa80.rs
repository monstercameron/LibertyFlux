// original: 0x00BDFA80 CTaskSimpleWaitUntilAreaCodesMatch::~CTaskSimpleWaitUntilAreaCodesMatch (proposed)

/// Scalar destructor of `CTaskSimpleWaitUntilAreaCodesMatch`.
///
/// `this` arrives in ECX and points to the task object. Reads the timing
/// node pointer from slot `+0x50`, stamps the class vtable, then, only
/// when the slot holds a non-null pointer, detaches the node through
/// intercepted callee 1 (node pointer in ECX, slot address as the single
/// stack argument). Falls through to the base-class destructor, which the
/// checker intercepts as a tail call (callee 2, object pointer in ECX);
/// the rewrite forwards it as a call and returns its answer, matching the
/// original's fall-through into the tail stub.
///
/// Original: 0x00BDFA80 (thiscall: ECX = this, no stack words; detach at
/// 0x00967280, base destructor at 0x00BDF9F0 reached by tail jump,
/// vtable at 0x00EB93A4).
lf_checker_rt::export!(thiscall, rw_00bdfa80(this: u32) -> u32 {
    unsafe {
        /// Recorder id of the timing-node detach call.
        const DETACH: u32 = 1;
        /// Recorder id of the base-destructor tail call.
        const BASE_DTOR: u32 = 2;
        /// Offset of the timing-node slot from the object base.
        const NODE_SLOT: u32 = 0x50;
        /// Class vtable stamped by this destructor.
        const VTABLE: u32 = 0x00EB_93A4;

        let slot = this.wrapping_add(NODE_SLOT);
        let node = (slot as *const u32).read();
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        if node != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, node, slot);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
