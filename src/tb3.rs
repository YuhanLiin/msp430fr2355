#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tb3ctl: Tb3ctl,
    tb3cctl0: Tb3cctl0,
    tb3cctl1: Tb3cctl1,
    tb3cctl2: Tb3cctl2,
    tb3cctl3: Tb3cctl3,
    tb3cctl4: Tb3cctl4,
    tb3cctl5: Tb3cctl5,
    tb3cctl6: Tb3cctl6,
    tb3r: Tb3r,
    tb3ccr0: Tb3ccr0,
    tb3ccr1: Tb3ccr1,
    tb3ccr2: Tb3ccr2,
    tb3ccr3: Tb3ccr3,
    tb3ccr4: Tb3ccr4,
    tb3ccr5: Tb3ccr5,
    tb3ccr6: Tb3ccr6,
    tb3ex0: Tb3ex0,
    _reserved17: [u8; 0x0c],
    tb3iv: Tb3iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    #[inline(always)]
    pub const fn tb3ctl(&self) -> &Tb3ctl {
        &self.tb3ctl
    }
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl0(&self) -> &Tb3cctl0 {
        &self.tb3cctl0
    }
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl1(&self) -> &Tb3cctl1 {
        &self.tb3cctl1
    }
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl2(&self) -> &Tb3cctl2 {
        &self.tb3cctl2
    }
    #[doc = "0x08 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl3(&self) -> &Tb3cctl3 {
        &self.tb3cctl3
    }
    #[doc = "0x0a - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl4(&self) -> &Tb3cctl4 {
        &self.tb3cctl4
    }
    #[doc = "0x0c - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl5(&self) -> &Tb3cctl5 {
        &self.tb3cctl5
    }
    #[doc = "0x0e - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb3cctl6(&self) -> &Tb3cctl6 {
        &self.tb3cctl6
    }
    #[doc = "0x10 - Timer_B count register"]
    #[inline(always)]
    pub const fn tb3r(&self) -> &Tb3r {
        &self.tb3r
    }
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr0(&self) -> &Tb3ccr0 {
        &self.tb3ccr0
    }
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr1(&self) -> &Tb3ccr1 {
        &self.tb3ccr1
    }
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr2(&self) -> &Tb3ccr2 {
        &self.tb3ccr2
    }
    #[doc = "0x18 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr3(&self) -> &Tb3ccr3 {
        &self.tb3ccr3
    }
    #[doc = "0x1a - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr4(&self) -> &Tb3ccr4 {
        &self.tb3ccr4
    }
    #[doc = "0x1c - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr5(&self) -> &Tb3ccr5 {
        &self.tb3ccr5
    }
    #[doc = "0x1e - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb3ccr6(&self) -> &Tb3ccr6 {
        &self.tb3ccr6
    }
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    #[inline(always)]
    pub const fn tb3ex0(&self) -> &Tb3ex0 {
        &self.tb3ex0
    }
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn tb3iv(&self) -> &Tb3iv {
        &self.tb3iv
    }
}
#[doc = "TB3CTL (rw) register accessor: Timer_B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ctl`] module"]
#[doc(alias = "TB3CTL")]
pub type Tb3ctl = crate::Reg<tb3ctl::Tb3ctlSpec>;
#[doc = "Timer_B Control Register"]
pub mod tb3ctl;
#[doc = "TB3CCTL0 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl0`] module"]
#[doc(alias = "TB3CCTL0")]
pub type Tb3cctl0 = crate::Reg<tb3cctl0::Tb3cctl0Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl0;
#[doc = "TB3CCTL1 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl1`] module"]
#[doc(alias = "TB3CCTL1")]
pub type Tb3cctl1 = crate::Reg<tb3cctl1::Tb3cctl1Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl1;
#[doc = "TB3CCTL2 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl2`] module"]
#[doc(alias = "TB3CCTL2")]
pub type Tb3cctl2 = crate::Reg<tb3cctl2::Tb3cctl2Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl2;
#[doc = "TB3CCTL3 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl3`] module"]
#[doc(alias = "TB3CCTL3")]
pub type Tb3cctl3 = crate::Reg<tb3cctl3::Tb3cctl3Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl3;
#[doc = "TB3CCTL4 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl4`] module"]
#[doc(alias = "TB3CCTL4")]
pub type Tb3cctl4 = crate::Reg<tb3cctl4::Tb3cctl4Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl4;
#[doc = "TB3CCTL5 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl5`] module"]
#[doc(alias = "TB3CCTL5")]
pub type Tb3cctl5 = crate::Reg<tb3cctl5::Tb3cctl5Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl5;
#[doc = "TB3CCTL6 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3cctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3cctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3cctl6`] module"]
#[doc(alias = "TB3CCTL6")]
pub type Tb3cctl6 = crate::Reg<tb3cctl6::Tb3cctl6Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl6;
#[doc = "TB3R (rw) register accessor: Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3r`] module"]
#[doc(alias = "TB3R")]
pub type Tb3r = crate::Reg<tb3r::Tb3rSpec>;
#[doc = "Timer_B count register"]
pub mod tb3r;
#[doc = "TB3CCR0 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr0`] module"]
#[doc(alias = "TB3CCR0")]
pub type Tb3ccr0 = crate::Reg<tb3ccr0::Tb3ccr0Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr0;
#[doc = "TB3CCR1 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr1`] module"]
#[doc(alias = "TB3CCR1")]
pub type Tb3ccr1 = crate::Reg<tb3ccr1::Tb3ccr1Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr1;
#[doc = "TB3CCR2 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr2`] module"]
#[doc(alias = "TB3CCR2")]
pub type Tb3ccr2 = crate::Reg<tb3ccr2::Tb3ccr2Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr2;
#[doc = "TB3CCR3 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr3`] module"]
#[doc(alias = "TB3CCR3")]
pub type Tb3ccr3 = crate::Reg<tb3ccr3::Tb3ccr3Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr3;
#[doc = "TB3CCR4 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr4`] module"]
#[doc(alias = "TB3CCR4")]
pub type Tb3ccr4 = crate::Reg<tb3ccr4::Tb3ccr4Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr4;
#[doc = "TB3CCR5 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr5`] module"]
#[doc(alias = "TB3CCR5")]
pub type Tb3ccr5 = crate::Reg<tb3ccr5::Tb3ccr5Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr5;
#[doc = "TB3CCR6 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ccr6`] module"]
#[doc(alias = "TB3CCR6")]
pub type Tb3ccr6 = crate::Reg<tb3ccr6::Tb3ccr6Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr6;
#[doc = "TB3EX0 (rw) register accessor: Timer_Bx Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3ex0`] module"]
#[doc(alias = "TB3EX0")]
pub type Tb3ex0 = crate::Reg<tb3ex0::Tb3ex0Spec>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb3ex0;
#[doc = "TB3IV (rw) register accessor: Timer_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb3iv`] module"]
#[doc(alias = "TB3IV")]
pub type Tb3iv = crate::Reg<tb3iv::Tb3ivSpec>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb3iv;
