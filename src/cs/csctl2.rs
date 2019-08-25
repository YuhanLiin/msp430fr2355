#[doc = "Reader of register CSCTL2"]
pub type R = crate::R<u16, super::CSCTL2>;
#[doc = "Writer for register CSCTL2"]
pub type W = crate::W<u16, super::CSCTL2>;
#[doc = "Register CSCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN_A {}
impl From<FLLN_A> for u16 {
    #[inline(always)]
    fn from(variant: FLLN_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `FLLN`"]
pub type FLLN_R = crate::R<u16, FLLN_A>;
impl FLLN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FLLN_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `FLLN`"]
pub struct FLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
#[doc = "FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLD_A {
    #[doc = "0: fDCOCLK / 1"]
    _1,
    #[doc = "1: fDCOCLK / 2"]
    _2,
    #[doc = "2: fDCOCLK / 4"]
    _4,
    #[doc = "3: fDCOCLK / 8"]
    _8,
    #[doc = "4: fDCOCLK / 16"]
    _16,
    #[doc = "5: fDCOCLK / 32"]
    _32,
    #[doc = "6: fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    FLLD_6,
    #[doc = "7: fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    FLLD_7,
}
impl From<FLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLD_A) -> Self {
        match variant {
            FLLD_A::_1 => 0,
            FLLD_A::_2 => 1,
            FLLD_A::_4 => 2,
            FLLD_A::_8 => 3,
            FLLD_A::_16 => 4,
            FLLD_A::_32 => 5,
            FLLD_A::FLLD_6 => 6,
            FLLD_A::FLLD_7 => 7,
        }
    }
}
#[doc = "Reader of field `FLLD`"]
pub type FLLD_R = crate::R<u8, FLLD_A>;
impl FLLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLD_A {
        match self.bits {
            0 => FLLD_A::_1,
            1 => FLLD_A::_2,
            2 => FLLD_A::_4,
            3 => FLLD_A::_8,
            4 => FLLD_A::_16,
            5 => FLLD_A::_32,
            6 => FLLD_A::FLLD_6,
            7 => FLLD_A::FLLD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLD_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == FLLD_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FLLD_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == FLLD_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == FLLD_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == FLLD_A::_32
    }
    #[doc = "Checks if the value of the field is `FLLD_6`"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        *self == FLLD_A::FLLD_6
    }
    #[doc = "Checks if the value of the field is `FLLD_7`"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        *self == FLLD_A::FLLD_7
    }
}
#[doc = "Write proxy for field `FLLD`"]
pub struct FLLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fDCOCLK / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLD_A::_1)
    }
    #[doc = "fDCOCLK / 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FLLD_A::_2)
    }
    #[doc = "fDCOCLK / 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FLLD_A::_4)
    }
    #[doc = "fDCOCLK / 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(FLLD_A::_8)
    }
    #[doc = "fDCOCLK / 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FLLD_A::_16)
    }
    #[doc = "fDCOCLK / 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FLLD_A::_32)
    }
    #[doc = "fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_6)
    }
    #[doc = "fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&self) -> FLLN_R {
        FLLN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&self) -> FLLD_R {
        FLLD_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&mut self) -> FLLN_W {
        FLLN_W { w: self }
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&mut self) -> FLLD_W {
        FLLD_W { w: self }
    }
}
