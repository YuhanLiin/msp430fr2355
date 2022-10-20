#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICCSC"]
    pub iccsc: ICCSC,
    #[doc = "0x02 - ICCMVS"]
    pub iccmvs: ICCMVS,
    #[doc = "0x04 - ICCILSR0"]
    pub iccilsr0: ICCILSR0,
    #[doc = "0x06 - ICCILSR1"]
    pub iccilsr1: ICCILSR1,
    #[doc = "0x08 - ICCILSR2"]
    pub iccilsr2: ICCILSR2,
    #[doc = "0x0a - ICCILSR3"]
    pub iccilsr3: ICCILSR3,
}
#[doc = "ICCSC (rw) register accessor: an alias for `Reg<ICCSC_SPEC>`"]
pub type ICCSC = crate::Reg<iccsc::ICCSC_SPEC>;
#[doc = "ICCSC"]
pub mod iccsc;
#[doc = "ICCMVS (rw) register accessor: an alias for `Reg<ICCMVS_SPEC>`"]
pub type ICCMVS = crate::Reg<iccmvs::ICCMVS_SPEC>;
#[doc = "ICCMVS"]
pub mod iccmvs;
#[doc = "ICCILSR0 (rw) register accessor: an alias for `Reg<ICCILSR0_SPEC>`"]
pub type ICCILSR0 = crate::Reg<iccilsr0::ICCILSR0_SPEC>;
#[doc = "ICCILSR0"]
pub mod iccilsr0;
#[doc = "ICCILSR1 (rw) register accessor: an alias for `Reg<ICCILSR1_SPEC>`"]
pub type ICCILSR1 = crate::Reg<iccilsr1::ICCILSR1_SPEC>;
#[doc = "ICCILSR1"]
pub mod iccilsr1;
#[doc = "ICCILSR2 (rw) register accessor: an alias for `Reg<ICCILSR2_SPEC>`"]
pub type ICCILSR2 = crate::Reg<iccilsr2::ICCILSR2_SPEC>;
#[doc = "ICCILSR2"]
pub mod iccilsr2;
#[doc = "ICCILSR3 (rw) register accessor: an alias for `Reg<ICCILSR3_SPEC>`"]
pub type ICCILSR3 = crate::Reg<iccilsr3::ICCILSR3_SPEC>;
#[doc = "ICCILSR3"]
pub mod iccilsr3;
