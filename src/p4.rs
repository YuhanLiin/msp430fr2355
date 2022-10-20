#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: P4IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: P4OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: P4DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: P4REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0b - Port 4 Select 0"]
    pub p4sel0: P4SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0d - Port 4 Select 1"]
    pub p4sel1: P4SEL1,
    _reserved6: [u8; 0x09],
    #[doc = "0x17 - Port 4 Complement Select"]
    pub p4selc: P4SELC,
    _reserved7: [u8; 0x01],
    #[doc = "0x19 - Port 4 Interrupt Edge Select"]
    pub p4ies: P4IES,
    _reserved8: [u8; 0x01],
    #[doc = "0x1b - Port 4 Interrupt Enable"]
    pub p4ie: P4IE,
    _reserved9: [u8; 0x01],
    #[doc = "0x1d - Port 4 Interrupt Flag"]
    pub p4ifg: P4IFG,
    #[doc = "0x1e - Port 4 Interrupt Vector Register"]
    pub p4iv: P4IV,
}
#[doc = "P4IN (rw) register accessor: an alias for `Reg<P4IN_SPEC>`"]
pub type P4IN = crate::Reg<p4in::P4IN_SPEC>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P4OUT (rw) register accessor: an alias for `Reg<P4OUT_SPEC>`"]
pub type P4OUT = crate::Reg<p4out::P4OUT_SPEC>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P4DIR (rw) register accessor: an alias for `Reg<P4DIR_SPEC>`"]
pub type P4DIR = crate::Reg<p4dir::P4DIR_SPEC>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P4REN (rw) register accessor: an alias for `Reg<P4REN_SPEC>`"]
pub type P4REN = crate::Reg<p4ren::P4REN_SPEC>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P4SEL0 (rw) register accessor: an alias for `Reg<P4SEL0_SPEC>`"]
pub type P4SEL0 = crate::Reg<p4sel0::P4SEL0_SPEC>;
#[doc = "Port 4 Select 0"]
pub mod p4sel0;
#[doc = "P4SEL1 (rw) register accessor: an alias for `Reg<P4SEL1_SPEC>`"]
pub type P4SEL1 = crate::Reg<p4sel1::P4SEL1_SPEC>;
#[doc = "Port 4 Select 1"]
pub mod p4sel1;
#[doc = "P4SELC (rw) register accessor: an alias for `Reg<P4SELC_SPEC>`"]
pub type P4SELC = crate::Reg<p4selc::P4SELC_SPEC>;
#[doc = "Port 4 Complement Select"]
pub mod p4selc;
#[doc = "P4IES (rw) register accessor: an alias for `Reg<P4IES_SPEC>`"]
pub type P4IES = crate::Reg<p4ies::P4IES_SPEC>;
#[doc = "Port 4 Interrupt Edge Select"]
pub mod p4ies;
#[doc = "P4IE (rw) register accessor: an alias for `Reg<P4IE_SPEC>`"]
pub type P4IE = crate::Reg<p4ie::P4IE_SPEC>;
#[doc = "Port 4 Interrupt Enable"]
pub mod p4ie;
#[doc = "P4IFG (rw) register accessor: an alias for `Reg<P4IFG_SPEC>`"]
pub type P4IFG = crate::Reg<p4ifg::P4IFG_SPEC>;
#[doc = "Port 4 Interrupt Flag"]
pub mod p4ifg;
#[doc = "P4IV (rw) register accessor: an alias for `Reg<P4IV_SPEC>`"]
pub type P4IV = crate::Reg<p4iv::P4IV_SPEC>;
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
