#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock System Control 0"]
    pub csctl0: CSCTL0,
    #[doc = "0x02 - Clock System Control 1"]
    pub csctl1: CSCTL1,
    #[doc = "0x04 - Clock System Control 2"]
    pub csctl2: CSCTL2,
    #[doc = "0x06 - Clock System Control 3"]
    pub csctl3: CSCTL3,
    #[doc = "0x08 - Clock System Control 4"]
    pub csctl4: CSCTL4,
    #[doc = "0x0a - Clock System Control 5"]
    pub csctl5: CSCTL5,
    #[doc = "0x0c - Clock System Control 6"]
    pub csctl6: CSCTL6,
    #[doc = "0x0e - Clock System Control Register 7"]
    pub csctl7: CSCTL7,
    #[doc = "0x10 - Clock System Control Register 8"]
    pub csctl8: CSCTL8,
}
#[doc = "CSCTL0 (rw) register accessor: an alias for `Reg<CSCTL0_SPEC>`"]
pub type CSCTL0 = crate::Reg<csctl0::CSCTL0_SPEC>;
#[doc = "Clock System Control 0"]
pub mod csctl0;
#[doc = "CSCTL1 (rw) register accessor: an alias for `Reg<CSCTL1_SPEC>`"]
pub type CSCTL1 = crate::Reg<csctl1::CSCTL1_SPEC>;
#[doc = "Clock System Control 1"]
pub mod csctl1;
#[doc = "CSCTL2 (rw) register accessor: an alias for `Reg<CSCTL2_SPEC>`"]
pub type CSCTL2 = crate::Reg<csctl2::CSCTL2_SPEC>;
#[doc = "Clock System Control 2"]
pub mod csctl2;
#[doc = "CSCTL3 (rw) register accessor: an alias for `Reg<CSCTL3_SPEC>`"]
pub type CSCTL3 = crate::Reg<csctl3::CSCTL3_SPEC>;
#[doc = "Clock System Control 3"]
pub mod csctl3;
#[doc = "CSCTL4 (rw) register accessor: an alias for `Reg<CSCTL4_SPEC>`"]
pub type CSCTL4 = crate::Reg<csctl4::CSCTL4_SPEC>;
#[doc = "Clock System Control 4"]
pub mod csctl4;
#[doc = "CSCTL5 (rw) register accessor: an alias for `Reg<CSCTL5_SPEC>`"]
pub type CSCTL5 = crate::Reg<csctl5::CSCTL5_SPEC>;
#[doc = "Clock System Control 5"]
pub mod csctl5;
#[doc = "CSCTL6 (rw) register accessor: an alias for `Reg<CSCTL6_SPEC>`"]
pub type CSCTL6 = crate::Reg<csctl6::CSCTL6_SPEC>;
#[doc = "Clock System Control 6"]
pub mod csctl6;
#[doc = "CSCTL7 (rw) register accessor: an alias for `Reg<CSCTL7_SPEC>`"]
pub type CSCTL7 = crate::Reg<csctl7::CSCTL7_SPEC>;
#[doc = "Clock System Control Register 7"]
pub mod csctl7;
#[doc = "CSCTL8 (rw) register accessor: an alias for `Reg<CSCTL8_SPEC>`"]
pub type CSCTL8 = crate::Reg<csctl8::CSCTL8_SPEC>;
#[doc = "Clock System Control Register 8"]
pub mod csctl8;
