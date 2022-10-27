#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cpctl0: CPCTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cpctl1: CPCTL1,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    pub cpint: CPINT,
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    pub cpiv: CPIV,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    pub cpdacctl: CPDACCTL,
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    pub cpdacdata: CPDACDATA,
}
#[doc = "CPCTL0 (rw) register accessor: an alias for `Reg<CPCTL0_SPEC>`"]
pub type CPCTL0 = crate::Reg<cpctl0::CPCTL0_SPEC>;
#[doc = "Comparator Control Register 0"]
pub mod cpctl0;
#[doc = "CPCTL1 (rw) register accessor: an alias for `Reg<CPCTL1_SPEC>`"]
pub type CPCTL1 = crate::Reg<cpctl1::CPCTL1_SPEC>;
#[doc = "Comparator Control Register 1"]
pub mod cpctl1;
#[doc = "CPINT (rw) register accessor: an alias for `Reg<CPINT_SPEC>`"]
pub type CPINT = crate::Reg<cpint::CPINT_SPEC>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cpint;
#[doc = "CPIV (rw) register accessor: an alias for `Reg<CPIV_SPEC>`"]
pub type CPIV = crate::Reg<cpiv::CPIV_SPEC>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cpiv;
#[doc = "CPDACCTL (rw) register accessor: an alias for `Reg<CPDACCTL_SPEC>`"]
pub type CPDACCTL = crate::Reg<cpdacctl::CPDACCTL_SPEC>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cpdacctl;
#[doc = "CPDACDATA (rw) register accessor: an alias for `Reg<CPDACDATA_SPEC>`"]
pub type CPDACDATA = crate::Reg<cpdacdata::CPDACDATA_SPEC>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cpdacdata;
