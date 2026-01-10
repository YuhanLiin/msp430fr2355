#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpctl0: Cpctl0,
    cpctl1: Cpctl1,
    _reserved2: [u8; 0x02],
    cpint: Cpint,
    cpiv: Cpiv,
    _reserved4: [u8; 0x06],
    cpdacctl: Cpdacctl,
    cpdacdata: Cpdacdata,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    #[inline(always)]
    pub const fn cpctl0(&self) -> &Cpctl0 {
        &self.cpctl0
    }
    #[doc = "0x02 - Comparator Control Register 1"]
    #[inline(always)]
    pub const fn cpctl1(&self) -> &Cpctl1 {
        &self.cpctl1
    }
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    #[inline(always)]
    pub const fn cpint(&self) -> &Cpint {
        &self.cpint
    }
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    #[inline(always)]
    pub const fn cpiv(&self) -> &Cpiv {
        &self.cpiv
    }
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    #[inline(always)]
    pub const fn cpdacctl(&self) -> &Cpdacctl {
        &self.cpdacctl
    }
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    #[inline(always)]
    pub const fn cpdacdata(&self) -> &Cpdacdata {
        &self.cpdacdata
    }
}
#[doc = "CPCTL0 (rw) register accessor: Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpctl0`] module"]
#[doc(alias = "CPCTL0")]
pub type Cpctl0 = crate::Reg<cpctl0::Cpctl0Spec>;
#[doc = "Comparator Control Register 0"]
pub mod cpctl0;
#[doc = "CPCTL1 (rw) register accessor: Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpctl1`] module"]
#[doc(alias = "CPCTL1")]
pub type Cpctl1 = crate::Reg<cpctl1::Cpctl1Spec>;
#[doc = "Comparator Control Register 1"]
pub mod cpctl1;
#[doc = "CPINT (rw) register accessor: Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpint`] module"]
#[doc(alias = "CPINT")]
pub type Cpint = crate::Reg<cpint::CpintSpec>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cpint;
#[doc = "CPIV (rw) register accessor: Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpiv`] module"]
#[doc(alias = "CPIV")]
pub type Cpiv = crate::Reg<cpiv::CpivSpec>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cpiv;
#[doc = "CPDACCTL (rw) register accessor: 6-bit Comparator built-in DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpdacctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpdacctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpdacctl`] module"]
#[doc(alias = "CPDACCTL")]
pub type Cpdacctl = crate::Reg<cpdacctl::CpdacctlSpec>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cpdacctl;
#[doc = "CPDACDATA (rw) register accessor: 6-bit Comparator built-in DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpdacdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpdacdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpdacdata`] module"]
#[doc(alias = "CPDACDATA")]
pub type Cpdacdata = crate::Reg<cpdacdata::CpdacdataSpec>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cpdacdata;
