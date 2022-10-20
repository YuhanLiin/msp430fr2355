#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac3oa: SAC3OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac3pga: SAC3PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac3dac: SAC3DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac3dat: SAC3DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac3dacsts: SAC3DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac3iv: SAC3IV,
}
#[doc = "SAC3OA (rw) register accessor: an alias for `Reg<SAC3OA_SPEC>`"]
pub type SAC3OA = crate::Reg<sac3oa::SAC3OA_SPEC>;
#[doc = "SAC OA Control Register"]
pub mod sac3oa;
#[doc = "SAC3PGA (rw) register accessor: an alias for `Reg<SAC3PGA_SPEC>`"]
pub type SAC3PGA = crate::Reg<sac3pga::SAC3PGA_SPEC>;
#[doc = "SAC PGA Control Register"]
pub mod sac3pga;
#[doc = "SAC3DAC (rw) register accessor: an alias for `Reg<SAC3DAC_SPEC>`"]
pub type SAC3DAC = crate::Reg<sac3dac::SAC3DAC_SPEC>;
#[doc = "SAC DAC Control Register"]
pub mod sac3dac;
#[doc = "SAC3DAT (rw) register accessor: an alias for `Reg<SAC3DAT_SPEC>`"]
pub type SAC3DAT = crate::Reg<sac3dat::SAC3DAT_SPEC>;
#[doc = "SAC DAC Data Register"]
pub mod sac3dat;
#[doc = "SAC3DACSTS (rw) register accessor: an alias for `Reg<SAC3DACSTS_SPEC>`"]
pub type SAC3DACSTS = crate::Reg<sac3dacsts::SAC3DACSTS_SPEC>;
#[doc = "SAC DAC Status Register"]
pub mod sac3dacsts;
#[doc = "SAC3IV (rw) register accessor: an alias for `Reg<SAC3IV_SPEC>`"]
pub type SAC3IV = crate::Reg<sac3iv::SAC3IV_SPEC>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac3iv;
