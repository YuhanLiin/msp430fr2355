#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: P5IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: P5OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: P5DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: P5REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0a - Port 5 Select 0"]
    pub p5sel0: P5SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - Port 5 Select 1"]
    pub p5sel1: P5SEL1,
    _reserved6: [u8; 0x09],
    #[doc = "0x16 - Port 5 Complement Select"]
    pub p5selc: P5SELC,
}
#[doc = "P5IN (rw) register accessor: an alias for `Reg<P5IN_SPEC>`"]
pub type P5IN = crate::Reg<p5in::P5IN_SPEC>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P5OUT (rw) register accessor: an alias for `Reg<P5OUT_SPEC>`"]
pub type P5OUT = crate::Reg<p5out::P5OUT_SPEC>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P5DIR (rw) register accessor: an alias for `Reg<P5DIR_SPEC>`"]
pub type P5DIR = crate::Reg<p5dir::P5DIR_SPEC>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P5REN (rw) register accessor: an alias for `Reg<P5REN_SPEC>`"]
pub type P5REN = crate::Reg<p5ren::P5REN_SPEC>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P5SEL0 (rw) register accessor: an alias for `Reg<P5SEL0_SPEC>`"]
pub type P5SEL0 = crate::Reg<p5sel0::P5SEL0_SPEC>;
#[doc = "Port 5 Select 0"]
pub mod p5sel0;
#[doc = "P5SEL1 (rw) register accessor: an alias for `Reg<P5SEL1_SPEC>`"]
pub type P5SEL1 = crate::Reg<p5sel1::P5SEL1_SPEC>;
#[doc = "Port 5 Select 1"]
pub mod p5sel1;
#[doc = "P5SELC (rw) register accessor: an alias for `Reg<P5SELC_SPEC>`"]
pub type P5SELC = crate::Reg<p5selc::P5SELC_SPEC>;
#[doc = "Port 5 Complement Select"]
pub mod p5selc;
