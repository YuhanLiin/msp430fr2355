#[doc = "Reader of register UCB0TXBUF_SPI"]
pub type R = crate::R<u16, super::UCB0TXBUF_SPI>;
#[doc = "Writer for register UCB0TXBUF_SPI"]
pub type W = crate::W<u16, super::UCB0TXBUF_SPI>;
#[doc = "Register UCB0TXBUF_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0TXBUF_SPI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCTXBUF`"]
pub type UCTXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCTXBUF`"]
pub struct UCTXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UCTXBUF_R {
        UCTXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UCTXBUF_W {
        UCTXBUF_W { w: self }
    }
}
