// original: 0x00b45bb0 CEventAttractor::~CEventAttractor
use lf_checker_rt::{callee_thiscall, export, relocated};
/// Destructor for CEventAttractor::~CEventAttractor: stamp the vtable, release 1 guarded member, tail into the base destructor.
///
/// Stamps the vtable pointer, then for each guarded member slot (0x1c) calls the member cleanup with the slot address when the guard word is non-zero. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45bb0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xeade84);
        let member = *(obj.add(0x1c) as *const u32);
        if member != 0 {
            callee_thiscall!(1, u32, member, (obj as u32).wrapping_add(0x1c));
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});
