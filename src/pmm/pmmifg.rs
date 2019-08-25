#[doc = "Reader of register PMMIFG"]
pub type R = crate::R<u16, super::PMMIFG>;
#[doc = "Writer for register PMMIFG"]
pub type W = crate::W<u16, super::PMMIFG>;
#[doc = "Register PMMIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMIFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PMM software brownout reset interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMBORIFG_A {
    #[doc = "0: Reset not due to PMMSWBOR"]
    PMMBORIFG_0,
    #[doc = "1: Reset due to PMMSWBOR"]
    PMMBORIFG_1,
}
impl From<PMMBORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMBORIFG_A) -> Self {
        match variant {
            PMMBORIFG_A::PMMBORIFG_0 => false,
            PMMBORIFG_A::PMMBORIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `PMMBORIFG`"]
pub type PMMBORIFG_R = crate::R<bool, PMMBORIFG_A>;
impl PMMBORIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMBORIFG_A {
        match self.bits {
            false => PMMBORIFG_A::PMMBORIFG_0,
            true => PMMBORIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMBORIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMBORIFG_A::PMMBORIFG_1
    }
}
#[doc = "Write proxy for field `PMMBORIFG`"]
pub struct PMMBORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMBORIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMBORIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMBORIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMBORIFG_A::PMMBORIFG_1)
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
#[doc = "PMM reset pin interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMRSTIFG_A {
    #[doc = "0: Reset not due to reset pin"]
    PMMBORIFG_0,
    #[doc = "1: Reset due to reset pin"]
    PMMBORIFG_1,
}
impl From<PMMRSTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMRSTIFG_A) -> Self {
        match variant {
            PMMRSTIFG_A::PMMBORIFG_0 => false,
            PMMRSTIFG_A::PMMBORIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `PMMRSTIFG`"]
pub type PMMRSTIFG_R = crate::R<bool, PMMRSTIFG_A>;
impl PMMRSTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMRSTIFG_A {
        match self.bits {
            false => PMMRSTIFG_A::PMMBORIFG_0,
            true => PMMRSTIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMRSTIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMRSTIFG_A::PMMBORIFG_1
    }
}
#[doc = "Write proxy for field `PMMRSTIFG`"]
pub struct PMMRSTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMRSTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMRSTIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMRSTIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMRSTIFG_A::PMMBORIFG_1)
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
#[doc = "PMM software POR interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMPORIFG_A {
    #[doc = "0: Reset not due to PMMSWPOR"]
    PMMBORIFG_0,
    #[doc = "1: Reset due to PMMSWPOR"]
    PMMBORIFG_1,
}
impl From<PMMPORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMPORIFG_A) -> Self {
        match variant {
            PMMPORIFG_A::PMMBORIFG_0 => false,
            PMMPORIFG_A::PMMBORIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `PMMPORIFG`"]
pub type PMMPORIFG_R = crate::R<bool, PMMPORIFG_A>;
impl PMMPORIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMPORIFG_A {
        match self.bits {
            false => PMMPORIFG_A::PMMBORIFG_0,
            true => PMMPORIFG_A::PMMBORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_0`"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == PMMPORIFG_A::PMMBORIFG_0
    }
    #[doc = "Checks if the value of the field is `PMMBORIFG_1`"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == PMMPORIFG_A::PMMBORIFG_1
    }
}
#[doc = "Write proxy for field `PMMPORIFG`"]
pub struct PMMPORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMPORIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMPORIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut W {
        self.variant(PMMPORIFG_A::PMMBORIFG_0)
    }
    #[doc = "Reset due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut W {
        self.variant(PMMPORIFG_A::PMMBORIFG_1)
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
#[doc = "Secondary Power interrupt flag. This bit only works in multi power supply systems. When the secondary power is ready to use, this bit is set., In single power supply systems, this bit does not work.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPWRIFG_A {}
impl From<SPWRIFG_A> for bool {
    #[inline(always)]
    fn from(variant: SPWRIFG_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `SPWRIFG`"]
pub type SPWRIFG_R = crate::R<bool, SPWRIFG_A>;
impl SPWRIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SPWRIFG_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "High-side SVS interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSHIFG_A {
    #[doc = "0: Reset not due to SVSH"]
    SVSHIFG_0,
    #[doc = "1: Reset due to SVSH"]
    SVSHIFG_1,
}
impl From<SVSHIFG_A> for bool {
    #[inline(always)]
    fn from(variant: SVSHIFG_A) -> Self {
        match variant {
            SVSHIFG_A::SVSHIFG_0 => false,
            SVSHIFG_A::SVSHIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `SVSHIFG`"]
pub type SVSHIFG_R = crate::R<bool, SVSHIFG_A>;
impl SVSHIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHIFG_A {
        match self.bits {
            false => SVSHIFG_A::SVSHIFG_0,
            true => SVSHIFG_A::SVSHIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSHIFG_0`"]
    #[inline(always)]
    pub fn is_svshifg_0(&self) -> bool {
        *self == SVSHIFG_A::SVSHIFG_0
    }
    #[doc = "Checks if the value of the field is `SVSHIFG_1`"]
    #[inline(always)]
    pub fn is_svshifg_1(&self) -> bool {
        *self == SVSHIFG_A::SVSHIFG_1
    }
}
#[doc = "Write proxy for field `SVSHIFG`"]
pub struct SVSHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSHIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not due to SVSH"]
    #[inline(always)]
    pub fn svshifg_0(self) -> &'a mut W {
        self.variant(SVSHIFG_A::SVSHIFG_0)
    }
    #[doc = "Reset due to SVSH"]
    #[inline(always)]
    pub fn svshifg_1(self) -> &'a mut W {
        self.variant(SVSHIFG_A::SVSHIFG_1)
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
#[doc = "LPMx.5 flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMLPM5IFG_A {
    #[doc = "0: Reset not due to wake-up from LPMx.5"]
    PMMLPM5IFG_0,
    #[doc = "1: Reset due to wake-up from LPMx.5"]
    PMMLPM5IFG_1,
}
impl From<PMMLPM5IFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMLPM5IFG_A) -> Self {
        match variant {
            PMMLPM5IFG_A::PMMLPM5IFG_0 => false,
            PMMLPM5IFG_A::PMMLPM5IFG_1 => true,
        }
    }
}
#[doc = "Reader of field `PMMLPM5IFG`"]
pub type PMMLPM5IFG_R = crate::R<bool, PMMLPM5IFG_A>;
impl PMMLPM5IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMLPM5IFG_A {
        match self.bits {
            false => PMMLPM5IFG_A::PMMLPM5IFG_0,
            true => PMMLPM5IFG_A::PMMLPM5IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMLPM5IFG_0`"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_0(&self) -> bool {
        *self == PMMLPM5IFG_A::PMMLPM5IFG_0
    }
    #[doc = "Checks if the value of the field is `PMMLPM5IFG_1`"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_1(&self) -> bool {
        *self == PMMLPM5IFG_A::PMMLPM5IFG_1
    }
}
#[doc = "Write proxy for field `PMMLPM5IFG`"]
pub struct PMMLPM5IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMLPM5IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMMLPM5IFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_0(self) -> &'a mut W {
        self.variant(PMMLPM5IFG_A::PMMLPM5IFG_0)
    }
    #[doc = "Reset due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_1(self) -> &'a mut W {
        self.variant(PMMLPM5IFG_A::PMMLPM5IFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "PMM secondary power supply interrupt flag. Reserved for future multi power supply systems.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMMSPSIFG_A {}
impl From<PMMSPSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMMSPSIFG_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `PMMSPSIFG`"]
pub type PMMSPSIFG_R = crate::R<bool, PMMSPSIFG_A>;
impl PMMSPSIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PMMSPSIFG_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Primary Power interrupt flag. This bit only works in multi power supply systems. When the primary power is ready to use, this bit is set. In single power supply systems, this bit does not work\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPWRIFG_A {}
impl From<PPWRIFG_A> for bool {
    #[inline(always)]
    fn from(variant: PPWRIFG_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `PPWRIFG`"]
pub type PPWRIFG_R = crate::R<bool, PPWRIFG_A>;
impl PPWRIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PPWRIFG_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
impl R {
    #[doc = "Bit 8 - PMM software brownout reset interrupt flag."]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PMMBORIFG_R {
        PMMBORIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMM reset pin interrupt flag."]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PMMRSTIFG_R {
        PMMRSTIFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PMM software POR interrupt flag."]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PMMPORIFG_R {
        PMMPORIFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Secondary Power interrupt flag. This bit only works in multi power supply systems. When the secondary power is ready to use, this bit is set., In single power supply systems, this bit does not work."]
    #[inline(always)]
    pub fn spwrifg(&self) -> SPWRIFG_R {
        SPWRIFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - High-side SVS interrupt flag."]
    #[inline(always)]
    pub fn svshifg(&self) -> SVSHIFG_R {
        SVSHIFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LPMx.5 flag."]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> PMMLPM5IFG_R {
        PMMLPM5IFG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PMM secondary power supply interrupt flag. Reserved for future multi power supply systems."]
    #[inline(always)]
    pub fn pmmspsifg(&self) -> PMMSPSIFG_R {
        PMMSPSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 12 - Primary Power interrupt flag. This bit only works in multi power supply systems. When the primary power is ready to use, this bit is set. In single power supply systems, this bit does not work"]
    #[inline(always)]
    pub fn ppwrifg(&self) -> PPWRIFG_R {
        PPWRIFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PMM software brownout reset interrupt flag."]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PMMBORIFG_W {
        PMMBORIFG_W { w: self }
    }
    #[doc = "Bit 9 - PMM reset pin interrupt flag."]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PMMRSTIFG_W {
        PMMRSTIFG_W { w: self }
    }
    #[doc = "Bit 10 - PMM software POR interrupt flag."]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PMMPORIFG_W {
        PMMPORIFG_W { w: self }
    }
    #[doc = "Bit 13 - High-side SVS interrupt flag."]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SVSHIFG_W {
        SVSHIFG_W { w: self }
    }
    #[doc = "Bit 15 - LPMx.5 flag."]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> PMMLPM5IFG_W {
        PMMLPM5IFG_W { w: self }
    }
}
