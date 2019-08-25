#[doc = "Reader of register SYSCFG0"]
pub type R = crate::R<u16, super::SYSCFG0>;
#[doc = "Writer for register SYSCFG0"]
pub type W = crate::W<u16, super::SYSCFG0>;
#[doc = "Register SYSCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Program FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFWP_A {
    #[doc = "0: Program FRAM write enable"]
    PFWP_0,
    #[doc = "1: Program FRAM write protected (not writable)"]
    PFWP_1,
}
impl From<PFWP_A> for bool {
    #[inline(always)]
    fn from(variant: PFWP_A) -> Self {
        match variant {
            PFWP_A::PFWP_0 => false,
            PFWP_A::PFWP_1 => true,
        }
    }
}
#[doc = "Reader of field `PFWP`"]
pub type PFWP_R = crate::R<bool, PFWP_A>;
impl PFWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFWP_A {
        match self.bits {
            false => PFWP_A::PFWP_0,
            true => PFWP_A::PFWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFWP_0`"]
    #[inline(always)]
    pub fn is_pfwp_0(&self) -> bool {
        *self == PFWP_A::PFWP_0
    }
    #[doc = "Checks if the value of the field is `PFWP_1`"]
    #[inline(always)]
    pub fn is_pfwp_1(&self) -> bool {
        *self == PFWP_A::PFWP_1
    }
}
#[doc = "Write proxy for field `PFWP`"]
pub struct PFWP_W<'a> {
    w: &'a mut W,
}
impl<'a> PFWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Program FRAM write enable"]
    #[inline(always)]
    pub fn pfwp_0(self) -> &'a mut W {
        self.variant(PFWP_A::PFWP_0)
    }
    #[doc = "Program FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn pfwp_1(self) -> &'a mut W {
        self.variant(PFWP_A::PFWP_1)
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
#[doc = "Data FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFWP_A {
    #[doc = "0: Data FRAM write enable"]
    DFWP_0,
    #[doc = "1: Data FRAM write protected (not writable)"]
    DFWP_1,
}
impl From<DFWP_A> for bool {
    #[inline(always)]
    fn from(variant: DFWP_A) -> Self {
        match variant {
            DFWP_A::DFWP_0 => false,
            DFWP_A::DFWP_1 => true,
        }
    }
}
#[doc = "Reader of field `DFWP`"]
pub type DFWP_R = crate::R<bool, DFWP_A>;
impl DFWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFWP_A {
        match self.bits {
            false => DFWP_A::DFWP_0,
            true => DFWP_A::DFWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DFWP_0`"]
    #[inline(always)]
    pub fn is_dfwp_0(&self) -> bool {
        *self == DFWP_A::DFWP_0
    }
    #[doc = "Checks if the value of the field is `DFWP_1`"]
    #[inline(always)]
    pub fn is_dfwp_1(&self) -> bool {
        *self == DFWP_A::DFWP_1
    }
}
#[doc = "Write proxy for field `DFWP`"]
pub struct DFWP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data FRAM write enable"]
    #[inline(always)]
    pub fn dfwp_0(self) -> &'a mut W {
        self.variant(DFWP_A::DFWP_0)
    }
    #[doc = "Data FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn dfwp_1(self) -> &'a mut W {
        self.variant(DFWP_A::DFWP_1)
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
#[doc = "FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRWPPW_A {}
impl From<FRWPPW_A> for u8 {
    #[inline(always)]
    fn from(variant: FRWPPW_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `FRWPPW`"]
pub type FRWPPW_R = crate::R<u8, FRWPPW_A>;
impl FRWPPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRWPPW_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `FRWPPW`"]
pub struct FRWPPW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRWPPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRWPPW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRWPOA_A {}
impl From<FRWPOA_A> for u8 {
    #[inline(always)]
    fn from(variant: FRWPOA_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `FRWPOA`"]
pub type FRWPOA_R = crate::R<u8, FRWPOA_A>;
impl FRWPOA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRWPOA_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `FRWPOA`"]
pub struct FRWPOA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRWPOA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRWPOA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u16) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PFWP_R {
        PFWP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DFWP_R {
        DFWP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&self) -> FRWPPW_R {
        FRWPPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&self) -> FRWPOA_R {
        FRWPOA_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PFWP_W {
        PFWP_W { w: self }
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DFWP_W {
        DFWP_W { w: self }
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&mut self) -> FRWPPW_W {
        FRWPPW_W { w: self }
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&mut self) -> FRWPOA_W {
        FRWPOA_W { w: self }
    }
}
