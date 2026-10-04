//! 5-byte jump hooks with trampolines.
//!
//! A hook overwrites the first bytes of a function with `E9 <rel32>` to the
//! detour. The overwritten bytes (whole instructions, at least 5) are copied
//! to a trampoline that ends with a jump back, so the detour can call the
//! original. Relative branches inside the moved range are rewritten to reach
//! the same absolute targets from the trampoline.

// A hook handle crosses threads only while held under the patch lock;
// the `unsafe impl Send` below carries that argument.
#![allow(unsafe_code)]

use crate::decode::{BranchKind, decode};
use crate::mem;
use std::sync::Mutex;

static PATCH_LOCK: Mutex<()> = Mutex::new(());

/// Maximum bytes ever moved into a trampoline.
pub const MAX_MOVED: usize = 16;
/// Size of the `E9 rel32` patch.
pub const PATCH_LEN: usize = 5;

/// Hook creation/installation failure. Inspecting bytes never patches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HookError {
    /// Target memory is not readable.
    UnreadableTarget,
    /// Could not decode instructions at the target.
    DecodeFailed,
    /// Function returns before 5 patchable bytes.
    TooShort,
    /// Needs more than [`MAX_MOVED`] moved bytes.
    TooLong,
    /// Unrelocatable instruction in the patch range.
    Unmovable,
    /// Detour is out of rel32 range from the patch site.
    DetourOutOfRange,
    /// Trampoline allocation failed (Win32 error).
    AllocFailed(u32),
    /// Memory patch failed (Win32 error).
    PatchFailed(u32),
}

impl std::fmt::Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::UnreadableTarget => write!(f, "target memory is not readable"),
            HookError::DecodeFailed => write!(f, "could not decode instructions at target"),
            HookError::TooShort => write!(f, "function returns before 5 patchable bytes"),
            HookError::TooLong => write!(f, "needs more than 16 moved bytes"),
            HookError::Unmovable => write!(f, "unrelocatable instruction in patch range"),
            HookError::DetourOutOfRange => write!(f, "detour is out of rel32 range"),
            HookError::AllocFailed(e) => write!(f, "trampoline alloc failed ({e})"),
            HookError::PatchFailed(e) => write!(f, "memory patch failed ({e})"),
        }
    }
}

/// Whether `create` follows leading thunk jumps to the real body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FollowJumps {
    /// Follow up to 4 leading `E9`/`FF 25` jumps before patching.
    On,
    /// Patch the given address as-is.
    Off,
}

struct PlannedInsn {
    /// Offset within the moved range.
    off: usize,
    len: usize,
    branch: BranchKind,
    /// Signed displacement of a relative branch (0 for non-branches).
    rel: i64,
}

fn plan_overwrite(code: &[u8]) -> Result<Vec<PlannedInsn>, HookError> {
    let mut plans = Vec::new();
    let mut off = 0usize;
    while off < PATCH_LEN {
        let d = decode(&code[off..]).ok_or(HookError::DecodeFailed)?;
        if d.is_ret {
            return Err(HookError::TooShort);
        }
        match d.branch {
            BranchKind::Loop => return Err(HookError::Unmovable),
            BranchKind::RelFull | BranchKind::Rel8 if d.rel_size == 2 => {
                // 16-bit relative branch: refuse rather than widen.
                return Err(HookError::Unmovable);
            }
            _ => {}
        }
        let len = d.len as usize;
        let rel = match d.branch {
            BranchKind::Rel8 => i64::from(code[off + 1] as i8),
            BranchKind::RelFull => i64::from(i32::from_le_bytes([
                code[off + len - 4],
                code[off + len - 3],
                code[off + len - 2],
                code[off + len - 1],
            ])),
            _ => 0,
        };
        plans.push(PlannedInsn {
            off,
            len,
            branch: d.branch,
            rel,
        });
        off += len;
        if off > MAX_MOVED {
            return Err(HookError::TooLong);
        }
    }
    Ok(plans)
}

/// Resolve `E9 rel32` / `FF 25 [disp32]` chains to the final target.
fn follow_jumps(mut target: usize) -> Result<usize, HookError> {
    for _ in 0..4 {
        let code = mem::read_bytes(target, 6).ok_or(HookError::UnreadableTarget)?;
        if code[0] == 0xE9 {
            let rel = i32::from_le_bytes([code[1], code[2], code[3], code[4]]) as isize;
            target = target.wrapping_add(5).wrapping_add(rel as usize);
            continue;
        }
        if code[0] == 0xFF && code[1] == 0x25 {
            let slot = u32::from_le_bytes([code[2], code[3], code[4], code[5]]) as usize;
            let ptr = mem::read_bytes(slot, 4).ok_or(HookError::UnreadableTarget)?;
            target = u32::from_le_bytes([ptr[0], ptr[1], ptr[2], ptr[3]]) as usize;
            continue;
        }
        break;
    }
    Ok(target)
}

fn rel32_bytes(from_end: usize, to: usize) -> Option<[u8; 4]> {
    let delta = (to as i64).wrapping_sub(from_end as i64);
    if (-0x8000_0000..0x8000_0000).contains(&delta) {
        Some((delta as i32).to_le_bytes())
    } else {
        None
    }
}

/// Build trampoline bytes: relocated copies of the moved instructions plus a
/// `push back; ret` tail. `plans` offsets are relative to `target`.
fn build_trampoline(target: usize, code: &[u8], plans: &[PlannedInsn], tramp_at: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for p in plans {
        let src = &code[p.off..p.off + p.len];
        // Absolute target of this branch at its original address.
        let abs = target
            .wrapping_add(p.off)
            .wrapping_add(p.len)
            .wrapping_add(p.rel as usize);
        match p.branch {
            BranchKind::None | BranchKind::Loop => {
                out.extend_from_slice(src);
            }
            BranchKind::Rel8 => {
                // Widen to a 32-bit branch. rel8 Jcc (70..7F) becomes
                // 0F 8x rel32; EB becomes E9 rel32.
                if src[0] == 0xEB {
                    let rel = rel32_bytes(tramp_at + out.len() + 5, abs).expect("rel32 range");
                    out.push(0xE9);
                    out.extend_from_slice(&rel);
                } else {
                    let cc = src[0] & 0x0F;
                    let rel = rel32_bytes(tramp_at + out.len() + 6, abs).expect("rel32 range");
                    out.push(0x0F);
                    out.push(0x80 | cc);
                    out.extend_from_slice(&rel);
                }
            }
            BranchKind::RelFull => {
                if src[0] == 0x0F {
                    out.push(0x0F);
                    out.push(src[1]);
                    let rel = rel32_bytes(tramp_at + out.len() + 4, abs).expect("rel32 range");
                    out.extend_from_slice(&rel);
                } else {
                    out.push(src[0]); // E8 or E9
                    let rel = rel32_bytes(tramp_at + out.len() + 4, abs).expect("rel32 range");
                    out.extend_from_slice(&rel);
                }
            }
        }
    }
    // Absolute tail: push back_addr; ret. Works at any distance.
    let back = (target + plans.iter().map(|p| p.len).sum::<usize>()) as u32;
    out.push(0x68);
    out.extend_from_slice(&back.to_le_bytes());
    out.push(0xC3);
    out
}

/// An installed-or-not 5-byte hook. Created disabled; `enable` patches.
pub struct Detour {
    target: usize,
    detour: usize,
    trampoline: *mut u8,
    moved: usize,
    saved: [u8; MAX_MOVED],
    patch: [u8; PATCH_LEN],
    enabled: bool,
}

// The raw pointer only crosses threads under PATCH_LOCK / registry locks.
unsafe impl Send for Detour {}

impl Detour {
    /// Prepare a hook: resolve jump thunks, decode, build the trampoline.
    /// No target bytes are touched until `enable`.
    pub fn create(target: usize, detour: usize, follow: FollowJumps) -> Result<Self, HookError> {
        let t = match follow {
            FollowJumps::On => follow_jumps(target)?,
            FollowJumps::Off => target,
        };
        let code = mem::read_bytes(t, MAX_MOVED).ok_or(HookError::UnreadableTarget)?;
        let plans = plan_overwrite(&code)?;
        let moved: usize = plans.iter().map(|p| p.len).sum();
        // Detour must be reachable by rel32 from the patch site.
        let rel = rel32_bytes(t + PATCH_LEN, detour).ok_or(HookError::DetourOutOfRange)?;
        let mut patch = [0u8; PATCH_LEN];
        patch[0] = 0xE9;
        patch[1..].copy_from_slice(&rel);

        // Trampoline size: moved bytes + worst-case widening (each Rel8 grows
        // by 4) + 6-byte tail.
        let extra = plans
            .iter()
            .filter(|p| p.branch == BranchKind::Rel8)
            .count()
            * 4;
        let tramp_len = moved + extra + 6;
        let tramp = mem::alloc_exec(tramp_len).map_err(HookError::AllocFailed)?;
        let bytes = build_trampoline(t, &code, &plans, tramp as usize);
        debug_assert!(bytes.len() == tramp_len);
        // SAFETY: `tramp` is a fresh `tramp_len`-byte allocation.
        unsafe { mem::write_exec(tramp, &bytes) };

        let mut saved = [0u8; MAX_MOVED];
        saved[..moved].copy_from_slice(&code[..moved]);
        Ok(Detour {
            target: t,
            detour,
            trampoline: tramp,
            moved,
            saved,
            patch,
            enabled: false,
        })
    }

    /// Patched address (after thunk following, if enabled).
    #[must_use]
    pub fn target(&self) -> usize {
        self.target
    }

    /// Address the patch jumps to.
    #[must_use]
    pub fn detour(&self) -> usize {
        self.detour
    }

    /// Callable original. Cast to the function's real type at the call site.
    #[must_use]
    pub fn trampoline(&self) -> usize {
        self.trampoline as usize
    }

    /// Bytes moved into the trampoline (5..=16).
    #[must_use]
    pub fn moved_len(&self) -> usize {
        self.moved
    }

    /// Whether the patch is currently written to the target.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Patch the target. Suspends other threads across the write.
    pub fn enable(&mut self) -> Result<usize, HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        if self.enabled {
            return Ok(0);
        }
        let freeze = mem::FreezeGuard::suspend_others();
        let frozen = freeze.frozen();
        let r = mem::patch_memory(self.target, &self.patch);
        drop(freeze);
        r.map_err(HookError::PatchFailed)?;
        self.enabled = true;
        Ok(frozen)
    }

    /// Restore the original bytes.
    pub fn disable(&mut self) -> Result<usize, HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        if !self.enabled {
            return Ok(0);
        }
        let freeze = mem::FreezeGuard::suspend_others();
        let frozen = freeze.frozen();
        let r = mem::patch_memory(self.target, &self.saved[..self.moved]);
        drop(freeze);
        r.map_err(HookError::PatchFailed)?;
        self.enabled = false;
        Ok(frozen)
    }

    /// True when the target currently holds the expected bytes.
    #[must_use]
    pub fn verify(&self) -> bool {
        let want: &[u8] = if self.enabled {
            &self.patch
        } else {
            &self.saved[..self.moved]
        };
        match mem::read_bytes(self.target, want.len()) {
            Some(cur) => cur == want,
            None => false,
        }
    }

    /// Re-apply the expected bytes (used by the integrity self-check).
    pub fn restore(&self) -> Result<(), HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        let want: &[u8] = if self.enabled {
            &self.patch
        } else {
            &self.saved[..self.moved]
        };
        mem::patch_memory(self.target, want).map_err(HookError::PatchFailed)
    }
}

impl Drop for Detour {
    fn drop(&mut self) {
        // Best-effort unpatch; a failing restore is still better than a
        // dangling jump into freed trampoline memory... but only when the
        // target is still mapped (process teardown order is not guaranteed).
        if self.enabled && mem::is_readable(self.target, self.moved) {
            let _ = mem::patch_memory(self.target, &self.saved[..self.moved]);
            self.enabled = false;
        }
        if !self.trampoline.is_null() {
            mem::free_exec(self.trampoline);
            self.trampoline = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_counts_whole_instructions() {
        // nop;nop;nop;mov eax,imm32;ret  -> 1+1+1+5 = 8 moved bytes
        let code = [0x90, 0x90, 0x90, 0xB8, 1, 2, 3, 4, 0xC3];
        let plans = plan_overwrite(&code).unwrap();
        let total: usize = plans.iter().map(|p| p.len).sum();
        assert_eq!(total, 8);
    }

    #[test]
    fn plan_refuses_short_function() {
        // xor eax,eax;ret -> ret before 5 bytes
        let code = [0x33, 0xC0, 0xC3, 0x90, 0x90, 0x90];
        assert!(matches!(plan_overwrite(&code), Err(HookError::TooShort)));
    }

    #[test]
    fn rel32_range_check() {
        assert!(rel32_bytes(0x1000, 0x2000).is_some());
        assert!(rel32_bytes(0x1000, 0xFFFF_FFFF).is_none());
    }
    fn moved(code: &[u8]) -> usize {
        plan_overwrite(code).unwrap().iter().map(|p| p.len).sum()
    }

    /// Trampoline size `Detour::create` allocates for these bytes: every
    /// rel8 branch is assumed to grow by 4.
    fn allocated_len(code: &[u8]) -> usize {
        let plans = plan_overwrite(code).unwrap();
        let rel8 = plans
            .iter()
            .filter(|p| p.branch == BranchKind::Rel8)
            .count();
        moved(code) + rel8 * 4 + 6
    }

    /// Exact size `build_trampoline` emits: a rel8 Jcc grows by 4
    /// (`0F 8x rel32`), a short `EB` jump by 3 (`E9 rel32`).
    fn built_len(code: &[u8]) -> usize {
        let plans = plan_overwrite(code).unwrap();
        let growth: usize = plans
            .iter()
            .filter(|p| p.branch == BranchKind::Rel8)
            .map(|p| if code[p.off] == 0xEB { 3 } else { 4 })
            .sum();
        moved(code) + growth + 6
    }

    fn rel_at(bytes: &[u8], at: usize) -> i64 {
        i64::from(i32::from_le_bytes([
            bytes[at],
            bytes[at + 1],
            bytes[at + 2],
            bytes[at + 3],
        ]))
    }

    #[test]
    fn plan_stops_at_the_first_boundary_past_five() {
        // push ebp; mov ebp,esp; sub esp,0x10 -> 1 + 2 + 3 = 6 bytes
        assert_eq!(moved(&[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, 0xCC]), 6);
        // mov edi,edi; push ebp; mov ebp,esp -> exactly 5
        assert_eq!(moved(&[0x8B, 0xFF, 0x55, 0x8B, 0xEC, 0xCC]), 5);
        // A long first instruction moves alone.
        assert_eq!(moved(&[0xC7, 0x05, 1, 2, 3, 4, 5, 6, 7, 8, 0xC3]), 10);
    }

    #[test]
    fn plan_refusals() {
        // loop rel8 inside the range
        assert_eq!(
            plan_overwrite(&[0x90, 0xE2, 0xFE, 0x90, 0x90, 0x90]).err(),
            Some(HookError::Unmovable)
        );
        // call rel16 (operand-size prefix): refused rather than widened
        assert_eq!(
            plan_overwrite(&[0x66, 0xE8, 0x10, 0x00, 0x90, 0x90]).err(),
            Some(HookError::Unmovable)
        );
        // undecodable bytes
        assert_eq!(
            plan_overwrite(&[0x0F, 0x04, 0x90, 0x90, 0x90, 0x90]).err(),
            Some(HookError::DecodeFailed)
        );
        // ret imm16 before five bytes
        assert_eq!(
            plan_overwrite(&[0x58, 0xC2, 0x04, 0x00, 0x90, 0x90]).err(),
            Some(HookError::TooShort)
        );
        // four nops, then a 13-byte instruction: 17 moved bytes > 16
        let mut code = vec![0x90, 0x90, 0x90, 0x90, 0x64, 0x65, 0x81, 0x84, 0x24];
        code.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 0xC3]);
        assert_eq!(plan_overwrite(&code).err(), Some(HookError::TooLong));
    }

    #[test]
    fn trampoline_copies_plain_code_and_jumps_back() {
        let code = [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, 0xCC];
        let plans = plan_overwrite(&code).unwrap();
        let t = build_trampoline(0x0040_1000, &code, &plans, 0x2000_0000);
        assert_eq!(t.len(), built_len(&code));
        assert_eq!(&t[..6], &code[..6]);
        // push 0x00401006; ret
        assert_eq!(&t[6..], &[0x68, 0x06, 0x10, 0x40, 0x00, 0xC3]);
    }

    #[test]
    fn trampoline_relocates_relative_branches() {
        let target = 0x0040_1000usize;
        let tramp = 0x2000_0000usize;
        // jz +5 (rel8) at 0; call rel32 at 2; jmp short -2 at 7 (moved range ends at 9)
        let code = [0x74, 0x05, 0xE8, 0x10, 0x00, 0x00, 0x00, 0xEB, 0xFE, 0xCC];
        // Only the first boundary at or past 5 counts: jz(2) + call(5) = 7.
        let plans = plan_overwrite(&code).unwrap();
        assert_eq!(plans.len(), 2);
        let t = build_trampoline(target, &code, &plans, tramp);
        assert_eq!(t.len(), built_len(&code));
        // jz widened to 0F 84 rel32, still reaching target + 2 + 5.
        assert_eq!(&t[..2], &[0x0F, 0x84]);
        assert_eq!(tramp as i64 + 6 + rel_at(&t, 2), (target + 2 + 5) as i64);
        // call rel32 re-aimed at target + 7 + 0x10.
        assert_eq!(t[6], 0xE8);
        assert_eq!(
            tramp as i64 + 11 + rel_at(&t, 7),
            (target + 7 + 0x10) as i64
        );
        // back to target + 7
        assert_eq!(&t[11..], &[0x68, 0x07, 0x10, 0x40, 0x00, 0xC3]);
    }

    #[test]
    fn trampoline_widens_short_jumps_and_keeps_near_jcc() {
        let target = 0x0040_2000usize;
        let tramp = 0x1000_0000usize;
        // jmp short +3 at 0, then jne rel32 at 2
        let code = [0xEB, 0x03, 0x0F, 0x85, 0x20, 0x00, 0x00, 0x00, 0xCC];
        let plans = plan_overwrite(&code).unwrap();
        let t = build_trampoline(target, &code, &plans, tramp);
        assert_eq!(t.len(), built_len(&code));
        assert_eq!(t[0], 0xE9);
        assert_eq!(tramp as i64 + 5 + rel_at(&t, 1), (target + 2 + 3) as i64);
        assert_eq!(&t[5..7], &[0x0F, 0x85]);
        assert_eq!(
            tramp as i64 + 11 + rel_at(&t, 7),
            (target + 8 + 0x20) as i64
        );
    }

    #[test]
    fn allocation_always_covers_the_trampoline() {
        // `Detour::create` allocates `allocated_len` and writes the built
        // bytes into it, so the built trampoline must never be longer.
        // Note: for a short `EB` jump the two differ by one byte, so the
        // `debug_assert!` of equality in `create` would fire in a debug
        // build; release builds only leave the byte unused.
        let samples: [&[u8]; 4] = [
            &[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, 0xCC],
            &[0x74, 0x05, 0xE8, 0x10, 0x00, 0x00, 0x00, 0xCC],
            &[0xEB, 0x03, 0x0F, 0x85, 0x20, 0x00, 0x00, 0x00, 0xCC],
            &[0x7C, 0x01, 0x75, 0x02, 0x90, 0x90, 0xCC],
        ];
        for code in samples {
            let plans = plan_overwrite(code).unwrap();
            let built = build_trampoline(0x0040_0000, code, &plans, 0x3000_0000).len();
            assert_eq!(built, built_len(code));
            assert!(built <= allocated_len(code));
        }
        let short_jump: &[u8] = &[0xEB, 0x03, 0x0F, 0x85, 0x20, 0x00, 0x00, 0x00, 0xCC];
        assert_eq!(allocated_len(short_jump) - built_len(short_jump), 1);
    }

    #[test]
    fn error_messages_name_the_cause() {
        assert_eq!(
            HookError::TooLong.to_string(),
            "needs more than 16 moved bytes"
        );
        assert_eq!(
            HookError::PatchFailed(5).to_string(),
            "memory patch failed (5)"
        );
    }

    // Live memory (Win32): runs on Windows only. Bytes are patched in a
    // private executable buffer and compared; nothing is executed.
    #[cfg(windows)]
    #[test]
    fn enable_and_disable_patch_and_restore_bytes() {
        let body = [0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, 0x8B, 0xE5, 0x5D, 0xC3];
        let buf = mem::alloc_exec(64).unwrap();
        // SAFETY: `buf` is a fresh 64-byte allocation.
        unsafe { mem::write_exec(buf, &body) };
        let target = buf as usize;
        let detour = target + 32;
        let mut hook = Detour::create(target, detour, FollowJumps::Off).unwrap();
        assert_eq!(
            (hook.target(), hook.detour(), hook.moved_len()),
            (target, detour, 6)
        );
        assert!(!hook.is_enabled() && hook.verify());
        // The trampoline starts with the moved bytes.
        assert_eq!(mem::read_bytes(hook.trampoline(), 6).unwrap(), &body[..6]);
        hook.enable().unwrap();
        let patched = mem::read_bytes(target, 5).unwrap();
        assert_eq!(patched[0], 0xE9);
        assert_eq!(target as i64 + 5 + rel_at(&patched, 1), detour as i64);
        assert!(hook.is_enabled() && hook.verify());
        hook.disable().unwrap();
        assert_eq!(mem::read_bytes(target, 10).unwrap(), &body);
        // Enabled hooks restore the original bytes when dropped.
        hook.enable().unwrap();
        drop(hook);
        assert_eq!(mem::read_bytes(target, 10).unwrap(), &body);
        mem::free_exec(buf);
    }

    #[cfg(windows)]
    #[test]
    fn create_follows_a_leading_jump() {
        // jmp +3 to the real body three bytes further on.
        let mut bytes = vec![0xE9, 0x03, 0x00, 0x00, 0x00, 0xCC, 0xCC, 0xCC];
        bytes.extend_from_slice(&[0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x10, 0xC3]);
        let buf = mem::alloc_exec(64).unwrap();
        // SAFETY: `buf` is a fresh 64-byte allocation.
        unsafe { mem::write_exec(buf, &bytes) };
        let hook = Detour::create(buf as usize, buf as usize + 40, FollowJumps::On).unwrap();
        assert_eq!(hook.target(), buf as usize + 8);
        drop(hook);
        mem::free_exec(buf);
    }
}
