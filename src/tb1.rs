#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tb1ctl: Tb1ctl,
    tb1cctl0: Tb1cctl0,
    tb1cctl1: Tb1cctl1,
    tb1cctl2: Tb1cctl2,
    _reserved4: [u8; 0x08],
    tb1r: Tb1r,
    tb1ccr0: Tb1ccr0,
    tb1ccr1: Tb1ccr1,
    tb1ccr2: Tb1ccr2,
    _reserved8: [u8; 0x08],
    tb1ex0: Tb1ex0,
    _reserved9: [u8; 0x0c],
    tb1iv: Tb1iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    #[inline(always)]
    pub const fn tb1ctl(&self) -> &Tb1ctl {
        &self.tb1ctl
    }
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb1cctl0(&self) -> &Tb1cctl0 {
        &self.tb1cctl0
    }
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb1cctl1(&self) -> &Tb1cctl1 {
        &self.tb1cctl1
    }
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb1cctl2(&self) -> &Tb1cctl2 {
        &self.tb1cctl2
    }
    #[doc = "0x10 - Timer_B count register"]
    #[inline(always)]
    pub const fn tb1r(&self) -> &Tb1r {
        &self.tb1r
    }
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb1ccr0(&self) -> &Tb1ccr0 {
        &self.tb1ccr0
    }
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb1ccr1(&self) -> &Tb1ccr1 {
        &self.tb1ccr1
    }
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb1ccr2(&self) -> &Tb1ccr2 {
        &self.tb1ccr2
    }
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    #[inline(always)]
    pub const fn tb1ex0(&self) -> &Tb1ex0 {
        &self.tb1ex0
    }
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn tb1iv(&self) -> &Tb1iv {
        &self.tb1iv
    }
}
#[doc = "TB1CTL (rw) register accessor: Timer_B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1ctl`] module"]
#[doc(alias = "TB1CTL")]
pub type Tb1ctl = crate::Reg<tb1ctl::Tb1ctlSpec>;
#[doc = "Timer_B Control Register"]
pub mod tb1ctl;
#[doc = "TB1CCTL0 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1cctl0`] module"]
#[doc(alias = "TB1CCTL0")]
pub type Tb1cctl0 = crate::Reg<tb1cctl0::Tb1cctl0Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl0;
#[doc = "TB1CCTL1 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1cctl1`] module"]
#[doc(alias = "TB1CCTL1")]
pub type Tb1cctl1 = crate::Reg<tb1cctl1::Tb1cctl1Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl1;
#[doc = "TB1CCTL2 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1cctl2`] module"]
#[doc(alias = "TB1CCTL2")]
pub type Tb1cctl2 = crate::Reg<tb1cctl2::Tb1cctl2Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl2;
#[doc = "TB1R (rw) register accessor: Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1r`] module"]
#[doc(alias = "TB1R")]
pub type Tb1r = crate::Reg<tb1r::Tb1rSpec>;
#[doc = "Timer_B count register"]
pub mod tb1r;
#[doc = "TB1CCR0 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1ccr0`] module"]
#[doc(alias = "TB1CCR0")]
pub type Tb1ccr0 = crate::Reg<tb1ccr0::Tb1ccr0Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr0;
#[doc = "TB1CCR1 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1ccr1`] module"]
#[doc(alias = "TB1CCR1")]
pub type Tb1ccr1 = crate::Reg<tb1ccr1::Tb1ccr1Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr1;
#[doc = "TB1CCR2 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1ccr2`] module"]
#[doc(alias = "TB1CCR2")]
pub type Tb1ccr2 = crate::Reg<tb1ccr2::Tb1ccr2Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr2;
#[doc = "TB1EX0 (rw) register accessor: Timer_Bx Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1ex0`] module"]
#[doc(alias = "TB1EX0")]
pub type Tb1ex0 = crate::Reg<tb1ex0::Tb1ex0Spec>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb1ex0;
#[doc = "TB1IV (rw) register accessor: Timer_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb1iv`] module"]
#[doc(alias = "TB1IV")]
pub type Tb1iv = crate::Reg<tb1iv::Tb1ivSpec>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb1iv;
