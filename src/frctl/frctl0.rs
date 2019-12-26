#[doc = "Reader of register FRCTL0"]
pub type R = crate::R<u16, super::FRCTL0>;
#[doc = "Writer for register FRCTL0"]
pub type W = crate::W<u16, super::FRCTL0>;
#[doc = "Register FRCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wait state numbers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NWAITS_A {
    #[doc = "0: FRAM wait states: 0"]
    NWAITS_0 = 0,
    #[doc = "1: FRAM wait states: 1"]
    NWAITS_1 = 1,
    #[doc = "2: FRAM wait states: 2"]
    NWAITS_2 = 2,
    #[doc = "3: FRAM wait states: 3"]
    NWAITS_3 = 3,
    #[doc = "4: FRAM wait states: 4"]
    NWAITS_4 = 4,
    #[doc = "5: FRAM wait states: 5"]
    NWAITS_5 = 5,
    #[doc = "6: FRAM wait states: 6"]
    NWAITS_6 = 6,
    #[doc = "7: FRAM wait states: 7"]
    NWAITS_7 = 7,
}
impl From<NWAITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NWAITS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NWAITS`"]
pub type NWAITS_R = crate::R<u8, NWAITS_A>;
impl NWAITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NWAITS_A {
        match self.bits {
            0 => NWAITS_A::NWAITS_0,
            1 => NWAITS_A::NWAITS_1,
            2 => NWAITS_A::NWAITS_2,
            3 => NWAITS_A::NWAITS_3,
            4 => NWAITS_A::NWAITS_4,
            5 => NWAITS_A::NWAITS_5,
            6 => NWAITS_A::NWAITS_6,
            7 => NWAITS_A::NWAITS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NWAITS_0`"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == NWAITS_A::NWAITS_0
    }
    #[doc = "Checks if the value of the field is `NWAITS_1`"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == NWAITS_A::NWAITS_1
    }
    #[doc = "Checks if the value of the field is `NWAITS_2`"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == NWAITS_A::NWAITS_2
    }
    #[doc = "Checks if the value of the field is `NWAITS_3`"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == NWAITS_A::NWAITS_3
    }
    #[doc = "Checks if the value of the field is `NWAITS_4`"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == NWAITS_A::NWAITS_4
    }
    #[doc = "Checks if the value of the field is `NWAITS_5`"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == NWAITS_A::NWAITS_5
    }
    #[doc = "Checks if the value of the field is `NWAITS_6`"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == NWAITS_A::NWAITS_6
    }
    #[doc = "Checks if the value of the field is `NWAITS_7`"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == NWAITS_A::NWAITS_7
    }
}
#[doc = "Write proxy for field `NWAITS`"]
pub struct NWAITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NWAITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NWAITS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_0)
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_1)
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_2)
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_3)
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_4)
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_5)
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_6)
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `FRCTLPW`"]
pub type FRCTLPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRCTLPW`"]
pub struct FRCTLPW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCTLPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&self) -> NWAITS_R {
        NWAITS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FRCTLPW_R {
        FRCTLPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&mut self) -> NWAITS_W {
        NWAITS_W { w: self }
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FRCTLPW_W {
        FRCTLPW_W { w: self }
    }
}
