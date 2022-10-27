#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb1ctl: TB1CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl0: TB1CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl1: TB1CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl2: TB1CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - Timer_B count register"]
    pub tb1r: TB1R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb1ccr0: TB1CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb1ccr1: TB1CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb1ccr2: TB1CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb1ex0: TB1EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb1iv: TB1IV,
}
#[doc = "TB1CTL (rw) register accessor: an alias for `Reg<TB1CTL_SPEC>`"]
pub type TB1CTL = crate::Reg<tb1ctl::TB1CTL_SPEC>;
#[doc = "Timer_B Control Register"]
pub mod tb1ctl;
#[doc = "TB1CCTL0 (rw) register accessor: an alias for `Reg<TB1CCTL0_SPEC>`"]
pub type TB1CCTL0 = crate::Reg<tb1cctl0::TB1CCTL0_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl0;
#[doc = "TB1CCTL1 (rw) register accessor: an alias for `Reg<TB1CCTL1_SPEC>`"]
pub type TB1CCTL1 = crate::Reg<tb1cctl1::TB1CCTL1_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl1;
#[doc = "TB1CCTL2 (rw) register accessor: an alias for `Reg<TB1CCTL2_SPEC>`"]
pub type TB1CCTL2 = crate::Reg<tb1cctl2::TB1CCTL2_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl2;
#[doc = "TB1R (rw) register accessor: an alias for `Reg<TB1R_SPEC>`"]
pub type TB1R = crate::Reg<tb1r::TB1R_SPEC>;
#[doc = "Timer_B count register"]
pub mod tb1r;
#[doc = "TB1CCR0 (rw) register accessor: an alias for `Reg<TB1CCR0_SPEC>`"]
pub type TB1CCR0 = crate::Reg<tb1ccr0::TB1CCR0_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr0;
#[doc = "TB1CCR1 (rw) register accessor: an alias for `Reg<TB1CCR1_SPEC>`"]
pub type TB1CCR1 = crate::Reg<tb1ccr1::TB1CCR1_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr1;
#[doc = "TB1CCR2 (rw) register accessor: an alias for `Reg<TB1CCR2_SPEC>`"]
pub type TB1CCR2 = crate::Reg<tb1ccr2::TB1CCR2_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr2;
#[doc = "TB1EX0 (rw) register accessor: an alias for `Reg<TB1EX0_SPEC>`"]
pub type TB1EX0 = crate::Reg<tb1ex0::TB1EX0_SPEC>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb1ex0;
#[doc = "TB1IV (rw) register accessor: an alias for `Reg<TB1IV_SPEC>`"]
pub type TB1IV = crate::Reg<tb1iv::TB1IV_SPEC>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb1iv;
