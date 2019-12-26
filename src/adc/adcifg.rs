#[doc = "Reader of register ADCIFG"]
pub type R = crate::R<u16, super::ADCIFG>;
#[doc = "Writer for register ADCIFG"]
pub type W = crate::W<u16, super::ADCIFG>;
#[doc = "Register ADCIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADCMEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIFG0_A {
    #[doc = "0: No interrupt pending"]
    ADCIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCIFG0_1 = 1,
}
impl From<ADCIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCIFG0`"]
pub type ADCIFG0_R = crate::R<bool, ADCIFG0_A>;
impl ADCIFG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIFG0_A {
        match self.bits {
            false => ADCIFG0_A::ADCIFG0_0,
            true => ADCIFG0_A::ADCIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCIFG0_0`"]
    #[inline(always)]
    pub fn is_adcifg0_0(&self) -> bool {
        *self == ADCIFG0_A::ADCIFG0_0
    }
    #[doc = "Checks if the value of the field is `ADCIFG0_1`"]
    #[inline(always)]
    pub fn is_adcifg0_1(&self) -> bool {
        *self == ADCIFG0_A::ADCIFG0_1
    }
}
#[doc = "Write proxy for field `ADCIFG0`"]
pub struct ADCIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIFG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_0(self) -> &'a mut W {
        self.variant(ADCIFG0_A::ADCIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_1(self) -> &'a mut W {
        self.variant(ADCIFG0_A::ADCIFG0_1)
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
#[doc = "The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCINIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCINIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCINIFG_1 = 1,
}
impl From<ADCINIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCINIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCINIFG`"]
pub type ADCINIFG_R = crate::R<bool, ADCINIFG_A>;
impl ADCINIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINIFG_A {
        match self.bits {
            false => ADCINIFG_A::ADCINIFG_0,
            true => ADCINIFG_A::ADCINIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCINIFG_0`"]
    #[inline(always)]
    pub fn is_adcinifg_0(&self) -> bool {
        *self == ADCINIFG_A::ADCINIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCINIFG_1`"]
    #[inline(always)]
    pub fn is_adcinifg_1(&self) -> bool {
        *self == ADCINIFG_A::ADCINIFG_1
    }
}
#[doc = "Write proxy for field `ADCINIFG`"]
pub struct ADCINIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCINIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCINIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_0(self) -> &'a mut W {
        self.variant(ADCINIFG_A::ADCINIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_1(self) -> &'a mut W {
        self.variant(ADCINIFG_A::ADCINIFG_1)
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
#[doc = "The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCLOIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCLOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCLOIFG_1 = 1,
}
impl From<ADCLOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCLOIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCLOIFG`"]
pub type ADCLOIFG_R = crate::R<bool, ADCLOIFG_A>;
impl ADCLOIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCLOIFG_A {
        match self.bits {
            false => ADCLOIFG_A::ADCLOIFG_0,
            true => ADCLOIFG_A::ADCLOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG_0`"]
    #[inline(always)]
    pub fn is_adcloifg_0(&self) -> bool {
        *self == ADCLOIFG_A::ADCLOIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG_1`"]
    #[inline(always)]
    pub fn is_adcloifg_1(&self) -> bool {
        *self == ADCLOIFG_A::ADCLOIFG_1
    }
}
#[doc = "Write proxy for field `ADCLOIFG`"]
pub struct ADCLOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCLOIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCLOIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_0(self) -> &'a mut W {
        self.variant(ADCLOIFG_A::ADCLOIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_1(self) -> &'a mut W {
        self.variant(ADCLOIFG_A::ADCLOIFG_1)
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
#[doc = "The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHIIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCHIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCHIIFG_1 = 1,
}
impl From<ADCHIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCHIIFG`"]
pub type ADCHIIFG_R = crate::R<bool, ADCHIIFG_A>;
impl ADCHIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHIIFG_A {
        match self.bits {
            false => ADCHIIFG_A::ADCHIIFG_0,
            true => ADCHIIFG_A::ADCHIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG_0`"]
    #[inline(always)]
    pub fn is_adchiifg_0(&self) -> bool {
        *self == ADCHIIFG_A::ADCHIIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG_1`"]
    #[inline(always)]
    pub fn is_adchiifg_1(&self) -> bool {
        *self == ADCHIIFG_A::ADCHIIFG_1
    }
}
#[doc = "Write proxy for field `ADCHIIFG`"]
pub struct ADCHIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCHIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_0(self) -> &'a mut W {
        self.variant(ADCHIIFG_A::ADCHIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_1(self) -> &'a mut W {
        self.variant(ADCHIIFG_A::ADCHIIFG_1)
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
#[doc = "The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCOVIFG_1 = 1,
}
impl From<ADCOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCOVIFG`"]
pub type ADCOVIFG_R = crate::R<bool, ADCOVIFG_A>;
impl ADCOVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCOVIFG_A {
        match self.bits {
            false => ADCOVIFG_A::ADCOVIFG_0,
            true => ADCOVIFG_A::ADCOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_0`"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == ADCOVIFG_A::ADCOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_1`"]
    #[inline(always)]
    pub fn is_adcovifg_1(&self) -> bool {
        *self == ADCOVIFG_A::ADCOVIFG_1
    }
}
#[doc = "Write proxy for field `ADCOVIFG`"]
pub struct ADCOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCOVIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut W {
        self.variant(ADCOVIFG_A::ADCOVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_1(self) -> &'a mut W {
        self.variant(ADCOVIFG_A::ADCOVIFG_1)
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
#[doc = "The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCTOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCTOVIFG_1 = 1,
}
impl From<ADCTOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCTOVIFG`"]
pub type ADCTOVIFG_R = crate::R<bool, ADCTOVIFG_A>;
impl ADCTOVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTOVIFG_A {
        match self.bits {
            false => ADCTOVIFG_A::ADCOVIFG_0,
            true => ADCTOVIFG_A::ADCTOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_0`"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == ADCTOVIFG_A::ADCOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCTOVIFG_1`"]
    #[inline(always)]
    pub fn is_adctovifg_1(&self) -> bool {
        *self == ADCTOVIFG_A::ADCTOVIFG_1
    }
}
#[doc = "Write proxy for field `ADCTOVIFG`"]
pub struct ADCTOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTOVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTOVIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut W {
        self.variant(ADCTOVIFG_A::ADCOVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adctovifg_1(self) -> &'a mut W {
        self.variant(ADCTOVIFG_A::ADCTOVIFG_1)
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
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> ADCIFG0_R {
        ADCIFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&self) -> ADCINIFG_R {
        ADCINIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&self) -> ADCLOIFG_R {
        ADCLOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&self) -> ADCHIIFG_R {
        ADCHIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&self) -> ADCOVIFG_R {
        ADCOVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&self) -> ADCTOVIFG_R {
        ADCTOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> ADCIFG0_W {
        ADCIFG0_W { w: self }
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> ADCINIFG_W {
        ADCINIFG_W { w: self }
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> ADCLOIFG_W {
        ADCLOIFG_W { w: self }
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> ADCHIIFG_W {
        ADCHIIFG_W { w: self }
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> ADCOVIFG_W {
        ADCOVIFG_W { w: self }
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> ADCTOVIFG_W {
        ADCTOVIFG_W { w: self }
    }
}
