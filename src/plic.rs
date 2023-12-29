pub use crate::Interrupt;

#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Priority {
    #[doc = "0: Priority 0 (never interrupt)"]
    P0 = 0,
    #[doc = "1: Priority 1"]
    P1 = 1,
    #[doc = "2: Priority 2"]
    P2 = 2,
    #[doc = "3: Priority 3"]
    P3 = 3,
    #[doc = "4: Priority 4"]
    P4 = 4,
    #[doc = "5: Priority 5"]
    P5 = 5,
    #[doc = "6: Priority 6"]
    P6 = 6,
    #[doc = "7: Priority 7"]
    P7 = 7,
}

/// Helper struct for dealing with priority conversion errors.
#[derive(Debug, Copy, Clone)]
pub struct TryFromPriorityError(());

impl Priority {
    /// Converts a number to the corresponding priority level.
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromPriorityError> {
        if value > 7 {
            Err(TryFromPriorityError(()))
        } else {
            // SAFETY: the value is in the range 0..=7
            Ok(unsafe { core::mem::transmute(value) })
        }
    }
}
