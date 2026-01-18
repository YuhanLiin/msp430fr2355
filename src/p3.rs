#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3in: P3in,
    _reserved1: [u8; 0x01],
    p3out: P3out,
    _reserved2: [u8; 0x01],
    p3dir: P3dir,
    _reserved3: [u8; 0x01],
    p3ren: P3ren,
    _reserved4: [u8; 0x03],
    p3sel0: P3sel0,
    _reserved5: [u8; 0x01],
    p3sel1: P3sel1,
    _reserved6: [u8; 0x01],
    p3iv: P3iv,
    _reserved7: [u8; 0x06],
    p3selc: P3selc,
    _reserved8: [u8; 0x01],
    p3ies: P3ies,
    _reserved9: [u8; 0x01],
    p3ie: P3ie,
    _reserved10: [u8; 0x01],
    p3ifg: P3ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3in {
        &self.p3in
    }
    #[doc = "0x02 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3out {
        &self.p3out
    }
    #[doc = "0x04 - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3dir {
        &self.p3dir
    }
    #[doc = "0x06 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3ren {
        &self.p3ren
    }
    #[doc = "0x0a - Port 3 Select 0"]
    #[inline(always)]
    pub const fn p3sel0(&self) -> &P3sel0 {
        &self.p3sel0
    }
    #[doc = "0x0c - Port 3 Select 1"]
    #[inline(always)]
    pub const fn p3sel1(&self) -> &P3sel1 {
        &self.p3sel1
    }
    #[doc = "0x0e - Port 3 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p3iv(&self) -> &P3iv {
        &self.p3iv
    }
    #[doc = "0x16 - Port 3 Complement Select"]
    #[inline(always)]
    pub const fn p3selc(&self) -> &P3selc {
        &self.p3selc
    }
    #[doc = "0x18 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p3ies(&self) -> &P3ies {
        &self.p3ies
    }
    #[doc = "0x1a - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub const fn p3ie(&self) -> &P3ie {
        &self.p3ie
    }
    #[doc = "0x1c - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub const fn p3ifg(&self) -> &P3ifg {
        &self.p3ifg
    }
}
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`] module"]
#[doc(alias = "P3IN")]
pub type P3in = crate::Reg<p3in::P3inSpec>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`] module"]
#[doc(alias = "P3OUT")]
pub type P3out = crate::Reg<p3out::P3outSpec>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`] module"]
#[doc(alias = "P3DIR")]
pub type P3dir = crate::Reg<p3dir::P3dirSpec>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`] module"]
#[doc(alias = "P3REN")]
pub type P3ren = crate::Reg<p3ren::P3renSpec>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3SEL0 (rw) register accessor: Port 3 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel0`] module"]
#[doc(alias = "P3SEL0")]
pub type P3sel0 = crate::Reg<p3sel0::P3sel0Spec>;
#[doc = "Port 3 Select 0"]
pub mod p3sel0;
#[doc = "P3SEL1 (rw) register accessor: Port 3 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel1`] module"]
#[doc(alias = "P3SEL1")]
pub type P3sel1 = crate::Reg<p3sel1::P3sel1Spec>;
#[doc = "Port 3 Select 1"]
pub mod p3sel1;
#[doc = "P3SELC (rw) register accessor: Port 3 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p3selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3selc`] module"]
#[doc(alias = "P3SELC")]
pub type P3selc = crate::Reg<p3selc::P3selcSpec>;
#[doc = "Port 3 Complement Select"]
pub mod p3selc;
#[doc = "P3IES (rw) register accessor: Port 3 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ies`] module"]
#[doc(alias = "P3IES")]
pub type P3ies = crate::Reg<p3ies::P3iesSpec>;
#[doc = "Port 3 Interrupt Edge Select"]
pub mod p3ies;
#[doc = "P3IE (rw) register accessor: Port 3 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ie`] module"]
#[doc(alias = "P3IE")]
pub type P3ie = crate::Reg<p3ie::P3ieSpec>;
#[doc = "Port 3 Interrupt Enable"]
pub mod p3ie;
#[doc = "P3IFG (rw) register accessor: Port 3 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ifg`] module"]
#[doc(alias = "P3IFG")]
pub type P3ifg = crate::Reg<p3ifg::P3ifgSpec>;
#[doc = "Port 3 Interrupt Flag"]
pub mod p3ifg;
#[doc = "P3IV (rw) register accessor: Port 3 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3iv`] module"]
#[doc(alias = "P3IV")]
pub type P3iv = crate::Reg<p3iv::P3ivSpec>;
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
