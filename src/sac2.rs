#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sac2oa: Sac2oa,
    sac2pga: Sac2pga,
    sac2dac: Sac2dac,
    sac2dat: Sac2dat,
    sac2dacsts: Sac2dacsts,
    sac2iv: Sac2iv,
}
impl RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    #[inline(always)]
    pub const fn sac2oa(&self) -> &Sac2oa {
        &self.sac2oa
    }
    #[doc = "0x02 - SAC PGA Control Register"]
    #[inline(always)]
    pub const fn sac2pga(&self) -> &Sac2pga {
        &self.sac2pga
    }
    #[doc = "0x04 - SAC DAC Control Register"]
    #[inline(always)]
    pub const fn sac2dac(&self) -> &Sac2dac {
        &self.sac2dac
    }
    #[doc = "0x06 - SAC DAC Data Register"]
    #[inline(always)]
    pub const fn sac2dat(&self) -> &Sac2dat {
        &self.sac2dat
    }
    #[doc = "0x08 - SAC DAC Status Register"]
    #[inline(always)]
    pub const fn sac2dacsts(&self) -> &Sac2dacsts {
        &self.sac2dacsts
    }
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub const fn sac2iv(&self) -> &Sac2iv {
        &self.sac2iv
    }
}
#[doc = "SAC2OA (rw) register accessor: SAC OA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2oa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2oa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2oa`] module"]
#[doc(alias = "SAC2OA")]
pub type Sac2oa = crate::Reg<sac2oa::Sac2oaSpec>;
#[doc = "SAC OA Control Register"]
pub mod sac2oa;
#[doc = "SAC2PGA (rw) register accessor: SAC PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2pga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2pga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2pga`] module"]
#[doc(alias = "SAC2PGA")]
pub type Sac2pga = crate::Reg<sac2pga::Sac2pgaSpec>;
#[doc = "SAC PGA Control Register"]
pub mod sac2pga;
#[doc = "SAC2DAC (rw) register accessor: SAC DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2dac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2dac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2dac`] module"]
#[doc(alias = "SAC2DAC")]
pub type Sac2dac = crate::Reg<sac2dac::Sac2dacSpec>;
#[doc = "SAC DAC Control Register"]
pub mod sac2dac;
#[doc = "SAC2DAT (rw) register accessor: SAC DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2dat`] module"]
#[doc(alias = "SAC2DAT")]
pub type Sac2dat = crate::Reg<sac2dat::Sac2datSpec>;
#[doc = "SAC DAC Data Register"]
pub mod sac2dat;
#[doc = "SAC2DACSTS (rw) register accessor: SAC DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2dacsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2dacsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2dacsts`] module"]
#[doc(alias = "SAC2DACSTS")]
pub type Sac2dacsts = crate::Reg<sac2dacsts::Sac2dacstsSpec>;
#[doc = "SAC DAC Status Register"]
pub mod sac2dacsts;
#[doc = "SAC2IV (rw) register accessor: SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sac2iv`] module"]
#[doc(alias = "SAC2IV")]
pub type Sac2iv = crate::Reg<sac2iv::Sac2ivSpec>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac2iv;
