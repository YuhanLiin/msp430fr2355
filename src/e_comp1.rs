#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cp1ctl0: Cp1ctl0,
    cp1ctl1: Cp1ctl1,
    _reserved2: [u8; 0x02],
    cp1int: Cp1int,
    cp1iv: Cp1iv,
    _reserved4: [u8; 0x06],
    cp1dacctl: Cp1dacctl,
    cp1dacdata: Cp1dacdata,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    #[inline(always)]
    pub const fn cp1ctl0(&self) -> &Cp1ctl0 {
        &self.cp1ctl0
    }
    #[doc = "0x02 - Comparator Control Register 1"]
    #[inline(always)]
    pub const fn cp1ctl1(&self) -> &Cp1ctl1 {
        &self.cp1ctl1
    }
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    #[inline(always)]
    pub const fn cp1int(&self) -> &Cp1int {
        &self.cp1int
    }
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    #[inline(always)]
    pub const fn cp1iv(&self) -> &Cp1iv {
        &self.cp1iv
    }
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    #[inline(always)]
    pub const fn cp1dacctl(&self) -> &Cp1dacctl {
        &self.cp1dacctl
    }
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    #[inline(always)]
    pub const fn cp1dacdata(&self) -> &Cp1dacdata {
        &self.cp1dacdata
    }
}
#[doc = "CP1CTL0 (rw) register accessor: Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1ctl0`] module"]
#[doc(alias = "CP1CTL0")]
pub type Cp1ctl0 = crate::Reg<cp1ctl0::Cp1ctl0Spec>;
#[doc = "Comparator Control Register 0"]
pub mod cp1ctl0;
#[doc = "CP1CTL1 (rw) register accessor: Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1ctl1`] module"]
#[doc(alias = "CP1CTL1")]
pub type Cp1ctl1 = crate::Reg<cp1ctl1::Cp1ctl1Spec>;
#[doc = "Comparator Control Register 1"]
pub mod cp1ctl1;
#[doc = "CP1INT (rw) register accessor: Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1int`] module"]
#[doc(alias = "CP1INT")]
pub type Cp1int = crate::Reg<cp1int::Cp1intSpec>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cp1int;
#[doc = "CP1IV (rw) register accessor: Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1iv`] module"]
#[doc(alias = "CP1IV")]
pub type Cp1iv = crate::Reg<cp1iv::Cp1ivSpec>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cp1iv;
#[doc = "CP1DACCTL (rw) register accessor: 6-bit Comparator built-in DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1dacctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1dacctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1dacctl`] module"]
#[doc(alias = "CP1DACCTL")]
pub type Cp1dacctl = crate::Reg<cp1dacctl::Cp1dacctlSpec>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cp1dacctl;
#[doc = "CP1DACDATA (rw) register accessor: 6-bit Comparator built-in DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp1dacdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp1dacdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp1dacdata`] module"]
#[doc(alias = "CP1DACDATA")]
pub type Cp1dacdata = crate::Reg<cp1dacdata::Cp1dacdataSpec>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cp1dacdata;
