#![doc = "Peripheral access API for FE310 microcontrollers (generated using svd2rust v0.26.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.26.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = "Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn WATCHDOG();
    fn RTC();
    fn UART0();
    fn UART1();
    fn QSPI0();
    fn QSPI1();
    fn QSPI2();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn GPIO5();
    fn GPIO6();
    fn GPIO7();
    fn GPIO8();
    fn GPIO9();
    fn GPIO10();
    fn GPIO11();
    fn GPIO12();
    fn GPIO13();
    fn GPIO14();
    fn GPIO15();
    fn GPIO16();
    fn GPIO17();
    fn GPIO18();
    fn GPIO19();
    fn GPIO20();
    fn GPIO21();
    fn GPIO22();
    fn GPIO23();
    fn GPIO24();
    fn GPIO25();
    fn GPIO26();
    fn GPIO27();
    fn GPIO28();
    fn GPIO29();
    fn GPIO30();
    fn GPIO31();
    fn PWM0CMP0();
    fn PWM0CMP1();
    fn PWM0CMP2();
    fn PWM0CMP3();
    fn PWM1CMP0();
    fn PWM1CMP1();
    fn PWM1CMP2();
    fn PWM1CMP3();
    fn PWM2CMP0();
    fn PWM2CMP1();
    fn PWM2CMP2();
    fn PWM2CMP3();
    fn I2C0();
}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 53] = [
    Vector { _reserved: 0 },
    Vector { _handler: WATCHDOG },
    Vector { _handler: RTC },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: QSPI0 },
    Vector { _handler: QSPI1 },
    Vector { _handler: QSPI2 },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector { _handler: GPIO5 },
    Vector { _handler: GPIO6 },
    Vector { _handler: GPIO7 },
    Vector { _handler: GPIO8 },
    Vector { _handler: GPIO9 },
    Vector { _handler: GPIO10 },
    Vector { _handler: GPIO11 },
    Vector { _handler: GPIO12 },
    Vector { _handler: GPIO13 },
    Vector { _handler: GPIO14 },
    Vector { _handler: GPIO15 },
    Vector { _handler: GPIO16 },
    Vector { _handler: GPIO17 },
    Vector { _handler: GPIO18 },
    Vector { _handler: GPIO19 },
    Vector { _handler: GPIO20 },
    Vector { _handler: GPIO21 },
    Vector { _handler: GPIO22 },
    Vector { _handler: GPIO23 },
    Vector { _handler: GPIO24 },
    Vector { _handler: GPIO25 },
    Vector { _handler: GPIO26 },
    Vector { _handler: GPIO27 },
    Vector { _handler: GPIO28 },
    Vector { _handler: GPIO29 },
    Vector { _handler: GPIO30 },
    Vector { _handler: GPIO31 },
    Vector { _handler: PWM0CMP0 },
    Vector { _handler: PWM0CMP1 },
    Vector { _handler: PWM0CMP2 },
    Vector { _handler: PWM0CMP3 },
    Vector { _handler: PWM1CMP0 },
    Vector { _handler: PWM1CMP1 },
    Vector { _handler: PWM1CMP2 },
    Vector { _handler: PWM1CMP3 },
    Vector { _handler: PWM2CMP0 },
    Vector { _handler: PWM2CMP1 },
    Vector { _handler: PWM2CMP2 },
    Vector { _handler: PWM2CMP3 },
    Vector { _handler: I2C0 },
];

/// Handler for vectored machine external interrupts (see the [`riscv-rt`] crate).
///
/// # Note
///
/// You need to activate the `v-extern` feature.
#[cfg(feature = "v-extern")]
#[no_mangle]
#[allow(non_snake_case)]
unsafe fn MachineExternal() {
    let claim = PLIC::ctx0().claim();
    while let Some(source) = claim.claim::<Interrupt>() {
        unsafe { (__EXTERNAL_INTERRUPTS[source as usize]._handler)() };
        claim.complete(source)
    }
}

pub mod pac;
pub use pac::*;

#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;

riscv_peripheral::clint_codegen!(
    base 0x0200_0000,
    freq 32_768,
    mtimecmps [mtimecmp0=(HartId::HART0,"`H0`")],
    msips [msip0=(HartId::HART0,"`H0`")],
);

#[doc = "Platform Level Interrupt Control"]
pub mod plic;
riscv_peripheral::plic_codegen!(base 0x0c00_0000, ctxs [ctx0 = (HartId::HART0, "0")],);

#[doc = "Platform Level Interrupt Control"]
pub use plic::Priority;
#[doc = "Watchdog"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdog::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG").finish()
    }
}
#[doc = "Watchdog"]
pub mod wdog;
#[doc = "Watchdog"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Watchdog"]
pub mod rtc;
#[doc = "Always-On Clock Configuration"]
pub struct AONCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AONCLK {}
impl AONCLK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aonclk::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aonclk::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AONCLK {
    type Target = aonclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AONCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AONCLK").finish()
    }
}
#[doc = "Always-On Clock Configuration"]
pub mod aonclk;
#[doc = "Backup Registers"]
pub struct BACKUP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP {}
impl BACKUP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const backup::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BACKUP {
    type Target = backup::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BACKUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP").finish()
    }
}
#[doc = "Backup Registers"]
pub mod backup;
#[doc = "PMU"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pmu::RegisterBlock = 0x1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU").finish()
    }
}
#[doc = "PMU"]
pub mod pmu;
#[doc = "Power Reset Clock Interrupts"]
pub struct PRCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCI {}
impl PRCI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const prci::RegisterBlock = 0x1000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prci::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PRCI {
    type Target = prci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PRCI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRCI").finish()
    }
}
#[doc = "Power Reset Clock Interrupts"]
pub mod prci;
#[doc = "One Time Programmable Memory"]
pub struct OTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTP {}
impl OTP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const otp::RegisterBlock = 0x1001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for OTP {
    type Target = otp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for OTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP").finish()
    }
}
#[doc = "One Time Programmable Memory"]
pub mod otp;
#[doc = "General Purpose Input Output"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio0::RegisterBlock = 0x1001_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO0").finish()
    }
}
#[doc = "General Purpose Input Output"]
pub mod gpio0;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x1001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart0;
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI0 {}
impl QSPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const qspi0::RegisterBlock = 0x1001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for QSPI0 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for QSPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QSPI0").finish()
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub mod qspi0;
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm0::RegisterBlock = 0x1001_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0").finish()
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub mod pwm0;
#[doc = "Inter-Integrated Circuit Master Interface (FE310-G002 only)"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x1001_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "Inter-Integrated Circuit Master Interface (FE310-G002 only)"]
pub mod i2c0;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x1002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub use self::uart0 as uart1;
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI1 {}
impl QSPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const qspi0::RegisterBlock = 0x1002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for QSPI1 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for QSPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QSPI1").finish()
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub use self::qspi0 as qspi1;
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm0::RegisterBlock = 0x1002_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1").finish()
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub use self::pwm0 as pwm1;
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI2 {}
impl QSPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const qspi0::RegisterBlock = 0x1003_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for QSPI2 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for QSPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QSPI2").finish()
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub use self::qspi0 as qspi2;
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm0::RegisterBlock = 0x1003_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM2 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM2").finish()
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub use self::pwm0 as pwm2;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "AONCLK"]
    pub AONCLK: AONCLK,
    #[doc = "BACKUP"]
    pub BACKUP: BACKUP,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "PRCI"]
    pub PRCI: PRCI,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "QSPI0"]
    pub QSPI0: QSPI0,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "QSPI1"]
    pub QSPI1: QSPI1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "QSPI2"]
    pub QSPI2: QSPI2,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WDOG: WDOG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            AONCLK: AONCLK {
                _marker: PhantomData,
            },
            BACKUP: BACKUP {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            PRCI: PRCI {
                _marker: PhantomData,
            },
            OTP: OTP {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            QSPI0: QSPI0 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            QSPI1: QSPI1 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            QSPI2: QSPI2 {
                _marker: PhantomData,
            },
            PWM2: PWM2 {
                _marker: PhantomData,
            },
        }
    }
}
