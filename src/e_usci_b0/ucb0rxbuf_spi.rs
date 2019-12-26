#[doc = "Reader of register UCB0RXBUF_SPI"]
pub type R = crate::R<u16, super::UCB0RXBUF_SPI>;
#[doc = "Reader of field `UCRXBUF`"]
pub type UCRXBUF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UCRXBUF_R {
        UCRXBUF_R::new((self.bits & 0xff) as u8)
    }
}
