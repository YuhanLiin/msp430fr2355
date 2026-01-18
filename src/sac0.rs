#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sac0oa: Sac0oa,
    sac0pga: Sac0pga,
    sac0dac: Sac0dac,
    sac0dat: Sac0dat,
    sac0dacsts: Sac0dacsts,
    sac0iv: Sac0iv,
}
impl RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    #[inline(always)]
    pub const fn sac0oa(&self) -> &Sac0oa {
        &self.sac0oa
    }
    #[doc = "0x02 - SAC PGA Control Register"]
    #[inline(always)]
    pub const fn sac0pga(&self) -> &Sac0pga {
        &self.sac0pga
    }
    #[doc = "0x04 - SAC DAC Control Register"]
    #[inline(always)]
    pub const fn sac0dac(&self) -> &Sac0dac {
        &self.sac0dac
    }
    #[doc = "0x06 - SAC DAC Data Register"]
    #[inline(always)]
    pub const fn sac0dat(&self) -> &Sac0dat {
        &self.sac0dat
    }
    #[doc = "0x08 - SAC DAC Status Register"]
    #[inline(always)]
    pub const fn sac0dacsts(&self) -> &Sac0dacsts {
        &self.sac0dacsts
    }
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub const fn sac0iv(&self) -> &Sac0iv {
        &self.sac0iv
    }
}
#[doc = "SAC0OA (rw) register accessor: SAC OA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0oa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0oa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0oa`] module"]
#[doc(alias = "SAC0OA")]
pub type Sac0oa = crate::Reg<sac0oa::Sac0oaSpec>;
#[doc = "SAC OA Control Register"]
pub mod sac0oa;
#[doc = "SAC0PGA (rw) register accessor: SAC PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0pga`] module"]
#[doc(alias = "SAC0PGA")]
pub type Sac0pga = crate::Reg<sac0pga::Sac0pgaSpec>;
#[doc = "SAC PGA Control Register"]
pub mod sac0pga;
#[doc = "SAC0DAC (rw) register accessor: SAC DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0dac`] module"]
#[doc(alias = "SAC0DAC")]
pub type Sac0dac = crate::Reg<sac0dac::Sac0dacSpec>;
#[doc = "SAC DAC Control Register"]
pub mod sac0dac;
#[doc = "SAC0DAT (rw) register accessor: SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0dat`] module"]
#[doc(alias = "SAC0DAT")]
pub type Sac0dat = crate::Reg<sac0dat::Sac0datSpec>;
#[doc = "SAC DAC Data Register"]
pub mod sac0dat;
#[doc = "SAC0DACSTS (rw) register accessor: SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0dacsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0dacsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0dacsts`] module"]
#[doc(alias = "SAC0DACSTS")]
pub type Sac0dacsts = crate::Reg<sac0dacsts::Sac0dacstsSpec>;
#[doc = "SAC DAC Status Register"]
pub mod sac0dacsts;
#[doc = "SAC0IV (rw) register accessor: SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac0iv`] module"]
#[doc(alias = "SAC0IV")]
pub type Sac0iv = crate::Reg<sac0iv::Sac0ivSpec>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac0iv;
