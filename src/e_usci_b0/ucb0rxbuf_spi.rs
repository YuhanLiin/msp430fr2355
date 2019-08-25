#[doc = "Reader of register UCB0RXBUF_SPI"]
pub type R = crate::R<u16, super::UCB0RXBUF_SPI>;
#[doc = "Receive data buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXBUF_A {}
impl From<UCRXBUF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCRXBUF_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `UCRXBUF`"]
pub type UCRXBUF_R = crate::R<u8, UCRXBUF_A>;
impl UCRXBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UCRXBUF_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UCRXBUF_R {
        UCRXBUF_R::new((self.bits & 0xff) as u8)
    }
}
