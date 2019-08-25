#[doc = "Reader of register UCA0IE"]
pub type R = crate::R<u16, super::UCA0IE>;
#[doc = "Writer for register UCA0IE"]
pub type W = crate::W<u16, super::UCA0IE>;
#[doc = "Register UCA0IE `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0IE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE_0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE_1,
}
impl From<UCRXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE_A) -> Self {
        match variant {
            UCRXIE_A::UCRXIE_0 => false,
            UCRXIE_A::UCRXIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIE`"]
pub type UCRXIE_R = crate::R<bool, UCRXIE_A>;
impl UCRXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE_A {
        match self.bits {
            false => UCRXIE_A::UCRXIE_0,
            true => UCRXIE_A::UCRXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE_0`"]
    #[inline(always)]
    pub fn is_ucrxie_0(&self) -> bool {
        *self == UCRXIE_A::UCRXIE_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE_1`"]
    #[inline(always)]
    pub fn is_ucrxie_1(&self) -> bool {
        *self == UCRXIE_A::UCRXIE_1
    }
}
#[doc = "Write proxy for field `UCRXIE`"]
pub struct UCRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie_0(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie_1(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_1)
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
#[doc = "Transmit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE_0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE_1,
}
impl From<UCTXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE_A) -> Self {
        match variant {
            UCTXIE_A::UCTXIE_0 => false,
            UCTXIE_A::UCTXIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIE`"]
pub type UCTXIE_R = crate::R<bool, UCTXIE_A>;
impl UCTXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE_A {
        match self.bits {
            false => UCTXIE_A::UCTXIE_0,
            true => UCTXIE_A::UCTXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE_0`"]
    #[inline(always)]
    pub fn is_uctxie_0(&self) -> bool {
        *self == UCTXIE_A::UCTXIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE_1`"]
    #[inline(always)]
    pub fn is_uctxie_1(&self) -> bool {
        *self == UCTXIE_A::UCTXIE_1
    }
}
#[doc = "Write proxy for field `UCTXIE`"]
pub struct UCTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie_0(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie_1(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_1)
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
#[doc = "Start bit interrupt enable\n\nValue on reset: 0"]
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
#[doc = "Transmit complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXCPTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXCPTIE_0,
    #[doc = "1: Interrupt enabled"]
    UCTXCPTIE_1,
}
impl From<UCTXCPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXCPTIE_A) -> Self {
        match variant {
            UCTXCPTIE_A::UCTXCPTIE_0 => false,
            UCTXCPTIE_A::UCTXCPTIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXCPTIE`"]
pub type UCTXCPTIE_R = crate::R<bool, UCTXCPTIE_A>;
impl UCTXCPTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXCPTIE_A {
        match self.bits {
            false => UCTXCPTIE_A::UCTXCPTIE_0,
            true => UCTXCPTIE_A::UCTXCPTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_0`"]
    #[inline(always)]
    pub fn is_uctxcptie_0(&self) -> bool {
        *self == UCTXCPTIE_A::UCTXCPTIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_1`"]
    #[inline(always)]
    pub fn is_uctxcptie_1(&self) -> bool {
        *self == UCTXCPTIE_A::UCTXCPTIE_1
    }
}
#[doc = "Write proxy for field `UCTXCPTIE`"]
pub struct UCTXCPTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXCPTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXCPTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxcptie_0(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxcptie_1(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_1)
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
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&self) -> UCTXCPTIE_R {
        UCTXCPTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W {
        UCRXIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W {
        UCTXIE_W { w: self }
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&mut self) -> UCTXCPTIE_W {
        UCTXCPTIE_W { w: self }
    }
}
