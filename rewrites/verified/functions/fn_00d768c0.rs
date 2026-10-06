// original: 0x00D768C0 render_phase_dtor_release_handle (proposed)

/// Destroy a render-phase object: detach its handle, then run the base.
///
/// Installs the vtable pointer; when the handle slot at `+0x940` is
/// non-null and the flag byte at `+0x1e` is clear, unlinks the handle
/// (cdecl/1), re-reads the slot and releases it through virtual slot 0
/// with argument 1, then clears the slot. Always clears the published
/// global slot and tail-jumps to the base destructor with `this`.
/// Returns the base destructor's answer. Thiscall: object in `ecx`,
/// no stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const UNLINK: u32 = 1;
const RELEASE_VSLOT: u32 = 2;
const BASE_DTOR: u32 = 3;

export!(thiscall, rw_00d768c0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eec43c;
        const HANDLE_OFF: u32 = 0x940;
        const FLAG_OFF: u32 = 0x1e;
        const PUBLISHED: u32 = 0x0166da10;
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        if handle != 0 && ((this + FLAG_OFF) as *const u8).read() == 0 {
            callee_cdecl!(UNLINK, u32, handle);
            let h = ((this + HANDLE_OFF) as *const u32).read_unaligned();
            if h != 0 {
                let vtable = (h as *const u32).read_unaligned();
                let slot = ((vtable + 0) as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                release(h, 1);
            }
            ((this + HANDLE_OFF) as *mut u32).write_unaligned(0);
        }
        global::<u32>(PUBLISHED).write(0);
        callee_thiscall!(BASE_DTOR, u32, this)
    }
});
