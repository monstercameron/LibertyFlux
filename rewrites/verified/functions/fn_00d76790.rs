// original: 0x00D76790 render_phase_ctor_alloc_init (proposed)

/// Construct a render-phase object and attach an allocated handle.
///
/// Forwards `arg0` to the base constructor, installs the vtable pointer
/// and slot constants (0x1818887, 3), allocates a 0xd0-byte block, and
/// when the allocator returns non-null activates it, links it at `+0x940`
/// (writing `this` back at handle `+0x50`), toggles it with argument 1,
/// registers it, publishes `this` to the global slot and sets `+0x40` to
/// 2. A null allocator answer skips activation; the link store then faults
/// on both sides identically. Returns `this`. Thiscall: one stack word.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const BASE_CTOR: u32 = 1;
const ALLOC: u32 = 2;
const ACTIVATE: u32 = 3;
const TOGGLE: u32 = 4;
const REGISTER: u32 = 5;

export!(thiscall, rw_00d76790(this: u32, arg0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eec43c;
        const SLOT_A: u32 = 0x8d0;
        const SLOT_A_VAL: u32 = 0x0181_8887;
        const SLOT_B: u32 = 0x8f4;
        const ALLOC_SIZE: u32 = 0xd0;
        const HANDLE_OFF: u32 = 0x940;
        const BACKLINK_OFF: u32 = 0x50;
        const PUBLISHED: u32 = 0x0166da10;
        const MODE_OFF: u32 = 0x40;
        callee_thiscall!(BASE_CTOR, u32, this, arg0);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this + SLOT_A) as *mut u32).write_unaligned(SLOT_A_VAL);
        ((this + SLOT_B) as *mut u32).write_unaligned(3);
        let block = callee_cdecl!(ALLOC, u32, ALLOC_SIZE);
        let handle = if block == 0 {
            0
        } else {
            callee_thiscall!(ACTIVATE, u32, block)
        };
        ((this + HANDLE_OFF) as *mut u32).write_unaligned(handle);
        ((handle + BACKLINK_OFF) as *mut u32).write_unaligned(this);
        let h = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        callee_thiscall!(TOGGLE, u32, h, 1);
        let h = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        callee_cdecl!(REGISTER, u32, h);
        global::<u32>(PUBLISHED).write(this);
        ((this + MODE_OFF) as *mut u32).write_unaligned(2);
        this
    }
});
