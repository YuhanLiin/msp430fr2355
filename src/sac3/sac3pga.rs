#[doc = "Reader of register SAC3PGA"]
pub type R = crate::R<u16, super::SAC3PGA>;
#[doc = "Writer for register SAC3PGA"]
pub type W = crate::W<u16, super::SAC3PGA>;
#[doc = "Register SAC3PGA `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC3PGA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAC PGA Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Inverting PGA mode (external pad IN- is selected)"]
    MSEL_0 = 0,
    #[doc = "1: Buffer mode (floating is selected )"]
    MSEL_1 = 1,
    #[doc = "2: Non-inverting mode"]
    MSEL_2 = 2,
    #[doc = "3: Cascade OA Inverting mode"]
    MSEL_3 = 3,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Inverting PGA mode (external pad IN- is selected)"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "Buffer mode (floating is selected )"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "Non-inverting mode"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "Cascade OA Inverting mode"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
}
