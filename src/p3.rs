#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: P3IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: P3OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: P3DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0a - Port 3 Select 0"]
    pub p3sel0: P3SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - Port 3 Select 1"]
    pub p3sel1: P3SEL1,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - Port 3 Interrupt Vector Register"]
    pub p3iv: P3IV,
    _reserved7: [u8; 0x06],
    #[doc = "0x16 - Port 3 Complement Select"]
    pub p3selc: P3SELC,
    _reserved8: [u8; 0x01],
    #[doc = "0x18 - Port 3 Interrupt Edge Select"]
    pub p3ies: P3IES,
    _reserved9: [u8; 0x01],
    #[doc = "0x1a - Port 3 Interrupt Enable"]
    pub p3ie: P3IE,
    _reserved10: [u8; 0x01],
    #[doc = "0x1c - Port 3 Interrupt Flag"]
    pub p3ifg: P3IFG,
}
#[doc = "P3IN (rw) register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3REN (rw) register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3SEL0 (rw) register accessor: an alias for `Reg<P3SEL0_SPEC>`"]
pub type P3SEL0 = crate::Reg<p3sel0::P3SEL0_SPEC>;
#[doc = "Port 3 Select 0"]
pub mod p3sel0;
#[doc = "P3SEL1 (rw) register accessor: an alias for `Reg<P3SEL1_SPEC>`"]
pub type P3SEL1 = crate::Reg<p3sel1::P3SEL1_SPEC>;
#[doc = "Port 3 Select 1"]
pub mod p3sel1;
#[doc = "P3SELC (rw) register accessor: an alias for `Reg<P3SELC_SPEC>`"]
pub type P3SELC = crate::Reg<p3selc::P3SELC_SPEC>;
#[doc = "Port 3 Complement Select"]
pub mod p3selc;
#[doc = "P3IES (rw) register accessor: an alias for `Reg<P3IES_SPEC>`"]
pub type P3IES = crate::Reg<p3ies::P3IES_SPEC>;
#[doc = "Port 3 Interrupt Edge Select"]
pub mod p3ies;
#[doc = "P3IE (rw) register accessor: an alias for `Reg<P3IE_SPEC>`"]
pub type P3IE = crate::Reg<p3ie::P3IE_SPEC>;
#[doc = "Port 3 Interrupt Enable"]
pub mod p3ie;
#[doc = "P3IFG (rw) register accessor: an alias for `Reg<P3IFG_SPEC>`"]
pub type P3IFG = crate::Reg<p3ifg::P3IFG_SPEC>;
#[doc = "Port 3 Interrupt Flag"]
pub mod p3ifg;
#[doc = "P3IV (rw) register accessor: an alias for `Reg<P3IV_SPEC>`"]
pub type P3IV = crate::Reg<p3iv::P3IV_SPEC>;
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
