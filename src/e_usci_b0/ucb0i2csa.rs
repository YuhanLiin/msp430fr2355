#[doc = "Reader of register UCB0I2CSA"]
pub type R = crate::R<u16, super::UCB0I2CSA>;
#[doc = "Writer for register UCB0I2CSA"]
pub type W = crate::W<u16, super::UCB0I2CSA>;
#[doc = "Register UCB0I2CSA `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0I2CSA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CSA`"]
pub type I2CSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2CSA`"]
pub struct I2CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&self) -> I2CSA_R {
        I2CSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&mut self) -> I2CSA_W {
        I2CSA_W { w: self }
    }
}
