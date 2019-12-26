#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430FR2355 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn SAC1_SAC3();
    fn SAC0_SAC2();
    fn ECOMP0_ECOMP1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_B1();
    fn TIMER3_B0();
    fn TIMER2_B1();
    fn TIMER2_B0();
    fn TIMER1_B1();
    fn TIMER1_B0();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT4 },
    Vector { _handler: PORT3 },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector {
        _handler: SAC1_SAC3,
    },
    Vector {
        _handler: SAC0_SAC2,
    },
    Vector {
        _handler: ECOMP0_ECOMP1,
    },
    Vector { _handler: ADC },
    Vector { _handler: EUSCI_B1 },
    Vector { _handler: EUSCI_B0 },
    Vector { _handler: EUSCI_A1 },
    Vector { _handler: EUSCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER3_B1,
    },
    Vector {
        _handler: TIMER3_B0,
    },
    Vector {
        _handler: TIMER2_B1,
    },
    Vector {
        _handler: TIMER2_B0,
    },
    Vector {
        _handler: TIMER1_B1,
    },
    Vector {
        _handler: TIMER1_B0,
    },
    Vector {
        _handler: TIMER0_B1,
    },
    Vector {
        _handler: TIMER0_B0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "21 - 0xFFCE"]
    PORT4,
    #[doc = "22 - 0xFFD0"]
    PORT3,
    #[doc = "23 - 0xFFD2"]
    PORT2,
    #[doc = "24 - 0xFFD4"]
    PORT1,
    #[doc = "25 - 0xFFD6"]
    SAC1_SAC3,
    #[doc = "26 - 0xFFD8"]
    SAC0_SAC2,
    #[doc = "27 - 0xFFDA"]
    ECOMP0_ECOMP1,
    #[doc = "28 - 0xFFDC"]
    ADC,
    #[doc = "29 - 0xFFDE"]
    EUSCI_B1,
    #[doc = "30 - 0xFFE0"]
    EUSCI_B0,
    #[doc = "31 - 0xFFE2"]
    EUSCI_A1,
    #[doc = "32 - 0xFFE4"]
    EUSCI_A0,
    #[doc = "33 - 0xFFE6"]
    WDT,
    #[doc = "34 - 0xFFE8"]
    RTC,
    #[doc = "35 - 0xFFEA"]
    TIMER3_B1,
    #[doc = "36 - 0xFFEC"]
    TIMER3_B0,
    #[doc = "37 - 0xFFEE"]
    TIMER2_B1,
    #[doc = "38 - 0xFFF0"]
    TIMER2_B0,
    #[doc = "39 - 0xFFF2"]
    TIMER1_B1,
    #[doc = "40 - 0xFFF4"]
    TIMER1_B0,
    #[doc = "41 - 0xFFF6"]
    TIMER0_B1,
    #[doc = "42 - 0xFFF8"]
    TIMER0_B0,
    #[doc = "43 - 0xFFFA"]
    UNMI,
    #[doc = "44 - 0xFFFC"]
    SYSNMI,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PORT4 => 21,
            Interrupt::PORT3 => 22,
            Interrupt::PORT2 => 23,
            Interrupt::PORT1 => 24,
            Interrupt::SAC1_SAC3 => 25,
            Interrupt::SAC0_SAC2 => 26,
            Interrupt::ECOMP0_ECOMP1 => 27,
            Interrupt::ADC => 28,
            Interrupt::EUSCI_B1 => 29,
            Interrupt::EUSCI_B0 => 30,
            Interrupt::EUSCI_A1 => 31,
            Interrupt::EUSCI_A0 => 32,
            Interrupt::WDT => 33,
            Interrupt::RTC => 34,
            Interrupt::TIMER3_B1 => 35,
            Interrupt::TIMER3_B0 => 36,
            Interrupt::TIMER2_B1 => 37,
            Interrupt::TIMER2_B0 => 38,
            Interrupt::TIMER1_B1 => 39,
            Interrupt::TIMER1_B0 => 40,
            Interrupt::TIMER0_B1 => 41,
            Interrupt::TIMER0_B0 => 42,
            Interrupt::UNMI => 43,
            Interrupt::SYSNMI => 44,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "P1"]
pub struct P1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1 {}
impl P1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p1::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P1 {
    type Target = p1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P1::ptr() }
    }
}
#[doc = "P1"]
pub mod p1;
#[doc = "P2"]
pub struct P2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P2 {}
impl P2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p2::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P2 {
    type Target = p2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P2::ptr() }
    }
}
#[doc = "P2"]
pub mod p2;
#[doc = "P3"]
pub struct P3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P3 {}
impl P3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p3::RegisterBlock {
        0x0220 as *const _
    }
}
impl Deref for P3 {
    type Target = p3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P3::ptr() }
    }
}
#[doc = "P3"]
pub mod p3;
#[doc = "P4"]
pub struct P4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P4 {}
impl P4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p4::RegisterBlock {
        0x0220 as *const _
    }
}
impl Deref for P4 {
    type Target = p4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P4::ptr() }
    }
}
#[doc = "P4"]
pub mod p4;
#[doc = "P5"]
pub struct P5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P5 {}
impl P5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p5::RegisterBlock {
        0x0240 as *const _
    }
}
impl Deref for P5 {
    type Target = p5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P5::ptr() }
    }
}
#[doc = "P5"]
pub mod p5;
#[doc = "P6"]
pub struct P6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P6 {}
impl P6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p6::RegisterBlock {
        0x0240 as *const _
    }
}
impl Deref for P6 {
    type Target = p6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P6::ptr() }
    }
}
#[doc = "P6"]
pub mod p6;
#[doc = "TB0"]
pub struct TB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB0 {}
impl TB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb0::RegisterBlock {
        0x0380 as *const _
    }
}
impl Deref for TB0 {
    type Target = tb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TB0::ptr() }
    }
}
#[doc = "TB0"]
pub mod tb0;
#[doc = "TB1"]
pub struct TB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB1 {}
impl TB1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb1::RegisterBlock {
        0x03c0 as *const _
    }
}
impl Deref for TB1 {
    type Target = tb1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TB1::ptr() }
    }
}
#[doc = "TB1"]
pub mod tb1;
#[doc = "TB2"]
pub struct TB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB2 {}
impl TB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb2::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for TB2 {
    type Target = tb2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TB2::ptr() }
    }
}
#[doc = "TB2"]
pub mod tb2;
#[doc = "TB3"]
pub struct TB3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB3 {}
impl TB3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb3::RegisterBlock {
        0x0440 as *const _
    }
}
impl Deref for TB3 {
    type Target = tb3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TB3::ptr() }
    }
}
#[doc = "TB3"]
pub mod tb3;
#[doc = "MPY32"]
pub struct MPY32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY32 {}
impl MPY32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy32::RegisterBlock {
        0x04c0 as *const _
    }
}
impl Deref for MPY32 {
    type Target = mpy32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY32::ptr() }
    }
}
#[doc = "MPY32"]
pub mod mpy32;
#[doc = "eUSCI_A0"]
pub struct E_USCI_A0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_A0 {}
impl E_USCI_A0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a0::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for E_USCI_A0 {
    type Target = e_usci_a0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_USCI_A0::ptr() }
    }
}
#[doc = "eUSCI_A0"]
pub mod e_usci_a0;
#[doc = "eUSCI_B0"]
pub struct E_USCI_B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_B0 {}
impl E_USCI_B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b0::RegisterBlock {
        0x0540 as *const _
    }
}
impl Deref for E_USCI_B0 {
    type Target = e_usci_b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_USCI_B0::ptr() }
    }
}
#[doc = "eUSCI_B0"]
pub mod e_usci_b0;
#[doc = "eUSCI_A1"]
pub struct E_USCI_A1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_A1 {}
impl E_USCI_A1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a1::RegisterBlock {
        0x0580 as *const _
    }
}
impl Deref for E_USCI_A1 {
    type Target = e_usci_a1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_USCI_A1::ptr() }
    }
}
#[doc = "eUSCI_A1"]
pub mod e_usci_a1;
#[doc = "eUSCI_B1"]
pub struct E_USCI_B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_B1 {}
impl E_USCI_B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b1::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for E_USCI_B1 {
    type Target = e_usci_b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_USCI_B1::ptr() }
    }
}
#[doc = "eUSCI_B1"]
pub mod e_usci_b1;
#[doc = "BKMEM"]
pub struct BKMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BKMEM {}
impl BKMEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bkmem::RegisterBlock {
        0x0660 as *const _
    }
}
impl Deref for BKMEM {
    type Target = bkmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BKMEM::ptr() }
    }
}
#[doc = "BKMEM"]
pub mod bkmem;
#[doc = "ICC"]
pub struct ICC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC {}
impl ICC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icc::RegisterBlock {
        0x06c0 as *const _
    }
}
impl Deref for ICC {
    type Target = icc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICC::ptr() }
    }
}
#[doc = "ICC"]
pub mod icc;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x0700 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "eCOMP0"]
pub struct E_COMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_COMP0 {}
impl E_COMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_comp0::RegisterBlock {
        0x08e0 as *const _
    }
}
impl Deref for E_COMP0 {
    type Target = e_comp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_COMP0::ptr() }
    }
}
#[doc = "eCOMP0"]
pub mod e_comp0;
#[doc = "eCOMP1"]
pub struct E_COMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_COMP1 {}
impl E_COMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_comp1::RegisterBlock {
        0x0900 as *const _
    }
}
impl Deref for E_COMP1 {
    type Target = e_comp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*E_COMP1::ptr() }
    }
}
#[doc = "eCOMP1"]
pub mod e_comp1;
#[doc = "SAC0"]
pub struct SAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAC0 {}
impl SAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sac0::RegisterBlock {
        0x0c80 as *const _
    }
}
impl Deref for SAC0 {
    type Target = sac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAC0::ptr() }
    }
}
#[doc = "SAC0"]
pub mod sac0;
#[doc = "SAC1"]
pub struct SAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAC1 {}
impl SAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sac1::RegisterBlock {
        0x0c90 as *const _
    }
}
impl Deref for SAC1 {
    type Target = sac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAC1::ptr() }
    }
}
#[doc = "SAC1"]
pub mod sac1;
#[doc = "SAC2"]
pub struct SAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAC2 {}
impl SAC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sac2::RegisterBlock {
        0x0ca0 as *const _
    }
}
impl Deref for SAC2 {
    type Target = sac2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAC2::ptr() }
    }
}
#[doc = "SAC2"]
pub mod sac2;
#[doc = "SAC3"]
pub struct SAC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAC3 {}
impl SAC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sac3::RegisterBlock {
        0x0cb0 as *const _
    }
}
impl Deref for SAC3 {
    type Target = sac3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAC3::ptr() }
    }
}
#[doc = "SAC3"]
pub mod sac3;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "P1"]
    pub P1: P1,
    #[doc = "P2"]
    pub P2: P2,
    #[doc = "P3"]
    pub P3: P3,
    #[doc = "P4"]
    pub P4: P4,
    #[doc = "P5"]
    pub P5: P5,
    #[doc = "P6"]
    pub P6: P6,
    #[doc = "TB0"]
    pub TB0: TB0,
    #[doc = "TB1"]
    pub TB1: TB1,
    #[doc = "TB2"]
    pub TB2: TB2,
    #[doc = "TB3"]
    pub TB3: TB3,
    #[doc = "MPY32"]
    pub MPY32: MPY32,
    #[doc = "E_USCI_A0"]
    pub E_USCI_A0: E_USCI_A0,
    #[doc = "E_USCI_B0"]
    pub E_USCI_B0: E_USCI_B0,
    #[doc = "E_USCI_A1"]
    pub E_USCI_A1: E_USCI_A1,
    #[doc = "E_USCI_B1"]
    pub E_USCI_B1: E_USCI_B1,
    #[doc = "BKMEM"]
    pub BKMEM: BKMEM,
    #[doc = "ICC"]
    pub ICC: ICC,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "E_COMP0"]
    pub E_COMP0: E_COMP0,
    #[doc = "E_COMP1"]
    pub E_COMP1: E_COMP1,
    #[doc = "SAC0"]
    pub SAC0: SAC0,
    #[doc = "SAC1"]
    pub SAC1: SAC1,
    #[doc = "SAC2"]
    pub SAC2: SAC2,
    #[doc = "SAC3"]
    pub SAC3: SAC3,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            P1: P1 {
                _marker: PhantomData,
            },
            P2: P2 {
                _marker: PhantomData,
            },
            P3: P3 {
                _marker: PhantomData,
            },
            P4: P4 {
                _marker: PhantomData,
            },
            P5: P5 {
                _marker: PhantomData,
            },
            P6: P6 {
                _marker: PhantomData,
            },
            TB0: TB0 {
                _marker: PhantomData,
            },
            TB1: TB1 {
                _marker: PhantomData,
            },
            TB2: TB2 {
                _marker: PhantomData,
            },
            TB3: TB3 {
                _marker: PhantomData,
            },
            MPY32: MPY32 {
                _marker: PhantomData,
            },
            E_USCI_A0: E_USCI_A0 {
                _marker: PhantomData,
            },
            E_USCI_B0: E_USCI_B0 {
                _marker: PhantomData,
            },
            E_USCI_A1: E_USCI_A1 {
                _marker: PhantomData,
            },
            E_USCI_B1: E_USCI_B1 {
                _marker: PhantomData,
            },
            BKMEM: BKMEM {
                _marker: PhantomData,
            },
            ICC: ICC {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            E_COMP0: E_COMP0 {
                _marker: PhantomData,
            },
            E_COMP1: E_COMP1 {
                _marker: PhantomData,
            },
            SAC0: SAC0 {
                _marker: PhantomData,
            },
            SAC1: SAC1 {
                _marker: PhantomData,
            },
            SAC2: SAC2 {
                _marker: PhantomData,
            },
            SAC3: SAC3 {
                _marker: PhantomData,
            },
        }
    }
}
