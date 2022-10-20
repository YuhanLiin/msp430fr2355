#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control"]
    pub sysctl: SYSCTL,
    #[doc = "0x02 - Bootstrap Loader Configuration Register"]
    pub sysbslc: SYSBSLC,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - JTAG Mailbox Control"]
    pub sysjmbc: SYSJMBC,
    #[doc = "0x08 - JTAG Mailbox Input"]
    pub sysjmbi0: SYSJMBI0,
    #[doc = "0x0a - JTAG Mailbox Input 1 Register"]
    pub sysjmbi1: SYSJMBI1,
    #[doc = "0x0c - JTAG Mailbox Output"]
    pub sysjmbo0: SYSJMBO0,
    #[doc = "0x0e - JTAG Mailbox Output 1 Register"]
    pub sysjmbo1: SYSJMBO1,
    _reserved7: [u8; 0x0a],
    #[doc = "0x1a - User NMI Vector Generator"]
    pub sysuniv: SYSUNIV,
    #[doc = "0x1c - System NMI Vector Generator"]
    pub syssniv: SYSSNIV,
    #[doc = "0x1e - Reset Vector Generator"]
    pub sysrstiv: SYSRSTIV,
    #[doc = "0x20 - System Configuration Register 0"]
    pub syscfg0: SYSCFG0,
    #[doc = "0x22 - System Configuration Register 1"]
    pub syscfg1: SYSCFG1,
    #[doc = "0x24 - System Configuration Register 2"]
    pub syscfg2: SYSCFG2,
    #[doc = "0x26 - System Configuration Register 3"]
    pub syscfg3: SYSCFG3,
}
#[doc = "SYSCTL (rw) register accessor: an alias for `Reg<SYSCTL_SPEC>`"]
pub type SYSCTL = crate::Reg<sysctl::SYSCTL_SPEC>;
#[doc = "System Control"]
pub mod sysctl;
#[doc = "SYSBSLC (rw) register accessor: an alias for `Reg<SYSBSLC_SPEC>`"]
pub type SYSBSLC = crate::Reg<sysbslc::SYSBSLC_SPEC>;
#[doc = "Bootstrap Loader Configuration Register"]
pub mod sysbslc;
#[doc = "SYSJMBC (rw) register accessor: an alias for `Reg<SYSJMBC_SPEC>`"]
pub type SYSJMBC = crate::Reg<sysjmbc::SYSJMBC_SPEC>;
#[doc = "JTAG Mailbox Control"]
pub mod sysjmbc;
#[doc = "SYSJMBI0 (rw) register accessor: an alias for `Reg<SYSJMBI0_SPEC>`"]
pub type SYSJMBI0 = crate::Reg<sysjmbi0::SYSJMBI0_SPEC>;
#[doc = "JTAG Mailbox Input"]
pub mod sysjmbi0;
#[doc = "SYSJMBI1 (rw) register accessor: an alias for `Reg<SYSJMBI1_SPEC>`"]
pub type SYSJMBI1 = crate::Reg<sysjmbi1::SYSJMBI1_SPEC>;
#[doc = "JTAG Mailbox Input 1 Register"]
pub mod sysjmbi1;
#[doc = "SYSJMBO0 (rw) register accessor: an alias for `Reg<SYSJMBO0_SPEC>`"]
pub type SYSJMBO0 = crate::Reg<sysjmbo0::SYSJMBO0_SPEC>;
#[doc = "JTAG Mailbox Output"]
pub mod sysjmbo0;
#[doc = "SYSJMBO1 (rw) register accessor: an alias for `Reg<SYSJMBO1_SPEC>`"]
pub type SYSJMBO1 = crate::Reg<sysjmbo1::SYSJMBO1_SPEC>;
#[doc = "JTAG Mailbox Output 1 Register"]
pub mod sysjmbo1;
#[doc = "SYSUNIV (rw) register accessor: an alias for `Reg<SYSUNIV_SPEC>`"]
pub type SYSUNIV = crate::Reg<sysuniv::SYSUNIV_SPEC>;
#[doc = "User NMI Vector Generator"]
pub mod sysuniv;
#[doc = "SYSSNIV (rw) register accessor: an alias for `Reg<SYSSNIV_SPEC>`"]
pub type SYSSNIV = crate::Reg<syssniv::SYSSNIV_SPEC>;
#[doc = "System NMI Vector Generator"]
pub mod syssniv;
#[doc = "SYSRSTIV (rw) register accessor: an alias for `Reg<SYSRSTIV_SPEC>`"]
pub type SYSRSTIV = crate::Reg<sysrstiv::SYSRSTIV_SPEC>;
#[doc = "Reset Vector Generator"]
pub mod sysrstiv;
#[doc = "SYSCFG0 (rw) register accessor: an alias for `Reg<SYSCFG0_SPEC>`"]
pub type SYSCFG0 = crate::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration Register 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (rw) register accessor: an alias for `Reg<SYSCFG1_SPEC>`"]
pub type SYSCFG1 = crate::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration Register 1"]
pub mod syscfg1;
#[doc = "SYSCFG2 (rw) register accessor: an alias for `Reg<SYSCFG2_SPEC>`"]
pub type SYSCFG2 = crate::Reg<syscfg2::SYSCFG2_SPEC>;
#[doc = "System Configuration Register 2"]
pub mod syscfg2;
#[doc = "SYSCFG3 (rw) register accessor: an alias for `Reg<SYSCFG3_SPEC>`"]
pub type SYSCFG3 = crate::Reg<syscfg3::SYSCFG3_SPEC>;
#[doc = "System Configuration Register 3"]
pub mod syscfg3;
