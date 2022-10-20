#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb0ctl: TB0CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl0: TB0CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl1: TB0CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl2: TB0CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - Timer_B count register"]
    pub tb0r: TB0R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb0ccr0: TB0CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb0ccr1: TB0CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb0ccr2: TB0CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb0ex0: TB0EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb0iv: TB0IV,
}
#[doc = "TB0CTL (rw) register accessor: an alias for `Reg<TB0CTL_SPEC>`"]
pub type TB0CTL = crate::Reg<tb0ctl::TB0CTL_SPEC>;
#[doc = "Timer_B Control Register"]
pub mod tb0ctl;
#[doc = "TB0CCTL0 (rw) register accessor: an alias for `Reg<TB0CCTL0_SPEC>`"]
pub type TB0CCTL0 = crate::Reg<tb0cctl0::TB0CCTL0_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl0;
#[doc = "TB0CCTL1 (rw) register accessor: an alias for `Reg<TB0CCTL1_SPEC>`"]
pub type TB0CCTL1 = crate::Reg<tb0cctl1::TB0CCTL1_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl1;
#[doc = "TB0CCTL2 (rw) register accessor: an alias for `Reg<TB0CCTL2_SPEC>`"]
pub type TB0CCTL2 = crate::Reg<tb0cctl2::TB0CCTL2_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl2;
#[doc = "TB0R (rw) register accessor: an alias for `Reg<TB0R_SPEC>`"]
pub type TB0R = crate::Reg<tb0r::TB0R_SPEC>;
#[doc = "Timer_B count register"]
pub mod tb0r;
#[doc = "TB0CCR0 (rw) register accessor: an alias for `Reg<TB0CCR0_SPEC>`"]
pub type TB0CCR0 = crate::Reg<tb0ccr0::TB0CCR0_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr0;
#[doc = "TB0CCR1 (rw) register accessor: an alias for `Reg<TB0CCR1_SPEC>`"]
pub type TB0CCR1 = crate::Reg<tb0ccr1::TB0CCR1_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr1;
#[doc = "TB0CCR2 (rw) register accessor: an alias for `Reg<TB0CCR2_SPEC>`"]
pub type TB0CCR2 = crate::Reg<tb0ccr2::TB0CCR2_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr2;
#[doc = "TB0EX0 (rw) register accessor: an alias for `Reg<TB0EX0_SPEC>`"]
pub type TB0EX0 = crate::Reg<tb0ex0::TB0EX0_SPEC>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb0ex0;
#[doc = "TB0IV (rw) register accessor: an alias for `Reg<TB0IV_SPEC>`"]
pub type TB0IV = crate::Reg<tb0iv::TB0IV_SPEC>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb0iv;
