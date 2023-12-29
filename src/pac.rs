//! Implementation of `riscv-pac` traits

pub use crate::interrupt::Interrupt;
pub use crate::plic::Priority;
pub use riscv_pac::{HartIdNumber, InterruptNumber, PriorityNumber};

/// HART IDs
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum HartId {
    #[doc = "0: HART 0"]
    HART0 = 0,
}

unsafe impl HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: u16 = Self::HART0 as _;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        match number {
            0 => Ok(Self::HART0),
            _ => Err(number),
        }
    }
}

unsafe impl InterruptNumber for Interrupt {
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

unsafe impl PriorityNumber for Priority {
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
