#[doc = "Register `SAC3OA` reader"]
pub struct R(crate::R<SAC3OA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC3OA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC3OA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC3OA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC3OA` writer"]
pub struct W(crate::W<SAC3OA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC3OA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAC3OA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC3OA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSEL` reader - SAC OA Positive input source selection"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "SAC OA Positive input source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSEL_A> {
        match self.bits {
            0 => Some(PSEL_A::PSEL_0),
            1 => Some(PSEL_A::PSEL_1),
            2 => Some(PSEL_A::PSEL_2),
            _ => None,
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
#[doc = "Field `PSEL` writer - SAC OA Positive input source selection"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SAC3OA_SPEC, u8, PSEL_A, 2, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
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
}
#[doc = "Field `PMUXEN` reader - SAC Positive input MUX control."]
pub type PMUXEN_R = crate::BitReader<PMUXEN_A>;
#[doc = "SAC Positive input MUX control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PMUXEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PMUXEN` writer - SAC Positive input MUX control."]
pub type PMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC3OA_SPEC, PMUXEN_A, O>;
impl<'a, const O: u8> PMUXEN_W<'a, O> {
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
}
#[doc = "Field `NSEL` reader - SAC OA Negative input source selection"]
pub type NSEL_R = crate::FieldReader<u8, NSEL_A>;
#[doc = "SAC OA Negative input source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSEL_A> {
        match self.bits {
            0 => Some(NSEL_A::NSEL_0),
            1 => Some(NSEL_A::NSEL_1),
            2 => Some(NSEL_A::NSEL_2),
            _ => None,
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
#[doc = "Field `NSEL` writer - SAC OA Negative input source selection"]
pub type NSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SAC3OA_SPEC, u8, NSEL_A, 2, O>;
impl<'a, const O: u8> NSEL_W<'a, O> {
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
}
#[doc = "Field `NMUXEN` reader - SAC Negative input MUX controL"]
pub type NMUXEN_R = crate::BitReader<NMUXEN_A>;
#[doc = "SAC Negative input MUX controL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl NMUXEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `NMUXEN` writer - SAC Negative input MUX controL"]
pub type NMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC3OA_SPEC, NMUXEN_A, O>;
impl<'a, const O: u8> NMUXEN_W<'a, O> {
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
}
#[doc = "Field `OAEN` reader - SAC OA Enable selection"]
pub type OAEN_R = crate::BitReader<OAEN_A>;
#[doc = "SAC OA Enable selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl OAEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `OAEN` writer - SAC OA Enable selection"]
pub type OAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC3OA_SPEC, OAEN_A, O>;
impl<'a, const O: u8> OAEN_W<'a, O> {
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
}
#[doc = "Field `OAPM` reader - SAC OA power mode selection"]
pub type OAPM_R = crate::BitReader<OAPM_A>;
#[doc = "SAC OA power mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl OAPM_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `OAPM` writer - SAC OA power mode selection"]
pub type OAPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC3OA_SPEC, OAPM_A, O>;
impl<'a, const O: u8> OAPM_W<'a, O> {
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
}
#[doc = "Field `SACEN` reader - SAC Enable selection"]
pub type SACEN_R = crate::BitReader<SACEN_A>;
#[doc = "SAC Enable selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SACEN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SACEN` writer - SAC Enable selection"]
pub type SACEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SAC3OA_SPEC, SACEN_A, O>;
impl<'a, const O: u8> SACEN_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&self) -> PMUXEN_R {
        PMUXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&self) -> NMUXEN_R {
        NMUXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&self) -> OAEN_R {
        OAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&self) -> OAPM_R {
        OAPM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&self) -> SACEN_R {
        SACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W<0> {
        PSEL_W::new(self)
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PMUXEN_W<3> {
        PMUXEN_W::new(self)
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&mut self) -> NSEL_W<4> {
        NSEL_W::new(self)
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&mut self) -> NMUXEN_W<7> {
        NMUXEN_W::new(self)
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&mut self) -> OAEN_W<8> {
        OAEN_W::new(self)
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&mut self) -> OAPM_W<9> {
        OAPM_W::new(self)
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&mut self) -> SACEN_W<10> {
        SACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAC OA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3oa](index.html) module"]
pub struct SAC3OA_SPEC;
impl crate::RegisterSpec for SAC3OA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac3oa::R](R) reader structure"]
impl crate::Readable for SAC3OA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac3oa::W](W) writer structure"]
impl crate::Writable for SAC3OA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC3OA to value 0"]
impl crate::Resettable for SAC3OA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
