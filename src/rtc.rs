#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    pub rtcctl: RTCCTL,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Real-Time Clock Interrupt Vector Register"]
    pub rtciv: RTCIV,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - RTC Counter Modulo Register"]
    pub rtcmod: RTCMOD,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - RTC Counter Register"]
    pub rtccnt: RTCCNT,
}
#[doc = "RTCCTL (rw) register accessor: an alias for `Reg<RTCCTL_SPEC>`"]
pub type RTCCTL = crate::Reg<rtcctl::RTCCTL_SPEC>;
#[doc = "RTCCTL0 Register"]
pub mod rtcctl;
#[doc = "RTCIV (rw) register accessor: an alias for `Reg<RTCIV_SPEC>`"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCMOD (rw) register accessor: an alias for `Reg<RTCMOD_SPEC>`"]
pub type RTCMOD = crate::Reg<rtcmod::RTCMOD_SPEC>;
#[doc = "RTC Counter Modulo Register"]
pub mod rtcmod;
#[doc = "RTCCNT (rw) register accessor: an alias for `Reg<RTCCNT_SPEC>`"]
pub type RTCCNT = crate::Reg<rtccnt::RTCCNT_SPEC>;
#[doc = "RTC Counter Register"]
pub mod rtccnt;
