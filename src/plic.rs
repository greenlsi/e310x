pub use crate::Interrupt;

riscv::plic_context!(PLIC, 0x0c00_0000, 0, Interrupt, Priority);

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
        match value {
            0 => Ok(Self::P0),
            1 => Ok(Self::P1),
            2 => Ok(Self::P2),
            3 => Ok(Self::P3),
            4 => Ok(Self::P4),
            5 => Ok(Self::P5),
            6 => Ok(Self::P6),
            7 => Ok(Self::P7),
            _ => Err(TryFromPriorityError(())),
        }
    }
}

unsafe impl riscv::peripheral::plic::InterruptNumber for Interrupt {
    #[cfg(not(feature = "g002"))]
    const MAX_INTERRUPT_NUMBER: u16 = Self::PWM2CMP3 as _;
    #[cfg(feature = "g002")]
    const MAX_INTERRUPT_NUMBER: u16 = Self::I2C0 as _;

    fn number(self) -> u16 {
        self as _
    }

    fn try_from(value: u16) -> Result<Self, u16> {
        match Self::try_from(value as _) {
            Ok(interrupt) => Ok(interrupt),
            _ => Err(value),
        }
    }
}

unsafe impl riscv::peripheral::plic::PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = Self::P7 as _;

    fn number(self) -> u8 {
        self as _
    }

    fn try_from(value: u8) -> Result<Self, u8> {
        match Self::try_from(value as _) {
            Ok(priority) => Ok(priority),
            _ => Err(value),
        }
    }
}
