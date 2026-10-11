// original: 0x00CEAF40 CBike::vf90
//! Initializes the object's subobject at offset `0x1730` through two helper
//! calls. The first helper receives that subobject in ECX and five stack
//! words: the original object pointer, selector `5`, two floating-point
//! words (`0` and `0xBFC90FDB`), and flags `0x00100001`.
//!
//! The method stores the subobject pointer at `0x0F88` before the first call,
//! invokes the configuration helper with the same subobject in ECX, writes
//! `1` to the ready field at `0x0F8C`, and returns the second helper's EAX.
//! Both helpers are scripted by the contract. The function uses x86 thiscall
//! and has no stack arguments of its own.
const SUBOBJECT_OFFSET: usize = 0x1730;
const SUBOBJECT_SLOT: usize = 0x0F88;
const INITIALIZED_FLAG: usize = 0x0F8C;
const RESOURCE_INDEX: u32 = 5;
const INITIAL_FLAGS: u32 = 0x0010_0001;
const FIRST_FLOAT_WORD: u32 = 0;
const SECOND_FLOAT_WORD: u32 = 0xBFC9_0FDB;

#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00ceaf40(this: *mut u8) -> u32 {
    let subobject = unsafe { this.add(SUBOBJECT_OFFSET) };
    unsafe {
        core::ptr::write_unaligned(this.add(SUBOBJECT_SLOT).cast::<*mut u8>(), subobject);
    }
    let initialize: unsafe extern "thiscall" fn(
        *mut u8,
        *mut u8,
        u32,
        u32,
        u32,
        u32,
    ) -> u32 = unsafe { core::mem::transmute(lf_checker_rt::callee_addr(1) as usize) };
    let _ = unsafe {
        initialize(
            subobject,
            this,
            RESOURCE_INDEX,
            FIRST_FLOAT_WORD,
            SECOND_FLOAT_WORD,
            INITIAL_FLAGS,
        )
    };

    let configure: unsafe extern "thiscall" fn(*mut u8) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    let result = unsafe { configure(subobject) };
    unsafe {
        core::ptr::write_unaligned(this.add(INITIALIZED_FLAG).cast::<u32>(), 1);
    }
    result
}
