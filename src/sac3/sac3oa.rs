#[doc = "Reader of register SAC3OA"]
pub type R = crate::R<u16, super::SAC3OA>;
#[doc = "Writer for register SAC3OA"]
pub type W = crate::W<u16, super::SAC3OA>;
#[doc = "Register SAC3OA `reset()`'s with value 0"]
impl crate::ResetValue for super::SAC3OA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAC OA Positive input source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: External source selected"]
    PSEL_0 = 0,
    #[doc = "1: 12-bit reference DAC source selected"]
    PSEL_1 = 1,
    #[doc = "2: Pair OA source selected"]
    PSEL_2 = 2,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSEL_A::PSEL_0),
            1 => Val(PSEL_A::PSEL_1),
            2 => Val(PSEL_A::PSEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "12-bit reference DAC source selected"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "Pair OA source selected"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "SAC Positive input MUX control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXEN_A {
    #[doc = "0: All positive input sources are disconnected to OA positive port"]
    PMUXEN_0 = 0,
    #[doc = "1: All positive input sources are connected to OA positive port"]
    PMUXEN_1 = 1,
}
impl From<PMUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: PMUXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMUXEN`"]
pub type PMUXEN_R = crate::R<bool, PMUXEN_A>;
impl PMUXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUXEN_A {
        match self.bits {
            false => PMUXEN_A::PMUXEN_0,
            true => PMUXEN_A::PMUXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMUXEN_0`"]
    #[inline(always)]
    pub fn is_pmuxen_0(&self) -> bool {
        *self == PMUXEN_A::PMUXEN_0
    }
    #[doc = "Checks if the value of the field is `PMUXEN_1`"]
    #[inline(always)]
    pub fn is_pmuxen_1(&self) -> bool {
        *self == PMUXEN_A::PMUXEN_1
    }
}
#[doc = "Write proxy for field `PMUXEN`"]
pub struct PMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All positive input sources are disconnected to OA positive port"]
    #[inline(always)]
    pub fn pmuxen_0(self) -> &'a mut W {
        self.variant(PMUXEN_A::PMUXEN_0)
    }
    #[doc = "All positive input sources are connected to OA positive port"]
    #[inline(always)]
    pub fn pmuxen_1(self) -> &'a mut W {
        self.variant(PMUXEN_A::PMUXEN_1)
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
#[doc = "SAC OA Negative input source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSEL_A {
    #[doc = "0: External source selected"]
    NSEL_0 = 0,
    #[doc = "1: PGA source selected"]
    NSEL_1 = 1,
    #[doc = "2: Device Specific"]
    NSEL_2 = 2,
}
impl From<NSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NSEL`"]
pub type NSEL_R = crate::R<u8, NSEL_A>;
impl NSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NSEL_A::NSEL_0),
            1 => Val(NSEL_A::NSEL_1),
            2 => Val(NSEL_A::NSEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NSEL_0`"]
    #[inline(always)]
    pub fn is_nsel_0(&self) -> bool {
        *self == NSEL_A::NSEL_0
    }
    #[doc = "Checks if the value of the field is `NSEL_1`"]
    #[inline(always)]
    pub fn is_nsel_1(&self) -> bool {
        *self == NSEL_A::NSEL_1
    }
    #[doc = "Checks if the value of the field is `NSEL_2`"]
    #[inline(always)]
    pub fn is_nsel_2(&self) -> bool {
        *self == NSEL_A::NSEL_2
    }
}
#[doc = "Write proxy for field `NSEL`"]
pub struct NSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn nsel_0(self) -> &'a mut W {
        self.variant(NSEL_A::NSEL_0)
    }
    #[doc = "PGA source selected"]
    #[inline(always)]
    pub fn nsel_1(self) -> &'a mut W {
        self.variant(NSEL_A::NSEL_1)
    }
    #[doc = "Device Specific"]
    #[inline(always)]
    pub fn nsel_2(self) -> &'a mut W {
        self.variant(NSEL_A::NSEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "SAC Negative input MUX controL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMUXEN_A {
    #[doc = "0: All negative input sources are disconnected to OA negative port"]
    NMUXEN_0 = 0,
    #[doc = "1: All negative input sources are connected to OA negative port"]
    NMUXEN_1 = 1,
}
impl From<NMUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMUXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMUXEN`"]
pub type NMUXEN_R = crate::R<bool, NMUXEN_A>;
impl NMUXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMUXEN_A {
        match self.bits {
            false => NMUXEN_A::NMUXEN_0,
            true => NMUXEN_A::NMUXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMUXEN_0`"]
    #[inline(always)]
    pub fn is_nmuxen_0(&self) -> bool {
        *self == NMUXEN_A::NMUXEN_0
    }
    #[doc = "Checks if the value of the field is `NMUXEN_1`"]
    #[inline(always)]
    pub fn is_nmuxen_1(&self) -> bool {
        *self == NMUXEN_A::NMUXEN_1
    }
}
#[doc = "Write proxy for field `NMUXEN`"]
pub struct NMUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMUXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMUXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All negative input sources are disconnected to OA negative port"]
    #[inline(always)]
    pub fn nmuxen_0(self) -> &'a mut W {
        self.variant(NMUXEN_A::NMUXEN_0)
    }
    #[doc = "All negative input sources are connected to OA negative port"]
    #[inline(always)]
    pub fn nmuxen_1(self) -> &'a mut W {
        self.variant(NMUXEN_A::NMUXEN_1)
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
#[doc = "SAC OA Enable selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OAEN_A {
    #[doc = "0: SAC OA is disabled, then the SAC OA output high impedance"]
    OAEN_0 = 0,
    #[doc = "1: SAC OA is enabled, normal mode"]
    OAEN_1 = 1,
}
impl From<OAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OAEN`"]
pub type OAEN_R = crate::R<bool, OAEN_A>;
impl OAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAEN_A {
        match self.bits {
            false => OAEN_A::OAEN_0,
            true => OAEN_A::OAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OAEN_0`"]
    #[inline(always)]
    pub fn is_oaen_0(&self) -> bool {
        *self == OAEN_A::OAEN_0
    }
    #[doc = "Checks if the value of the field is `OAEN_1`"]
    #[inline(always)]
    pub fn is_oaen_1(&self) -> bool {
        *self == OAEN_A::OAEN_1
    }
}
#[doc = "Write proxy for field `OAEN`"]
pub struct OAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAC OA is disabled, then the SAC OA output high impedance"]
    #[inline(always)]
    pub fn oaen_0(self) -> &'a mut W {
        self.variant(OAEN_A::OAEN_0)
    }
    #[doc = "SAC OA is enabled, normal mode"]
    #[inline(always)]
    pub fn oaen_1(self) -> &'a mut W {
        self.variant(OAEN_A::OAEN_1)
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
#[doc = "SAC OA power mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OAPM_A {
    #[doc = "0: High speed and high power"]
    OAPM_0 = 0,
    #[doc = "1: Llow speed and low power"]
    OAPM_1 = 1,
}
impl From<OAPM_A> for bool {
    #[inline(always)]
    fn from(variant: OAPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OAPM`"]
pub type OAPM_R = crate::R<bool, OAPM_A>;
impl OAPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAPM_A {
        match self.bits {
            false => OAPM_A::OAPM_0,
            true => OAPM_A::OAPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `OAPM_0`"]
    #[inline(always)]
    pub fn is_oapm_0(&self) -> bool {
        *self == OAPM_A::OAPM_0
    }
    #[doc = "Checks if the value of the field is `OAPM_1`"]
    #[inline(always)]
    pub fn is_oapm_1(&self) -> bool {
        *self == OAPM_A::OAPM_1
    }
}
#[doc = "Write proxy for field `OAPM`"]
pub struct OAPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OAPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OAPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High speed and high power"]
    #[inline(always)]
    pub fn oapm_0(self) -> &'a mut W {
        self.variant(OAPM_A::OAPM_0)
    }
    #[doc = "Llow speed and low power"]
    #[inline(always)]
    pub fn oapm_1(self) -> &'a mut W {
        self.variant(OAPM_A::OAPM_1)
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
#[doc = "SAC Enable selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACEN_A {
    #[doc = "0: SAC all modules are disabled, then the SAC output high impedance"]
    SACEN_0 = 0,
    #[doc = "1: SAC all modules are enabled, normal mode"]
    SACEN_1 = 1,
}
impl From<SACEN_A> for bool {
    #[inline(always)]
    fn from(variant: SACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SACEN`"]
pub type SACEN_R = crate::R<bool, SACEN_A>;
impl SACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACEN_A {
        match self.bits {
            false => SACEN_A::SACEN_0,
            true => SACEN_A::SACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SACEN_0`"]
    #[inline(always)]
    pub fn is_sacen_0(&self) -> bool {
        *self == SACEN_A::SACEN_0
    }
    #[doc = "Checks if the value of the field is `SACEN_1`"]
    #[inline(always)]
    pub fn is_sacen_1(&self) -> bool {
        *self == SACEN_A::SACEN_1
    }
}
#[doc = "Write proxy for field `SACEN`"]
pub struct SACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAC all modules are disabled, then the SAC output high impedance"]
    #[inline(always)]
    pub fn sacen_0(self) -> &'a mut W {
        self.variant(SACEN_A::SACEN_0)
    }
    #[doc = "SAC all modules are enabled, normal mode"]
    #[inline(always)]
    pub fn sacen_1(self) -> &'a mut W {
        self.variant(SACEN_A::SACEN_1)
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
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&self) -> NMUXEN_R {
        NMUXEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&self) -> OAEN_R {
        OAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&self) -> OAPM_R {
        OAPM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&self) -> SACEN_R {
        SACEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W {
        PMUXEN_W { w: self }
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&mut self) -> NSEL_W {
        NSEL_W { w: self }
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&mut self) -> NMUXEN_W {
        NMUXEN_W { w: self }
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&mut self) -> OAEN_W {
        OAEN_W { w: self }
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&mut self) -> OAPM_W {
        OAPM_W { w: self }
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&mut self) -> SACEN_W {
        SACEN_W { w: self }
    }
}
