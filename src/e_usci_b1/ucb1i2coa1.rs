#[doc = "Reader of register UCB1I2COA1"]
pub type R = crate::R<u16, super::UCB1I2COA1>;
#[doc = "Writer for register UCB1I2COA1"]
pub type W = crate::W<u16, super::UCB1I2COA1>;
#[doc = "Register UCB1I2COA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB1I2COA1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C own address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2COA1_A {}
impl From<I2COA1_A> for u16 {
    #[inline(always)]
    fn from(variant: I2COA1_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `I2COA1`"]
pub type I2COA1_R = crate::R<u16, I2COA1_A>;
impl I2COA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, I2COA1_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `I2COA1`"]
pub struct I2COA1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2COA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2COA1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOAEN_A {
    #[doc = "0: The slave address defined in I2COA1 is disabled"]
    DISABLE,
    #[doc = "1: The slave address defined in I2COA1 is enabled"]
    ENABLE,
}
impl From<UCOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCOAEN_A) -> Self {
        match variant {
            UCOAEN_A::DISABLE => false,
            UCOAEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UCOAEN`"]
pub type UCOAEN_R = crate::R<bool, UCOAEN_A>;
impl UCOAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOAEN_A {
        match self.bits {
            false => UCOAEN_A::DISABLE,
            true => UCOAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UCOAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UCOAEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `UCOAEN`"]
pub struct UCOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The slave address defined in I2COA1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UCOAEN_A::DISABLE)
    }
    #[doc = "The slave address defined in I2COA1 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UCOAEN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&self) -> I2COA1_R {
        I2COA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UCOAEN_R {
        UCOAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&mut self) -> I2COA1_W {
        I2COA1_W { w: self }
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UCOAEN_W {
        UCOAEN_W { w: self }
    }
}
