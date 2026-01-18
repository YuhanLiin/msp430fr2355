#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frctl0: Frctl0,
    _reserved1: [u8; 0x02],
    gcctl0: Gcctl0,
    gcctl1: Gcctl1,
}
impl RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control Register 0"]
    #[inline(always)]
    pub const fn frctl0(&self) -> &Frctl0 {
        &self.frctl0
    }
    #[doc = "0x04 - General Control Register 0"]
    #[inline(always)]
    pub const fn gcctl0(&self) -> &Gcctl0 {
        &self.gcctl0
    }
    #[doc = "0x06 - General Control Register 1"]
    #[inline(always)]
    pub const fn gcctl1(&self) -> &Gcctl1 {
        &self.gcctl1
    }
}
#[doc = "FRCTL0 (rw) register accessor: FRAM Controller Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`frctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frctl0`] module"]
#[doc(alias = "FRCTL0")]
pub type Frctl0 = crate::Reg<frctl0::Frctl0Spec>;
#[doc = "FRAM Controller Control Register 0"]
pub mod frctl0;
#[doc = "GCCTL0 (rw) register accessor: General Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcctl0`] module"]
#[doc(alias = "GCCTL0")]
pub type Gcctl0 = crate::Reg<gcctl0::Gcctl0Spec>;
#[doc = "General Control Register 0"]
pub mod gcctl0;
#[doc = "GCCTL1 (rw) register accessor: General Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcctl1`] module"]
#[doc(alias = "GCCTL1")]
pub type Gcctl1 = crate::Reg<gcctl1::Gcctl1Spec>;
#[doc = "General Control Register 1"]
pub mod gcctl1;
