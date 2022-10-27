#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac1oa: SAC1OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac1pga: SAC1PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac1dac: SAC1DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac1dat: SAC1DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac1dacsts: SAC1DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac1iv: SAC1IV,
}
#[doc = "SAC1OA (rw) register accessor: an alias for `Reg<SAC1OA_SPEC>`"]
pub type SAC1OA = crate::Reg<sac1oa::SAC1OA_SPEC>;
#[doc = "SAC OA Control Register"]
pub mod sac1oa;
#[doc = "SAC1PGA (rw) register accessor: an alias for `Reg<SAC1PGA_SPEC>`"]
pub type SAC1PGA = crate::Reg<sac1pga::SAC1PGA_SPEC>;
#[doc = "SAC PGA Control Register"]
pub mod sac1pga;
#[doc = "SAC1DAC (rw) register accessor: an alias for `Reg<SAC1DAC_SPEC>`"]
pub type SAC1DAC = crate::Reg<sac1dac::SAC1DAC_SPEC>;
#[doc = "SAC DAC Control Register"]
pub mod sac1dac;
#[doc = "SAC1DAT (rw) register accessor: an alias for `Reg<SAC1DAT_SPEC>`"]
pub type SAC1DAT = crate::Reg<sac1dat::SAC1DAT_SPEC>;
#[doc = "SAC DAC Data Register"]
pub mod sac1dat;
#[doc = "SAC1DACSTS (rw) register accessor: an alias for `Reg<SAC1DACSTS_SPEC>`"]
pub type SAC1DACSTS = crate::Reg<sac1dacsts::SAC1DACSTS_SPEC>;
#[doc = "SAC DAC Status Register"]
pub mod sac1dacsts;
#[doc = "SAC1IV (rw) register accessor: an alias for `Reg<SAC1IV_SPEC>`"]
pub type SAC1IV = crate::Reg<sac1iv::SAC1IV_SPEC>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac1iv;
