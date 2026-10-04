// original: 0x00b6df00 CTaskSimpleCreateCarAndGetIn::~CTaskSimpleCreateCarAndGetIn__deleting

/// Scalar deleting destructor of `CTaskSimpleCreateCarAndGetIn`.
///
/// `this` arrives in ECX and points to the task object; `flags` is the
/// MSVC scalar-destructor flag word, of which only bit 0 is read. Runs
/// the class destructor through intercepted callee 1, then, only when
/// bit 0 of `flags` is set, loads the heap handle from the global dword
/// and releases `this` through the shared deallocation callee 2 (heap
/// handle in ECX, object pointer on the stack). Returns `this` in EAX
/// on both branches.
///
/// Original: 0x00b6df00 (thiscall: ECX = this, one stack word, callee
/// cleans 4 bytes; destructor at 0x00b6db30, deallocation at
/// 0x004999b0, heap-handle global at 0x0167e2a0).
lf_checker_rt::export!(thiscall, rw_00b6df00(this: u32, flags: u32) -> u32 {
    unsafe {
        /// Recorder id of the class-destructor call.
        const DTOR: u32 = 1;
        /// Recorder id of the shared deallocation call.
        const DEALLOC: u32 = 2;
        /// Global dword holding the heap handle for the deallocation call.
        const TASK_HEAP: u32 = 0x0167_E2A0;
        /// Flag bit selecting the deallocating branch.
        const DELETE_IF_SET: u32 = 1;

        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & DELETE_IF_SET != 0 {
            let heap: u32 = (lf_checker_rt::global::<u32>(TASK_HEAP) as *const u32).read();
            let _: u32 = lf_checker_rt::callee_thiscall!(DEALLOC, u32, heap, this);
        }
        this
    }
});
