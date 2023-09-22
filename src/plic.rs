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

/// PLIC context of the target
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum Context {
    #[doc = "0: Context 0"]
    C0 = 0,
}

unsafe impl riscv_peripheral::plic::InterruptNumber for Interrupt {
    #[cfg(not(feature = "g002"))]
    const MAX_INTERRUPT_NUMBER: u16 = Self::PWM2CMP3 as _;
    #[cfg(feature = "g002")]
    const MAX_INTERRUPT_NUMBER: u16 = Self::I2C0 as _;

    fn number(self) -> u16 {
        self as _
    }

    fn from_number(number: u16) -> Result<Self, u16> {
        match Self::try_from(number) {
            Ok(interrupt) => Ok(interrupt),
            _ => Err(number),
        }
    }
}

unsafe impl riscv_peripheral::plic::PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = Self::P7 as _;

    fn number(self) -> u8 {
        self as _
    }

    fn from_number(number: u8) -> Result<Self, u8> {
        match Self::try_from(number) {
            Ok(priority) => Ok(priority),
            _ => Err(number),
        }
    }
}

unsafe impl riscv_peripheral::plic::ContextNumber for Context {
    const MAX_CONTEXT_NUMBER: u16 = Self::C0 as _;

    fn number(self) -> u16 {
        self as _
    }

    fn from_number(number: u16) -> Result<Self, u16> {
        match number {
            0 => Ok(Self::C0),
            _ => Err(number),
        }
    }
}
