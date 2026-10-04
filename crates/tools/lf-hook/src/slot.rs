//! Single-pointer slot hooks: vtable entries and import table slots.
//!
//! A pointer-sized write is atomic on x86, so slot toggles take the patch
//! lock but do not freeze threads; a thread already inside the old target
//! simply finishes there. Page protection is flipped around the write
//! (vtables live in read-only pages).

// A slot handle crosses threads only while held under the slot lock;
// the `unsafe impl Send` below carries that argument.
#![allow(unsafe_code)]

use crate::detour::HookError;
use crate::mem;
use std::sync::Mutex;

static SLOT_LOCK: Mutex<()> = Mutex::new(());

/// One hooked pointer slot. Dropping an enabled hook restores the original.
pub struct SlotHook {
    slot: *mut usize,
    original: usize,
    detour: usize,
    enabled: bool,
}

// Crosses threads only under SLOT_LOCK / registry locks.
unsafe impl Send for SlotHook {}

impl SlotHook {
    /// Read the current slot value. Patches nothing.
    pub fn create(slot: *mut usize, detour: usize) -> Result<Self, HookError> {
        let bytes = mem::read_bytes(slot as usize, 4).ok_or(HookError::UnreadableTarget)?;
        let original = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        Ok(SlotHook {
            slot,
            original,
            detour,
            enabled: false,
        })
    }

    /// Address of the hooked slot.
    #[must_use]
    pub fn slot(&self) -> usize {
        self.slot as usize
    }

    /// Value found in the slot at creation (the unhooked target).
    #[must_use]
    pub fn original(&self) -> usize {
        self.original
    }

    /// Whether the detour is currently written to the slot.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn write(&self, value: usize) -> Result<(), HookError> {
        mem::patch_memory(self.slot as usize, &(value as u32).to_le_bytes())
            .map_err(HookError::PatchFailed)
    }

    /// Write the detour address to the slot (no-op if already enabled).
    pub fn enable(&mut self) -> Result<(), HookError> {
        let _lock = SLOT_LOCK.lock().unwrap();
        if self.enabled {
            return Ok(());
        }
        self.write(self.detour)?;
        self.enabled = true;
        Ok(())
    }

    /// Restore the original address to the slot (no-op if already disabled).
    pub fn disable(&mut self) -> Result<(), HookError> {
        let _lock = SLOT_LOCK.lock().unwrap();
        if !self.enabled {
            return Ok(());
        }
        self.write(self.original)?;
        self.enabled = false;
        Ok(())
    }

    /// True when the slot holds the address matching the current state.
    #[must_use]
    pub fn verify(&self) -> bool {
        let want = if self.enabled {
            self.detour
        } else {
            self.original
        };
        match mem::read_bytes(self.slot as usize, 4) {
            Some(cur) => cur == (want as u32).to_le_bytes(),
            None => false,
        }
    }

    /// Rewrite the address matching the current state (repairs drift).
    pub fn restore(&self) -> Result<(), HookError> {
        let _lock = SLOT_LOCK.lock().unwrap();
        let want = if self.enabled {
            self.detour
        } else {
            self.original
        };
        self.write(want)
    }
}

impl Drop for SlotHook {
    fn drop(&mut self) {
        if self.enabled && mem::is_readable(self.slot as usize, 4) {
            let _ = self.write(self.original);
            self.enabled = false;
        }
    }
}


#[cfg(test)]
mod tests {
    #[cfg(windows)]
    use super::*;

    // SlotHook reads and writes through Win32 memory helpers, so these run
    // on Windows only. The slot is a plain heap word, never game memory.
    #[cfg(windows)]
    #[test]
    fn enable_writes_the_detour_disable_restores() {
        let mut slot: usize = 0xDEAD_BEEF;
        let original = slot;
        let detour = 0x1234_5678usize;
        let mut hook = SlotHook::create(&raw mut slot, detour).unwrap();
        assert_eq!((hook.slot(), hook.original(), hook.is_enabled()), (&raw mut slot as usize, original, false));
        assert!(hook.verify());
        hook.enable().unwrap();
        assert_eq!(slot, detour);
        assert!(hook.is_enabled() && hook.verify());
        // enable is idempotent.
        hook.enable().unwrap();
        assert_eq!(slot, detour);
        hook.disable().unwrap();
        assert_eq!(slot, original);
        assert!(!hook.is_enabled() && hook.verify());
        hook.disable().unwrap();
        assert_eq!(slot, original);
    }

    #[cfg(windows)]
    #[test]
    fn verify_sees_drift_and_restore_repairs_it() {
        let mut slot: usize = 0x1000;
        let detour = 0x2000usize;
        let mut hook = SlotHook::create(&raw mut slot, detour).unwrap();
        hook.enable().unwrap();
        // Something else overwrites the slot: verify must notice.
        slot = 0x9999;
        assert!(!hook.verify());
        hook.restore().unwrap();
        assert_eq!(slot, detour);
        assert!(hook.verify());
    }

    #[cfg(windows)]
    #[test]
    fn drop_restores_an_enabled_slot() {
        let mut slot: usize = 0x5555;
        let original = slot;
        {
            let mut hook = SlotHook::create(&raw mut slot, 0x6666).unwrap();
            hook.enable().unwrap();
            assert_eq!(slot, 0x6666);
        }
        assert_eq!(slot, original);
    }
}
