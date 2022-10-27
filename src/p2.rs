#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Port 2 Input"]
    pub p2in: P2IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - Port 2 Output"]
    pub p2out: P2OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x05 - Port 2 Direction"]
    pub p2dir: P2DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x07 - Port 2 Resistor Enable"]
    pub p2ren: P2REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0b - Port 2 Select 0"]
    pub p2sel0: P2SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0d - Port 2 Select 1"]
    pub p2sel1: P2SEL1,
    _reserved6: [u8; 0x09],
    #[doc = "0x17 - Port 2 Complement Select"]
    pub p2selc: P2SELC,
    _reserved7: [u8; 0x01],
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    pub p2ies: P2IES,
    _reserved8: [u8; 0x01],
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    pub p2ie: P2IE,
    _reserved9: [u8; 0x01],
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    pub p2ifg: P2IFG,
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    pub p2iv: P2IV,
}
#[doc = "P2IN (rw) register accessor: an alias for `Reg<P2IN_SPEC>`"]
pub type P2IN = crate::Reg<p2in::P2IN_SPEC>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P2OUT (rw) register accessor: an alias for `Reg<P2OUT_SPEC>`"]
pub type P2OUT = crate::Reg<p2out::P2OUT_SPEC>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P2DIR (rw) register accessor: an alias for `Reg<P2DIR_SPEC>`"]
pub type P2DIR = crate::Reg<p2dir::P2DIR_SPEC>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P2REN (rw) register accessor: an alias for `Reg<P2REN_SPEC>`"]
pub type P2REN = crate::Reg<p2ren::P2REN_SPEC>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "P2SEL0 (rw) register accessor: an alias for `Reg<P2SEL0_SPEC>`"]
pub type P2SEL0 = crate::Reg<p2sel0::P2SEL0_SPEC>;
#[doc = "Port 2 Select 0"]
pub mod p2sel0;
#[doc = "P2SEL1 (rw) register accessor: an alias for `Reg<P2SEL1_SPEC>`"]
pub type P2SEL1 = crate::Reg<p2sel1::P2SEL1_SPEC>;
#[doc = "Port 2 Select 1"]
pub mod p2sel1;
#[doc = "P2SELC (rw) register accessor: an alias for `Reg<P2SELC_SPEC>`"]
pub type P2SELC = crate::Reg<p2selc::P2SELC_SPEC>;
#[doc = "Port 2 Complement Select"]
pub mod p2selc;
#[doc = "P2IES (rw) register accessor: an alias for `Reg<P2IES_SPEC>`"]
pub type P2IES = crate::Reg<p2ies::P2IES_SPEC>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P2IE (rw) register accessor: an alias for `Reg<P2IE_SPEC>`"]
pub type P2IE = crate::Reg<p2ie::P2IE_SPEC>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P2IFG (rw) register accessor: an alias for `Reg<P2IFG_SPEC>`"]
pub type P2IFG = crate::Reg<p2ifg::P2IFG_SPEC>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P2IV (rw) register accessor: an alias for `Reg<P2IV_SPEC>`"]
pub type P2IV = crate::Reg<p2iv::P2IV_SPEC>;
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
