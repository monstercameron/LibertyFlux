// original: 0x00B46A30 CEventStaticCountReachedMax::~CEventStaticCountReachedMax__deleting

/// Scalar deleting destructor for `CEventStaticCountReachedMax` (MSVC vftable slot 0).
///
/// `this` is the object in ECX; `flags` is the one stack word (bit 0 =
/// "delete this after destroying", the rest ignored). Always runs the
/// scalar destructor (callee 1, thiscall, ECX = this, no stack args),
/// whose return value is discarded. When bit 0 of `flags` is set, loads
/// the game heap handle from the global dword at `GAME_HEAP_PTR` and
/// frees the object through callee 2 (thiscall: ECX = heap handle, one
/// stack arg = this). Returns `this` in EAX on every path, preserves
/// ESI, and pops its one stack word (the callee pops 4 bytes).
///
/// Edge cases: `flags` values with bit 0 clear skip the free entirely
/// (no heap read, no second call); upper bits of `flags` never matter.
/// The object itself is never dereferenced here: whatever the destructor
/// and the free do to it happens behind the intercepted calls.
///
/// Original: 0x00B46A30 (thiscall, ECX = this, one stack word).
lf_checker_rt::export!(thiscall, rw_00b46a30(this: u32, flags: u32) -> u32 {
    unsafe {
        const SCALAR_DTOR: u32 = 1;
        const HEAP_FREE: u32 = 2;
        const DELETE_FLAG: u32 = 0x1;
        const GAME_HEAP_PTR: u32 = 0x016669c8;
        lf_checker_rt::callee_thiscall!(SCALAR_DTOR, u32, this);
        if (flags & DELETE_FLAG) != 0 {
            let heap = lf_checker_rt::global::<u32>(GAME_HEAP_PTR).read();
            lf_checker_rt::callee_thiscall!(HEAP_FREE, u32, heap, this);
        }
        this
    }
});
