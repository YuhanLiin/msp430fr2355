#[doc = "Register `PMMCTL0` reader"]
pub struct R(crate::R<PMMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL0` writer"]
pub struct W(crate::W<PMMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL0_SPEC>;
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
impl From<crate::W<PMMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFLOW` reader - Reflow pre-conditioning. Prepares device for reflow soldering. Write as 0 during normal operation."]
pub type REFLOW_R = crate::BitReader<REFLOW_A>;
#[doc = "Reflow pre-conditioning. Prepares device for reflow soldering. Write as 0 during normal operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFLOW_A {
    #[doc = "0: Normal operation"]
    REFLOW_0 = 0,
    #[doc = "1: Enable reflow pre-conditioning"]
    REFLOW_1 = 1,
}
impl From<REFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: REFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl REFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFLOW_A {
        match self.bits {
            false => REFLOW_A::REFLOW_0,
            true => REFLOW_A::REFLOW_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFLOW_0`"]
    #[inline(always)]
    pub fn is_reflow_0(&self) -> bool {
        *self == REFLOW_A::REFLOW_0
    }
    #[doc = "Checks if the value of the field is `REFLOW_1`"]
    #[inline(always)]
    pub fn is_reflow_1(&self) -> bool {
        *self == REFLOW_A::REFLOW_1
    }
}
#[doc = "Field `REFLOW` writer - Reflow pre-conditioning. Prepares device for reflow soldering. Write as 0 during normal operation."]
pub type REFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, REFLOW_A, O>;
impl<'a, const O: u8> REFLOW_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn reflow_0(self) -> &'a mut W {
        self.variant(REFLOW_A::REFLOW_0)
    }
    #[doc = "Enable reflow pre-conditioning"]
    #[inline(always)]
    pub fn reflow_1(self) -> &'a mut W {
        self.variant(REFLOW_A::REFLOW_1)
    }
}
#[doc = "Field `PMMSWBOR` reader - Software brownout reset."]
pub type PMMSWBOR_R = crate::BitReader<PMMSWBOR_A>;
#[doc = "Software brownout reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMSWBOR_A {
    #[doc = "0: Normal operation"]
    PMMSWBOR_0 = 0,
    #[doc = "1: Set to 1 to trigger a BOR"]
    PMMSWBOR_1 = 1,
}
impl From<PMMSWBOR_A> for bool {
    #[inline(always)]
    fn from(variant: PMMSWBOR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMSWBOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMSWBOR_A {
        match self.bits {
            false => PMMSWBOR_A::PMMSWBOR_0,
            true => PMMSWBOR_A::PMMSWBOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR_0`"]
    #[inline(always)]
    pub fn is_pmmswbor_0(&self) -> bool {
        *self == PMMSWBOR_A::PMMSWBOR_0
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR_1`"]
    #[inline(always)]
    pub fn is_pmmswbor_1(&self) -> bool {
        *self == PMMSWBOR_A::PMMSWBOR_1
    }
}
#[doc = "Field `PMMSWBOR` writer - Software brownout reset."]
pub type PMMSWBOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, PMMSWBOR_A, O>;
impl<'a, const O: u8> PMMSWBOR_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswbor_0(self) -> &'a mut W {
        self.variant(PMMSWBOR_A::PMMSWBOR_0)
    }
    #[doc = "Set to 1 to trigger a BOR"]
    #[inline(always)]
    pub fn pmmswbor_1(self) -> &'a mut W {
        self.variant(PMMSWBOR_A::PMMSWBOR_1)
    }
}
#[doc = "Field `PMMSWPOR` reader - Software POR."]
pub type PMMSWPOR_R = crate::BitReader<PMMSWPOR_A>;
#[doc = "Software POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMSWPOR_A {
    #[doc = "0: Normal operation"]
    PMMSWPOR_0 = 0,
    #[doc = "1: Set to 1 to trigger a POR"]
    PMMSWPOR_1 = 1,
}
impl From<PMMSWPOR_A> for bool {
    #[inline(always)]
    fn from(variant: PMMSWPOR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMSWPOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMSWPOR_A {
        match self.bits {
            false => PMMSWPOR_A::PMMSWPOR_0,
            true => PMMSWPOR_A::PMMSWPOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR_0`"]
    #[inline(always)]
    pub fn is_pmmswpor_0(&self) -> bool {
        *self == PMMSWPOR_A::PMMSWPOR_0
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR_1`"]
    #[inline(always)]
    pub fn is_pmmswpor_1(&self) -> bool {
        *self == PMMSWPOR_A::PMMSWPOR_1
    }
}
#[doc = "Field `PMMSWPOR` writer - Software POR."]
pub type PMMSWPOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, PMMSWPOR_A, O>;
impl<'a, const O: u8> PMMSWPOR_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswpor_0(self) -> &'a mut W {
        self.variant(PMMSWPOR_A::PMMSWPOR_0)
    }
    #[doc = "Set to 1 to trigger a POR"]
    #[inline(always)]
    pub fn pmmswpor_1(self) -> &'a mut W {
        self.variant(PMMSWPOR_A::PMMSWPOR_1)
    }
}
#[doc = "Field `PMMREGOFF` reader - Regulator off"]
pub type PMMREGOFF_R = crate::BitReader<PMMREGOFF_A>;
#[doc = "Regulator off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMMREGOFF_A {
    #[doc = "0: Regulator remains on when going into LPM3 or LPM4"]
    PMMREGOFF_0 = 0,
    #[doc = "1: Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    PMMREGOFF_1 = 1,
}
impl From<PMMREGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PMMREGOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PMMREGOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMMREGOFF_A {
        match self.bits {
            false => PMMREGOFF_A::PMMREGOFF_0,
            true => PMMREGOFF_A::PMMREGOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMMREGOFF_0`"]
    #[inline(always)]
    pub fn is_pmmregoff_0(&self) -> bool {
        *self == PMMREGOFF_A::PMMREGOFF_0
    }
    #[doc = "Checks if the value of the field is `PMMREGOFF_1`"]
    #[inline(always)]
    pub fn is_pmmregoff_1(&self) -> bool {
        *self == PMMREGOFF_A::PMMREGOFF_1
    }
}
#[doc = "Field `PMMREGOFF` writer - Regulator off"]
pub type PMMREGOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, PMMREGOFF_A, O>;
impl<'a, const O: u8> PMMREGOFF_W<'a, O> {
    #[doc = "Regulator remains on when going into LPM3 or LPM4"]
    #[inline(always)]
    pub fn pmmregoff_0(self) -> &'a mut W {
        self.variant(PMMREGOFF_A::PMMREGOFF_0)
    }
    #[doc = "Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    #[inline(always)]
    pub fn pmmregoff_1(self) -> &'a mut W {
        self.variant(PMMREGOFF_A::PMMREGOFF_1)
    }
}
#[doc = "Field `SVSHE` reader - High-side SVS enable."]
pub type SVSHE_R = crate::BitReader<SVSHE_A>;
#[doc = "High-side SVS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVSHE_A {
    #[doc = "0: High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    SVSHE_0 = 0,
    #[doc = "1: SVSH is always enabled."]
    SVSHE_1 = 1,
}
impl From<SVSHE_A> for bool {
    #[inline(always)]
    fn from(variant: SVSHE_A) -> Self {
        variant as u8 != 0
    }
}
impl SVSHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHE_A {
        match self.bits {
            false => SVSHE_A::SVSHE_0,
            true => SVSHE_A::SVSHE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSHE_0`"]
    #[inline(always)]
    pub fn is_svshe_0(&self) -> bool {
        *self == SVSHE_A::SVSHE_0
    }
    #[doc = "Checks if the value of the field is `SVSHE_1`"]
    #[inline(always)]
    pub fn is_svshe_1(&self) -> bool {
        *self == SVSHE_A::SVSHE_1
    }
}
#[doc = "Field `SVSHE` writer - High-side SVS enable."]
pub type SVSHE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, SVSHE_A, O>;
impl<'a, const O: u8> SVSHE_W<'a, O> {
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn svshe_0(self) -> &'a mut W {
        self.variant(SVSHE_A::SVSHE_0)
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn svshe_1(self) -> &'a mut W {
        self.variant(SVSHE_A::SVSHE_1)
    }
}
#[doc = "Field `PMMPW` reader - PMM password."]
pub type PMMPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMMPW` writer - PMM password."]
pub type PMMPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PMMCTL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Reflow pre-conditioning. Prepares device for reflow soldering. Write as 0 during normal operation."]
    #[inline(always)]
    pub fn reflow(&self) -> REFLOW_R {
        REFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PMM password."]
    #[inline(always)]
    pub fn pmmpw(&self) -> PMMPW_R {
        PMMPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reflow pre-conditioning. Prepares device for reflow soldering. Write as 0 during normal operation."]
    #[inline(always)]
    pub fn reflow(&mut self) -> REFLOW_W<0> {
        REFLOW_W::new(self)
    }
    #[doc = "Bit 2 - Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W<2> {
        PMMSWBOR_W::new(self)
    }
    #[doc = "Bit 3 - Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W<3> {
        PMMSWPOR_W::new(self)
    }
    #[doc = "Bit 4 - Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W<4> {
        PMMREGOFF_W::new(self)
    }
    #[doc = "Bit 6 - High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W<6> {
        SVSHE_W::new(self)
    }
    #[doc = "Bits 8:15 - PMM password."]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PMMPW_W<8> {
        PMMPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management Module control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](index.html) module"]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl0::R](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
