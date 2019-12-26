#[doc = "Reader of register SAC0DAC"]
pub type R = crate::R<u16, super::SAC0DAC>;
#[doc = "Writer for register SAC0DAC"]
pub type W = crate::W<u16, super::SAC0DAC>;
#[doc = "Register SAC0DAC `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC0DAC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAC DAC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: Disabled"]
    DACEN_0 = 0,
    #[doc = "1: Enabled"]
    DACEN_1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DACEN_0,
            true => DACEN_A::DACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACEN_0`"]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        *self == DACEN_A::DACEN_0
    }
    #[doc = "Checks if the value of the field is `DACEN_1`"]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        *self == DACEN_A::DACEN_1
    }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_1)
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
#[doc = "SAC DAC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACIE_A {
    #[doc = "0: Disabled"]
    DACIE_0 = 0,
    #[doc = "1: Enabled"]
    DACIE_1 = 1,
}
impl From<DACIE_A> for bool {
    #[inline(always)]
    fn from(variant: DACIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACIE`"]
pub type DACIE_R = crate::R<bool, DACIE_A>;
impl DACIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACIE_A {
        match self.bits {
            false => DACIE_A::DACIE_0,
            true => DACIE_A::DACIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACIE_0`"]
    #[inline(always)]
    pub fn is_dacie_0(&self) -> bool {
        *self == DACIE_A::DACIE_0
    }
    #[doc = "Checks if the value of the field is `DACIE_1`"]
    #[inline(always)]
    pub fn is_dacie_1(&self) -> bool {
        *self == DACIE_A::DACIE_1
    }
}
#[doc = "Write proxy for field `DACIE`"]
pub struct DACIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacie_0(self) -> &'a mut W {
        self.variant(DACIE_A::DACIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacie_1(self) -> &'a mut W {
        self.variant(DACIE_A::DACIE_1)
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
#[doc = "SAC DAC DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACDMAE_A {
    #[doc = "0: DMA request disabled"]
    DACDMAE_0 = 0,
    #[doc = "1: DMA request enabled"]
    DACDMAE_1 = 1,
}
impl From<DACDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DACDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACDMAE`"]
pub type DACDMAE_R = crate::R<bool, DACDMAE_A>;
impl DACDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACDMAE_A {
        match self.bits {
            false => DACDMAE_A::DACDMAE_0,
            true => DACDMAE_A::DACDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACDMAE_0`"]
    #[inline(always)]
    pub fn is_dacdmae_0(&self) -> bool {
        *self == DACDMAE_A::DACDMAE_0
    }
    #[doc = "Checks if the value of the field is `DACDMAE_1`"]
    #[inline(always)]
    pub fn is_dacdmae_1(&self) -> bool {
        *self == DACDMAE_A::DACDMAE_1
    }
}
#[doc = "Write proxy for field `DACDMAE`"]
pub struct DACDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DACDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn dacdmae_0(self) -> &'a mut W {
        self.variant(DACDMAE_A::DACDMAE_0)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn dacdmae_1(self) -> &'a mut W {
        self.variant(DACDMAE_A::DACDMAE_1)
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
#[doc = "SAC DAC load select. Selects the load trigger for the DAC latch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DACLSEL_A {
    #[doc = "0: DAC latch loads when DACDAT written"]
    DACLSEL_0 = 0,
    #[doc = "2: Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    DACLSEL_2 = 2,
    #[doc = "3: Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    DACLSEL_3 = 3,
}
impl From<DACLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DACLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DACLSEL`"]
pub type DACLSEL_R = crate::R<u8, DACLSEL_A>;
impl DACLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DACLSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DACLSEL_A::DACLSEL_0),
            2 => Val(DACLSEL_A::DACLSEL_2),
            3 => Val(DACLSEL_A::DACLSEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DACLSEL_0`"]
    #[inline(always)]
    pub fn is_daclsel_0(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_0
    }
    #[doc = "Checks if the value of the field is `DACLSEL_2`"]
    #[inline(always)]
    pub fn is_daclsel_2(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_2
    }
    #[doc = "Checks if the value of the field is `DACLSEL_3`"]
    #[inline(always)]
    pub fn is_daclsel_3(&self) -> bool {
        *self == DACLSEL_A::DACLSEL_3
    }
}
#[doc = "Write proxy for field `DACLSEL`"]
pub struct DACLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DAC latch loads when DACDAT written"]
    #[inline(always)]
    pub fn daclsel_0(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_0)
    }
    #[doc = "Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_2(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_2)
    }
    #[doc = "Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_3(self) -> &'a mut W {
        self.variant(DACLSEL_A::DACLSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "SAC DAC select reference voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACSREF_A {
    #[doc = "0: AVCC"]
    DACSREF_0 = 0,
    #[doc = "1: Alternative reference"]
    DACSREF_1 = 1,
}
impl From<DACSREF_A> for bool {
    #[inline(always)]
    fn from(variant: DACSREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACSREF`"]
pub type DACSREF_R = crate::R<bool, DACSREF_A>;
impl DACSREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSREF_A {
        match self.bits {
            false => DACSREF_A::DACSREF_0,
            true => DACSREF_A::DACSREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACSREF_0`"]
    #[inline(always)]
    pub fn is_dacsref_0(&self) -> bool {
        *self == DACSREF_A::DACSREF_0
    }
    #[doc = "Checks if the value of the field is `DACSREF_1`"]
    #[inline(always)]
    pub fn is_dacsref_1(&self) -> bool {
        *self == DACSREF_A::DACSREF_1
    }
}
#[doc = "Write proxy for field `DACSREF`"]
pub struct DACSREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSREF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn dacsref_0(self) -> &'a mut W {
        self.variant(DACSREF_A::DACSREF_0)
    }
    #[doc = "Alternative reference"]
    #[inline(always)]
    pub fn dacsref_1(self) -> &'a mut W {
        self.variant(DACSREF_A::DACSREF_1)
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
impl R {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&self) -> DACIE_R {
        DACIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&self) -> DACDMAE_R {
        DACDMAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&self) -> DACLSEL_R {
        DACLSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&self) -> DACSREF_R {
        DACSREF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&mut self) -> DACIE_W {
        DACIE_W { w: self }
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&mut self) -> DACDMAE_W {
        DACDMAE_W { w: self }
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&mut self) -> DACLSEL_W {
        DACLSEL_W { w: self }
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&mut self) -> DACSREF_W {
        DACSREF_W { w: self }
    }
}
