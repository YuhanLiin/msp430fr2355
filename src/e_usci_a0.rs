#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uca0ctlw: [u8; 0x02],
    uca0ctlw1: Uca0ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca0: [u8; 0x02],
    uca0mctlw: Uca0mctlw,
    _reserved_4_uca0: [u8; 0x02],
    _reserved_5_uca0: [u8; 0x02],
    _reserved_6_uca0: [u8; 0x02],
    uca0abctl: Uca0abctl,
    uca0irctl: Uca0irctl,
    _reserved9: [u8; 0x06],
    _reserved_9_uca0: [u8; 0x02],
    _reserved_10_uca0: [u8; 0x02],
    _reserved_11_uca0: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca0ctlw0_spi(&self) -> &Uca0ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn uca0ctlw0(&self) -> &Uca0ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    #[inline(always)]
    pub const fn uca0ctlw1(&self) -> &Uca0ctlw1 {
        &self.uca0ctlw1
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn uca0brw_spi(&self) -> &Uca0brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn uca0brw(&self) -> &Uca0brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    #[inline(always)]
    pub const fn uca0mctlw(&self) -> &Uca0mctlw {
        &self.uca0mctlw
    }
    #[doc = "0x0a - UCA0STATW_SPI"]
    #[inline(always)]
    pub const fn uca0statw_spi(&self) -> &Uca0statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub const fn uca0statw(&self) -> &Uca0statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca0rxbuf_spi(&self) -> &Uca0rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn uca0rxbuf(&self) -> &Uca0rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca0txbuf_spi(&self) -> &Uca0txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uca0txbuf(&self) -> &Uca0txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    #[inline(always)]
    pub const fn uca0abctl(&self) -> &Uca0abctl {
        &self.uca0abctl
    }
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn uca0irctl(&self) -> &Uca0irctl {
        &self.uca0irctl
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca0ie_spi(&self) -> &Uca0ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca0ie(&self) -> &Uca0ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca0ifg_spi(&self) -> &Uca0ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn uca0ifg(&self) -> &Uca0ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv_spi(&self) -> &Uca0ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv(&self) -> &Uca0iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
}
#[doc = "UCA0CTLW0 (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctlw0`] module"]
#[doc(alias = "UCA0CTLW0")]
pub type Uca0ctlw0 = crate::Reg<uca0ctlw0::Uca0ctlw0Spec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0;
#[doc = "UCA0CTLW0_SPI (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctlw0_spi`] module"]
#[doc(alias = "UCA0CTLW0_SPI")]
pub type Uca0ctlw0Spi = crate::Reg<uca0ctlw0_spi::Uca0ctlw0SpiSpec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0_spi;
#[doc = "UCA0CTLW1 (rw) register accessor: eUSCI_Ax Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctlw1`] module"]
#[doc(alias = "UCA0CTLW1")]
pub type Uca0ctlw1 = crate::Reg<uca0ctlw1::Uca0ctlw1Spec>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "UCA0BRW (rw) register accessor: eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0brw`] module"]
#[doc(alias = "UCA0BRW")]
pub type Uca0brw = crate::Reg<uca0brw::Uca0brwSpec>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca0brw;
#[doc = "UCA0BRW_SPI (rw) register accessor: eUSCI_Ax Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0brw_spi`] module"]
#[doc(alias = "UCA0BRW_SPI")]
pub type Uca0brwSpi = crate::Reg<uca0brw_spi::Uca0brwSpiSpec>;
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca0brw_spi;
#[doc = "UCA0MCTLW (rw) register accessor: eUSCI_Ax Modulation Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctlw`] module"]
#[doc(alias = "UCA0MCTLW")]
pub type Uca0mctlw = crate::Reg<uca0mctlw::Uca0mctlwSpec>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca0mctlw;
#[doc = "UCA0STATW (rw) register accessor: eUSCI_Ax Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0statw`] module"]
#[doc(alias = "UCA0STATW")]
pub type Uca0statw = crate::Reg<uca0statw::Uca0statwSpec>;
#[doc = "eUSCI_Ax Status Register"]
pub mod uca0statw;
#[doc = "UCA0STATW_SPI (rw) register accessor: UCA0STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0statw_spi`] module"]
#[doc(alias = "UCA0STATW_SPI")]
pub type Uca0statwSpi = crate::Reg<uca0statw_spi::Uca0statwSpiSpec>;
#[doc = "UCA0STATW_SPI"]
pub mod uca0statw_spi;
#[doc = "UCA0RXBUF (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf`] module"]
#[doc(alias = "UCA0RXBUF")]
pub type Uca0rxbuf = crate::Reg<uca0rxbuf::Uca0rxbufSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf;
#[doc = "UCA0RXBUF_SPI (rw) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf_spi`] module"]
#[doc(alias = "UCA0RXBUF_SPI")]
pub type Uca0rxbufSpi = crate::Reg<uca0rxbuf_spi::Uca0rxbufSpiSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf`] module"]
#[doc(alias = "UCA0TXBUF")]
pub type Uca0txbuf = crate::Reg<uca0txbuf::Uca0txbufSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf;
#[doc = "UCA0TXBUF_SPI (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf_spi`] module"]
#[doc(alias = "UCA0TXBUF_SPI")]
pub type Uca0txbufSpi = crate::Reg<uca0txbuf_spi::Uca0txbufSpiSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf_spi;
#[doc = "UCA0ABCTL (rw) register accessor: eUSCI_Ax Auto Baud Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0abctl`] module"]
#[doc(alias = "UCA0ABCTL")]
pub type Uca0abctl = crate::Reg<uca0abctl::Uca0abctlSpec>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca0abctl;
#[doc = "UCA0IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0irctl`] module"]
#[doc(alias = "UCA0IRCTL")]
pub type Uca0irctl = crate::Reg<uca0irctl::Uca0irctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca0irctl;
#[doc = "UCA0IE (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ie`] module"]
#[doc(alias = "UCA0IE")]
pub type Uca0ie = crate::Reg<uca0ie::Uca0ieSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie;
#[doc = "UCA0IE_SPI (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ie_spi`] module"]
#[doc(alias = "UCA0IE_SPI")]
pub type Uca0ieSpi = crate::Reg<uca0ie_spi::Uca0ieSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "UCA0IFG (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ifg`] module"]
#[doc(alias = "UCA0IFG")]
pub type Uca0ifg = crate::Reg<uca0ifg::Uca0ifgSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg;
#[doc = "UCA0IFG_SPI (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ifg_spi`] module"]
#[doc(alias = "UCA0IFG_SPI")]
pub type Uca0ifgSpi = crate::Reg<uca0ifg_spi::Uca0ifgSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg_spi;
#[doc = "UCA0IV (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv`] module"]
#[doc(alias = "UCA0IV")]
pub type Uca0iv = crate::Reg<uca0iv::Uca0ivSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv;
#[doc = "UCA0IV_SPI (rw) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv_spi`] module"]
#[doc(alias = "UCA0IV_SPI")]
pub type Uca0ivSpi = crate::Reg<uca0iv_spi::Uca0ivSpiSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv_spi;
