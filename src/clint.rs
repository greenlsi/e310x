/// CLINT HART IDs of the target
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum HartId {
    #[doc = "0: HART 0"]
    HART0 = 0,
}

unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
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
