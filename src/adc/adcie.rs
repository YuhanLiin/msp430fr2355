#[doc = "Reader of register ADCIE"]
pub type R = crate::R<u16, super::ADCIE>;
#[doc = "Writer for register ADCIE"]
pub type W = crate::W<u16, super::ADCIE>;
#[doc = "Register ADCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIE0_A {
    #[doc = "0: 0b = Interrupt disabled"]
    ADCIE0_0 = 0,
    #[doc = "1: 1b = Interrupt enabled"]
    ADCIE0_1 = 1,
}
impl From<ADCIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCIE0`"]
pub type ADCIE0_R = crate::R<bool, ADCIE0_A>;
impl ADCIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIE0_A {
        match self.bits {
            false => ADCIE0_A::ADCIE0_0,
            true => ADCIE0_A::ADCIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCIE0_0`"]
    #[inline(always)]
    pub fn is_adcie0_0(&self) -> bool {
        *self == ADCIE0_A::ADCIE0_0
    }
    #[doc = "Checks if the value of the field is `ADCIE0_1`"]
    #[inline(always)]
    pub fn is_adcie0_1(&self) -> bool {
        *self == ADCIE0_A::ADCIE0_1
    }
}
#[doc = "Write proxy for field `ADCIE0`"]
pub struct ADCIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Interrupt disabled"]
    #[inline(always)]
    pub fn adcie0_0(self) -> &'a mut W {
        self.variant(ADCIE0_A::ADCIE0_0)
    }
    #[doc = "1b = Interrupt enabled"]
    #[inline(always)]
    pub fn adcie0_1(self) -> &'a mut W {
        self.variant(ADCIE0_A::ADCIE0_1)
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
#[doc = "Interrupt enable for the inside of window interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCINIE_A {
    #[doc = "0: 0b = Inside of window interrupt disabled"]
    ADCINIE_0 = 0,
    #[doc = "1: 1b = Inside of window interrupt enabled"]
    ADCINIE_1 = 1,
}
impl From<ADCINIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCINIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCINIE`"]
pub type ADCINIE_R = crate::R<bool, ADCINIE_A>;
impl ADCINIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINIE_A {
        match self.bits {
            false => ADCINIE_A::ADCINIE_0,
            true => ADCINIE_A::ADCINIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCINIE_0`"]
    #[inline(always)]
    pub fn is_adcinie_0(&self) -> bool {
        *self == ADCINIE_A::ADCINIE_0
    }
    #[doc = "Checks if the value of the field is `ADCINIE_1`"]
    #[inline(always)]
    pub fn is_adcinie_1(&self) -> bool {
        *self == ADCINIE_A::ADCINIE_1
    }
}
#[doc = "Write proxy for field `ADCINIE`"]
pub struct ADCINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCINIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCINIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Inside of window interrupt disabled"]
    #[inline(always)]
    pub fn adcinie_0(self) -> &'a mut W {
        self.variant(ADCINIE_A::ADCINIE_0)
    }
    #[doc = "1b = Inside of window interrupt enabled"]
    #[inline(always)]
    pub fn adcinie_1(self) -> &'a mut W {
        self.variant(ADCINIE_A::ADCINIE_1)
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
#[doc = "Interrupt enable for the below lower threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCLOIE_A {
    #[doc = "0: 0b = Below lower threshold interrupt disabled"]
    ADCLOIE_0 = 0,
    #[doc = "1: 1b = Below lower threshold interrupt enabled"]
    ADCLOIE_1 = 1,
}
impl From<ADCLOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCLOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCLOIE`"]
pub type ADCLOIE_R = crate::R<bool, ADCLOIE_A>;
impl ADCLOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCLOIE_A {
        match self.bits {
            false => ADCLOIE_A::ADCLOIE_0,
            true => ADCLOIE_A::ADCLOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCLOIE_0`"]
    #[inline(always)]
    pub fn is_adcloie_0(&self) -> bool {
        *self == ADCLOIE_A::ADCLOIE_0
    }
    #[doc = "Checks if the value of the field is `ADCLOIE_1`"]
    #[inline(always)]
    pub fn is_adcloie_1(&self) -> bool {
        *self == ADCLOIE_A::ADCLOIE_1
    }
}
#[doc = "Write proxy for field `ADCLOIE`"]
pub struct ADCLOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCLOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCLOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Below lower threshold interrupt disabled"]
    #[inline(always)]
    pub fn adcloie_0(self) -> &'a mut W {
        self.variant(ADCLOIE_A::ADCLOIE_0)
    }
    #[doc = "1b = Below lower threshold interrupt enabled"]
    #[inline(always)]
    pub fn adcloie_1(self) -> &'a mut W {
        self.variant(ADCLOIE_A::ADCLOIE_1)
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
#[doc = "Interrupt enable for the above upper threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHIIE_A {
    #[doc = "0: 0b = Above upper threshold interrupt disabled"]
    ADCHIIE_0 = 0,
    #[doc = "1: 1b = Above upper threshold interrupt enabled"]
    ADCHIIE_1 = 1,
}
impl From<ADCHIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCHIIE`"]
pub type ADCHIIE_R = crate::R<bool, ADCHIIE_A>;
impl ADCHIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHIIE_A {
        match self.bits {
            false => ADCHIIE_A::ADCHIIE_0,
            true => ADCHIIE_A::ADCHIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCHIIE_0`"]
    #[inline(always)]
    pub fn is_adchiie_0(&self) -> bool {
        *self == ADCHIIE_A::ADCHIIE_0
    }
    #[doc = "Checks if the value of the field is `ADCHIIE_1`"]
    #[inline(always)]
    pub fn is_adchiie_1(&self) -> bool {
        *self == ADCHIIE_A::ADCHIIE_1
    }
}
#[doc = "Write proxy for field `ADCHIIE`"]
pub struct ADCHIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCHIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Above upper threshold interrupt disabled"]
    #[inline(always)]
    pub fn adchiie_0(self) -> &'a mut W {
        self.variant(ADCHIIE_A::ADCHIIE_0)
    }
    #[doc = "1b = Above upper threshold interrupt enabled"]
    #[inline(always)]
    pub fn adchiie_1(self) -> &'a mut W {
        self.variant(ADCHIIE_A::ADCHIIE_1)
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
#[doc = "ADCMEM0 overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCOVIE_A {
    #[doc = "0: 0b = Overflow interrupt disabled"]
    ADCOVIE_0 = 0,
    #[doc = "1: 1b = Overflow interrupt enabled"]
    ADCOVIE_1 = 1,
}
impl From<ADCOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCOVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCOVIE`"]
pub type ADCOVIE_R = crate::R<bool, ADCOVIE_A>;
impl ADCOVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCOVIE_A {
        match self.bits {
            false => ADCOVIE_A::ADCOVIE_0,
            true => ADCOVIE_A::ADCOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIE_0`"]
    #[inline(always)]
    pub fn is_adcovie_0(&self) -> bool {
        *self == ADCOVIE_A::ADCOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADCOVIE_1`"]
    #[inline(always)]
    pub fn is_adcovie_1(&self) -> bool {
        *self == ADCOVIE_A::ADCOVIE_1
    }
}
#[doc = "Write proxy for field `ADCOVIE`"]
pub struct ADCOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCOVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Overflow interrupt disabled"]
    #[inline(always)]
    pub fn adcovie_0(self) -> &'a mut W {
        self.variant(ADCOVIE_A::ADCOVIE_0)
    }
    #[doc = "1b = Overflow interrupt enabled"]
    #[inline(always)]
    pub fn adcovie_1(self) -> &'a mut W {
        self.variant(ADCOVIE_A::ADCOVIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC conversion-time-overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTOVIE_A {
    #[doc = "0: 0b = Conversion time overflow interrupt disabled"]
    ADCTOVIE_0 = 0,
    #[doc = "1: 1b = Conversion time overflow interrupt enabled"]
    ADCTOVIE_1 = 1,
}
impl From<ADCTOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTOVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCTOVIE`"]
pub type ADCTOVIE_R = crate::R<bool, ADCTOVIE_A>;
impl ADCTOVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTOVIE_A {
        match self.bits {
            false => ADCTOVIE_A::ADCTOVIE_0,
            true => ADCTOVIE_A::ADCTOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCTOVIE_0`"]
    #[inline(always)]
    pub fn is_adctovie_0(&self) -> bool {
        *self == ADCTOVIE_A::ADCTOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADCTOVIE_1`"]
    #[inline(always)]
    pub fn is_adctovie_1(&self) -> bool {
        *self == ADCTOVIE_A::ADCTOVIE_1
    }
}
#[doc = "Write proxy for field `ADCTOVIE`"]
pub struct ADCTOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTOVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTOVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0b = Conversion time overflow interrupt disabled"]
    #[inline(always)]
    pub fn adctovie_0(self) -> &'a mut W {
        self.variant(ADCTOVIE_A::ADCTOVIE_0)
    }
    #[doc = "1b = Conversion time overflow interrupt enabled"]
    #[inline(always)]
    pub fn adctovie_1(self) -> &'a mut W {
        self.variant(ADCTOVIE_A::ADCTOVIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&self) -> ADCIE0_R {
        ADCIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&self) -> ADCINIE_R {
        ADCINIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&self) -> ADCLOIE_R {
        ADCLOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&self) -> ADCHIIE_R {
        ADCHIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&self) -> ADCOVIE_R {
        ADCOVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&self) -> ADCTOVIE_R {
        ADCTOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&mut self) -> ADCIE0_W {
        ADCIE0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&mut self) -> ADCINIE_W {
        ADCINIE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&mut self) -> ADCLOIE_W {
        ADCLOIE_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&mut self) -> ADCHIIE_W {
        ADCHIIE_W { w: self }
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&mut self) -> ADCOVIE_W {
        ADCOVIE_W { w: self }
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&mut self) -> ADCTOVIE_W {
        ADCTOVIE_W { w: self }
    }
}
