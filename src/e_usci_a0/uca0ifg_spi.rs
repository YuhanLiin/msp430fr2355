#[doc = "Reader of register UCA0IFG_SPI"]
pub type R = crate::R<u16, super::UCA0IFG_SPI>;
#[doc = "Writer for register UCA0IFG_SPI"]
pub type W = crate::W<u16, super::UCA0IFG_SPI>;
#[doc = "Register UCA0IFG_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0IFG_SPI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG_0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG_1,
}
impl From<UCRXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG_A) -> Self {
        match variant {
            UCRXIFG_A::UCRXIFG_0 => false,
            UCRXIFG_A::UCRXIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXIFG`"]
pub type UCRXIFG_R = crate::R<bool, UCRXIFG_A>;
impl UCRXIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG_A {
        match self.bits {
            false => UCRXIFG_A::UCRXIFG_0,
            true => UCRXIFG_A::UCRXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_0`"]
    #[inline(always)]
    pub fn is_ucrxifg_0(&self) -> bool {
        *self == UCRXIFG_A::UCRXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_1`"]
    #[inline(always)]
    pub fn is_ucrxifg_1(&self) -> bool {
        *self == UCRXIFG_A::UCRXIFG_1
    }
}
#[doc = "Write proxy for field `UCRXIFG`"]
pub struct UCRXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_0(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_1(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_1)
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
#[doc = "Transmit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG_0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG_1,
}
impl From<UCTXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG_A) -> Self {
        match variant {
            UCTXIFG_A::UCTXIFG_0 => false,
            UCTXIFG_A::UCTXIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXIFG`"]
pub type UCTXIFG_R = crate::R<bool, UCTXIFG_A>;
impl UCTXIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG_A {
        match self.bits {
            false => UCTXIFG_A::UCTXIFG_0,
            true => UCTXIFG_A::UCTXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_0`"]
    #[inline(always)]
    pub fn is_uctxifg_0(&self) -> bool {
        *self == UCTXIFG_A::UCTXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_1`"]
    #[inline(always)]
    pub fn is_uctxifg_1(&self) -> bool {
        *self == UCTXIFG_A::UCTXIFG_1
    }
}
#[doc = "Write proxy for field `UCTXIFG`"]
pub struct UCTXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_0(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_1(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_1)
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
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W { w: self }
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W { w: self }
    }
}
