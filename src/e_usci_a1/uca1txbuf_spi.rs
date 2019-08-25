#[doc = "Reader of register UCA1TXBUF_SPI"]
pub type R = crate::R<u16, super::UCA1TXBUF_SPI>;
#[doc = "Writer for register UCA1TXBUF_SPI"]
pub type W = crate::W<u16, super::UCA1TXBUF_SPI>;
#[doc = "Register UCA1TXBUF_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA1TXBUF_SPI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit data buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXBUF_A {}
impl From<UCTXBUF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCTXBUF_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `UCTXBUF`"]
pub type UCTXBUF_R = crate::R<u8, UCTXBUF_A>;
impl UCTXBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UCTXBUF_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `UCTXBUF`"]
pub struct UCTXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXBUF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
