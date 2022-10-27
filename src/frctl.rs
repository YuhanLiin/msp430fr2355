#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control Register 0"]
    pub frctl0: FRCTL0,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - General Control Register 0"]
    pub gcctl0: GCCTL0,
    #[doc = "0x06 - General Control Register 1"]
    pub gcctl1: GCCTL1,
}
#[doc = "FRCTL0 (rw) register accessor: an alias for `Reg<FRCTL0_SPEC>`"]
pub type FRCTL0 = crate::Reg<frctl0::FRCTL0_SPEC>;
#[doc = "FRAM Controller Control Register 0"]
pub mod frctl0;
#[doc = "GCCTL0 (rw) register accessor: an alias for `Reg<GCCTL0_SPEC>`"]
pub type GCCTL0 = crate::Reg<gcctl0::GCCTL0_SPEC>;
#[doc = "General Control Register 0"]
pub mod gcctl0;
#[doc = "GCCTL1 (rw) register accessor: an alias for `Reg<GCCTL1_SPEC>`"]
pub type GCCTL1 = crate::Reg<gcctl1::GCCTL1_SPEC>;
#[doc = "General Control Register 1"]
pub mod gcctl1;
