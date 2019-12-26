#[doc = "Reader of register CP1DACCTL"]
pub type R = crate::R<u16, super::CP1DACCTL>;
#[doc = "Writer for register CP1DACCTL"]
pub type W = crate::W<u16, super::CP1DACCTL>;
#[doc = "Register CP1DACCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CP1DACCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This bit is only valid when CPDACBUFS is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPDACSW_A {
    #[doc = "0: CPDACBUF1 selected"]
    CPDACSW_0 = 0,
    #[doc = "1: CPDACBUF2 selected"]
    CPDACSW_1 = 1,
}
impl From<CPDACSW_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPDACSW`"]
pub type CPDACSW_R = crate::R<bool, CPDACSW_A>;
impl CPDACSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACSW_A {
        match self.bits {
            false => CPDACSW_A::CPDACSW_0,
            true => CPDACSW_A::CPDACSW_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACSW_0`"]
    #[inline(always)]
    pub fn is_cpdacsw_0(&self) -> bool {
        *self == CPDACSW_A::CPDACSW_0
    }
    #[doc = "Checks if the value of the field is `CPDACSW_1`"]
    #[inline(always)]
    pub fn is_cpdacsw_1(&self) -> bool {
        *self == CPDACSW_A::CPDACSW_1
    }
}
#[doc = "Write proxy for field `CPDACSW`"]
pub struct CPDACSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CPDACSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPDACSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPDACBUF1 selected"]
    #[inline(always)]
    pub fn cpdacsw_0(self) -> &'a mut W {
        self.variant(CPDACSW_A::CPDACSW_0)
    }
    #[doc = "CPDACBUF2 selected"]
    #[inline(always)]
    pub fn cpdacsw_1(self) -> &'a mut W {
        self.variant(CPDACSW_A::CPDACSW_1)
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
#[doc = "Comparator built-in DAC buffer controlled source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPDACBUFS_A {
    #[doc = "0: Comparator output is selected as the buffer control source"]
    CPDACBUFS_0 = 0,
    #[doc = "1: CPDACSW bit is selected as the buffer control source"]
    CPDACBUFS_1 = 1,
}
impl From<CPDACBUFS_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACBUFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPDACBUFS`"]
pub type CPDACBUFS_R = crate::R<bool, CPDACBUFS_A>;
impl CPDACBUFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACBUFS_A {
        match self.bits {
            false => CPDACBUFS_A::CPDACBUFS_0,
            true => CPDACBUFS_A::CPDACBUFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACBUFS_0`"]
    #[inline(always)]
    pub fn is_cpdacbufs_0(&self) -> bool {
        *self == CPDACBUFS_A::CPDACBUFS_0
    }
    #[doc = "Checks if the value of the field is `CPDACBUFS_1`"]
    #[inline(always)]
    pub fn is_cpdacbufs_1(&self) -> bool {
        *self == CPDACBUFS_A::CPDACBUFS_1
    }
}
#[doc = "Write proxy for field `CPDACBUFS`"]
pub struct CPDACBUFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPDACBUFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPDACBUFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator output is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_0(self) -> &'a mut W {
        self.variant(CPDACBUFS_A::CPDACBUFS_0)
    }
    #[doc = "CPDACSW bit is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_1(self) -> &'a mut W {
        self.variant(CPDACBUFS_A::CPDACBUFS_1)
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
#[doc = "Comparator built-in DAC reference voltage selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPDACREFS_A {
    #[doc = "0: VDD selected"]
    CPDACREFS_0 = 0,
    #[doc = "1: on-chip VREF selected"]
    CPDACREFS_1 = 1,
}
impl From<CPDACREFS_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACREFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPDACREFS`"]
pub type CPDACREFS_R = crate::R<bool, CPDACREFS_A>;
impl CPDACREFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACREFS_A {
        match self.bits {
            false => CPDACREFS_A::CPDACREFS_0,
            true => CPDACREFS_A::CPDACREFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACREFS_0`"]
    #[inline(always)]
    pub fn is_cpdacrefs_0(&self) -> bool {
        *self == CPDACREFS_A::CPDACREFS_0
    }
    #[doc = "Checks if the value of the field is `CPDACREFS_1`"]
    #[inline(always)]
    pub fn is_cpdacrefs_1(&self) -> bool {
        *self == CPDACREFS_A::CPDACREFS_1
    }
}
#[doc = "Write proxy for field `CPDACREFS`"]
pub struct CPDACREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPDACREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPDACREFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VDD selected"]
    #[inline(always)]
    pub fn cpdacrefs_0(self) -> &'a mut W {
        self.variant(CPDACREFS_A::CPDACREFS_0)
    }
    #[doc = "on-chip VREF selected"]
    #[inline(always)]
    pub fn cpdacrefs_1(self) -> &'a mut W {
        self.variant(CPDACREFS_A::CPDACREFS_1)
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
#[doc = "Comparator built-in DAC output control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPDACEN_A {
    #[doc = "0: DAC output is disabled."]
    CPDACEN_0 = 0,
    #[doc = "1: DAC output is enabled."]
    CPDACEN_1 = 1,
}
impl From<CPDACEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPDACEN`"]
pub type CPDACEN_R = crate::R<bool, CPDACEN_A>;
impl CPDACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACEN_A {
        match self.bits {
            false => CPDACEN_A::CPDACEN_0,
            true => CPDACEN_A::CPDACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACEN_0`"]
    #[inline(always)]
    pub fn is_cpdacen_0(&self) -> bool {
        *self == CPDACEN_A::CPDACEN_0
    }
    #[doc = "Checks if the value of the field is `CPDACEN_1`"]
    #[inline(always)]
    pub fn is_cpdacen_1(&self) -> bool {
        *self == CPDACEN_A::CPDACEN_1
    }
}
#[doc = "Write proxy for field `CPDACEN`"]
pub struct CPDACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPDACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPDACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC output is disabled."]
    #[inline(always)]
    pub fn cpdacen_0(self) -> &'a mut W {
        self.variant(CPDACEN_A::CPDACEN_0)
    }
    #[doc = "DAC output is enabled."]
    #[inline(always)]
    pub fn cpdacen_1(self) -> &'a mut W {
        self.variant(CPDACEN_A::CPDACEN_1)
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
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&self) -> CPDACSW_R {
        CPDACSW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&self) -> CPDACBUFS_R {
        CPDACBUFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&self) -> CPDACREFS_R {
        CPDACREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&self) -> CPDACEN_R {
        CPDACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&mut self) -> CPDACSW_W {
        CPDACSW_W { w: self }
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&mut self) -> CPDACBUFS_W {
        CPDACBUFS_W { w: self }
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&mut self) -> CPDACREFS_W {
        CPDACREFS_W { w: self }
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&mut self) -> CPDACEN_W {
        CPDACEN_W { w: self }
    }
}
