#![feature(abi_msp430_interrupt)]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![doc = "Peripheral access API for MSP430FR2355 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
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
    fn deref(&self) -> &Self::Target {
        unsafe { &*P6::ptr() }
    }
}
#[doc = "P6"]
pub mod p6;
#[doc = "P7"]
pub struct P7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P7 {}
impl P7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p7::RegisterBlock {
        0x0260 as *const _
    }
}
impl Deref for P7 {
    type Target = p7::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P7::ptr() }
    }
}
#[doc = "P7"]
pub mod p7;
#[doc = "P8"]
pub struct P8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P8 {}
impl P8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p8::RegisterBlock {
        0x0260 as *const _
    }
}
impl Deref for P8 {
    type Target = p8::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P8::ptr() }
    }
}
#[doc = "P8"]
pub mod p8;
#[doc = "P9"]
pub struct P9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P9 {}
impl P9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p9::RegisterBlock {
        0x0280 as *const _
    }
}
impl Deref for P9 {
    type Target = p9::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P9::ptr() }
    }
}
#[doc = "P9"]
pub mod p9;
#[doc = "P10"]
pub struct P10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P10 {}
impl P10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p10::RegisterBlock {
        0x0280 as *const _
    }
}
impl Deref for P10 {
    type Target = p10::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*P10::ptr() }
    }
}
#[doc = "P10"]
pub mod p10;
#[doc = "SFR"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        0x0100 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR"]
pub mod sfr;
#[doc = "PMM"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM"]
pub mod pmm;
#[doc = "SYS"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        0x0140 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS"]
pub mod sys;
#[doc = "CS"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        0x0180 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS"]
pub mod cs;
#[doc = "FRCTL"]
pub struct FRCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRCTL {}
impl FRCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const frctl::RegisterBlock {
        0x01a0 as *const _
    }
}
impl Deref for FRCTL {
    type Target = frctl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FRCTL::ptr() }
    }
}
#[doc = "FRCTL"]
pub mod frctl;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x01c0 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "WDT_A"]
pub struct WDT_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_A {}
impl WDT_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt_a::RegisterBlock {
        0x01cc as *const _
    }
}
impl Deref for WDT_A {
    type Target = wdt_a::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT_A::ptr() }
    }
}
#[doc = "WDT_A"]
pub mod wdt_a;
#[doc = "CAPTIO"]
pub struct CAPTIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPTIO {}
impl CAPTIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const captio::RegisterBlock {
        0x02ee as *const _
    }
}
impl Deref for CAPTIO {
    type Target = captio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPTIO::ptr() }
    }
}
#[doc = "CAPTIO"]
pub mod captio;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x0300 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "PJ"]
pub struct PJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PJ {}
impl PJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pj::RegisterBlock {
        0x0320 as *const _
    }
}
impl Deref for PJ {
    type Target = pj::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PJ::ptr() }
    }
}
#[doc = "PJ"]
pub mod pj;
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
    #[doc = "P7"]
    pub P7: P7,
    #[doc = "P8"]
    pub P8: P8,
    #[doc = "P9"]
    pub P9: P9,
    #[doc = "P10"]
    pub P10: P10,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "FRCTL"]
    pub FRCTL: FRCTL,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "WDT_A"]
    pub WDT_A: WDT_A,
    #[doc = "CAPTIO"]
    pub CAPTIO: CAPTIO,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "PJ"]
    pub PJ: PJ,
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
            P7: P7 {
                _marker: PhantomData,
            },
            P8: P8 {
                _marker: PhantomData,
            },
            P9: P9 {
                _marker: PhantomData,
            },
            P10: P10 {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            FRCTL: FRCTL {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            WDT_A: WDT_A {
                _marker: PhantomData,
            },
            CAPTIO: CAPTIO {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            PJ: PJ {
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
