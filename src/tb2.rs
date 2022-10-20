#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb2ctl: TB2CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl0: TB2CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl1: TB2CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl2: TB2CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - Timer_B count register"]
    pub tb2r: TB2R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb2ccr0: TB2CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb2ccr1: TB2CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb2ccr2: TB2CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb2ex0: TB2EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb2iv: TB2IV,
}
#[doc = "TB2CTL (rw) register accessor: an alias for `Reg<TB2CTL_SPEC>`"]
pub type TB2CTL = crate::Reg<tb2ctl::TB2CTL_SPEC>;
#[doc = "Timer_B Control Register"]
pub mod tb2ctl;
#[doc = "TB2CCTL0 (rw) register accessor: an alias for `Reg<TB2CCTL0_SPEC>`"]
pub type TB2CCTL0 = crate::Reg<tb2cctl0::TB2CCTL0_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl0;
#[doc = "TB2CCTL1 (rw) register accessor: an alias for `Reg<TB2CCTL1_SPEC>`"]
pub type TB2CCTL1 = crate::Reg<tb2cctl1::TB2CCTL1_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl1;
#[doc = "TB2CCTL2 (rw) register accessor: an alias for `Reg<TB2CCTL2_SPEC>`"]
pub type TB2CCTL2 = crate::Reg<tb2cctl2::TB2CCTL2_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl2;
#[doc = "TB2R (rw) register accessor: an alias for `Reg<TB2R_SPEC>`"]
pub type TB2R = crate::Reg<tb2r::TB2R_SPEC>;
#[doc = "Timer_B count register"]
pub mod tb2r;
#[doc = "TB2CCR0 (rw) register accessor: an alias for `Reg<TB2CCR0_SPEC>`"]
pub type TB2CCR0 = crate::Reg<tb2ccr0::TB2CCR0_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr0;
#[doc = "TB2CCR1 (rw) register accessor: an alias for `Reg<TB2CCR1_SPEC>`"]
pub type TB2CCR1 = crate::Reg<tb2ccr1::TB2CCR1_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr1;
#[doc = "TB2CCR2 (rw) register accessor: an alias for `Reg<TB2CCR2_SPEC>`"]
pub type TB2CCR2 = crate::Reg<tb2ccr2::TB2CCR2_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr2;
#[doc = "TB2EX0 (rw) register accessor: an alias for `Reg<TB2EX0_SPEC>`"]
pub type TB2EX0 = crate::Reg<tb2ex0::TB2EX0_SPEC>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb2ex0;
#[doc = "TB2IV (rw) register accessor: an alias for `Reg<TB2IV_SPEC>`"]
pub type TB2IV = crate::Reg<tb2iv::TB2IV_SPEC>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb2iv;
