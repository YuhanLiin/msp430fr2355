#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sac1oa: Sac1oa,
    sac1pga: Sac1pga,
    sac1dac: Sac1dac,
    sac1dat: Sac1dat,
    sac1dacsts: Sac1dacsts,
    sac1iv: Sac1iv,
}
impl RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    #[inline(always)]
    pub const fn sac1oa(&self) -> &Sac1oa {
        &self.sac1oa
    }
    #[doc = "0x02 - SAC PGA Control Register"]
    #[inline(always)]
    pub const fn sac1pga(&self) -> &Sac1pga {
        &self.sac1pga
    }
    #[doc = "0x04 - SAC DAC Control Register"]
    #[inline(always)]
    pub const fn sac1dac(&self) -> &Sac1dac {
        &self.sac1dac
    }
    #[doc = "0x06 - SAC DAC Data Register"]
    #[inline(always)]
    pub const fn sac1dat(&self) -> &Sac1dat {
        &self.sac1dat
    }
    #[doc = "0x08 - SAC DAC Status Register"]
    #[inline(always)]
    pub const fn sac1dacsts(&self) -> &Sac1dacsts {
        &self.sac1dacsts
    }
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub const fn sac1iv(&self) -> &Sac1iv {
        &self.sac1iv
    }
}
#[doc = "SAC1OA (rw) register accessor: SAC OA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1oa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1oa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1oa`] module"]
#[doc(alias = "SAC1OA")]
pub type Sac1oa = crate::Reg<sac1oa::Sac1oaSpec>;
#[doc = "SAC OA Control Register"]
pub mod sac1oa;
#[doc = "SAC1PGA (rw) register accessor: SAC PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1pga`] module"]
#[doc(alias = "SAC1PGA")]
pub type Sac1pga = crate::Reg<sac1pga::Sac1pgaSpec>;
#[doc = "SAC PGA Control Register"]
pub mod sac1pga;
#[doc = "SAC1DAC (rw) register accessor: SAC DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1dac`] module"]
#[doc(alias = "SAC1DAC")]
pub type Sac1dac = crate::Reg<sac1dac::Sac1dacSpec>;
#[doc = "SAC DAC Control Register"]
pub mod sac1dac;
#[doc = "SAC1DAT (rw) register accessor: SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1dat`] module"]
#[doc(alias = "SAC1DAT")]
pub type Sac1dat = crate::Reg<sac1dat::Sac1datSpec>;
#[doc = "SAC DAC Data Register"]
pub mod sac1dat;
#[doc = "SAC1DACSTS (rw) register accessor: SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1dacsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1dacsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1dacsts`] module"]
#[doc(alias = "SAC1DACSTS")]
pub type Sac1dacsts = crate::Reg<sac1dacsts::Sac1dacstsSpec>;
#[doc = "SAC DAC Status Register"]
pub mod sac1dacsts;
#[doc = "SAC1IV (rw) register accessor: SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac1iv`] module"]
#[doc(alias = "SAC1IV")]
pub type Sac1iv = crate::Reg<sac1iv::Sac1ivSpec>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac1iv;
