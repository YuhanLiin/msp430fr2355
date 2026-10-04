#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iccsc: Iccsc,
    iccmvs: Iccmvs,
    iccilsr: [Iccilsr; 4],
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
    #[doc = "0x04..0x0c - Interrupt Compare Controller Interrupt Level Setting Register"]
    #[inline(always)]
    pub const fn iccilsr(&self, n: usize) -> &Iccilsr {
        &self.iccilsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Interrupt Compare Controller Interrupt Level Setting Register"]
    #[inline(always)]
    pub fn iccilsr_iter(&self) -> impl Iterator<Item = &Iccilsr> {
        self.iccilsr.iter()
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
#[doc = "ICCILSR (rw) register accessor: Interrupt Compare Controller Interrupt Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccilsr`] module"]
#[doc(alias = "ICCILSR")]
pub type Iccilsr = crate::Reg<iccilsr::IccilsrSpec>;
#[doc = "Interrupt Compare Controller Interrupt Level Setting Register"]
pub mod iccilsr;
