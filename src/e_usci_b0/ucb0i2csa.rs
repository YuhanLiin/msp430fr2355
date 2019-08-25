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
#[doc = "I2C slave address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CSA_A {}
impl From<I2CSA_A> for u16 {
    #[inline(always)]
    fn from(variant: I2CSA_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `I2CSA`"]
pub type I2CSA_R = crate::R<u16, I2CSA_A>;
impl I2CSA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, I2CSA_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `I2CSA`"]
pub struct I2CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CSA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
