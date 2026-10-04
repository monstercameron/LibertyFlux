// original: 0x00cfd6c0 CTaskComplexSetAndGuardArea::vf0

/// Scalar deleting destructor for CTaskComplexSetAndGuardArea: runs the class destructor, then
/// frees the object through the game heap when the caller asked for it.
///
/// `this` (ECX) is the object. `flags` is the deleting-destructor flag word:
/// bit 0 set means "destroy and free", clear means "destroy only". The class
/// destructor (callee 1, thiscall, no stack args) always runs first with
/// ECX = `this`. When bit 0 of the low `flags` byte is set, the global game
/// heap pointer is loaded and the freeing function (callee 2, thiscall, one
/// stack arg = `this`) runs with ECX = heap. Returns `this` in EAX either way.
///
/// Original: 0x00cfd6c0 (thiscall, one stack word). Only the low byte of
/// `flags` is tested; upper bytes are ignored.
lf_checker_rt::export!(thiscall, rw_00cfd6c0(this: u32, flags: u32) -> u32 {
    unsafe {
        const HEAP_GLOBAL: u32 = 0x0167_E2A0;
        const DELETE_FLAG: u32 = 0x01;
        const DTOR_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flags & DELETE_FLAG != 0 {
            let heap = lf_checker_rt::global::<u32>(HEAP_GLOBAL).read();
            lf_checker_rt::callee_thiscall!(FREE_CALLEE, u32, heap, this);
        }
        this
    }
});
