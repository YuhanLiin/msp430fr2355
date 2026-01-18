#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ucb0ctlw: [u8; 0x02],
    ucb0ctlw1: Ucb0ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb0: [u8; 0x02],
    _reserved_3_ucb0: [u8; 0x02],
    ucb0tbcnt: Ucb0tbcnt,
    _reserved_5_ucb0: [u8; 0x02],
    _reserved_6_ucb0: [u8; 0x02],
    _reserved7: [u8; 0x04],
    ucb0i2coa0: Ucb0i2coa0,
    ucb0i2coa1: Ucb0i2coa1,
    ucb0i2coa2: Ucb0i2coa2,
    ucb0i2coa3: Ucb0i2coa3,
    ucb0addrx: Ucb0addrx,
    ucb0addmask: Ucb0addmask,
    ucb0i2csa: Ucb0i2csa,
    _reserved14: [u8; 0x08],
    _reserved_14_ucb0: [u8; 0x02],
    _reserved_15_ucb0: [u8; 0x02],
    _reserved_16_ucb0: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb0ctlw0_spi(&self) -> &Ucb0ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb0ctlw0(&self) -> &Ucb0ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb0ctlw1(&self) -> &Ucb0ctlw1 {
        &self.ucb0ctlw1
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn ucb0brw_spi(&self) -> &Ucb0brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucb0brw(&self) -> &Ucb0brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - UCB0STATW_SPI"]
    #[inline(always)]
    pub const fn ucb0statw_spi(&self) -> &Ucb0statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub const fn ucb0statw(&self) -> &Ucb0statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb0tbcnt(&self) -> &Ucb0tbcnt {
        &self.ucb0tbcnt
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb0rxbuf_spi(&self) -> &Ucb0rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &Ucb0rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb0txbuf_spi(&self) -> &Ucb0txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &Ucb0txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    #[inline(always)]
    pub const fn ucb0i2coa0(&self) -> &Ucb0i2coa0 {
        &self.ucb0i2coa0
    }
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn ucb0i2coa1(&self) -> &Ucb0i2coa1 {
        &self.ucb0i2coa1
    }
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn ucb0i2coa2(&self) -> &Ucb0i2coa2 {
        &self.ucb0i2coa2
    }
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn ucb0i2coa3(&self) -> &Ucb0i2coa3 {
        &self.ucb0i2coa3
    }
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    #[inline(always)]
    pub const fn ucb0addrx(&self) -> &Ucb0addrx {
        &self.ucb0addrx
    }
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    #[inline(always)]
    pub const fn ucb0addmask(&self) -> &Ucb0addmask {
        &self.ucb0addmask
    }
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ucb0i2csa(&self) -> &Ucb0i2csa {
        &self.ucb0i2csa
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie_spi(&self) -> &Ucb0ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie(&self) -> &Ucb0ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb0ifg_spi(&self) -> &Ucb0ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb0ifg(&self) -> &Ucb0ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv_spi(&self) -> &Ucb0ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv(&self) -> &Ucb0iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
}
#[doc = "UCB0CTLW0 (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctlw0`] module"]
#[doc(alias = "UCB0CTLW0")]
pub type Ucb0ctlw0 = crate::Reg<ucb0ctlw0::Ucb0ctlw0Spec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0;
#[doc = "UCB0CTLW0_SPI (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctlw0_spi`] module"]
#[doc(alias = "UCB0CTLW0_SPI")]
pub type Ucb0ctlw0Spi = crate::Reg<ucb0ctlw0_spi::Ucb0ctlw0SpiSpec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0_spi;
#[doc = "UCB0CTLW1 (rw) register accessor: eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctlw1`] module"]
#[doc(alias = "UCB0CTLW1")]
pub type Ucb0ctlw1 = crate::Reg<ucb0ctlw1::Ucb0ctlw1Spec>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "UCB0BRW (rw) register accessor: eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0brw`] module"]
#[doc(alias = "UCB0BRW")]
pub type Ucb0brw = crate::Reg<ucb0brw::Ucb0brwSpec>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb0brw;
#[doc = "UCB0BRW_SPI (rw) register accessor: eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0brw_spi`] module"]
#[doc(alias = "UCB0BRW_SPI")]
pub type Ucb0brwSpi = crate::Reg<ucb0brw_spi::Ucb0brwSpiSpec>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb0brw_spi;
#[doc = "UCB0STATW (rw) register accessor: eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0statw`] module"]
#[doc(alias = "UCB0STATW")]
pub type Ucb0statw = crate::Reg<ucb0statw::Ucb0statwSpec>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb0statw;
#[doc = "UCB0STATW_SPI (rw) register accessor: UCB0STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0statw_spi`] module"]
#[doc(alias = "UCB0STATW_SPI")]
pub type Ucb0statwSpi = crate::Reg<ucb0statw_spi::Ucb0statwSpiSpec>;
#[doc = "UCB0STATW_SPI"]
pub mod ucb0statw_spi;
#[doc = "UCB0TBCNT (rw) register accessor: eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0tbcnt`] module"]
#[doc(alias = "UCB0TBCNT")]
pub type Ucb0tbcnt = crate::Reg<ucb0tbcnt::Ucb0tbcntSpec>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "UCB0RXBUF (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`] module"]
#[doc(alias = "UCB0RXBUF")]
pub type Ucb0rxbuf = crate::Reg<ucb0rxbuf::Ucb0rxbufSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf;
#[doc = "UCB0RXBUF_SPI (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf_spi`] module"]
#[doc(alias = "UCB0RXBUF_SPI")]
pub type Ucb0rxbufSpi = crate::Reg<ucb0rxbuf_spi::Ucb0rxbufSpiSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`] module"]
#[doc(alias = "UCB0TXBUF")]
pub type Ucb0txbuf = crate::Reg<ucb0txbuf::Ucb0txbufSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf;
#[doc = "UCB0TXBUF_SPI (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf_spi`] module"]
#[doc(alias = "UCB0TXBUF_SPI")]
pub type Ucb0txbufSpi = crate::Reg<ucb0txbuf_spi::Ucb0txbufSpiSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf_spi;
#[doc = "UCB0I2COA0 (rw) register accessor: eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa0`] module"]
#[doc(alias = "UCB0I2COA0")]
pub type Ucb0i2coa0 = crate::Reg<ucb0i2coa0::Ucb0i2coa0Spec>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb0i2coa0;
#[doc = "UCB0I2COA1 (rw) register accessor: eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa1`] module"]
#[doc(alias = "UCB0I2COA1")]
pub type Ucb0i2coa1 = crate::Reg<ucb0i2coa1::Ucb0i2coa1Spec>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb0i2coa1;
#[doc = "UCB0I2COA2 (rw) register accessor: eUSCI_Bx I2C Own Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa2`] module"]
#[doc(alias = "UCB0I2COA2")]
pub type Ucb0i2coa2 = crate::Reg<ucb0i2coa2::Ucb0i2coa2Spec>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb0i2coa2;
#[doc = "UCB0I2COA3 (rw) register accessor: eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa3`] module"]
#[doc(alias = "UCB0I2COA3")]
pub type Ucb0i2coa3 = crate::Reg<ucb0i2coa3::Ucb0i2coa3Spec>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb0i2coa3;
#[doc = "UCB0ADDRX (rw) register accessor: eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addrx`] module"]
#[doc(alias = "UCB0ADDRX")]
pub type Ucb0addrx = crate::Reg<ucb0addrx::Ucb0addrxSpec>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb0addrx;
#[doc = "UCB0ADDMASK (rw) register accessor: eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addmask`] module"]
#[doc(alias = "UCB0ADDMASK")]
pub type Ucb0addmask = crate::Reg<ucb0addmask::Ucb0addmaskSpec>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb0addmask;
#[doc = "UCB0I2CSA (rw) register accessor: eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2csa`] module"]
#[doc(alias = "UCB0I2CSA")]
pub type Ucb0i2csa = crate::Reg<ucb0i2csa::Ucb0i2csaSpec>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb0i2csa;
#[doc = "UCB0IE (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie`] module"]
#[doc(alias = "UCB0IE")]
pub type Ucb0ie = crate::Reg<ucb0ie::Ucb0ieSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IE_SPI (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie_spi`] module"]
#[doc(alias = "UCB0IE_SPI")]
pub type Ucb0ieSpi = crate::Reg<ucb0ie_spi::Ucb0ieSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "UCB0IFG (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg`] module"]
#[doc(alias = "UCB0IFG")]
pub type Ucb0ifg = crate::Reg<ucb0ifg::Ucb0ifgSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg;
#[doc = "UCB0IFG_SPI (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg_spi`] module"]
#[doc(alias = "UCB0IFG_SPI")]
pub type Ucb0ifgSpi = crate::Reg<ucb0ifg_spi::Ucb0ifgSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg_spi;
#[doc = "UCB0IV (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv`] module"]
#[doc(alias = "UCB0IV")]
pub type Ucb0iv = crate::Reg<ucb0iv::Ucb0ivSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv;
#[doc = "UCB0IV_SPI (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv_spi`] module"]
#[doc(alias = "UCB0IV_SPI")]
pub type Ucb0ivSpi = crate::Reg<ucb0iv_spi::Ucb0ivSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv_spi;
