// original: 0x00b45c60 CEventCommunicateEvent::~CEventCommunicateEvent
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};
/// Destructor for CEventCommunicateEvent::~CEventCommunicateEvent: stamp the vtable, release three members, tail into the base destructor.
///
/// Stamps the vtable pointer, then releases the guarded members at +0x0c (member cleanup, slot cleared), +0x30 (value cleanup, slot cleared) and +0x10 (child slot-0 destructor with flag 1, slot cleared), each only when its guard word is non-zero. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45c60(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xeae50c);
        let member = *(obj.add(0xc) as *const u32);
        if member != 0 {
            callee_thiscall!(1, u32, member, (obj as u32).wrapping_add(0xc));
            *(obj.add(0xc) as *mut u32) = 0;
        }
        let value = *(obj.add(0x30) as *const u32);
        if value != 0 {
            callee_cdecl!(2, u32, value);
            *(obj.add(0x30) as *mut u32) = 0;
        }
        let child = *(obj.add(0x10) as *const u32);
        if child != 0 {
            let vtable = *(child as *const u32);
            let target = *(vtable as *const u32);
            let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            destroy(child, 1);
            *(obj.add(0x10) as *mut u32) = 0;
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});
