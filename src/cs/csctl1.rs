#[doc = "Reader of register CSCTL1"]
pub type R = crate::R<u16, super::CSCTL1>;
#[doc = "Writer for register CSCTL1"]
pub type W = crate::W<u16, super::CSCTL1>;
#[doc = "Register CSCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Modulation. This bit enables/disables the modulation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISMOD_A {
    #[doc = "0: Modulation enabled"]
    DISMOD_0,
    #[doc = "1: Modulation disabled"]
    DISMOD_1,
}
impl From<DISMOD_A> for bool {
    #[inline(always)]
    fn from(variant: DISMOD_A) -> Self {
        match variant {
            DISMOD_A::DISMOD_0 => false,
            DISMOD_A::DISMOD_1 => true,
        }
    }
}
#[doc = "Reader of field `DISMOD`"]
pub type DISMOD_R = crate::R<bool, DISMOD_A>;
impl DISMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISMOD_A {
        match self.bits {
            false => DISMOD_A::DISMOD_0,
            true => DISMOD_A::DISMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISMOD_0`"]
    #[inline(always)]
    pub fn is_dismod_0(&self) -> bool {
        *self == DISMOD_A::DISMOD_0
    }
    #[doc = "Checks if the value of the field is `DISMOD_1`"]
    #[inline(always)]
    pub fn is_dismod_1(&self) -> bool {
        *self == DISMOD_A::DISMOD_1
    }
}
#[doc = "Write proxy for field `DISMOD`"]
pub struct DISMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Modulation enabled"]
    #[inline(always)]
    pub fn dismod_0(self) -> &'a mut W {
        self.variant(DISMOD_A::DISMOD_0)
    }
    #[doc = "Modulation disabled"]
    #[inline(always)]
    pub fn dismod_1(self) -> &'a mut W {
        self.variant(DISMOD_A::DISMOD_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCORSEL_A {
    #[doc = "0: 1 MHz"]
    DCORSEL_0,
    #[doc = "1: 2 MHz"]
    DCORSEL_1,
    #[doc = "2: 4 MHz"]
    DCORSEL_2,
    #[doc = "3: 8 MHz"]
    DCORSEL_3,
    #[doc = "4: 12 MHz"]
    DCORSEL_4,
    #[doc = "5: 16 MHz"]
    DCORSEL_5,
    #[doc = "6: 20 MHz(Only avaliable in 24MHz clock system)"]
    DCORSEL_6,
    #[doc = "7: 24 MHz(Only avaliable in 24MHz clock system)"]
    DCORSEL_7,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        match variant {
            DCORSEL_A::DCORSEL_0 => 0,
            DCORSEL_A::DCORSEL_1 => 1,
            DCORSEL_A::DCORSEL_2 => 2,
            DCORSEL_A::DCORSEL_3 => 3,
            DCORSEL_A::DCORSEL_4 => 4,
            DCORSEL_A::DCORSEL_5 => 5,
            DCORSEL_A::DCORSEL_6 => 6,
            DCORSEL_A::DCORSEL_7 => 7,
        }
    }
}
#[doc = "Reader of field `DCORSEL`"]
pub type DCORSEL_R = crate::R<u8, DCORSEL_A>;
impl DCORSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORSEL_A {
        match self.bits {
            0 => DCORSEL_A::DCORSEL_0,
            1 => DCORSEL_A::DCORSEL_1,
            2 => DCORSEL_A::DCORSEL_2,
            3 => DCORSEL_A::DCORSEL_3,
            4 => DCORSEL_A::DCORSEL_4,
            5 => DCORSEL_A::DCORSEL_5,
            6 => DCORSEL_A::DCORSEL_6,
            7 => DCORSEL_A::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
    #[doc = "Checks if the value of the field is `DCORSEL_6`"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_6
    }
    #[doc = "Checks if the value of the field is `DCORSEL_7`"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_7
    }
}
#[doc = "Write proxy for field `DCORSEL`"]
pub struct DCORSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCORSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCORSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = "20 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_6)
    }
    #[doc = "24 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u16) & 0x07) << 1);
        self.w
    }
}
#[doc = "DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFTRIM_A {}
impl From<DCOFTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: DCOFTRIM_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `DCOFTRIM`"]
pub type DCOFTRIM_R = crate::R<u8, DCOFTRIM_A>;
impl DCOFTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCOFTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `DCOFTRIM`"]
pub struct DCOFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOFTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFTRIMEN_A {
    #[doc = "0: Disable frequency trim"]
    DCOFTRIMEN_0,
    #[doc = "1: Enable frequency trim"]
    DCOFTRIMEN_1,
}
impl From<DCOFTRIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCOFTRIMEN_A) -> Self {
        match variant {
            DCOFTRIMEN_A::DCOFTRIMEN_0 => false,
            DCOFTRIMEN_A::DCOFTRIMEN_1 => true,
        }
    }
}
#[doc = "Reader of field `DCOFTRIMEN`"]
pub type DCOFTRIMEN_R = crate::R<bool, DCOFTRIMEN_A>;
impl DCOFTRIMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFTRIMEN_A {
        match self.bits {
            false => DCOFTRIMEN_A::DCOFTRIMEN_0,
            true => DCOFTRIMEN_A::DCOFTRIMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOFTRIMEN_0`"]
    #[inline(always)]
    pub fn is_dcoftrimen_0(&self) -> bool {
        *self == DCOFTRIMEN_A::DCOFTRIMEN_0
    }
    #[doc = "Checks if the value of the field is `DCOFTRIMEN_1`"]
    #[inline(always)]
    pub fn is_dcoftrimen_1(&self) -> bool {
        *self == DCOFTRIMEN_A::DCOFTRIMEN_1
    }
}
#[doc = "Write proxy for field `DCOFTRIMEN`"]
pub struct DCOFTRIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFTRIMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOFTRIMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_0(self) -> &'a mut W {
        self.variant(DCOFTRIMEN_A::DCOFTRIMEN_0)
    }
    #[doc = "Enable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_1(self) -> &'a mut W {
        self.variant(DCOFTRIMEN_A::DCOFTRIMEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DCOFTRIM_R {
        DCOFTRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DCOFTRIMEN_R {
        DCOFTRIMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&mut self) -> DISMOD_W {
        DISMOD_W { w: self }
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W {
        DCORSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DCOFTRIM_W {
        DCOFTRIM_W { w: self }
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DCOFTRIMEN_W {
        DCOFTRIMEN_W { w: self }
    }
}
