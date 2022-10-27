#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac2oa: SAC2OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac2pga: SAC2PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac2dac: SAC2DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac2dat: SAC2DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac2dacsts: SAC2DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac2iv: SAC2IV,
}
#[doc = "SAC2OA (rw) register accessor: an alias for `Reg<SAC2OA_SPEC>`"]
pub type SAC2OA = crate::Reg<sac2oa::SAC2OA_SPEC>;
#[doc = "SAC OA Control Register"]
pub mod sac2oa;
#[doc = "SAC2PGA (rw) register accessor: an alias for `Reg<SAC2PGA_SPEC>`"]
pub type SAC2PGA = crate::Reg<sac2pga::SAC2PGA_SPEC>;
#[doc = "SAC PGA Control Register"]
pub mod sac2pga;
#[doc = "SAC2DAC (rw) register accessor: an alias for `Reg<SAC2DAC_SPEC>`"]
pub type SAC2DAC = crate::Reg<sac2dac::SAC2DAC_SPEC>;
#[doc = "SAC DAC Control Register"]
pub mod sac2dac;
#[doc = "SAC2DAT (rw) register accessor: an alias for `Reg<SAC2DAT_SPEC>`"]
pub type SAC2DAT = crate::Reg<sac2dat::SAC2DAT_SPEC>;
#[doc = "SAC DAC Data Register"]
pub mod sac2dat;
#[doc = "SAC2DACSTS (rw) register accessor: an alias for `Reg<SAC2DACSTS_SPEC>`"]
pub type SAC2DACSTS = crate::Reg<sac2dacsts::SAC2DACSTS_SPEC>;
#[doc = "SAC DAC Status Register"]
pub mod sac2dacsts;
#[doc = "SAC2IV (rw) register accessor: an alias for `Reg<SAC2IV_SPEC>`"]
pub type SAC2IV = crate::Reg<sac2iv::SAC2IV_SPEC>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac2iv;
