#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iccsc: Iccsc,
    iccmvs: Iccmvs,
    iccilsr0: Iccilsr0,
    iccilsr1: Iccilsr1,
    iccilsr2: Iccilsr2,
    iccilsr3: Iccilsr3,
}
impl RegisterBlock {
    #[doc = "0x00 - ICCSC"]
    #[inline(always)]
    pub const fn iccsc(&self) -> &Iccsc {
        &self.iccsc
    }
    #[doc = "0x02 - ICCMVS"]
    #[inline(always)]
    pub const fn iccmvs(&self) -> &Iccmvs {
        &self.iccmvs
    }
    #[doc = "0x04 - ICCILSR0"]
    #[inline(always)]
    pub const fn iccilsr0(&self) -> &Iccilsr0 {
        &self.iccilsr0
    }
    #[doc = "0x06 - ICCILSR1"]
    #[inline(always)]
    pub const fn iccilsr1(&self) -> &Iccilsr1 {
        &self.iccilsr1
    }
    #[doc = "0x08 - ICCILSR2"]
    #[inline(always)]
    pub const fn iccilsr2(&self) -> &Iccilsr2 {
        &self.iccilsr2
    }
    #[doc = "0x0a - ICCILSR3"]
    #[inline(always)]
    pub const fn iccilsr3(&self) -> &Iccilsr3 {
        &self.iccilsr3
    }
}
#[doc = "ICCSC (rw) register accessor: ICCSC\n\nYou can [`read`](crate::Reg::read) this register and get [`iccsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccsc`] module"]
#[doc(alias = "ICCSC")]
pub type Iccsc = crate::Reg<iccsc::IccscSpec>;
#[doc = "ICCSC"]
pub mod iccsc;
#[doc = "ICCMVS (rw) register accessor: ICCMVS\n\nYou can [`read`](crate::Reg::read) this register and get [`iccmvs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccmvs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccmvs`] module"]
#[doc(alias = "ICCMVS")]
pub type Iccmvs = crate::Reg<iccmvs::IccmvsSpec>;
#[doc = "ICCMVS"]
pub mod iccmvs;
#[doc = "ICCILSR0 (rw) register accessor: ICCILSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccilsr0`] module"]
#[doc(alias = "ICCILSR0")]
pub type Iccilsr0 = crate::Reg<iccilsr0::Iccilsr0Spec>;
#[doc = "ICCILSR0"]
pub mod iccilsr0;
#[doc = "ICCILSR1 (rw) register accessor: ICCILSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccilsr1`] module"]
#[doc(alias = "ICCILSR1")]
pub type Iccilsr1 = crate::Reg<iccilsr1::Iccilsr1Spec>;
#[doc = "ICCILSR1"]
pub mod iccilsr1;
#[doc = "ICCILSR2 (rw) register accessor: ICCILSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccilsr2`] module"]
#[doc(alias = "ICCILSR2")]
pub type Iccilsr2 = crate::Reg<iccilsr2::Iccilsr2Spec>;
#[doc = "ICCILSR2"]
pub mod iccilsr2;
#[doc = "ICCILSR3 (rw) register accessor: ICCILSR3\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccilsr3`] module"]
#[doc(alias = "ICCILSR3")]
pub type Iccilsr3 = crate::Reg<iccilsr3::Iccilsr3Spec>;
#[doc = "ICCILSR3"]
pub mod iccilsr3;
