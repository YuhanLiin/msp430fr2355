#[doc = "Reader of register UCB1TBCNT"]
pub type R = crate::R<u16, super::UCB1TBCNT>;
#[doc = "Writer for register UCB1TBCNT"]
pub type W = crate::W<u16, super::UCB1TBCNT>;
#[doc = "Register UCB1TBCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB1TBCNT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCTBCNT`"]
pub type UCTBCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCTBCNT`"]
pub struct UCTBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTBCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UCTBCNT_R {
        UCTBCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UCTBCNT_W {
        UCTBCNT_W { w: self }
    }
}
