#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sac3oa: Sac3oa,
    sac3pga: Sac3pga,
    sac3dac: Sac3dac,
    sac3dat: Sac3dat,
    sac3dacsts: Sac3dacsts,
    sac3iv: Sac3iv,
}
impl RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    #[inline(always)]
    pub const fn sac3oa(&self) -> &Sac3oa {
        &self.sac3oa
    }
    #[doc = "0x02 - SAC PGA Control Register"]
    #[inline(always)]
    pub const fn sac3pga(&self) -> &Sac3pga {
        &self.sac3pga
    }
    #[doc = "0x04 - SAC DAC Control Register"]
    #[inline(always)]
    pub const fn sac3dac(&self) -> &Sac3dac {
        &self.sac3dac
    }
    #[doc = "0x06 - SAC DAC Data Register"]
    #[inline(always)]
    pub const fn sac3dat(&self) -> &Sac3dat {
        &self.sac3dat
    }
    #[doc = "0x08 - SAC DAC Status Register"]
    #[inline(always)]
    pub const fn sac3dacsts(&self) -> &Sac3dacsts {
        &self.sac3dacsts
    }
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub const fn sac3iv(&self) -> &Sac3iv {
        &self.sac3iv
    }
}
#[doc = "SAC3OA (rw) register accessor: SAC OA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3oa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3oa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3oa`] module"]
#[doc(alias = "SAC3OA")]
pub type Sac3oa = crate::Reg<sac3oa::Sac3oaSpec>;
#[doc = "SAC OA Control Register"]
pub mod sac3oa;
#[doc = "SAC3PGA (rw) register accessor: SAC PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3pga`] module"]
#[doc(alias = "SAC3PGA")]
pub type Sac3pga = crate::Reg<sac3pga::Sac3pgaSpec>;
#[doc = "SAC PGA Control Register"]
pub mod sac3pga;
#[doc = "SAC3DAC (rw) register accessor: SAC DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3dac`] module"]
#[doc(alias = "SAC3DAC")]
pub type Sac3dac = crate::Reg<sac3dac::Sac3dacSpec>;
#[doc = "SAC DAC Control Register"]
pub mod sac3dac;
#[doc = "SAC3DAT (rw) register accessor: SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3dat`] module"]
#[doc(alias = "SAC3DAT")]
pub type Sac3dat = crate::Reg<sac3dat::Sac3datSpec>;
#[doc = "SAC DAC Data Register"]
pub mod sac3dat;
#[doc = "SAC3DACSTS (rw) register accessor: SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3dacsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3dacsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3dacsts`] module"]
#[doc(alias = "SAC3DACSTS")]
pub type Sac3dacsts = crate::Reg<sac3dacsts::Sac3dacstsSpec>;
#[doc = "SAC DAC Status Register"]
pub mod sac3dacsts;
#[doc = "SAC3IV (rw) register accessor: SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac3iv`] module"]
#[doc(alias = "SAC3IV")]
pub type Sac3iv = crate::Reg<sac3iv::Sac3ivSpec>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac3iv;
