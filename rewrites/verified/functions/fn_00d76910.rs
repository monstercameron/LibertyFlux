// original: 0x00D76910 CRenderPhaseMirrorReflection::vf0

/// Deleting destructor: run the destructor, free on flag.
///
/// Runs the object destructor, then frees `this` through the scalar
/// operator delete when bit 0 of `flags` is set. Returns `this`.
/// Thiscall: object in `ecx`, one flag word on the stack.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const DTOR: u32 = 1;
const OP_DELETE: u32 = 2;

export!(thiscall, rw_00d76910(this: u32, flags: u32) -> u32 {
    unsafe {
        const FREE_BIT: u32 = 1;
        callee_thiscall!(DTOR, u32, this);
        if flags & FREE_BIT != 0 {
            callee_cdecl!(OP_DELETE, u32, this);
        }
        this
    }
});
