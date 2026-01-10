#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tb2ctl: Tb2ctl,
    tb2cctl0: Tb2cctl0,
    tb2cctl1: Tb2cctl1,
    tb2cctl2: Tb2cctl2,
    _reserved4: [u8; 0x08],
    tb2r: Tb2r,
    tb2ccr0: Tb2ccr0,
    tb2ccr1: Tb2ccr1,
    tb2ccr2: Tb2ccr2,
    _reserved8: [u8; 0x08],
    tb2ex0: Tb2ex0,
    _reserved9: [u8; 0x0c],
    tb2iv: Tb2iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    #[inline(always)]
    pub const fn tb2ctl(&self) -> &Tb2ctl {
        &self.tb2ctl
    }
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb2cctl0(&self) -> &Tb2cctl0 {
        &self.tb2cctl0
    }
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb2cctl1(&self) -> &Tb2cctl1 {
        &self.tb2cctl1
    }
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb2cctl2(&self) -> &Tb2cctl2 {
        &self.tb2cctl2
    }
    #[doc = "0x10 - Timer_B count register"]
    #[inline(always)]
    pub const fn tb2r(&self) -> &Tb2r {
        &self.tb2r
    }
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb2ccr0(&self) -> &Tb2ccr0 {
        &self.tb2ccr0
    }
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb2ccr1(&self) -> &Tb2ccr1 {
        &self.tb2ccr1
    }
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb2ccr2(&self) -> &Tb2ccr2 {
        &self.tb2ccr2
    }
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    #[inline(always)]
    pub const fn tb2ex0(&self) -> &Tb2ex0 {
        &self.tb2ex0
    }
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn tb2iv(&self) -> &Tb2iv {
        &self.tb2iv
    }
}
#[doc = "TB2CTL (rw) register accessor: Timer_B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2ctl`] module"]
#[doc(alias = "TB2CTL")]
pub type Tb2ctl = crate::Reg<tb2ctl::Tb2ctlSpec>;
#[doc = "Timer_B Control Register"]
pub mod tb2ctl;
#[doc = "TB2CCTL0 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2cctl0`] module"]
#[doc(alias = "TB2CCTL0")]
pub type Tb2cctl0 = crate::Reg<tb2cctl0::Tb2cctl0Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl0;
#[doc = "TB2CCTL1 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2cctl1`] module"]
#[doc(alias = "TB2CCTL1")]
pub type Tb2cctl1 = crate::Reg<tb2cctl1::Tb2cctl1Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl1;
#[doc = "TB2CCTL2 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2cctl2`] module"]
#[doc(alias = "TB2CCTL2")]
pub type Tb2cctl2 = crate::Reg<tb2cctl2::Tb2cctl2Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl2;
#[doc = "TB2R (rw) register accessor: Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2r`] module"]
#[doc(alias = "TB2R")]
pub type Tb2r = crate::Reg<tb2r::Tb2rSpec>;
#[doc = "Timer_B count register"]
pub mod tb2r;
#[doc = "TB2CCR0 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2ccr0`] module"]
#[doc(alias = "TB2CCR0")]
pub type Tb2ccr0 = crate::Reg<tb2ccr0::Tb2ccr0Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr0;
#[doc = "TB2CCR1 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2ccr1`] module"]
#[doc(alias = "TB2CCR1")]
pub type Tb2ccr1 = crate::Reg<tb2ccr1::Tb2ccr1Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr1;
#[doc = "TB2CCR2 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2ccr2`] module"]
#[doc(alias = "TB2CCR2")]
pub type Tb2ccr2 = crate::Reg<tb2ccr2::Tb2ccr2Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr2;
#[doc = "TB2EX0 (rw) register accessor: Timer_Bx Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2ex0`] module"]
#[doc(alias = "TB2EX0")]
pub type Tb2ex0 = crate::Reg<tb2ex0::Tb2ex0Spec>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb2ex0;
#[doc = "TB2IV (rw) register accessor: Timer_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb2iv`] module"]
#[doc(alias = "TB2IV")]
pub type Tb2iv = crate::Reg<tb2iv::Tb2ivSpec>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb2iv;
