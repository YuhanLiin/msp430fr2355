#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    pub adcctl0: ADCCTL0,
    #[doc = "0x02 - ADC Control 1"]
    pub adcctl1: ADCCTL1,
    #[doc = "0x04 - ADC Control 2"]
    pub adcctl2: ADCCTL2,
    #[doc = "0x06 - ADC Window Comparator Low Threshold Register"]
    pub adclo: ADCLO,
    #[doc = "0x08 - ADC Window Comparator High Threshold Register"]
    pub adchi: ADCHI,
    #[doc = "0x0a - ADC Conversion Memory Control Register"]
    pub adcmctl0: ADCMCTL0,
    _reserved6: [u8; 0x06],
    #[doc = "0x12 - ADC Conversion Memory Register"]
    pub adcmem0: ADCMEM0,
    _reserved7: [u8; 0x06],
    #[doc = "0x1a - ADC Interrupt Enable 0"]
    pub adcie: ADCIE,
    #[doc = "0x1c - ADC Interrupt Flag"]
    pub adcifg: ADCIFG,
    #[doc = "0x1e - ADC Interrupt Vector"]
    pub adciv: ADCIV,
}
#[doc = "ADCCTL0 (rw) register accessor: an alias for `Reg<ADCCTL0_SPEC>`"]
pub type ADCCTL0 = crate::Reg<adcctl0::ADCCTL0_SPEC>;
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADCCTL1 (rw) register accessor: an alias for `Reg<ADCCTL1_SPEC>`"]
pub type ADCCTL1 = crate::Reg<adcctl1::ADCCTL1_SPEC>;
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADCCTL2 (rw) register accessor: an alias for `Reg<ADCCTL2_SPEC>`"]
pub type ADCCTL2 = crate::Reg<adcctl2::ADCCTL2_SPEC>;
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADCLO (rw) register accessor: an alias for `Reg<ADCLO_SPEC>`"]
pub type ADCLO = crate::Reg<adclo::ADCLO_SPEC>;
#[doc = "ADC Window Comparator Low Threshold Register"]
pub mod adclo;
#[doc = "ADCHI (rw) register accessor: an alias for `Reg<ADCHI_SPEC>`"]
pub type ADCHI = crate::Reg<adchi::ADCHI_SPEC>;
#[doc = "ADC Window Comparator High Threshold Register"]
pub mod adchi;
#[doc = "ADCMCTL0 (rw) register accessor: an alias for `Reg<ADCMCTL0_SPEC>`"]
pub type ADCMCTL0 = crate::Reg<adcmctl0::ADCMCTL0_SPEC>;
#[doc = "ADC Conversion Memory Control Register"]
pub mod adcmctl0;
#[doc = "ADCMEM0 (rw) register accessor: an alias for `Reg<ADCMEM0_SPEC>`"]
pub type ADCMEM0 = crate::Reg<adcmem0::ADCMEM0_SPEC>;
#[doc = "ADC Conversion Memory Register"]
pub mod adcmem0;
#[doc = "ADCIE (rw) register accessor: an alias for `Reg<ADCIE_SPEC>`"]
pub type ADCIE = crate::Reg<adcie::ADCIE_SPEC>;
#[doc = "ADC Interrupt Enable 0"]
pub mod adcie;
#[doc = "ADCIFG (rw) register accessor: an alias for `Reg<ADCIFG_SPEC>`"]
pub type ADCIFG = crate::Reg<adcifg::ADCIFG_SPEC>;
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADCIV (rw) register accessor: an alias for `Reg<ADCIV_SPEC>`"]
pub type ADCIV = crate::Reg<adciv::ADCIV_SPEC>;
#[doc = "ADC Interrupt Vector"]
pub mod adciv;
