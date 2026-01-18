#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtcctl: Rtcctl,
    _reserved1: [u8; 0x02],
    rtciv: Rtciv,
    _reserved2: [u8; 0x02],
    rtcmod: Rtcmod,
    _reserved3: [u8; 0x02],
    rtccnt: Rtccnt,
}
impl RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    #[inline(always)]
    pub const fn rtcctl(&self) -> &Rtcctl {
        &self.rtcctl
    }
    #[doc = "0x04 - Real-Time Clock Interrupt Vector Register"]
    #[inline(always)]
    pub const fn rtciv(&self) -> &Rtciv {
        &self.rtciv
    }
    #[doc = "0x08 - RTC Counter Modulo Register"]
    #[inline(always)]
    pub const fn rtcmod(&self) -> &Rtcmod {
        &self.rtcmod
    }
    #[doc = "0x0c - RTC Counter Register"]
    #[inline(always)]
    pub const fn rtccnt(&self) -> &Rtccnt {
        &self.rtccnt
    }
}
#[doc = "RTCCTL (rw) register accessor: RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl`] module"]
#[doc(alias = "RTCCTL")]
pub type Rtcctl = crate::Reg<rtcctl::RtcctlSpec>;
#[doc = "RTCCTL0 Register"]
pub mod rtcctl;
#[doc = "RTCIV (rw) register accessor: Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtciv`] module"]
#[doc(alias = "RTCIV")]
pub type Rtciv = crate::Reg<rtciv::RtcivSpec>;
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCMOD (rw) register accessor: RTC Counter Modulo Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcmod`] module"]
#[doc(alias = "RTCMOD")]
pub type Rtcmod = crate::Reg<rtcmod::RtcmodSpec>;
#[doc = "RTC Counter Modulo Register"]
pub mod rtcmod;
#[doc = "RTCCNT (rw) register accessor: RTC Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt`] module"]
#[doc(alias = "RTCCNT")]
pub type Rtccnt = crate::Reg<rtccnt::RtccntSpec>;
#[doc = "RTC Counter Register"]
pub mod rtccnt;
