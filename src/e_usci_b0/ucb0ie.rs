#[doc = "Reader of register UCB0IE"]
pub type R = crate::R<u16, super::UCB0IE>;
#[doc = "Writer for register UCB0IE"]
pub type W = crate::W<u16, super::UCB0IE>;
#[doc = "Register UCB0IE `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0IE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE0_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE0_0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE0_1,
}
impl From<UCRXIE0_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE0_A) -> Self {
        match variant {
            UCRXIE0_A::UCRXIE0_0 => false,
            UCRXIE0_A::UCRXIE0_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIE0`"]
pub type UCRXIE0_R = crate::R<bool, UCRXIE0_A>;
impl UCRXIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE0_A {
        match self.bits {
            false => UCRXIE0_A::UCRXIE0_0,
            true => UCRXIE0_A::UCRXIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE0_0`"]
    #[inline(always)]
    pub fn is_ucrxie0_0(&self) -> bool {
        *self == UCRXIE0_A::UCRXIE0_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE0_1`"]
    #[inline(always)]
    pub fn is_ucrxie0_1(&self) -> bool {
        *self == UCRXIE0_A::UCRXIE0_1
    }
}
#[doc = "Write proxy for field `UCRXIE0`"]
pub struct UCRXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie0_0(self) -> &'a mut W {
        self.variant(UCRXIE0_A::UCRXIE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie0_1(self) -> &'a mut W {
        self.variant(UCRXIE0_A::UCRXIE0_1)
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
#[doc = "Transmit interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE0_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE0_0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE0_1,
}
impl From<UCTXIE0_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE0_A) -> Self {
        match variant {
            UCTXIE0_A::UCTXIE0_0 => false,
            UCTXIE0_A::UCTXIE0_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIE0`"]
pub type UCTXIE0_R = crate::R<bool, UCTXIE0_A>;
impl UCTXIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE0_A {
        match self.bits {
            false => UCTXIE0_A::UCTXIE0_0,
            true => UCTXIE0_A::UCTXIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE0_0`"]
    #[inline(always)]
    pub fn is_uctxie0_0(&self) -> bool {
        *self == UCTXIE0_A::UCTXIE0_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE0_1`"]
    #[inline(always)]
    pub fn is_uctxie0_1(&self) -> bool {
        *self == UCTXIE0_A::UCTXIE0_1
    }
}
#[doc = "Write proxy for field `UCTXIE0`"]
pub struct UCTXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie0_0(self) -> &'a mut W {
        self.variant(UCTXIE0_A::UCTXIE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie0_1(self) -> &'a mut W {
        self.variant(UCTXIE0_A::UCTXIE0_1)
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
#[doc = "START condition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCSTTIE_0,
    #[doc = "1: Interrupt enabled"]
    UCSTTIE_1,
}
impl From<UCSTTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIE_A) -> Self {
        match variant {
            UCSTTIE_A::UCSTTIE_0 => false,
            UCSTTIE_A::UCSTTIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCSTTIE`"]
pub type UCSTTIE_R = crate::R<bool, UCSTTIE_A>;
impl UCSTTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIE_A {
        match self.bits {
            false => UCSTTIE_A::UCSTTIE_0,
            true => UCSTTIE_A::UCSTTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_0`"]
    #[inline(always)]
    pub fn is_ucsttie_0(&self) -> bool {
        *self == UCSTTIE_A::UCSTTIE_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_1`"]
    #[inline(always)]
    pub fn is_ucsttie_1(&self) -> bool {
        *self == UCSTTIE_A::UCSTTIE_1
    }
}
#[doc = "Write proxy for field `UCSTTIE`"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucsttie_0(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucsttie_1(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_1)
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
#[doc = "STOP condition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTPIE_A {
    #[doc = "0: Interrupt disabled"]
    UCSTPIE_0,
    #[doc = "1: Interrupt enabled"]
    UCSTPIE_1,
}
impl From<UCSTPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTPIE_A) -> Self {
        match variant {
            UCSTPIE_A::UCSTPIE_0 => false,
            UCSTPIE_A::UCSTPIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCSTPIE`"]
pub type UCSTPIE_R = crate::R<bool, UCSTPIE_A>;
impl UCSTPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTPIE_A {
        match self.bits {
            false => UCSTPIE_A::UCSTPIE_0,
            true => UCSTPIE_A::UCSTPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTPIE_0`"]
    #[inline(always)]
    pub fn is_ucstpie_0(&self) -> bool {
        *self == UCSTPIE_A::UCSTPIE_0
    }
    #[doc = "Checks if the value of the field is `UCSTPIE_1`"]
    #[inline(always)]
    pub fn is_ucstpie_1(&self) -> bool {
        *self == UCSTPIE_A::UCSTPIE_1
    }
}
#[doc = "Write proxy for field `UCSTPIE`"]
pub struct UCSTPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucstpie_0(self) -> &'a mut W {
        self.variant(UCSTPIE_A::UCSTPIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucstpie_1(self) -> &'a mut W {
        self.variant(UCSTPIE_A::UCSTPIE_1)
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
#[doc = "Arbitration lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCALIE_A {
    #[doc = "0: Interrupt disabled"]
    UCALIE_0,
    #[doc = "1: Interrupt enabled"]
    UCALIE_1,
}
impl From<UCALIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCALIE_A) -> Self {
        match variant {
            UCALIE_A::UCALIE_0 => false,
            UCALIE_A::UCALIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCALIE`"]
pub type UCALIE_R = crate::R<bool, UCALIE_A>;
impl UCALIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCALIE_A {
        match self.bits {
            false => UCALIE_A::UCALIE_0,
            true => UCALIE_A::UCALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCALIE_0`"]
    #[inline(always)]
    pub fn is_ucalie_0(&self) -> bool {
        *self == UCALIE_A::UCALIE_0
    }
    #[doc = "Checks if the value of the field is `UCALIE_1`"]
    #[inline(always)]
    pub fn is_ucalie_1(&self) -> bool {
        *self == UCALIE_A::UCALIE_1
    }
}
#[doc = "Write proxy for field `UCALIE`"]
pub struct UCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCALIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucalie_0(self) -> &'a mut W {
        self.variant(UCALIE_A::UCALIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucalie_1(self) -> &'a mut W {
        self.variant(UCALIE_A::UCALIE_1)
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
#[doc = "Not-acknowledge interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCNACKIE_A {
    #[doc = "0: Interrupt disabled"]
    UCNACKIE_0,
    #[doc = "1: Interrupt enabled"]
    UCNACKIE_1,
}
impl From<UCNACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCNACKIE_A) -> Self {
        match variant {
            UCNACKIE_A::UCNACKIE_0 => false,
            UCNACKIE_A::UCNACKIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCNACKIE`"]
pub type UCNACKIE_R = crate::R<bool, UCNACKIE_A>;
impl UCNACKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCNACKIE_A {
        match self.bits {
            false => UCNACKIE_A::UCNACKIE_0,
            true => UCNACKIE_A::UCNACKIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCNACKIE_0`"]
    #[inline(always)]
    pub fn is_ucnackie_0(&self) -> bool {
        *self == UCNACKIE_A::UCNACKIE_0
    }
    #[doc = "Checks if the value of the field is `UCNACKIE_1`"]
    #[inline(always)]
    pub fn is_ucnackie_1(&self) -> bool {
        *self == UCNACKIE_A::UCNACKIE_1
    }
}
#[doc = "Write proxy for field `UCNACKIE`"]
pub struct UCNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCNACKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucnackie_0(self) -> &'a mut W {
        self.variant(UCNACKIE_A::UCNACKIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucnackie_1(self) -> &'a mut W {
        self.variant(UCNACKIE_A::UCNACKIE_1)
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
#[doc = "Byte counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCBCNTIE_0,
    #[doc = "1: Interrupt enabled"]
    UCBCNTIE_1,
}
impl From<UCBCNTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBCNTIE_A) -> Self {
        match variant {
            UCBCNTIE_A::UCBCNTIE_0 => false,
            UCBCNTIE_A::UCBCNTIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCBCNTIE`"]
pub type UCBCNTIE_R = crate::R<bool, UCBCNTIE_A>;
impl UCBCNTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBCNTIE_A {
        match self.bits {
            false => UCBCNTIE_A::UCBCNTIE_0,
            true => UCBCNTIE_A::UCBCNTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBCNTIE_0`"]
    #[inline(always)]
    pub fn is_ucbcntie_0(&self) -> bool {
        *self == UCBCNTIE_A::UCBCNTIE_0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIE_1`"]
    #[inline(always)]
    pub fn is_ucbcntie_1(&self) -> bool {
        *self == UCBCNTIE_A::UCBCNTIE_1
    }
}
#[doc = "Write proxy for field `UCBCNTIE`"]
pub struct UCBCNTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBCNTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbcntie_0(self) -> &'a mut W {
        self.variant(UCBCNTIE_A::UCBCNTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbcntie_1(self) -> &'a mut W {
        self.variant(UCBCNTIE_A::UCBCNTIE_1)
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
#[doc = "Clock low timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCLTOIE_A {
    #[doc = "0: Interrupt disabled"]
    UCCLTOIE_0,
    #[doc = "1: Interrupt enabled"]
    UCCLTOIE_1,
}
impl From<UCCLTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCCLTOIE_A) -> Self {
        match variant {
            UCCLTOIE_A::UCCLTOIE_0 => false,
            UCCLTOIE_A::UCCLTOIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCCLTOIE`"]
pub type UCCLTOIE_R = crate::R<bool, UCCLTOIE_A>;
impl UCCLTOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCLTOIE_A {
        match self.bits {
            false => UCCLTOIE_A::UCCLTOIE_0,
            true => UCCLTOIE_A::UCCLTOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTOIE_0`"]
    #[inline(always)]
    pub fn is_uccltoie_0(&self) -> bool {
        *self == UCCLTOIE_A::UCCLTOIE_0
    }
    #[doc = "Checks if the value of the field is `UCCLTOIE_1`"]
    #[inline(always)]
    pub fn is_uccltoie_1(&self) -> bool {
        *self == UCCLTOIE_A::UCCLTOIE_1
    }
}
#[doc = "Write proxy for field `UCCLTOIE`"]
pub struct UCCLTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCCLTOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uccltoie_0(self) -> &'a mut W {
        self.variant(UCCLTOIE_A::UCCLTOIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uccltoie_1(self) -> &'a mut W {
        self.variant(UCCLTOIE_A::UCCLTOIE_1)
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
#[doc = "Receive interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE1_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE1_0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE1_1,
}
impl From<UCRXIE1_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE1_A) -> Self {
        match variant {
            UCRXIE1_A::UCRXIE1_0 => false,
            UCRXIE1_A::UCRXIE1_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIE1`"]
pub type UCRXIE1_R = crate::R<bool, UCRXIE1_A>;
impl UCRXIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE1_A {
        match self.bits {
            false => UCRXIE1_A::UCRXIE1_0,
            true => UCRXIE1_A::UCRXIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE1_0`"]
    #[inline(always)]
    pub fn is_ucrxie1_0(&self) -> bool {
        *self == UCRXIE1_A::UCRXIE1_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE1_1`"]
    #[inline(always)]
    pub fn is_ucrxie1_1(&self) -> bool {
        *self == UCRXIE1_A::UCRXIE1_1
    }
}
#[doc = "Write proxy for field `UCRXIE1`"]
pub struct UCRXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie1_0(self) -> &'a mut W {
        self.variant(UCRXIE1_A::UCRXIE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie1_1(self) -> &'a mut W {
        self.variant(UCRXIE1_A::UCRXIE1_1)
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
#[doc = "Transmit interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE1_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE1_0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE1_1,
}
impl From<UCTXIE1_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE1_A) -> Self {
        match variant {
            UCTXIE1_A::UCTXIE1_0 => false,
            UCTXIE1_A::UCTXIE1_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIE1`"]
pub type UCTXIE1_R = crate::R<bool, UCTXIE1_A>;
impl UCTXIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE1_A {
        match self.bits {
            false => UCTXIE1_A::UCTXIE1_0,
            true => UCTXIE1_A::UCTXIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE1_0`"]
    #[inline(always)]
    pub fn is_uctxie1_0(&self) -> bool {
        *self == UCTXIE1_A::UCTXIE1_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE1_1`"]
    #[inline(always)]
    pub fn is_uctxie1_1(&self) -> bool {
        *self == UCTXIE1_A::UCTXIE1_1
    }
}
#[doc = "Write proxy for field `UCTXIE1`"]
pub struct UCTXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie1_0(self) -> &'a mut W {
        self.variant(UCTXIE1_A::UCTXIE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie1_1(self) -> &'a mut W {
        self.variant(UCTXIE1_A::UCTXIE1_1)
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
#[doc = "Receive interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE2_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE2_0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE2_1,
}
impl From<UCRXIE2_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE2_A) -> Self {
        match variant {
            UCRXIE2_A::UCRXIE2_0 => false,
            UCRXIE2_A::UCRXIE2_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIE2`"]
pub type UCRXIE2_R = crate::R<bool, UCRXIE2_A>;
impl UCRXIE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE2_A {
        match self.bits {
            false => UCRXIE2_A::UCRXIE2_0,
            true => UCRXIE2_A::UCRXIE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE2_0`"]
    #[inline(always)]
    pub fn is_ucrxie2_0(&self) -> bool {
        *self == UCRXIE2_A::UCRXIE2_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE2_1`"]
    #[inline(always)]
    pub fn is_ucrxie2_1(&self) -> bool {
        *self == UCRXIE2_A::UCRXIE2_1
    }
}
#[doc = "Write proxy for field `UCRXIE2`"]
pub struct UCRXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie2_0(self) -> &'a mut W {
        self.variant(UCRXIE2_A::UCRXIE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie2_1(self) -> &'a mut W {
        self.variant(UCRXIE2_A::UCRXIE2_1)
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
#[doc = "Transmit interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE2_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE2_0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE2_1,
}
impl From<UCTXIE2_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE2_A) -> Self {
        match variant {
            UCTXIE2_A::UCTXIE2_0 => false,
            UCTXIE2_A::UCTXIE2_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIE2`"]
pub type UCTXIE2_R = crate::R<bool, UCTXIE2_A>;
impl UCTXIE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE2_A {
        match self.bits {
            false => UCTXIE2_A::UCTXIE2_0,
            true => UCTXIE2_A::UCTXIE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE2_0`"]
    #[inline(always)]
    pub fn is_uctxie2_0(&self) -> bool {
        *self == UCTXIE2_A::UCTXIE2_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE2_1`"]
    #[inline(always)]
    pub fn is_uctxie2_1(&self) -> bool {
        *self == UCTXIE2_A::UCTXIE2_1
    }
}
#[doc = "Write proxy for field `UCTXIE2`"]
pub struct UCTXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie2_0(self) -> &'a mut W {
        self.variant(UCTXIE2_A::UCTXIE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie2_1(self) -> &'a mut W {
        self.variant(UCTXIE2_A::UCTXIE2_1)
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
#[doc = "Receive interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE3_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE3_0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE3_1,
}
impl From<UCRXIE3_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE3_A) -> Self {
        match variant {
            UCRXIE3_A::UCRXIE3_0 => false,
            UCRXIE3_A::UCRXIE3_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIE3`"]
pub type UCRXIE3_R = crate::R<bool, UCRXIE3_A>;
impl UCRXIE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE3_A {
        match self.bits {
            false => UCRXIE3_A::UCRXIE3_0,
            true => UCRXIE3_A::UCRXIE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE3_0`"]
    #[inline(always)]
    pub fn is_ucrxie3_0(&self) -> bool {
        *self == UCRXIE3_A::UCRXIE3_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE3_1`"]
    #[inline(always)]
    pub fn is_ucrxie3_1(&self) -> bool {
        *self == UCRXIE3_A::UCRXIE3_1
    }
}
#[doc = "Write proxy for field `UCRXIE3`"]
pub struct UCRXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie3_0(self) -> &'a mut W {
        self.variant(UCRXIE3_A::UCRXIE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie3_1(self) -> &'a mut W {
        self.variant(UCRXIE3_A::UCRXIE3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Transmit interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE3_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE3_0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE3_1,
}
impl From<UCTXIE3_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE3_A) -> Self {
        match variant {
            UCTXIE3_A::UCTXIE3_0 => false,
            UCTXIE3_A::UCTXIE3_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIE3`"]
pub type UCTXIE3_R = crate::R<bool, UCTXIE3_A>;
impl UCTXIE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE3_A {
        match self.bits {
            false => UCTXIE3_A::UCTXIE3_0,
            true => UCTXIE3_A::UCTXIE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE3_0`"]
    #[inline(always)]
    pub fn is_uctxie3_0(&self) -> bool {
        *self == UCTXIE3_A::UCTXIE3_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE3_1`"]
    #[inline(always)]
    pub fn is_uctxie3_1(&self) -> bool {
        *self == UCTXIE3_A::UCTXIE3_1
    }
}
#[doc = "Write proxy for field `UCTXIE3`"]
pub struct UCTXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie3_0(self) -> &'a mut W {
        self.variant(UCTXIE3_A::UCTXIE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie3_1(self) -> &'a mut W {
        self.variant(UCTXIE3_A::UCTXIE3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Bit position 9 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBIT9IE_A {
    #[doc = "0: Interrupt disabled"]
    UCBIT9IE_0,
    #[doc = "1: Interrupt enabled"]
    UCBIT9IE_1,
}
impl From<UCBIT9IE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBIT9IE_A) -> Self {
        match variant {
            UCBIT9IE_A::UCBIT9IE_0 => false,
            UCBIT9IE_A::UCBIT9IE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCBIT9IE`"]
pub type UCBIT9IE_R = crate::R<bool, UCBIT9IE_A>;
impl UCBIT9IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBIT9IE_A {
        match self.bits {
            false => UCBIT9IE_A::UCBIT9IE_0,
            true => UCBIT9IE_A::UCBIT9IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBIT9IE_0`"]
    #[inline(always)]
    pub fn is_ucbit9ie_0(&self) -> bool {
        *self == UCBIT9IE_A::UCBIT9IE_0
    }
    #[doc = "Checks if the value of the field is `UCBIT9IE_1`"]
    #[inline(always)]
    pub fn is_ucbit9ie_1(&self) -> bool {
        *self == UCBIT9IE_A::UCBIT9IE_1
    }
}
#[doc = "Write proxy for field `UCBIT9IE`"]
pub struct UCBIT9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBIT9IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBIT9IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbit9ie_0(self) -> &'a mut W {
        self.variant(UCBIT9IE_A::UCBIT9IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbit9ie_1(self) -> &'a mut W {
        self.variant(UCBIT9IE_A::UCBIT9IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W {
        UCRXIE0_W { w: self }
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> UCTXIE0_W {
        UCTXIE0_W { w: self }
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W { w: self }
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W { w: self }
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W {
        UCBCNTIE_W { w: self }
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W {
        UCCLTOIE_W { w: self }
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W {
        UCRXIE1_W { w: self }
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> UCTXIE1_W {
        UCTXIE1_W { w: self }
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W {
        UCRXIE2_W { w: self }
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> UCTXIE2_W {
        UCTXIE2_W { w: self }
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W {
        UCRXIE3_W { w: self }
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> UCTXIE3_W {
        UCTXIE3_W { w: self }
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W {
        UCBIT9IE_W { w: self }
    }
}
