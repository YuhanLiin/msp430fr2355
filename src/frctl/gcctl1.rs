#[doc = "Reader of register GCCTL1"]
pub type R = crate::R<u16, super::GCCTL1>;
#[doc = "Writer for register GCCTL1"]
pub type W = crate::W<u16, super::GCCTL1>;
#[doc = "Register GCCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GCCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access time error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCTEIFG_A {
    #[doc = "0: No interrupt pending."]
    ACCTEIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    ACCTEIFG_1 = 1,
}
impl From<ACCTEIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ACCTEIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACCTEIFG`"]
pub type ACCTEIFG_R = crate::R<bool, ACCTEIFG_A>;
impl ACCTEIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCTEIFG_A {
        match self.bits {
            false => ACCTEIFG_A::ACCTEIFG_0,
            true => ACCTEIFG_A::ACCTEIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG_0`"]
    #[inline(always)]
    pub fn is_accteifg_0(&self) -> bool {
        *self == ACCTEIFG_A::ACCTEIFG_0
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG_1`"]
    #[inline(always)]
    pub fn is_accteifg_1(&self) -> bool {
        *self == ACCTEIFG_A::ACCTEIFG_1
    }
}
#[doc = "Write proxy for field `ACCTEIFG`"]
pub struct ACCTEIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCTEIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCTEIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn accteifg_0(self) -> &'a mut W {
        self.variant(ACCTEIFG_A::ACCTEIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn accteifg_1(self) -> &'a mut W {
        self.variant(ACCTEIFG_A::ACCTEIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "FRAM uncorrectable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UBDIFG_A {
    #[doc = "0: No interrupt pending."]
    UBDIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    UBDIFG_1 = 1,
}
impl From<UBDIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UBDIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UBDIFG`"]
pub type UBDIFG_R = crate::R<bool, UBDIFG_A>;
impl UBDIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDIFG_A {
        match self.bits {
            false => UBDIFG_A::UBDIFG_0,
            true => UBDIFG_A::UBDIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDIFG_0`"]
    #[inline(always)]
    pub fn is_ubdifg_0(&self) -> bool {
        *self == UBDIFG_A::UBDIFG_0
    }
    #[doc = "Checks if the value of the field is `UBDIFG_1`"]
    #[inline(always)]
    pub fn is_ubdifg_1(&self) -> bool {
        *self == UBDIFG_A::UBDIFG_1
    }
}
#[doc = "Write proxy for field `UBDIFG`"]
pub struct UBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UBDIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn ubdifg_0(self) -> &'a mut W {
        self.variant(UBDIFG_A::UBDIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn ubdifg_1(self) -> &'a mut W {
        self.variant(UBDIFG_A::UBDIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "FRAM correctable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBDIFG_A {
    #[doc = "0: No interrupt is pending"]
    CBDIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    CBDIFG_1 = 1,
}
impl From<CBDIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CBDIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CBDIFG`"]
pub type CBDIFG_R = crate::R<bool, CBDIFG_A>;
impl CBDIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBDIFG_A {
        match self.bits {
            false => CBDIFG_A::CBDIFG_0,
            true => CBDIFG_A::CBDIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CBDIFG_0`"]
    #[inline(always)]
    pub fn is_cbdifg_0(&self) -> bool {
        *self == CBDIFG_A::CBDIFG_0
    }
    #[doc = "Checks if the value of the field is `CBDIFG_1`"]
    #[inline(always)]
    pub fn is_cbdifg_1(&self) -> bool {
        *self == CBDIFG_A::CBDIFG_1
    }
}
#[doc = "Write proxy for field `CBDIFG`"]
pub struct CBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBDIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn cbdifg_0(self) -> &'a mut W {
        self.variant(CBDIFG_A::CBDIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    #[inline(always)]
    pub fn cbdifg_1(self) -> &'a mut W {
        self.variant(CBDIFG_A::CBDIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> ACCTEIFG_W {
        ACCTEIFG_W { w: self }
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UBDIFG_W {
        UBDIFG_W { w: self }
    }
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CBDIFG_W {
        CBDIFG_W { w: self }
    }
}
