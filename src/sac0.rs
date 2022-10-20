#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac0oa: SAC0OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac0pga: SAC0PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac0dac: SAC0DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac0dat: SAC0DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac0dacsts: SAC0DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac0iv: SAC0IV,
}
#[doc = "SAC0OA (rw) register accessor: an alias for `Reg<SAC0OA_SPEC>`"]
pub type SAC0OA = crate::Reg<sac0oa::SAC0OA_SPEC>;
#[doc = "SAC OA Control Register"]
pub mod sac0oa;
#[doc = "SAC0PGA (rw) register accessor: an alias for `Reg<SAC0PGA_SPEC>`"]
pub type SAC0PGA = crate::Reg<sac0pga::SAC0PGA_SPEC>;
#[doc = "SAC PGA Control Register"]
pub mod sac0pga;
#[doc = "SAC0DAC (rw) register accessor: an alias for `Reg<SAC0DAC_SPEC>`"]
pub type SAC0DAC = crate::Reg<sac0dac::SAC0DAC_SPEC>;
#[doc = "SAC DAC Control Register"]
pub mod sac0dac;
#[doc = "SAC0DAT (rw) register accessor: an alias for `Reg<SAC0DAT_SPEC>`"]
pub type SAC0DAT = crate::Reg<sac0dat::SAC0DAT_SPEC>;
#[doc = "SAC DAC Data Register"]
pub mod sac0dat;
#[doc = "SAC0DACSTS (rw) register accessor: an alias for `Reg<SAC0DACSTS_SPEC>`"]
pub type SAC0DACSTS = crate::Reg<sac0dacsts::SAC0DACSTS_SPEC>;
#[doc = "SAC DAC Status Register"]
pub mod sac0dacsts;
#[doc = "SAC0IV (rw) register accessor: an alias for `Reg<SAC0IV_SPEC>`"]
pub type SAC0IV = crate::Reg<sac0iv::SAC0IV_SPEC>;
#[doc = "SAC Interrupt Vector Register"]
pub mod sac0iv;
