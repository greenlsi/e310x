#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - WATCHDOG"]
    WATCHDOG = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - UART0"]
    UART0 = 3,
    #[doc = "4 - UART1"]
    UART1 = 4,
    #[doc = "5 - QSPI0"]
    QSPI0 = 5,
    #[doc = "6 - QSPI1"]
    QSPI1 = 6,
    #[doc = "7 - QSPI2"]
    QSPI2 = 7,
    #[doc = "8 - GPIO0"]
    GPIO0 = 8,
    #[doc = "9 - GPIO1"]
    GPIO1 = 9,
    #[doc = "10 - GPIO2"]
    GPIO2 = 10,
    #[doc = "11 - GPIO3"]
    GPIO3 = 11,
    #[doc = "12 - GPIO4"]
    GPIO4 = 12,
    #[doc = "13 - GPIO5"]
    GPIO5 = 13,
    #[doc = "14 - GPIO6"]
    GPIO6 = 14,
    #[doc = "15 - GPIO7"]
    GPIO7 = 15,
    #[doc = "16 - GPIO8"]
    GPIO8 = 16,
    #[doc = "17 - GPIO9"]
    GPIO9 = 17,
    #[doc = "18 - GPIO10"]
    GPIO10 = 18,
    #[doc = "19 - GPIO11"]
    GPIO11 = 19,
    #[doc = "20 - GPIO12"]
    GPIO12 = 20,
    #[doc = "21 - GPIO13"]
    GPIO13 = 21,
    #[doc = "22 - GPIO14"]
    GPIO14 = 22,
    #[doc = "23 - GPIO15"]
    GPIO15 = 23,
    #[doc = "24 - GPIO16"]
    GPIO16 = 24,
    #[doc = "25 - GPIO17"]
    GPIO17 = 25,
    #[doc = "26 - GPIO18"]
    GPIO18 = 26,
    #[doc = "27 - GPIO19"]
    GPIO19 = 27,
    #[doc = "28 - GPIO20"]
    GPIO20 = 28,
    #[doc = "29 - GPIO21"]
    GPIO21 = 29,
    #[doc = "30 - GPIO22"]
    GPIO22 = 30,
    #[doc = "31 - GPIO23"]
    GPIO23 = 31,
    #[doc = "32 - GPIO24"]
    GPIO24 = 32,
    #[doc = "33 - GPIO25"]
    GPIO25 = 33,
    #[doc = "34 - GPIO26"]
    GPIO26 = 34,
    #[doc = "35 - GPIO27"]
    GPIO27 = 35,
    #[doc = "36 - GPIO28"]
    GPIO28 = 36,
    #[doc = "37 - GPIO29"]
    GPIO29 = 37,
    #[doc = "38 - GPIO30"]
    GPIO30 = 38,
    #[doc = "39 - GPIO31"]
    GPIO31 = 39,
    #[doc = "40 - PWM0CMP0"]
    PWM0CMP0 = 40,
    #[doc = "41 - PWM0CMP1"]
    PWM0CMP1 = 41,
    #[doc = "42 - PWM0CMP2"]
    PWM0CMP2 = 42,
    #[doc = "43 - PWM0CMP3"]
    PWM0CMP3 = 43,
    #[doc = "44 - PWM1CMP0"]
    PWM1CMP0 = 44,
    #[doc = "45 - PWM1CMP1"]
    PWM1CMP1 = 45,
    #[doc = "46 - PWM1CMP2"]
    PWM1CMP2 = 46,
    #[doc = "47 - PWM1CMP3"]
    PWM1CMP3 = 47,
    #[doc = "48 - PWM2CMP0"]
    PWM2CMP0 = 48,
    #[doc = "49 - PWM2CMP1"]
    PWM2CMP1 = 49,
    #[doc = "50 - PWM2CMP2"]
    PWM2CMP2 = 50,
    #[doc = "51 - PWM2CMP3"]
    PWM2CMP3 = 51,
    #[doc = "52 - I2C0"]
    I2C0 = 52,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u16) -> Result<Self, TryFromInterruptError> {
        if value == 0 || value > 52 {
            Err(TryFromInterruptError(()))
        } else {
            // SAFETY: the value is in the range 1..=52
            Ok(unsafe { core::mem::transmute(value) })
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
