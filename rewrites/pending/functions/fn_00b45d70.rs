// original: 0x00b45d70 CEventGivePedTask::~CEventGivePedTask
use lf_checker_rt::{callee_thiscall, export, relocated};
/// Destructor for CEventGivePedTask::~CEventGivePedTask: stamp the vtable, release the owned child, tail into the base destructor.
///
/// Stamps the vtable pointer, then, only when the child pointer at +0x10 is non-null, calls its slot-0 destructor with flag 1. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45d70(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xeae05c);
        let child = *(obj.add(0x10) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let target = *(vtable as *const u32);
            let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            destroy(child, 1);
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});
