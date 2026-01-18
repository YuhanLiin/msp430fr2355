#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p1in: P1in,
    _reserved1: [u8; 0x01],
    p1out: P1out,
    _reserved2: [u8; 0x01],
    p1dir: P1dir,
    _reserved3: [u8; 0x01],
    p1ren: P1ren,
    _reserved4: [u8; 0x03],
    p1sel0: P1sel0,
    _reserved5: [u8; 0x01],
    p1sel1: P1sel1,
    _reserved6: [u8; 0x01],
    p1iv: P1iv,
    _reserved7: [u8; 0x06],
    p1selc: P1selc,
    _reserved8: [u8; 0x01],
    p1ies: P1ies,
    _reserved9: [u8; 0x01],
    p1ie: P1ie,
    _reserved10: [u8; 0x01],
    p1ifg: P1ifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    #[inline(always)]
    pub const fn p1in(&self) -> &P1in {
        &self.p1in
    }
    #[doc = "0x02 - Port 1 Output"]
    #[inline(always)]
    pub const fn p1out(&self) -> &P1out {
        &self.p1out
    }
    #[doc = "0x04 - Port 1 Direction"]
    #[inline(always)]
    pub const fn p1dir(&self) -> &P1dir {
        &self.p1dir
    }
    #[doc = "0x06 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub const fn p1ren(&self) -> &P1ren {
        &self.p1ren
    }
    #[doc = "0x0a - Port 1 Select 0"]
    #[inline(always)]
    pub const fn p1sel0(&self) -> &P1sel0 {
        &self.p1sel0
    }
    #[doc = "0x0c - Port 1 Select 1"]
    #[inline(always)]
    pub const fn p1sel1(&self) -> &P1sel1 {
        &self.p1sel1
    }
    #[doc = "0x0e - Port 1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p1iv(&self) -> &P1iv {
        &self.p1iv
    }
    #[doc = "0x16 - Port 1 Complement Select"]
    #[inline(always)]
    pub const fn p1selc(&self) -> &P1selc {
        &self.p1selc
    }
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p1ies(&self) -> &P1ies {
        &self.p1ies
    }
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn p1ie(&self) -> &P1ie {
        &self.p1ie
    }
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn p1ifg(&self) -> &P1ifg {
        &self.p1ifg
    }
}
#[doc = "P1IN (rw) register accessor: Port 1 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p1in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1in`] module"]
#[doc(alias = "P1IN")]
pub type P1in = crate::Reg<p1in::P1inSpec>;
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "P1OUT (rw) register accessor: Port 1 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p1out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1out`] module"]
#[doc(alias = "P1OUT")]
pub type P1out = crate::Reg<p1out::P1outSpec>;
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "P1DIR (rw) register accessor: Port 1 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p1dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1dir`] module"]
#[doc(alias = "P1DIR")]
pub type P1dir = crate::Reg<p1dir::P1dirSpec>;
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "P1REN (rw) register accessor: Port 1 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ren`] module"]
#[doc(alias = "P1REN")]
pub type P1ren = crate::Reg<p1ren::P1renSpec>;
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "P1SEL0 (rw) register accessor: Port 1 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1sel0`] module"]
#[doc(alias = "P1SEL0")]
pub type P1sel0 = crate::Reg<p1sel0::P1sel0Spec>;
#[doc = "Port 1 Select 0"]
pub mod p1sel0;
#[doc = "P1SEL1 (rw) register accessor: Port 1 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p1sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1sel1`] module"]
#[doc(alias = "P1SEL1")]
pub type P1sel1 = crate::Reg<p1sel1::P1sel1Spec>;
#[doc = "Port 1 Select 1"]
pub mod p1sel1;
#[doc = "P1SELC (rw) register accessor: Port 1 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1selc`] module"]
#[doc(alias = "P1SELC")]
pub type P1selc = crate::Reg<p1selc::P1selcSpec>;
#[doc = "Port 1 Complement Select"]
pub mod p1selc;
#[doc = "P1IES (rw) register accessor: Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ies`] module"]
#[doc(alias = "P1IES")]
pub type P1ies = crate::Reg<p1ies::P1iesSpec>;
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "P1IE (rw) register accessor: Port 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ie`] module"]
#[doc(alias = "P1IE")]
pub type P1ie = crate::Reg<p1ie::P1ieSpec>;
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "P1IFG (rw) register accessor: Port 1 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ifg`] module"]
#[doc(alias = "P1IFG")]
pub type P1ifg = crate::Reg<p1ifg::P1ifgSpec>;
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "P1IV (rw) register accessor: Port 1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1iv`] module"]
#[doc(alias = "P1IV")]
pub type P1iv = crate::Reg<p1iv::P1ivSpec>;
#[doc = "Port 1 Interrupt Vector Register"]
pub mod p1iv;
