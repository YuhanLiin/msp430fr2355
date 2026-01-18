#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    p2in: P2in,
    _reserved1: [u8; 0x01],
    p2out: P2out,
    _reserved2: [u8; 0x01],
    p2dir: P2dir,
    _reserved3: [u8; 0x01],
    p2ren: P2ren,
    _reserved4: [u8; 0x03],
    p2sel0: P2sel0,
    _reserved5: [u8; 0x01],
    p2sel1: P2sel1,
    _reserved6: [u8; 0x09],
    p2selc: P2selc,
    _reserved7: [u8; 0x01],
    p2ies: P2ies,
    _reserved8: [u8; 0x01],
    p2ie: P2ie,
    _reserved9: [u8; 0x01],
    p2ifg: P2ifg,
    p2iv: P2iv,
}
impl RegisterBlock {
    #[doc = "0x01 - Port 2 Input"]
    #[inline(always)]
    pub const fn p2in(&self) -> &P2in {
        &self.p2in
    }
    #[doc = "0x03 - Port 2 Output"]
    #[inline(always)]
    pub const fn p2out(&self) -> &P2out {
        &self.p2out
    }
    #[doc = "0x05 - Port 2 Direction"]
    #[inline(always)]
    pub const fn p2dir(&self) -> &P2dir {
        &self.p2dir
    }
    #[doc = "0x07 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub const fn p2ren(&self) -> &P2ren {
        &self.p2ren
    }
    #[doc = "0x0b - Port 2 Select 0"]
    #[inline(always)]
    pub const fn p2sel0(&self) -> &P2sel0 {
        &self.p2sel0
    }
    #[doc = "0x0d - Port 2 Select 1"]
    #[inline(always)]
    pub const fn p2sel1(&self) -> &P2sel1 {
        &self.p2sel1
    }
    #[doc = "0x17 - Port 2 Complement Select"]
    #[inline(always)]
    pub const fn p2selc(&self) -> &P2selc {
        &self.p2selc
    }
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p2ies(&self) -> &P2ies {
        &self.p2ies
    }
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn p2ie(&self) -> &P2ie {
        &self.p2ie
    }
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn p2ifg(&self) -> &P2ifg {
        &self.p2ifg
    }
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p2iv(&self) -> &P2iv {
        &self.p2iv
    }
}
#[doc = "P2IN (rw) register accessor: Port 2 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p2in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2in`] module"]
#[doc(alias = "P2IN")]
pub type P2in = crate::Reg<p2in::P2inSpec>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P2OUT (rw) register accessor: Port 2 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p2out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2out`] module"]
#[doc(alias = "P2OUT")]
pub type P2out = crate::Reg<p2out::P2outSpec>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P2DIR (rw) register accessor: Port 2 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p2dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2dir`] module"]
#[doc(alias = "P2DIR")]
pub type P2dir = crate::Reg<p2dir::P2dirSpec>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P2REN (rw) register accessor: Port 2 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ren`] module"]
#[doc(alias = "P2REN")]
pub type P2ren = crate::Reg<p2ren::P2renSpec>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "P2SEL0 (rw) register accessor: Port 2 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p2sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2sel0`] module"]
#[doc(alias = "P2SEL0")]
pub type P2sel0 = crate::Reg<p2sel0::P2sel0Spec>;
#[doc = "Port 2 Select 0"]
pub mod p2sel0;
#[doc = "P2SEL1 (rw) register accessor: Port 2 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p2sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2sel1`] module"]
#[doc(alias = "P2SEL1")]
pub type P2sel1 = crate::Reg<p2sel1::P2sel1Spec>;
#[doc = "Port 2 Select 1"]
pub mod p2sel1;
#[doc = "P2SELC (rw) register accessor: Port 2 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2selc`] module"]
#[doc(alias = "P2SELC")]
pub type P2selc = crate::Reg<p2selc::P2selcSpec>;
#[doc = "Port 2 Complement Select"]
pub mod p2selc;
#[doc = "P2IES (rw) register accessor: Port 2 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ies`] module"]
#[doc(alias = "P2IES")]
pub type P2ies = crate::Reg<p2ies::P2iesSpec>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P2IE (rw) register accessor: Port 2 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ie`] module"]
#[doc(alias = "P2IE")]
pub type P2ie = crate::Reg<p2ie::P2ieSpec>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P2IFG (rw) register accessor: Port 2 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ifg`] module"]
#[doc(alias = "P2IFG")]
pub type P2ifg = crate::Reg<p2ifg::P2ifgSpec>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P2IV (rw) register accessor: Port 2 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2iv`] module"]
#[doc(alias = "P2IV")]
pub type P2iv = crate::Reg<p2iv::P2ivSpec>;
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
