//! Raw Win32 bindings used by the hook engine.
//!
//! Kept dependency-free on purpose: a hand-maintained `extern "system"`
//! block against kernel32 only, so the loader links nothing but the system
//! libraries. All declarations are the standard public Win32 signatures.

// Raw FFI declarations and the calls through them; every caller upholds
// the documented Win32 contract for the function it calls.
#![allow(unsafe_code)]
// Every type, constant and extern declaration below mirrors the public
// Win32 API one-to-one under its documented name; per-item doc comments
// would restate the platform documentation, so the module docs cover them.
#![allow(missing_docs)]

use std::ffi::c_void;

pub type Handle = *mut c_void;
pub type Hmodule = *mut c_void;
pub type FarProc = *mut c_void;
pub type Bool = i32;
pub type Dword = u32;
pub type Word = u16;
pub type Long = i32;

pub const PAGE_READONLY: Dword = 0x02;
pub const PAGE_READWRITE: Dword = 0x04;
pub const PAGE_EXECUTE_READ: Dword = 0x20;
pub const PAGE_EXECUTE_READWRITE: Dword = 0x40;

pub const MEM_COMMIT: Dword = 0x1000;
pub const MEM_RESERVE: Dword = 0x2000;
pub const MEM_RELEASE: Dword = 0x8000;

pub const TH32CS_SNAPTHREAD: Dword = 0x0000_0004;
pub const THREAD_SUSPEND_RESUME: Dword = 0x0002;
pub const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;

pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ThreadEntry32 {
    pub size: Dword,
    pub usage: Dword,
    pub thread_id: Dword,
    pub owner_pid: Dword,
    pub base_pri: Long,
    pub delta_pri: Long,
    pub flags: Dword,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SystemTime {
    pub year: Word,
    pub month: Word,
    pub weekday: Word,
    pub day: Word,
    pub hour: Word,
    pub minute: Word,
    pub second: Word,
    pub millis: Word,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryBasicInformation {
    pub base_address: *mut c_void,
    pub allocation_base: *mut c_void,
    pub allocation_protect: Dword,
    pub region_size: usize,
    pub state: Dword,
    pub protect: Dword,
    pub type_: Dword,
}

impl Default for MemoryBasicInformation {
    fn default() -> Self {
        Self {
            base_address: std::ptr::null_mut(),
            allocation_base: std::ptr::null_mut(),
            allocation_protect: 0,
            region_size: 0,
            state: 0,
            protect: 0,
            type_: 0,
        }
    }
}

#[repr(C)]
pub struct ExceptionRecord {
    pub code: Dword,
    pub flags: Dword,
    pub record: *mut ExceptionRecord,
    pub address: *mut c_void,
    pub num_params: Dword,
    pub info: [usize; 15],
}

#[repr(C)]
pub struct ExceptionPointers {
    pub record: *mut ExceptionRecord,
    pub context: *mut c_void,
}

pub type ExceptionFilter = extern "system" fn(*mut ExceptionPointers) -> Long;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn VirtualProtect(addr: *mut c_void, size: usize, new: Dword, old: *mut Dword) -> Bool;
    pub fn VirtualAlloc(
        addr: *mut c_void,
        size: usize,
        alloc: Dword,
        protect: Dword,
    ) -> *mut c_void;
    pub fn VirtualFree(addr: *mut c_void, size: usize, free_type: Dword) -> Bool;
    pub fn VirtualQuery(
        addr: *const c_void,
        info: *mut MemoryBasicInformation,
        len: usize,
    ) -> usize;
    pub fn FlushInstructionCache(process: Handle, addr: *const c_void, size: usize) -> Bool;
    pub fn GetCurrentProcess() -> Handle;
    pub fn GetCurrentProcessId() -> Dword;
    pub fn GetCurrentThreadId() -> Dword;
    pub fn CreateToolhelp32Snapshot(flags: Dword, pid: Dword) -> Handle;
    pub fn Thread32First(snapshot: Handle, entry: *mut ThreadEntry32) -> Bool;
    pub fn Thread32Next(snapshot: Handle, entry: *mut ThreadEntry32) -> Bool;
    pub fn OpenThread(access: Dword, inherit: Bool, tid: Dword) -> Handle;
    pub fn SuspendThread(handle: Handle) -> Dword;
    pub fn ResumeThread(handle: Handle) -> Dword;
    pub fn CloseHandle(handle: Handle) -> Bool;
    pub fn GetModuleHandleW(name: *const u16) -> Hmodule;
    pub fn GetProcAddress(module: Hmodule, name: *const u8) -> FarProc;
    pub fn LoadLibraryExW(name: *const u16, file: Handle, flags: Dword) -> Hmodule;
    pub fn GetSystemDirectoryW(buf: *mut u16, size: Dword) -> Dword;
    pub fn GetModuleFileNameW(module: Hmodule, buf: *mut u16, size: Dword) -> Dword;
    pub fn SetUnhandledExceptionFilter(filter: Option<ExceptionFilter>) -> Option<ExceptionFilter>;
    pub fn GetSystemTime(time: *mut SystemTime);
    pub fn CreateThread(
        attrs: *mut c_void,
        stack: usize,
        start: extern "system" fn(*mut c_void) -> Dword,
        param: *mut c_void,
        flags: Dword,
        tid: *mut Dword,
    ) -> Handle;
    pub fn Sleep(ms: Dword);
    pub fn GetLastError() -> Dword;
}

/// Rust-allocated `[u16]` copy of a string with a trailing NUL.
#[must_use]
pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// True when `addr..addr+len` sits in committed, readable pages.
#[must_use]
pub fn is_readable(addr: usize, len: usize) -> bool {
    if addr == 0 || len == 0 || len > 1 << 30 {
        return false;
    }
    unsafe {
        let mut info = MemoryBasicInformation::default();
        let got = VirtualQuery(
            addr as *const c_void,
            &raw mut info,
            std::mem::size_of::<MemoryBasicInformation>(),
        );
        if got == 0 {
            return false;
        }
        const MEM_COMMIT: Dword = 0x1000;
        const GUARD: Dword = 0x100;
        const NOACCESS: Dword = 0x01;
        if info.state != MEM_COMMIT {
            return false;
        }
        if info.protect & GUARD != 0 || info.protect & 0xFF == NOACCESS {
            return false;
        }
        let region_end = info.base_address as usize + info.region_size;
        addr.saturating_add(len) <= region_end
    }
}

/// Copy `len` bytes from a possibly foreign address. Fails instead of faulting.
#[must_use]
pub fn read_bytes(addr: usize, len: usize) -> Option<Vec<u8>> {
    if !is_readable(addr, len) {
        return None;
    }
    let mut out = vec![0u8; len];
    unsafe {
        std::ptr::copy_nonoverlapping(addr as *const u8, out.as_mut_ptr(), len);
    }
    Some(out)
}

/// Overwrite code/data bytes: change protection, write, flush, restore.
pub fn patch_memory(addr: usize, bytes: &[u8]) -> Result<(), Dword> {
    if bytes.is_empty() {
        return Ok(());
    }
    unsafe {
        let mut old = 0;
        let ok = VirtualProtect(
            addr as *mut c_void,
            bytes.len(),
            PAGE_EXECUTE_READWRITE,
            &raw mut old,
        );
        if ok == 0 {
            return Err(GetLastError());
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
        FlushInstructionCache(GetCurrentProcess(), addr as *const c_void, bytes.len());
        let mut _tmp = 0;
        VirtualProtect(addr as *mut c_void, bytes.len(), old, &raw mut _tmp);
        Ok(())
    }
}

/// Allocate executable scratch memory (trampolines, adapter stubs).
pub fn alloc_exec(size: usize) -> Result<*mut u8, Dword> {
    unsafe {
        let p = VirtualAlloc(
            std::ptr::null_mut(),
            size.max(1),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        if p.is_null() {
            return Err(GetLastError());
        }
        FlushInstructionCache(GetCurrentProcess(), p, size.max(1));
        Ok(p.cast::<u8>())
    }
}

/// Write bytes into executable memory allocated by [`alloc_exec`] and flush.
///
/// # Safety
/// `dst` must point to `bytes.len()` writable bytes.
pub unsafe fn write_exec(dst: *mut u8, bytes: &[u8]) {
    // SAFETY: upheld by the caller (fresh allocation of matching size).
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        FlushInstructionCache(GetCurrentProcess(), dst as *const c_void, bytes.len());
    }
}

/// Free executable memory allocated by [`alloc_exec`].
// The pointer is only passed to VirtualFree, never dereferenced; a safe
// wrapper is the point of this module, so the fn stays safe.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn free_exec(addr: *mut u8) {
    unsafe {
        VirtualFree(addr.cast::<c_void>(), 0, MEM_RELEASE);
    }
}

/// Suspends every other thread of this process; resumes them on drop.
///
/// Used around multi-byte code patches so no thread can execute a torn
/// instruction. Best-effort per thread: failures are counted, never fatal.
pub struct FreezeGuard {
    handles: Vec<Handle>,
    /// Other threads found in this process (suspended or not).
    pub total_threads: u32,
    /// Threads that could not be suspended (best-effort, never fatal).
    pub failed: u32,
}

impl FreezeGuard {
    /// Suspend every other thread of this process; resumes them on drop.
    #[must_use]
    pub fn suspend_others() -> Self {
        let mut guard = FreezeGuard {
            handles: Vec::new(),
            total_threads: 0,
            failed: 0,
        };
        unsafe {
            let me_tid = GetCurrentThreadId();
            let me_pid = GetCurrentProcessId();
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
            if snap == INVALID_HANDLE_VALUE || snap.is_null() {
                guard.failed += 1;
                return guard;
            }
            let mut te = ThreadEntry32 {
                size: std::mem::size_of::<ThreadEntry32>() as Dword,
                ..Default::default()
            };
            let mut ok = Thread32First(snap, &raw mut te);
            while ok != 0 {
                if te.owner_pid == me_pid && te.thread_id != me_tid {
                    guard.total_threads += 1;
                    let h = OpenThread(THREAD_SUSPEND_RESUME, 0, te.thread_id);
                    if h.is_null() {
                        guard.failed += 1;
                    } else if SuspendThread(h) == 0xFFFF_FFFF {
                        guard.failed += 1;
                        CloseHandle(h);
                    } else {
                        guard.handles.push(h);
                    }
                }
                ok = Thread32Next(snap, &raw mut te);
            }
            CloseHandle(snap);
        }
        guard
    }

    /// Threads actually suspended (and resumed on drop).
    #[must_use]
    pub fn frozen(&self) -> usize {
        self.handles.len()
    }
}

impl Drop for FreezeGuard {
    fn drop(&mut self) {
        unsafe {
            for h in self.handles.drain(..) {
                ResumeThread(h);
                CloseHandle(h);
            }
        }
    }
}
