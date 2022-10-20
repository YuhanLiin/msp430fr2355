#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb3ctl: TB3CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl0: TB3CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl1: TB3CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl2: TB3CCTL2,
    #[doc = "0x08 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl3: TB3CCTL3,
    #[doc = "0x0a - Timer_B Capture/Compare Control Register"]
    pub tb3cctl4: TB3CCTL4,
    #[doc = "0x0c - Timer_B Capture/Compare Control Register"]
    pub tb3cctl5: TB3CCTL5,
    #[doc = "0x0e - Timer_B Capture/Compare Control Register"]
    pub tb3cctl6: TB3CCTL6,
    #[doc = "0x10 - Timer_B count register"]
    pub tb3r: TB3R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb3ccr0: TB3CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb3ccr1: TB3CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb3ccr2: TB3CCR2,
    #[doc = "0x18 - Timer_B Capture/Compare Register"]
    pub tb3ccr3: TB3CCR3,
    #[doc = "0x1a - Timer_B Capture/Compare Register"]
    pub tb3ccr4: TB3CCR4,
    #[doc = "0x1c - Timer_B Capture/Compare Register"]
    pub tb3ccr5: TB3CCR5,
    #[doc = "0x1e - Timer_B Capture/Compare Register"]
    pub tb3ccr6: TB3CCR6,
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb3ex0: TB3EX0,
    _reserved17: [u8; 0x0c],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb3iv: TB3IV,
}
#[doc = "TB3CTL (rw) register accessor: an alias for `Reg<TB3CTL_SPEC>`"]
pub type TB3CTL = crate::Reg<tb3ctl::TB3CTL_SPEC>;
#[doc = "Timer_B Control Register"]
pub mod tb3ctl;
#[doc = "TB3CCTL0 (rw) register accessor: an alias for `Reg<TB3CCTL0_SPEC>`"]
pub type TB3CCTL0 = crate::Reg<tb3cctl0::TB3CCTL0_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl0;
#[doc = "TB3CCTL1 (rw) register accessor: an alias for `Reg<TB3CCTL1_SPEC>`"]
pub type TB3CCTL1 = crate::Reg<tb3cctl1::TB3CCTL1_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl1;
#[doc = "TB3CCTL2 (rw) register accessor: an alias for `Reg<TB3CCTL2_SPEC>`"]
pub type TB3CCTL2 = crate::Reg<tb3cctl2::TB3CCTL2_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl2;
#[doc = "TB3CCTL3 (rw) register accessor: an alias for `Reg<TB3CCTL3_SPEC>`"]
pub type TB3CCTL3 = crate::Reg<tb3cctl3::TB3CCTL3_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl3;
#[doc = "TB3CCTL4 (rw) register accessor: an alias for `Reg<TB3CCTL4_SPEC>`"]
pub type TB3CCTL4 = crate::Reg<tb3cctl4::TB3CCTL4_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl4;
#[doc = "TB3CCTL5 (rw) register accessor: an alias for `Reg<TB3CCTL5_SPEC>`"]
pub type TB3CCTL5 = crate::Reg<tb3cctl5::TB3CCTL5_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl5;
#[doc = "TB3CCTL6 (rw) register accessor: an alias for `Reg<TB3CCTL6_SPEC>`"]
pub type TB3CCTL6 = crate::Reg<tb3cctl6::TB3CCTL6_SPEC>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl6;
#[doc = "TB3R (rw) register accessor: an alias for `Reg<TB3R_SPEC>`"]
pub type TB3R = crate::Reg<tb3r::TB3R_SPEC>;
#[doc = "Timer_B count register"]
pub mod tb3r;
#[doc = "TB3CCR0 (rw) register accessor: an alias for `Reg<TB3CCR0_SPEC>`"]
pub type TB3CCR0 = crate::Reg<tb3ccr0::TB3CCR0_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr0;
#[doc = "TB3CCR1 (rw) register accessor: an alias for `Reg<TB3CCR1_SPEC>`"]
pub type TB3CCR1 = crate::Reg<tb3ccr1::TB3CCR1_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr1;
#[doc = "TB3CCR2 (rw) register accessor: an alias for `Reg<TB3CCR2_SPEC>`"]
pub type TB3CCR2 = crate::Reg<tb3ccr2::TB3CCR2_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr2;
#[doc = "TB3CCR3 (rw) register accessor: an alias for `Reg<TB3CCR3_SPEC>`"]
pub type TB3CCR3 = crate::Reg<tb3ccr3::TB3CCR3_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr3;
#[doc = "TB3CCR4 (rw) register accessor: an alias for `Reg<TB3CCR4_SPEC>`"]
pub type TB3CCR4 = crate::Reg<tb3ccr4::TB3CCR4_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr4;
#[doc = "TB3CCR5 (rw) register accessor: an alias for `Reg<TB3CCR5_SPEC>`"]
pub type TB3CCR5 = crate::Reg<tb3ccr5::TB3CCR5_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr5;
#[doc = "TB3CCR6 (rw) register accessor: an alias for `Reg<TB3CCR6_SPEC>`"]
pub type TB3CCR6 = crate::Reg<tb3ccr6::TB3CCR6_SPEC>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr6;
#[doc = "TB3EX0 (rw) register accessor: an alias for `Reg<TB3EX0_SPEC>`"]
pub type TB3EX0 = crate::Reg<tb3ex0::TB3EX0_SPEC>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb3ex0;
#[doc = "TB3IV (rw) register accessor: an alias for `Reg<TB3IV_SPEC>`"]
pub type TB3IV = crate::Reg<tb3iv::TB3IV_SPEC>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb3iv;
