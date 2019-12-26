#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca1ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca1ctlw1: UCA1CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_uca1: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca1mctlw: UCA1MCTLW,
    _reserved_4_uca1: [u8; 2usize],
    _reserved_5_uca1: [u8; 2usize],
    _reserved_6_uca1: [u8; 2usize],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca1abctl: UCA1ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca1irctl: UCA1IRCTL,
    _reserved9: [u8; 6usize],
    _reserved_9_uca1: [u8; 2usize],
    _reserved_10_uca1: [u8; 2usize],
    _reserved_11_uca1: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0_spi(&self) -> &UCA1CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0_spi_mut(&self) -> &mut UCA1CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0(&self) -> &UCA1CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA1CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0_mut(&self) -> &mut UCA1CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA1CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca1brw_spi(&self) -> &UCA1BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca1brw_spi_mut(&self) -> &mut UCA1BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca1brw(&self) -> &UCA1BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA1BRW) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca1brw_mut(&self) -> &mut UCA1BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA1BRW) }
    }
    #[doc = "0x0a - UCA1STATW_SPI"]
    #[inline(always)]
    pub fn uca1statw_spi(&self) -> &UCA1STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA1STATW_SPI) }
    }
    #[doc = "0x0a - UCA1STATW_SPI"]
    #[inline(always)]
    pub fn uca1statw_spi_mut(&self) -> &mut UCA1STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA1STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca1statw(&self) -> &UCA1STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA1STATW) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca1statw_mut(&self) -> &mut UCA1STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA1STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf_spi(&self) -> &UCA1RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf_spi_mut(&self) -> &mut UCA1RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf(&self) -> &UCA1RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA1RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf_mut(&self) -> &mut UCA1RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA1RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf_spi(&self) -> &UCA1TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf_spi_mut(&self) -> &mut UCA1TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf(&self) -> &UCA1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA1TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf_mut(&self) -> &mut UCA1TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA1TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie_spi(&self) -> &UCA1IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA1IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie_spi_mut(&self) -> &mut UCA1IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA1IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie(&self) -> &UCA1IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA1IE) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie_mut(&self) -> &mut UCA1IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA1IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg_spi(&self) -> &UCA1IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA1IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg_spi_mut(&self) -> &mut UCA1IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA1IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg(&self) -> &UCA1IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA1IFG) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg_mut(&self) -> &mut UCA1IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA1IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv_spi(&self) -> &UCA1IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA1IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv_spi_mut(&self) -> &mut UCA1IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA1IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv(&self) -> &UCA1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA1IV) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv_mut(&self) -> &mut UCA1IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA1IV) }
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctlw0](uca1ctlw0) module"]
pub type UCA1CTLW0 = crate::Reg<u16, _UCA1CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTLW0;
#[doc = "`read()` method returns [uca1ctlw0::R](uca1ctlw0::R) reader structure"]
impl crate::Readable for UCA1CTLW0 {}
#[doc = "`write(|w| ..)` method takes [uca1ctlw0::W](uca1ctlw0::W) writer structure"]
impl crate::Writable for UCA1CTLW0 {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca1ctlw0;
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctlw0_spi](uca1ctlw0_spi) module"]
pub type UCA1CTLW0_SPI = crate::Reg<u16, _UCA1CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTLW0_SPI;
#[doc = "`read()` method returns [uca1ctlw0_spi::R](uca1ctlw0_spi::R) reader structure"]
impl crate::Readable for UCA1CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ctlw0_spi::W](uca1ctlw0_spi::W) writer structure"]
impl crate::Writable for UCA1CTLW0_SPI {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca1ctlw0_spi;
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctlw1](uca1ctlw1) module"]
pub type UCA1CTLW1 = crate::Reg<u16, _UCA1CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTLW1;
#[doc = "`read()` method returns [uca1ctlw1::R](uca1ctlw1::R) reader structure"]
impl crate::Readable for UCA1CTLW1 {}
#[doc = "`write(|w| ..)` method takes [uca1ctlw1::W](uca1ctlw1::W) writer structure"]
impl crate::Writable for UCA1CTLW1 {}
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1brw](uca1brw) module"]
pub type UCA1BRW = crate::Reg<u16, _UCA1BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BRW;
#[doc = "`read()` method returns [uca1brw::R](uca1brw::R) reader structure"]
impl crate::Readable for UCA1BRW {}
#[doc = "`write(|w| ..)` method takes [uca1brw::W](uca1brw::W) writer structure"]
impl crate::Writable for UCA1BRW {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca1brw;
#[doc = "eUSCI_Ax Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1brw_spi](uca1brw_spi) module"]
pub type UCA1BRW_SPI = crate::Reg<u16, _UCA1BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BRW_SPI;
#[doc = "`read()` method returns [uca1brw_spi::R](uca1brw_spi::R) reader structure"]
impl crate::Readable for UCA1BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1brw_spi::W](uca1brw_spi::W) writer structure"]
impl crate::Writable for UCA1BRW_SPI {}
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca1brw_spi;
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1mctlw](uca1mctlw) module"]
pub type UCA1MCTLW = crate::Reg<u16, _UCA1MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1MCTLW;
#[doc = "`read()` method returns [uca1mctlw::R](uca1mctlw::R) reader structure"]
impl crate::Readable for UCA1MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca1mctlw::W](uca1mctlw::W) writer structure"]
impl crate::Writable for UCA1MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca1mctlw;
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1statw](uca1statw) module"]
pub type UCA1STATW = crate::Reg<u16, _UCA1STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1STATW;
#[doc = "`read()` method returns [uca1statw::R](uca1statw::R) reader structure"]
impl crate::Readable for UCA1STATW {}
#[doc = "`write(|w| ..)` method takes [uca1statw::W](uca1statw::W) writer structure"]
impl crate::Writable for UCA1STATW {}
#[doc = "eUSCI_Ax Status Register"]
pub mod uca1statw;
#[doc = "UCA1STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1statw_spi](uca1statw_spi) module"]
pub type UCA1STATW_SPI = crate::Reg<u16, _UCA1STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1STATW_SPI;
#[doc = "`read()` method returns [uca1statw_spi::R](uca1statw_spi::R) reader structure"]
impl crate::Readable for UCA1STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1statw_spi::W](uca1statw_spi::W) writer structure"]
impl crate::Writable for UCA1STATW_SPI {}
#[doc = "UCA1STATW_SPI"]
pub mod uca1statw_spi;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1rxbuf](uca1rxbuf) module"]
pub type UCA1RXBUF = crate::Reg<u16, _UCA1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1RXBUF;
#[doc = "`read()` method returns [uca1rxbuf::R](uca1rxbuf::R) reader structure"]
impl crate::Readable for UCA1RXBUF {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca1rxbuf;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1rxbuf_spi](uca1rxbuf_spi) module"]
pub type UCA1RXBUF_SPI = crate::Reg<u16, _UCA1RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1RXBUF_SPI;
#[doc = "`read()` method returns [uca1rxbuf_spi::R](uca1rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA1RXBUF_SPI {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca1rxbuf_spi;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1txbuf](uca1txbuf) module"]
pub type UCA1TXBUF = crate::Reg<u16, _UCA1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF;
#[doc = "`read()` method returns [uca1txbuf::R](uca1txbuf::R) reader structure"]
impl crate::Readable for UCA1TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf::W](uca1txbuf::W) writer structure"]
impl crate::Writable for UCA1TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca1txbuf;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1txbuf_spi](uca1txbuf_spi) module"]
pub type UCA1TXBUF_SPI = crate::Reg<u16, _UCA1TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF_SPI;
#[doc = "`read()` method returns [uca1txbuf_spi::R](uca1txbuf_spi::R) reader structure"]
impl crate::Readable for UCA1TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf_spi::W](uca1txbuf_spi::W) writer structure"]
impl crate::Writable for UCA1TXBUF_SPI {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca1txbuf_spi;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1abctl](uca1abctl) module"]
pub type UCA1ABCTL = crate::Reg<u16, _UCA1ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1ABCTL;
#[doc = "`read()` method returns [uca1abctl::R](uca1abctl::R) reader structure"]
impl crate::Readable for UCA1ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca1abctl::W](uca1abctl::W) writer structure"]
impl crate::Writable for UCA1ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca1abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irctl](uca1irctl) module"]
pub type UCA1IRCTL = crate::Reg<u16, _UCA1IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRCTL;
#[doc = "`read()` method returns [uca1irctl::R](uca1irctl::R) reader structure"]
impl crate::Readable for UCA1IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irctl::W](uca1irctl::W) writer structure"]
impl crate::Writable for UCA1IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca1irctl;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie](uca1ie) module"]
pub type UCA1IE = crate::Reg<u16, _UCA1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IE;
#[doc = "`read()` method returns [uca1ie::R](uca1ie::R) reader structure"]
impl crate::Readable for UCA1IE {}
#[doc = "`write(|w| ..)` method takes [uca1ie::W](uca1ie::W) writer structure"]
impl crate::Writable for UCA1IE {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie_spi](uca1ie_spi) module"]
pub type UCA1IE_SPI = crate::Reg<u16, _UCA1IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IE_SPI;
#[doc = "`read()` method returns [uca1ie_spi::R](uca1ie_spi::R) reader structure"]
impl crate::Readable for UCA1IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ie_spi::W](uca1ie_spi::W) writer structure"]
impl crate::Writable for UCA1IE_SPI {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ifg](uca1ifg) module"]
pub type UCA1IFG = crate::Reg<u16, _UCA1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IFG;
#[doc = "`read()` method returns [uca1ifg::R](uca1ifg::R) reader structure"]
impl crate::Readable for UCA1IFG {}
#[doc = "`write(|w| ..)` method takes [uca1ifg::W](uca1ifg::W) writer structure"]
impl crate::Writable for UCA1IFG {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca1ifg;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ifg_spi](uca1ifg_spi) module"]
pub type UCA1IFG_SPI = crate::Reg<u16, _UCA1IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IFG_SPI;
#[doc = "`read()` method returns [uca1ifg_spi::R](uca1ifg_spi::R) reader structure"]
impl crate::Readable for UCA1IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca1ifg_spi::W](uca1ifg_spi::W) writer structure"]
impl crate::Writable for UCA1IFG_SPI {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca1ifg_spi;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1iv](uca1iv) module"]
pub type UCA1IV = crate::Reg<u16, _UCA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV;
#[doc = "`read()` method returns [uca1iv::R](uca1iv::R) reader structure"]
impl crate::Readable for UCA1IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca1iv;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1iv_spi](uca1iv_spi) module"]
pub type UCA1IV_SPI = crate::Reg<u16, _UCA1IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV_SPI;
#[doc = "`read()` method returns [uca1iv_spi::R](uca1iv_spi::R) reader structure"]
impl crate::Readable for UCA1IV_SPI {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca1iv_spi;
