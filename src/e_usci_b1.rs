#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ucb1ctlw: [u8; 0x02],
    ucb1ctlw1: Ucb1ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb1: [u8; 0x02],
    _reserved_3_ucb1: [u8; 0x02],
    ucb1tbcnt: Ucb1tbcnt,
    _reserved_5_ucb1: [u8; 0x02],
    _reserved_6_ucb1: [u8; 0x02],
    _reserved7: [u8; 0x04],
    ucb1i2coa0: Ucb1i2coa0,
    ucb1i2coa1: Ucb1i2coa1,
    ucb1i2coa2: Ucb1i2coa2,
    ucb1i2coa3: Ucb1i2coa3,
    ucb1addrx: Ucb1addrx,
    ucb1addmask: Ucb1addmask,
    ucb1i2csa: Ucb1i2csa,
    _reserved14: [u8; 0x08],
    _reserved_14_ucb1: [u8; 0x02],
    _reserved_15_ucb1: [u8; 0x02],
    _reserved_16_ucb1: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb1ctlw0_spi(&self) -> &Ucb1ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb1ctlw0(&self) -> &Ucb1ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb1ctlw1(&self) -> &Ucb1ctlw1 {
        &self.ucb1ctlw1
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn ucb1brw_spi(&self) -> &Ucb1brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucb1brw(&self) -> &Ucb1brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - UCB1STATW_SPI"]
    #[inline(always)]
    pub const fn ucb1statw_spi(&self) -> &Ucb1statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub const fn ucb1statw(&self) -> &Ucb1statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb1tbcnt(&self) -> &Ucb1tbcnt {
        &self.ucb1tbcnt
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb1rxbuf_spi(&self) -> &Ucb1rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb1rxbuf(&self) -> &Ucb1rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb1txbuf_spi(&self) -> &Ucb1txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb1txbuf(&self) -> &Ucb1txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    #[inline(always)]
    pub const fn ucb1i2coa0(&self) -> &Ucb1i2coa0 {
        &self.ucb1i2coa0
    }
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn ucb1i2coa1(&self) -> &Ucb1i2coa1 {
        &self.ucb1i2coa1
    }
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn ucb1i2coa2(&self) -> &Ucb1i2coa2 {
        &self.ucb1i2coa2
    }
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn ucb1i2coa3(&self) -> &Ucb1i2coa3 {
        &self.ucb1i2coa3
    }
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    #[inline(always)]
    pub const fn ucb1addrx(&self) -> &Ucb1addrx {
        &self.ucb1addrx
    }
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    #[inline(always)]
    pub const fn ucb1addmask(&self) -> &Ucb1addmask {
        &self.ucb1addmask
    }
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ucb1i2csa(&self) -> &Ucb1i2csa {
        &self.ucb1i2csa
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb1ie_spi(&self) -> &Ucb1ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb1ie(&self) -> &Ucb1ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb1ifg_spi(&self) -> &Ucb1ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb1ifg(&self) -> &Ucb1ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb1iv_spi(&self) -> &Ucb1ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb1iv(&self) -> &Ucb1iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
}
#[doc = "UCB1CTLW0 (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctlw0`] module"]
#[doc(alias = "UCB1CTLW0")]
pub type Ucb1ctlw0 = crate::Reg<ucb1ctlw0::Ucb1ctlw0Spec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0;
#[doc = "UCB1CTLW0_SPI (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctlw0_spi`] module"]
#[doc(alias = "UCB1CTLW0_SPI")]
pub type Ucb1ctlw0Spi = crate::Reg<ucb1ctlw0_spi::Ucb1ctlw0SpiSpec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0_spi;
#[doc = "UCB1CTLW1 (rw) register accessor: eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ctlw1`] module"]
#[doc(alias = "UCB1CTLW1")]
pub type Ucb1ctlw1 = crate::Reg<ucb1ctlw1::Ucb1ctlw1Spec>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb1ctlw1;
#[doc = "UCB1BRW (rw) register accessor: eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1brw`] module"]
#[doc(alias = "UCB1BRW")]
pub type Ucb1brw = crate::Reg<ucb1brw::Ucb1brwSpec>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb1brw;
#[doc = "UCB1BRW_SPI (rw) register accessor: eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1brw_spi`] module"]
#[doc(alias = "UCB1BRW_SPI")]
pub type Ucb1brwSpi = crate::Reg<ucb1brw_spi::Ucb1brwSpiSpec>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb1brw_spi;
#[doc = "UCB1STATW (rw) register accessor: eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1statw`] module"]
#[doc(alias = "UCB1STATW")]
pub type Ucb1statw = crate::Reg<ucb1statw::Ucb1statwSpec>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb1statw;
#[doc = "UCB1STATW_SPI (rw) register accessor: UCB1STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1statw_spi`] module"]
#[doc(alias = "UCB1STATW_SPI")]
pub type Ucb1statwSpi = crate::Reg<ucb1statw_spi::Ucb1statwSpiSpec>;
#[doc = "UCB1STATW_SPI"]
pub mod ucb1statw_spi;
#[doc = "UCB1TBCNT (rw) register accessor: eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1tbcnt`] module"]
#[doc(alias = "UCB1TBCNT")]
pub type Ucb1tbcnt = crate::Reg<ucb1tbcnt::Ucb1tbcntSpec>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb1tbcnt;
#[doc = "UCB1RXBUF (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf`] module"]
#[doc(alias = "UCB1RXBUF")]
pub type Ucb1rxbuf = crate::Reg<ucb1rxbuf::Ucb1rxbufSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf;
#[doc = "UCB1RXBUF_SPI (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1rxbuf_spi`] module"]
#[doc(alias = "UCB1RXBUF_SPI")]
pub type Ucb1rxbufSpi = crate::Reg<ucb1rxbuf_spi::Ucb1rxbufSpiSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf_spi;
#[doc = "UCB1TXBUF (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf`] module"]
#[doc(alias = "UCB1TXBUF")]
pub type Ucb1txbuf = crate::Reg<ucb1txbuf::Ucb1txbufSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf;
#[doc = "UCB1TXBUF_SPI (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1txbuf_spi`] module"]
#[doc(alias = "UCB1TXBUF_SPI")]
pub type Ucb1txbufSpi = crate::Reg<ucb1txbuf_spi::Ucb1txbufSpiSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf_spi;
#[doc = "UCB1I2COA0 (rw) register accessor: eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa0`] module"]
#[doc(alias = "UCB1I2COA0")]
pub type Ucb1i2coa0 = crate::Reg<ucb1i2coa0::Ucb1i2coa0Spec>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb1i2coa0;
#[doc = "UCB1I2COA1 (rw) register accessor: eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa1`] module"]
#[doc(alias = "UCB1I2COA1")]
pub type Ucb1i2coa1 = crate::Reg<ucb1i2coa1::Ucb1i2coa1Spec>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb1i2coa1;
#[doc = "UCB1I2COA2 (rw) register accessor: eUSCI_Bx I2C Own Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa2`] module"]
#[doc(alias = "UCB1I2COA2")]
pub type Ucb1i2coa2 = crate::Reg<ucb1i2coa2::Ucb1i2coa2Spec>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb1i2coa2;
#[doc = "UCB1I2COA3 (rw) register accessor: eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2coa3`] module"]
#[doc(alias = "UCB1I2COA3")]
pub type Ucb1i2coa3 = crate::Reg<ucb1i2coa3::Ucb1i2coa3Spec>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb1i2coa3;
#[doc = "UCB1ADDRX (rw) register accessor: eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1addrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1addrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1addrx`] module"]
#[doc(alias = "UCB1ADDRX")]
pub type Ucb1addrx = crate::Reg<ucb1addrx::Ucb1addrxSpec>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb1addrx;
#[doc = "UCB1ADDMASK (rw) register accessor: eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1addmask`] module"]
#[doc(alias = "UCB1ADDMASK")]
pub type Ucb1addmask = crate::Reg<ucb1addmask::Ucb1addmaskSpec>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb1addmask;
#[doc = "UCB1I2CSA (rw) register accessor: eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1i2csa`] module"]
#[doc(alias = "UCB1I2CSA")]
pub type Ucb1i2csa = crate::Reg<ucb1i2csa::Ucb1i2csaSpec>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb1i2csa;
#[doc = "UCB1IE (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ie`] module"]
#[doc(alias = "UCB1IE")]
pub type Ucb1ie = crate::Reg<ucb1ie::Ucb1ieSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "UCB1IE_SPI (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ie_spi`] module"]
#[doc(alias = "UCB1IE_SPI")]
pub type Ucb1ieSpi = crate::Reg<ucb1ie_spi::Ucb1ieSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "UCB1IFG (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ifg`] module"]
#[doc(alias = "UCB1IFG")]
pub type Ucb1ifg = crate::Reg<ucb1ifg::Ucb1ifgSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg;
#[doc = "UCB1IFG_SPI (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1ifg_spi`] module"]
#[doc(alias = "UCB1IFG_SPI")]
pub type Ucb1ifgSpi = crate::Reg<ucb1ifg_spi::Ucb1ifgSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg_spi;
#[doc = "UCB1IV (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1iv`] module"]
#[doc(alias = "UCB1IV")]
pub type Ucb1iv = crate::Reg<ucb1iv::Ucb1ivSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv;
#[doc = "UCB1IV_SPI (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb1iv_spi`] module"]
#[doc(alias = "UCB1IV_SPI")]
pub type Ucb1ivSpi = crate::Reg<ucb1iv_spi::Ucb1ivSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv_spi;
