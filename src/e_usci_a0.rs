#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca0ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca0ctlw1: UCA0CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_uca0: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca0mctlw: UCA0MCTLW,
    _reserved_4_uca0: [u8; 2usize],
    _reserved_5_uca0: [u8; 2usize],
    _reserved_6_uca0: [u8; 2usize],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca0abctl: UCA0ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca0irctl: UCA0IRCTL,
    _reserved9: [u8; 6usize],
    _reserved_9_uca0: [u8; 2usize],
    _reserved_10_uca0: [u8; 2usize],
    _reserved_11_uca0: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0_spi(&self) -> &UCA0CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0_spi_mut(&self) -> &mut UCA0CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0(&self) -> &UCA0CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA0CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0_mut(&self) -> &mut UCA0CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA0CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca0brw_spi(&self) -> &UCA0BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca0brw_spi_mut(&self) -> &mut UCA0BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca0brw(&self) -> &UCA0BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA0BRW) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca0brw_mut(&self) -> &mut UCA0BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA0BRW) }
    }
    #[doc = "0x0a - UCA0STATW_SPI"]
    #[inline(always)]
    pub fn uca0statw_spi(&self) -> &UCA0STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA0STATW_SPI) }
    }
    #[doc = "0x0a - UCA0STATW_SPI"]
    #[inline(always)]
    pub fn uca0statw_spi_mut(&self) -> &mut UCA0STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA0STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca0statw(&self) -> &UCA0STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA0STATW) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca0statw_mut(&self) -> &mut UCA0STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA0STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf_spi(&self) -> &UCA0RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf_spi_mut(&self) -> &mut UCA0RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> &UCA0RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA0RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf_mut(&self) -> &mut UCA0RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA0RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf_spi(&self) -> &UCA0TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf_spi_mut(&self) -> &mut UCA0TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> &UCA0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA0TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf_mut(&self) -> &mut UCA0TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA0TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie_spi(&self) -> &UCA0IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA0IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie_spi_mut(&self) -> &mut UCA0IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA0IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie(&self) -> &UCA0IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA0IE) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie_mut(&self) -> &mut UCA0IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA0IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg_spi(&self) -> &UCA0IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA0IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg_spi_mut(&self) -> &mut UCA0IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA0IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg(&self) -> &UCA0IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA0IFG) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg_mut(&self) -> &mut UCA0IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA0IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv_spi(&self) -> &UCA0IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA0IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv_spi_mut(&self) -> &mut UCA0IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA0IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv(&self) -> &UCA0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA0IV) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv_mut(&self) -> &mut UCA0IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA0IV) }
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ctlw0](uca0ctlw0) module"]
pub type UCA0CTLW0 = crate::Reg<u16, _UCA0CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTLW0;
#[doc = "`read()` method returns [uca0ctlw0::R](uca0ctlw0::R) reader structure"]
impl crate::Readable for UCA0CTLW0 {}
#[doc = "`write(|w| ..)` method takes [uca0ctlw0::W](uca0ctlw0::W) writer structure"]
impl crate::Writable for UCA0CTLW0 {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0;
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ctlw0_spi](uca0ctlw0_spi) module"]
pub type UCA0CTLW0_SPI = crate::Reg<u16, _UCA0CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTLW0_SPI;
#[doc = "`read()` method returns [uca0ctlw0_spi::R](uca0ctlw0_spi::R) reader structure"]
impl crate::Readable for UCA0CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ctlw0_spi::W](uca0ctlw0_spi::W) writer structure"]
impl crate::Writable for UCA0CTLW0_SPI {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0_spi;
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ctlw1](uca0ctlw1) module"]
pub type UCA0CTLW1 = crate::Reg<u16, _UCA0CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTLW1;
#[doc = "`read()` method returns [uca0ctlw1::R](uca0ctlw1::R) reader structure"]
impl crate::Readable for UCA0CTLW1 {}
#[doc = "`write(|w| ..)` method takes [uca0ctlw1::W](uca0ctlw1::W) writer structure"]
impl crate::Writable for UCA0CTLW1 {}
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0brw](uca0brw) module"]
pub type UCA0BRW = crate::Reg<u16, _UCA0BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BRW;
#[doc = "`read()` method returns [uca0brw::R](uca0brw::R) reader structure"]
impl crate::Readable for UCA0BRW {}
#[doc = "`write(|w| ..)` method takes [uca0brw::W](uca0brw::W) writer structure"]
impl crate::Writable for UCA0BRW {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca0brw;
#[doc = "eUSCI_Ax Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0brw_spi](uca0brw_spi) module"]
pub type UCA0BRW_SPI = crate::Reg<u16, _UCA0BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BRW_SPI;
#[doc = "`read()` method returns [uca0brw_spi::R](uca0brw_spi::R) reader structure"]
impl crate::Readable for UCA0BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0brw_spi::W](uca0brw_spi::W) writer structure"]
impl crate::Writable for UCA0BRW_SPI {}
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca0brw_spi;
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0mctlw](uca0mctlw) module"]
pub type UCA0MCTLW = crate::Reg<u16, _UCA0MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0MCTLW;
#[doc = "`read()` method returns [uca0mctlw::R](uca0mctlw::R) reader structure"]
impl crate::Readable for UCA0MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca0mctlw::W](uca0mctlw::W) writer structure"]
impl crate::Writable for UCA0MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca0mctlw;
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0statw](uca0statw) module"]
pub type UCA0STATW = crate::Reg<u16, _UCA0STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0STATW;
#[doc = "`read()` method returns [uca0statw::R](uca0statw::R) reader structure"]
impl crate::Readable for UCA0STATW {}
#[doc = "`write(|w| ..)` method takes [uca0statw::W](uca0statw::W) writer structure"]
impl crate::Writable for UCA0STATW {}
#[doc = "eUSCI_Ax Status Register"]
pub mod uca0statw;
#[doc = "UCA0STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0statw_spi](uca0statw_spi) module"]
pub type UCA0STATW_SPI = crate::Reg<u16, _UCA0STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0STATW_SPI;
#[doc = "`read()` method returns [uca0statw_spi::R](uca0statw_spi::R) reader structure"]
impl crate::Readable for UCA0STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0statw_spi::W](uca0statw_spi::W) writer structure"]
impl crate::Writable for UCA0STATW_SPI {}
#[doc = "UCA0STATW_SPI"]
pub mod uca0statw_spi;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0rxbuf](uca0rxbuf) module"]
pub type UCA0RXBUF = crate::Reg<u16, _UCA0RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0RXBUF;
#[doc = "`read()` method returns [uca0rxbuf::R](uca0rxbuf::R) reader structure"]
impl crate::Readable for UCA0RXBUF {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0rxbuf_spi](uca0rxbuf_spi) module"]
pub type UCA0RXBUF_SPI = crate::Reg<u16, _UCA0RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0RXBUF_SPI;
#[doc = "`read()` method returns [uca0rxbuf_spi::R](uca0rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA0RXBUF_SPI {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf_spi;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0txbuf](uca0txbuf) module"]
pub type UCA0TXBUF = crate::Reg<u16, _UCA0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0TXBUF;
#[doc = "`read()` method returns [uca0txbuf::R](uca0txbuf::R) reader structure"]
impl crate::Readable for UCA0TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca0txbuf::W](uca0txbuf::W) writer structure"]
impl crate::Writable for UCA0TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0txbuf_spi](uca0txbuf_spi) module"]
pub type UCA0TXBUF_SPI = crate::Reg<u16, _UCA0TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0TXBUF_SPI;
#[doc = "`read()` method returns [uca0txbuf_spi::R](uca0txbuf_spi::R) reader structure"]
impl crate::Readable for UCA0TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0txbuf_spi::W](uca0txbuf_spi::W) writer structure"]
impl crate::Writable for UCA0TXBUF_SPI {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf_spi;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0abctl](uca0abctl) module"]
pub type UCA0ABCTL = crate::Reg<u16, _UCA0ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0ABCTL;
#[doc = "`read()` method returns [uca0abctl::R](uca0abctl::R) reader structure"]
impl crate::Readable for UCA0ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca0abctl::W](uca0abctl::W) writer structure"]
impl crate::Writable for UCA0ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca0abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0irctl](uca0irctl) module"]
pub type UCA0IRCTL = crate::Reg<u16, _UCA0IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IRCTL;
#[doc = "`read()` method returns [uca0irctl::R](uca0irctl::R) reader structure"]
impl crate::Readable for UCA0IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca0irctl::W](uca0irctl::W) writer structure"]
impl crate::Writable for UCA0IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca0irctl;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ie](uca0ie) module"]
pub type UCA0IE = crate::Reg<u16, _UCA0IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IE;
#[doc = "`read()` method returns [uca0ie::R](uca0ie::R) reader structure"]
impl crate::Readable for UCA0IE {}
#[doc = "`write(|w| ..)` method takes [uca0ie::W](uca0ie::W) writer structure"]
impl crate::Writable for UCA0IE {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ie_spi](uca0ie_spi) module"]
pub type UCA0IE_SPI = crate::Reg<u16, _UCA0IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IE_SPI;
#[doc = "`read()` method returns [uca0ie_spi::R](uca0ie_spi::R) reader structure"]
impl crate::Readable for UCA0IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ie_spi::W](uca0ie_spi::W) writer structure"]
impl crate::Writable for UCA0IE_SPI {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ifg](uca0ifg) module"]
pub type UCA0IFG = crate::Reg<u16, _UCA0IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IFG;
#[doc = "`read()` method returns [uca0ifg::R](uca0ifg::R) reader structure"]
impl crate::Readable for UCA0IFG {}
#[doc = "`write(|w| ..)` method takes [uca0ifg::W](uca0ifg::W) writer structure"]
impl crate::Writable for UCA0IFG {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ifg_spi](uca0ifg_spi) module"]
pub type UCA0IFG_SPI = crate::Reg<u16, _UCA0IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IFG_SPI;
#[doc = "`read()` method returns [uca0ifg_spi::R](uca0ifg_spi::R) reader structure"]
impl crate::Readable for UCA0IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca0ifg_spi::W](uca0ifg_spi::W) writer structure"]
impl crate::Writable for UCA0IFG_SPI {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg_spi;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0iv](uca0iv) module"]
pub type UCA0IV = crate::Reg<u16, _UCA0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IV;
#[doc = "`read()` method returns [uca0iv::R](uca0iv::R) reader structure"]
impl crate::Readable for UCA0IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0iv_spi](uca0iv_spi) module"]
pub type UCA0IV_SPI = crate::Reg<u16, _UCA0IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IV_SPI;
#[doc = "`read()` method returns [uca0iv_spi::R](uca0iv_spi::R) reader structure"]
impl crate::Readable for UCA0IV_SPI {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv_spi;
