#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_pmmctl: [u8; 0x02],
    pmmctl1: Pmmctl1,
    pmmctl2: Pmmctl2,
    _reserved3: [u8; 0x04],
    pmmifg: Pmmifg,
    _reserved4: [u8; 0x04],
    pm5ctl0: Pm5ctl0,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Management Module control register 0"]
    #[inline(always)]
    pub const fn pmmctl0(&self) -> &Pmmctl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x01 - Power Management Module Control Register 0, upper byte"]
    #[inline(always)]
    pub const fn pmmctl0_h(&self) -> &Pmmctl0H {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x02 - Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings."]
    #[inline(always)]
    pub const fn pmmctl1(&self) -> &Pmmctl1 {
        &self.pmmctl1
    }
    #[doc = "0x04 - Power Management Module Control Register 2"]
    #[inline(always)]
    pub const fn pmmctl2(&self) -> &Pmmctl2 {
        &self.pmmctl2
    }
    #[doc = "0x0a - PMM interrupt flag register"]
    #[inline(always)]
    pub const fn pmmifg(&self) -> &Pmmifg {
        &self.pmmifg
    }
    #[doc = "0x10 - Power mode 5 control register 0"]
    #[inline(always)]
    pub const fn pm5ctl0(&self) -> &Pm5ctl0 {
        &self.pm5ctl0
    }
}
#[doc = "PMMCTL0 (rw) register accessor: Power Management Module control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl0`] module"]
#[doc(alias = "PMMCTL0")]
pub type Pmmctl0 = crate::Reg<pmmctl0::Pmmctl0Spec>;
#[doc = "Power Management Module control register 0"]
pub mod pmmctl0;
#[doc = "PMMCTL0_H (rw) register accessor: Power Management Module Control Register 0, upper byte\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl0_h`] module"]
#[doc(alias = "PMMCTL0_H")]
pub type Pmmctl0H = crate::Reg<pmmctl0_h::Pmmctl0HSpec>;
#[doc = "Power Management Module Control Register 0, upper byte"]
pub mod pmmctl0_h;
#[doc = "PMMCTL1 (rw) register accessor: Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl1`] module"]
#[doc(alias = "PMMCTL1")]
pub type Pmmctl1 = crate::Reg<pmmctl1::Pmmctl1Spec>;
#[doc = "Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings."]
pub mod pmmctl1;
#[doc = "PMMCTL2 (rw) register accessor: Power Management Module Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl2`] module"]
#[doc(alias = "PMMCTL2")]
pub type Pmmctl2 = crate::Reg<pmmctl2::Pmmctl2Spec>;
#[doc = "Power Management Module Control Register 2"]
pub mod pmmctl2;
#[doc = "PMMIFG (rw) register accessor: PMM interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmifg`] module"]
#[doc(alias = "PMMIFG")]
pub type Pmmifg = crate::Reg<pmmifg::PmmifgSpec>;
#[doc = "PMM interrupt flag register"]
pub mod pmmifg;
#[doc = "PM5CTL0 (rw) register accessor: Power mode 5 control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pm5ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm5ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm5ctl0`] module"]
#[doc(alias = "PM5CTL0")]
pub type Pm5ctl0 = crate::Reg<pm5ctl0::Pm5ctl0Spec>;
#[doc = "Power mode 5 control register 0"]
pub mod pm5ctl0;
