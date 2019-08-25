#[doc = "Reader of register UCA0STATW_SPI"]
pub type R = crate::R<u16, super::UCA0STATW_SPI>;
#[doc = "Writer for register UCA0STATW_SPI"]
pub type W = crate::W<u16, super::UCA0STATW_SPI>;
#[doc = "Register UCA0STATW_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0STATW_SPI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Overrun error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOE_A {
    #[doc = "0: No error"]
    UCOE_0,
    #[doc = "1: Overrun error occurred"]
    UCOE_1,
}
impl From<UCOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCOE_A) -> Self {
        match variant {
            UCOE_A::UCOE_0 => false,
            UCOE_A::UCOE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCOE`"]
pub type UCOE_R = crate::R<bool, UCOE_A>;
impl UCOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOE_A {
        match self.bits {
            false => UCOE_A::UCOE_0,
            true => UCOE_A::UCOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOE_0`"]
    #[inline(always)]
    pub fn is_ucoe_0(&self) -> bool {
        *self == UCOE_A::UCOE_0
    }
    #[doc = "Checks if the value of the field is `UCOE_1`"]
    #[inline(always)]
    pub fn is_ucoe_1(&self) -> bool {
        *self == UCOE_A::UCOE_1
    }
}
#[doc = "Write proxy for field `UCOE`"]
pub struct UCOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucoe_0(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_0)
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn ucoe_1(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_1)
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
#[doc = "Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCFE_A {
    #[doc = "0: No error"]
    UCFE_0,
    #[doc = "1: Bus conflict occurred"]
    UCFE_1,
}
impl From<UCFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCFE_A) -> Self {
        match variant {
            UCFE_A::UCFE_0 => false,
            UCFE_A::UCFE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCFE`"]
pub type UCFE_R = crate::R<bool, UCFE_A>;
impl UCFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCFE_A {
        match self.bits {
            false => UCFE_A::UCFE_0,
            true => UCFE_A::UCFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCFE_0`"]
    #[inline(always)]
    pub fn is_ucfe_0(&self) -> bool {
        *self == UCFE_A::UCFE_0
    }
    #[doc = "Checks if the value of the field is `UCFE_1`"]
    #[inline(always)]
    pub fn is_ucfe_1(&self) -> bool {
        *self == UCFE_A::UCFE_1
    }
}
#[doc = "Write proxy for field `UCFE`"]
pub struct UCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucfe_0(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_0)
    }
    #[doc = "Bus conflict occurred"]
    #[inline(always)]
    pub fn ucfe_1(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_1)
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
#[doc = "Listen enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCLISTEN_A {
    #[doc = "0: Disabled"]
    UCLISTEN_0,
    #[doc = "1: Enabled. UCAxTXD is internally fed back to the receiver"]
    UCLISTEN_1,
}
impl From<UCLISTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCLISTEN_A) -> Self {
        match variant {
            UCLISTEN_A::UCLISTEN_0 => false,
            UCLISTEN_A::UCLISTEN_1 => true,
        }
    }
}
#[doc = "Reader of field `UCLISTEN`"]
pub type UCLISTEN_R = crate::R<bool, UCLISTEN_A>;
impl UCLISTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCLISTEN_A {
        match self.bits {
            false => UCLISTEN_A::UCLISTEN_0,
            true => UCLISTEN_A::UCLISTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_0`"]
    #[inline(always)]
    pub fn is_uclisten_0(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_0
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_1`"]
    #[inline(always)]
    pub fn is_uclisten_1(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_1
    }
}
#[doc = "Write proxy for field `UCLISTEN`"]
pub struct UCLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCLISTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCLISTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn uclisten_0(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_0)
    }
    #[doc = "Enabled. UCAxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_1)
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
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UCOE_W {
        UCOE_W { w: self }
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UCFE_W {
        UCFE_W { w: self }
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W {
        UCLISTEN_W { w: self }
    }
}
