#[doc = "Reader of register SYSCFG2"]
pub type R = crate::R<u16, super::SYSCFG2>;
#[doc = "Writer for register SYSCFG2"]
pub type W = crate::W<u16, super::SYSCFG2>;
#[doc = "Register SYSCFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC input A0 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL0_A {
    #[doc = "0: ADC input A0 disabled"]
    ADCPCTL0_0 = 0,
    #[doc = "1: ADC input A0 enabled"]
    ADCPCTL0_1 = 1,
}
impl From<ADCPCTL0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL0`"]
pub type ADCPCTL0_R = crate::R<bool, ADCPCTL0_A>;
impl ADCPCTL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL0_A {
        match self.bits {
            false => ADCPCTL0_A::ADCPCTL0_0,
            true => ADCPCTL0_A::ADCPCTL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL0_0`"]
    #[inline(always)]
    pub fn is_adcpctl0_0(&self) -> bool {
        *self == ADCPCTL0_A::ADCPCTL0_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL0_1`"]
    #[inline(always)]
    pub fn is_adcpctl0_1(&self) -> bool {
        *self == ADCPCTL0_A::ADCPCTL0_1
    }
}
#[doc = "Write proxy for field `ADCPCTL0`"]
pub struct ADCPCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A0 disabled"]
    #[inline(always)]
    pub fn adcpctl0_0(self) -> &'a mut W {
        self.variant(ADCPCTL0_A::ADCPCTL0_0)
    }
    #[doc = "ADC input A0 enabled"]
    #[inline(always)]
    pub fn adcpctl0_1(self) -> &'a mut W {
        self.variant(ADCPCTL0_A::ADCPCTL0_1)
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
#[doc = "ADC input A1 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL1_A {
    #[doc = "0: ADC input A1 disabled"]
    ADCPCTL1_0 = 0,
    #[doc = "1: ADC input A1 enabled"]
    ADCPCTL1_1 = 1,
}
impl From<ADCPCTL1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL1`"]
pub type ADCPCTL1_R = crate::R<bool, ADCPCTL1_A>;
impl ADCPCTL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL1_A {
        match self.bits {
            false => ADCPCTL1_A::ADCPCTL1_0,
            true => ADCPCTL1_A::ADCPCTL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL1_0`"]
    #[inline(always)]
    pub fn is_adcpctl1_0(&self) -> bool {
        *self == ADCPCTL1_A::ADCPCTL1_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL1_1`"]
    #[inline(always)]
    pub fn is_adcpctl1_1(&self) -> bool {
        *self == ADCPCTL1_A::ADCPCTL1_1
    }
}
#[doc = "Write proxy for field `ADCPCTL1`"]
pub struct ADCPCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A1 disabled"]
    #[inline(always)]
    pub fn adcpctl1_0(self) -> &'a mut W {
        self.variant(ADCPCTL1_A::ADCPCTL1_0)
    }
    #[doc = "ADC input A1 enabled"]
    #[inline(always)]
    pub fn adcpctl1_1(self) -> &'a mut W {
        self.variant(ADCPCTL1_A::ADCPCTL1_1)
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
#[doc = "ADC input A2 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL2_A {
    #[doc = "0: ADC input A2 disabled"]
    ADCPCTL2_0 = 0,
    #[doc = "1: ADC input A2 enabled"]
    ADCPCTL2_1 = 1,
}
impl From<ADCPCTL2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL2`"]
pub type ADCPCTL2_R = crate::R<bool, ADCPCTL2_A>;
impl ADCPCTL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL2_A {
        match self.bits {
            false => ADCPCTL2_A::ADCPCTL2_0,
            true => ADCPCTL2_A::ADCPCTL2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL2_0`"]
    #[inline(always)]
    pub fn is_adcpctl2_0(&self) -> bool {
        *self == ADCPCTL2_A::ADCPCTL2_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL2_1`"]
    #[inline(always)]
    pub fn is_adcpctl2_1(&self) -> bool {
        *self == ADCPCTL2_A::ADCPCTL2_1
    }
}
#[doc = "Write proxy for field `ADCPCTL2`"]
pub struct ADCPCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A2 disabled"]
    #[inline(always)]
    pub fn adcpctl2_0(self) -> &'a mut W {
        self.variant(ADCPCTL2_A::ADCPCTL2_0)
    }
    #[doc = "ADC input A2 enabled"]
    #[inline(always)]
    pub fn adcpctl2_1(self) -> &'a mut W {
        self.variant(ADCPCTL2_A::ADCPCTL2_1)
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
#[doc = "ADC input A3 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL3_A {
    #[doc = "0: ADC input A3 disabled"]
    ADCPCTL3_0 = 0,
    #[doc = "1: ADC input A3 enabled"]
    ADCPCTL3_1 = 1,
}
impl From<ADCPCTL3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL3`"]
pub type ADCPCTL3_R = crate::R<bool, ADCPCTL3_A>;
impl ADCPCTL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL3_A {
        match self.bits {
            false => ADCPCTL3_A::ADCPCTL3_0,
            true => ADCPCTL3_A::ADCPCTL3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL3_0`"]
    #[inline(always)]
    pub fn is_adcpctl3_0(&self) -> bool {
        *self == ADCPCTL3_A::ADCPCTL3_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL3_1`"]
    #[inline(always)]
    pub fn is_adcpctl3_1(&self) -> bool {
        *self == ADCPCTL3_A::ADCPCTL3_1
    }
}
#[doc = "Write proxy for field `ADCPCTL3`"]
pub struct ADCPCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A3 disabled"]
    #[inline(always)]
    pub fn adcpctl3_0(self) -> &'a mut W {
        self.variant(ADCPCTL3_A::ADCPCTL3_0)
    }
    #[doc = "ADC input A3 enabled"]
    #[inline(always)]
    pub fn adcpctl3_1(self) -> &'a mut W {
        self.variant(ADCPCTL3_A::ADCPCTL3_1)
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
#[doc = "ADC input A4 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL4_A {
    #[doc = "0: ADC input A4 disabled"]
    ADCPCTL4_0 = 0,
    #[doc = "1: ADC input A4 enabled"]
    ADCPCTL4_1 = 1,
}
impl From<ADCPCTL4_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL4`"]
pub type ADCPCTL4_R = crate::R<bool, ADCPCTL4_A>;
impl ADCPCTL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL4_A {
        match self.bits {
            false => ADCPCTL4_A::ADCPCTL4_0,
            true => ADCPCTL4_A::ADCPCTL4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL4_0`"]
    #[inline(always)]
    pub fn is_adcpctl4_0(&self) -> bool {
        *self == ADCPCTL4_A::ADCPCTL4_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL4_1`"]
    #[inline(always)]
    pub fn is_adcpctl4_1(&self) -> bool {
        *self == ADCPCTL4_A::ADCPCTL4_1
    }
}
#[doc = "Write proxy for field `ADCPCTL4`"]
pub struct ADCPCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A4 disabled"]
    #[inline(always)]
    pub fn adcpctl4_0(self) -> &'a mut W {
        self.variant(ADCPCTL4_A::ADCPCTL4_0)
    }
    #[doc = "ADC input A4 enabled"]
    #[inline(always)]
    pub fn adcpctl4_1(self) -> &'a mut W {
        self.variant(ADCPCTL4_A::ADCPCTL4_1)
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
#[doc = "ADC input A5 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL5_A {
    #[doc = "0: ADC input A5 disabled"]
    ADCPCTL5_0 = 0,
    #[doc = "1: ADC input A5 enabled"]
    ADCPCTL5_1 = 1,
}
impl From<ADCPCTL5_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL5`"]
pub type ADCPCTL5_R = crate::R<bool, ADCPCTL5_A>;
impl ADCPCTL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL5_A {
        match self.bits {
            false => ADCPCTL5_A::ADCPCTL5_0,
            true => ADCPCTL5_A::ADCPCTL5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL5_0`"]
    #[inline(always)]
    pub fn is_adcpctl5_0(&self) -> bool {
        *self == ADCPCTL5_A::ADCPCTL5_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL5_1`"]
    #[inline(always)]
    pub fn is_adcpctl5_1(&self) -> bool {
        *self == ADCPCTL5_A::ADCPCTL5_1
    }
}
#[doc = "Write proxy for field `ADCPCTL5`"]
pub struct ADCPCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A5 disabled"]
    #[inline(always)]
    pub fn adcpctl5_0(self) -> &'a mut W {
        self.variant(ADCPCTL5_A::ADCPCTL5_0)
    }
    #[doc = "ADC input A5 enabled"]
    #[inline(always)]
    pub fn adcpctl5_1(self) -> &'a mut W {
        self.variant(ADCPCTL5_A::ADCPCTL5_1)
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
#[doc = "ADC input A6 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL6_A {
    #[doc = "0: ADC input A6 disabled"]
    ADCPCTL6_0 = 0,
    #[doc = "1: ADC input A6 enabled"]
    ADCPCTL6_1 = 1,
}
impl From<ADCPCTL6_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL6`"]
pub type ADCPCTL6_R = crate::R<bool, ADCPCTL6_A>;
impl ADCPCTL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL6_A {
        match self.bits {
            false => ADCPCTL6_A::ADCPCTL6_0,
            true => ADCPCTL6_A::ADCPCTL6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL6_0`"]
    #[inline(always)]
    pub fn is_adcpctl6_0(&self) -> bool {
        *self == ADCPCTL6_A::ADCPCTL6_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL6_1`"]
    #[inline(always)]
    pub fn is_adcpctl6_1(&self) -> bool {
        *self == ADCPCTL6_A::ADCPCTL6_1
    }
}
#[doc = "Write proxy for field `ADCPCTL6`"]
pub struct ADCPCTL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A6 disabled"]
    #[inline(always)]
    pub fn adcpctl6_0(self) -> &'a mut W {
        self.variant(ADCPCTL6_A::ADCPCTL6_0)
    }
    #[doc = "ADC input A6 enabled"]
    #[inline(always)]
    pub fn adcpctl6_1(self) -> &'a mut W {
        self.variant(ADCPCTL6_A::ADCPCTL6_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "ADC input A7 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL7_A {
    #[doc = "0: ADC input A7 disabled"]
    ADCPCTL7_0 = 0,
    #[doc = "1: ADC input A7 enabled"]
    ADCPCTL7_1 = 1,
}
impl From<ADCPCTL7_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL7`"]
pub type ADCPCTL7_R = crate::R<bool, ADCPCTL7_A>;
impl ADCPCTL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL7_A {
        match self.bits {
            false => ADCPCTL7_A::ADCPCTL7_0,
            true => ADCPCTL7_A::ADCPCTL7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL7_0`"]
    #[inline(always)]
    pub fn is_adcpctl7_0(&self) -> bool {
        *self == ADCPCTL7_A::ADCPCTL7_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL7_1`"]
    #[inline(always)]
    pub fn is_adcpctl7_1(&self) -> bool {
        *self == ADCPCTL7_A::ADCPCTL7_1
    }
}
#[doc = "Write proxy for field `ADCPCTL7`"]
pub struct ADCPCTL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A7 disabled"]
    #[inline(always)]
    pub fn adcpctl7_0(self) -> &'a mut W {
        self.variant(ADCPCTL7_A::ADCPCTL7_0)
    }
    #[doc = "ADC input A7 enabled"]
    #[inline(always)]
    pub fn adcpctl7_1(self) -> &'a mut W {
        self.variant(ADCPCTL7_A::ADCPCTL7_1)
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
#[doc = "ADC input A8 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL8_A {
    #[doc = "0: ADC input A8 disabled"]
    ADCPCTL8_0 = 0,
    #[doc = "1: ADC input A8 enabled"]
    ADCPCTL8_1 = 1,
}
impl From<ADCPCTL8_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL8`"]
pub type ADCPCTL8_R = crate::R<bool, ADCPCTL8_A>;
impl ADCPCTL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL8_A {
        match self.bits {
            false => ADCPCTL8_A::ADCPCTL8_0,
            true => ADCPCTL8_A::ADCPCTL8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL8_0`"]
    #[inline(always)]
    pub fn is_adcpctl8_0(&self) -> bool {
        *self == ADCPCTL8_A::ADCPCTL8_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL8_1`"]
    #[inline(always)]
    pub fn is_adcpctl8_1(&self) -> bool {
        *self == ADCPCTL8_A::ADCPCTL8_1
    }
}
#[doc = "Write proxy for field `ADCPCTL8`"]
pub struct ADCPCTL8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A8 disabled"]
    #[inline(always)]
    pub fn adcpctl8_0(self) -> &'a mut W {
        self.variant(ADCPCTL8_A::ADCPCTL8_0)
    }
    #[doc = "ADC input A8 enabled"]
    #[inline(always)]
    pub fn adcpctl8_1(self) -> &'a mut W {
        self.variant(ADCPCTL8_A::ADCPCTL8_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "ADC input A9 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPCTL9_A {
    #[doc = "0: ADC input A9 disabled"]
    ADCPCTL9_0 = 0,
    #[doc = "1: ADC input A9 enabled"]
    ADCPCTL9_1 = 1,
}
impl From<ADCPCTL9_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCPCTL9`"]
pub type ADCPCTL9_R = crate::R<bool, ADCPCTL9_A>;
impl ADCPCTL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL9_A {
        match self.bits {
            false => ADCPCTL9_A::ADCPCTL9_0,
            true => ADCPCTL9_A::ADCPCTL9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL9_0`"]
    #[inline(always)]
    pub fn is_adcpctl9_0(&self) -> bool {
        *self == ADCPCTL9_A::ADCPCTL9_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL9_1`"]
    #[inline(always)]
    pub fn is_adcpctl9_1(&self) -> bool {
        *self == ADCPCTL9_A::ADCPCTL9_1
    }
}
#[doc = "Write proxy for field `ADCPCTL9`"]
pub struct ADCPCTL9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPCTL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCPCTL9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC input A9 disabled"]
    #[inline(always)]
    pub fn adcpctl9_0(self) -> &'a mut W {
        self.variant(ADCPCTL9_A::ADCPCTL9_0)
    }
    #[doc = "ADC input A9 enabled"]
    #[inline(always)]
    pub fn adcpctl9_1(self) -> &'a mut W {
        self.variant(ADCPCTL9_A::ADCPCTL9_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "eUSCIB Remapping source selection , please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCIBRMP_A {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    USCIBRMP_0 = 0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    USCIBRMP_1 = 1,
}
impl From<USCIBRMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIBRMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USCIBRMP`"]
pub type USCIBRMP_R = crate::R<bool, USCIBRMP_A>;
impl USCIBRMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIBRMP_A {
        match self.bits {
            false => USCIBRMP_A::USCIBRMP_0,
            true => USCIBRMP_A::USCIBRMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIBRMP_0`"]
    #[inline(always)]
    pub fn is_uscibrmp_0(&self) -> bool {
        *self == USCIBRMP_A::USCIBRMP_0
    }
    #[doc = "Checks if the value of the field is `USCIBRMP_1`"]
    #[inline(always)]
    pub fn is_uscibrmp_1(&self) -> bool {
        *self == USCIBRMP_A::USCIBRMP_1
    }
}
#[doc = "Write proxy for field `USCIBRMP`"]
pub struct USCIBRMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USCIBRMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USCIBRMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_0(self) -> &'a mut W {
        self.variant(USCIBRMP_A::USCIBRMP_0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_1(self) -> &'a mut W {
        self.variant(USCIBRMP_A::USCIBRMP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "RTC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCKSEL_A {
    #[doc = "0: SMCLK is selected"]
    RTC_SMCLK = 0,
    #[doc = "1: ACLK is selected"]
    RTC_ACLK = 1,
}
impl From<RTCCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCCKSEL`"]
pub type RTCCKSEL_R = crate::R<bool, RTCCKSEL_A>;
impl RTCCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCKSEL_A {
        match self.bits {
            false => RTCCKSEL_A::RTC_SMCLK,
            true => RTCCKSEL_A::RTC_ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_SMCLK`"]
    #[inline(always)]
    pub fn is_rtc_smclk(&self) -> bool {
        *self == RTCCKSEL_A::RTC_SMCLK
    }
    #[doc = "Checks if the value of the field is `RTC_ACLK`"]
    #[inline(always)]
    pub fn is_rtc_aclk(&self) -> bool {
        *self == RTCCKSEL_A::RTC_ACLK
    }
}
#[doc = "Write proxy for field `RTCCKSEL`"]
pub struct RTCCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn rtc_smclk(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTC_SMCLK)
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn rtc_aclk(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTC_ACLK)
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
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> ADCPCTL0_R {
        ADCPCTL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> ADCPCTL1_R {
        ADCPCTL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> ADCPCTL2_R {
        ADCPCTL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> ADCPCTL3_R {
        ADCPCTL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> ADCPCTL4_R {
        ADCPCTL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> ADCPCTL5_R {
        ADCPCTL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> ADCPCTL6_R {
        ADCPCTL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> ADCPCTL7_R {
        ADCPCTL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&self) -> ADCPCTL8_R {
        ADCPCTL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&self) -> ADCPCTL9_R {
        ADCPCTL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&self) -> USCIBRMP_R {
        USCIBRMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RTCCKSEL_R {
        RTCCKSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> ADCPCTL0_W {
        ADCPCTL0_W { w: self }
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> ADCPCTL1_W {
        ADCPCTL1_W { w: self }
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> ADCPCTL2_W {
        ADCPCTL2_W { w: self }
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> ADCPCTL3_W {
        ADCPCTL3_W { w: self }
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> ADCPCTL4_W {
        ADCPCTL4_W { w: self }
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> ADCPCTL5_W {
        ADCPCTL5_W { w: self }
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> ADCPCTL6_W {
        ADCPCTL6_W { w: self }
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> ADCPCTL7_W {
        ADCPCTL7_W { w: self }
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&mut self) -> ADCPCTL8_W {
        ADCPCTL8_W { w: self }
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&mut self) -> ADCPCTL9_W {
        ADCPCTL9_W { w: self }
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&mut self) -> USCIBRMP_W {
        USCIBRMP_W { w: self }
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&mut self) -> RTCCKSEL_W {
        RTCCKSEL_W { w: self }
    }
}
