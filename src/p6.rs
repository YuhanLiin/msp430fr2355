#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Port 6 Input"]
    pub p6in: P6IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x03 - Port 6 Output"]
    pub p6out: P6OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x05 - Port 6 Direction"]
    pub p6dir: P6DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x07 - Port 6 Resistor Enable"]
    pub p6ren: P6REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0b - Port 6 Select 0"]
    pub p6sel0: P6SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0d - Port 6 Select 1"]
    pub p6sel1: P6SEL1,
    _reserved6: [u8; 0x09],
    #[doc = "0x17 - Port 6 Complement Select"]
    pub p6selc: P6SELC,
}
#[doc = "P6IN (rw) register accessor: an alias for `Reg<P6IN_SPEC>`"]
pub type P6IN = crate::Reg<p6in::P6IN_SPEC>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P6OUT (rw) register accessor: an alias for `Reg<P6OUT_SPEC>`"]
pub type P6OUT = crate::Reg<p6out::P6OUT_SPEC>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P6DIR (rw) register accessor: an alias for `Reg<P6DIR_SPEC>`"]
pub type P6DIR = crate::Reg<p6dir::P6DIR_SPEC>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P6REN (rw) register accessor: an alias for `Reg<P6REN_SPEC>`"]
pub type P6REN = crate::Reg<p6ren::P6REN_SPEC>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P6SEL0 (rw) register accessor: an alias for `Reg<P6SEL0_SPEC>`"]
pub type P6SEL0 = crate::Reg<p6sel0::P6SEL0_SPEC>;
#[doc = "Port 6 Select 0"]
pub mod p6sel0;
#[doc = "P6SEL1 (rw) register accessor: an alias for `Reg<P6SEL1_SPEC>`"]
pub type P6SEL1 = crate::Reg<p6sel1::P6SEL1_SPEC>;
#[doc = "Port 6 Select 1"]
pub mod p6sel1;
#[doc = "P6SELC (rw) register accessor: an alias for `Reg<P6SELC_SPEC>`"]
pub type P6SELC = crate::Reg<p6selc::P6SELC_SPEC>;
#[doc = "Port 6 Complement Select"]
pub mod p6selc;
