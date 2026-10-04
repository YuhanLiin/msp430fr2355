#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cp0ctl0: Cp0ctl0,
    cp0ctl1: Cp0ctl1,
    _reserved2: [u8; 0x02],
    cp0int: Cp0int,
    cp0iv: Cp0iv,
    _reserved4: [u8; 0x06],
    cp0dacctl: Cp0dacctl,
    cp0dacdata: Cp0dacdata,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    #[inline(always)]
    pub const fn cp0ctl0(&self) -> &Cp0ctl0 {
        &self.cp0ctl0
    }
    #[doc = "0x02 - Comparator Control Register 1"]
    #[inline(always)]
    pub const fn cp0ctl1(&self) -> &Cp0ctl1 {
        &self.cp0ctl1
    }
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    #[inline(always)]
    pub const fn cp0int(&self) -> &Cp0int {
        &self.cp0int
    }
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    #[inline(always)]
    pub const fn cp0iv(&self) -> &Cp0iv {
        &self.cp0iv
    }
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    #[inline(always)]
    pub const fn cp0dacctl(&self) -> &Cp0dacctl {
        &self.cp0dacctl
    }
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    #[inline(always)]
    pub const fn cp0dacdata(&self) -> &Cp0dacdata {
        &self.cp0dacdata
    }
}
#[doc = "CP0CTL0 (rw) register accessor: Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0ctl0`] module"]
#[doc(alias = "CP0CTL0")]
pub type Cp0ctl0 = crate::Reg<cp0ctl0::Cp0ctl0Spec>;
#[doc = "Comparator Control Register 0"]
pub mod cp0ctl0;
#[doc = "CP0CTL1 (rw) register accessor: Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0ctl1`] module"]
#[doc(alias = "CP0CTL1")]
pub type Cp0ctl1 = crate::Reg<cp0ctl1::Cp0ctl1Spec>;
#[doc = "Comparator Control Register 1"]
pub mod cp0ctl1;
#[doc = "CP0INT (rw) register accessor: Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0int`] module"]
#[doc(alias = "CP0INT")]
pub type Cp0int = crate::Reg<cp0int::Cp0intSpec>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cp0int;
#[doc = "CP0IV (rw) register accessor: Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0iv`] module"]
#[doc(alias = "CP0IV")]
pub type Cp0iv = crate::Reg<cp0iv::Cp0ivSpec>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cp0iv;
#[doc = "CP0DACCTL (rw) register accessor: 6-bit Comparator built-in DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0dacctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0dacctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0dacctl`] module"]
#[doc(alias = "CP0DACCTL")]
pub type Cp0dacctl = crate::Reg<cp0dacctl::Cp0dacctlSpec>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cp0dacctl;
#[doc = "CP0DACDATA (rw) register accessor: 6-bit Comparator built-in DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0dacdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0dacdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp0dacdata`] module"]
#[doc(alias = "CP0DACDATA")]
pub type Cp0dacdata = crate::Reg<cp0dacdata::Cp0dacdataSpec>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cp0dacdata;
