#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    p4in: P4in,
    _reserved1: [u8; 0x01],
    p4out: P4out,
    _reserved2: [u8; 0x01],
    p4dir: P4dir,
    _reserved3: [u8; 0x01],
    p4ren: P4ren,
    _reserved4: [u8; 0x03],
    p4sel0: P4sel0,
    _reserved5: [u8; 0x01],
    p4sel1: P4sel1,
    _reserved6: [u8; 0x09],
    p4selc: P4selc,
    _reserved7: [u8; 0x01],
    p4ies: P4ies,
    _reserved8: [u8; 0x01],
    p4ie: P4ie,
    _reserved9: [u8; 0x01],
    p4ifg: P4ifg,
    p4iv: P4iv,
}
impl RegisterBlock {
    #[doc = "0x01 - Port 4 Input"]
    #[inline(always)]
    pub const fn p4in(&self) -> &P4in {
        &self.p4in
    }
    #[doc = "0x03 - Port 4 Output"]
    #[inline(always)]
    pub const fn p4out(&self) -> &P4out {
        &self.p4out
    }
    #[doc = "0x05 - Port 4 Direction"]
    #[inline(always)]
    pub const fn p4dir(&self) -> &P4dir {
        &self.p4dir
    }
    #[doc = "0x07 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub const fn p4ren(&self) -> &P4ren {
        &self.p4ren
    }
    #[doc = "0x0b - Port 4 Select 0"]
    #[inline(always)]
    pub const fn p4sel0(&self) -> &P4sel0 {
        &self.p4sel0
    }
    #[doc = "0x0d - Port 4 Select 1"]
    #[inline(always)]
    pub const fn p4sel1(&self) -> &P4sel1 {
        &self.p4sel1
    }
    #[doc = "0x17 - Port 4 Complement Select"]
    #[inline(always)]
    pub const fn p4selc(&self) -> &P4selc {
        &self.p4selc
    }
    #[doc = "0x19 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p4ies(&self) -> &P4ies {
        &self.p4ies
    }
    #[doc = "0x1b - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub const fn p4ie(&self) -> &P4ie {
        &self.p4ie
    }
    #[doc = "0x1d - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub const fn p4ifg(&self) -> &P4ifg {
        &self.p4ifg
    }
    #[doc = "0x1e - Port 4 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p4iv(&self) -> &P4iv {
        &self.p4iv
    }
}
#[doc = "P4IN (rw) register accessor: Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4in`] module"]
#[doc(alias = "P4IN")]
pub type P4in = crate::Reg<p4in::P4inSpec>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P4OUT (rw) register accessor: Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4out`] module"]
#[doc(alias = "P4OUT")]
pub type P4out = crate::Reg<p4out::P4outSpec>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P4DIR (rw) register accessor: Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4dir`] module"]
#[doc(alias = "P4DIR")]
pub type P4dir = crate::Reg<p4dir::P4dirSpec>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P4REN (rw) register accessor: Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ren`] module"]
#[doc(alias = "P4REN")]
pub type P4ren = crate::Reg<p4ren::P4renSpec>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P4SEL0 (rw) register accessor: Port 4 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4sel0`] module"]
#[doc(alias = "P4SEL0")]
pub type P4sel0 = crate::Reg<p4sel0::P4sel0Spec>;
#[doc = "Port 4 Select 0"]
pub mod p4sel0;
#[doc = "P4SEL1 (rw) register accessor: Port 4 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4sel1`] module"]
#[doc(alias = "P4SEL1")]
pub type P4sel1 = crate::Reg<p4sel1::P4sel1Spec>;
#[doc = "Port 4 Select 1"]
pub mod p4sel1;
#[doc = "P4SELC (rw) register accessor: Port 4 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p4selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4selc`] module"]
#[doc(alias = "P4SELC")]
pub type P4selc = crate::Reg<p4selc::P4selcSpec>;
#[doc = "Port 4 Complement Select"]
pub mod p4selc;
#[doc = "P4IES (rw) register accessor: Port 4 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ies`] module"]
#[doc(alias = "P4IES")]
pub type P4ies = crate::Reg<p4ies::P4iesSpec>;
#[doc = "Port 4 Interrupt Edge Select"]
pub mod p4ies;
#[doc = "P4IE (rw) register accessor: Port 4 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ie`] module"]
#[doc(alias = "P4IE")]
pub type P4ie = crate::Reg<p4ie::P4ieSpec>;
#[doc = "Port 4 Interrupt Enable"]
pub mod p4ie;
#[doc = "P4IFG (rw) register accessor: Port 4 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ifg`] module"]
#[doc(alias = "P4IFG")]
pub type P4ifg = crate::Reg<p4ifg::P4ifgSpec>;
#[doc = "Port 4 Interrupt Flag"]
pub mod p4ifg;
#[doc = "P4IV (rw) register accessor: Port 4 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p4iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4iv`] module"]
#[doc(alias = "P4IV")]
pub type P4iv = crate::Reg<p4iv::P4ivSpec>;
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
