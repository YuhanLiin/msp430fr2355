#[doc = "Register `CSCTL8` reader"]
pub struct R(crate::R<CSCTL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL8` writer"]
pub struct W(crate::W<CSCTL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL8_SPEC>;
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
impl From<crate::W<CSCTL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACLKREQEN` reader - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
pub type ACLKREQEN_R = crate::BitReader<ACLKREQEN_A>;
#[doc = "ACLK clock request enable. Setting this enables conditional module requests for ACLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACLKREQEN_A {
    #[doc = "0: ACLK conditional requests are disabled."]
    ACLKREQEN_0 = 0,
    #[doc = "1: ACLK conditional requests are enabled."]
    ACLKREQEN_1 = 1,
}
impl From<ACLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACLKREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLKREQEN_A {
        match self.bits {
            false => ACLKREQEN_A::ACLKREQEN_0,
            true => ACLKREQEN_A::ACLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACLKREQEN_0`"]
    #[inline(always)]
    pub fn is_aclkreqen_0(&self) -> bool {
        *self == ACLKREQEN_A::ACLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `ACLKREQEN_1`"]
    #[inline(always)]
    pub fn is_aclkreqen_1(&self) -> bool {
        *self == ACLKREQEN_A::ACLKREQEN_1
    }
}
#[doc = "Field `ACLKREQEN` writer - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
pub type ACLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, ACLKREQEN_A, O>;
impl<'a, const O: u8> ACLKREQEN_W<'a, O> {
    #[doc = "ACLK conditional requests are disabled."]
    #[inline(always)]
    pub fn aclkreqen_0(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::ACLKREQEN_0)
    }
    #[doc = "ACLK conditional requests are enabled."]
    #[inline(always)]
    pub fn aclkreqen_1(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::ACLKREQEN_1)
    }
}
#[doc = "Field `MCLKREQEN` reader - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
pub type MCLKREQEN_R = crate::BitReader<MCLKREQEN_A>;
#[doc = "MCLK clock request enable. Setting this enables conditional module requests for MCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLKREQEN_A {
    #[doc = "0: MCLK conditional requests are disabled."]
    MCLKREQEN_0 = 0,
    #[doc = "1: MCLK conditional requests are enabled."]
    MCLKREQEN_1 = 1,
}
impl From<MCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCLKREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKREQEN_A {
        match self.bits {
            false => MCLKREQEN_A::MCLKREQEN_0,
            true => MCLKREQEN_A::MCLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLKREQEN_0`"]
    #[inline(always)]
    pub fn is_mclkreqen_0(&self) -> bool {
        *self == MCLKREQEN_A::MCLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `MCLKREQEN_1`"]
    #[inline(always)]
    pub fn is_mclkreqen_1(&self) -> bool {
        *self == MCLKREQEN_A::MCLKREQEN_1
    }
}
#[doc = "Field `MCLKREQEN` writer - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
pub type MCLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, MCLKREQEN_A, O>;
impl<'a, const O: u8> MCLKREQEN_W<'a, O> {
    #[doc = "MCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn mclkreqen_0(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::MCLKREQEN_0)
    }
    #[doc = "MCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn mclkreqen_1(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::MCLKREQEN_1)
    }
}
#[doc = "Field `SMCLKREQEN` reader - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
pub type SMCLKREQEN_R = crate::BitReader<SMCLKREQEN_A>;
#[doc = "SMCLK clock request enable. Setting this enables conditional module requests for SMCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMCLKREQEN_A {
    #[doc = "0: SMCLK conditional requests are disabled."]
    SMCLKREQEN_0 = 0,
    #[doc = "1: SMCLK conditional requests are enabled."]
    SMCLKREQEN_1 = 1,
}
impl From<SMCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMCLKREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKREQEN_A {
        match self.bits {
            false => SMCLKREQEN_A::SMCLKREQEN_0,
            true => SMCLKREQEN_A::SMCLKREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLKREQEN_0`"]
    #[inline(always)]
    pub fn is_smclkreqen_0(&self) -> bool {
        *self == SMCLKREQEN_A::SMCLKREQEN_0
    }
    #[doc = "Checks if the value of the field is `SMCLKREQEN_1`"]
    #[inline(always)]
    pub fn is_smclkreqen_1(&self) -> bool {
        *self == SMCLKREQEN_A::SMCLKREQEN_1
    }
}
#[doc = "Field `SMCLKREQEN` writer - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
pub type SMCLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, SMCLKREQEN_A, O>;
impl<'a, const O: u8> SMCLKREQEN_W<'a, O> {
    #[doc = "SMCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn smclkreqen_0(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::SMCLKREQEN_0)
    }
    #[doc = "SMCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn smclkreqen_1(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::SMCLKREQEN_1)
    }
}
#[doc = "Field `MODOSCREQEN` reader - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
pub type MODOSCREQEN_R = crate::BitReader<MODOSCREQEN_A>;
#[doc = "MODOSC clock request enable. Setting this enables conditional module requests for MODOSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODOSCREQEN_A {
    #[doc = "0: MODOSC conditional requests are disabled."]
    MODOSCREQEN_0 = 0,
    #[doc = "1: MODOSC conditional requests are enabled."]
    MODOSCREQEN_1 = 1,
}
impl From<MODOSCREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODOSCREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODOSCREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODOSCREQEN_A {
        match self.bits {
            false => MODOSCREQEN_A::MODOSCREQEN_0,
            true => MODOSCREQEN_A::MODOSCREQEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODOSCREQEN_0`"]
    #[inline(always)]
    pub fn is_modoscreqen_0(&self) -> bool {
        *self == MODOSCREQEN_A::MODOSCREQEN_0
    }
    #[doc = "Checks if the value of the field is `MODOSCREQEN_1`"]
    #[inline(always)]
    pub fn is_modoscreqen_1(&self) -> bool {
        *self == MODOSCREQEN_A::MODOSCREQEN_1
    }
}
#[doc = "Field `MODOSCREQEN` writer - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
pub type MODOSCREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, MODOSCREQEN_A, O>;
impl<'a, const O: u8> MODOSCREQEN_W<'a, O> {
    #[doc = "MODOSC conditional requests are disabled."]
    #[inline(always)]
    pub fn modoscreqen_0(self) -> &'a mut W {
        self.variant(MODOSCREQEN_A::MODOSCREQEN_0)
    }
    #[doc = "MODOSC conditional requests are enabled."]
    #[inline(always)]
    pub fn modoscreqen_1(self) -> &'a mut W {
        self.variant(MODOSCREQEN_A::MODOSCREQEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W<0> {
        ACLKREQEN_W::new(self)
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W<1> {
        MCLKREQEN_W::new(self)
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W<2> {
        SMCLKREQEN_W::new(self)
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W<3> {
        MODOSCREQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl8](index.html) module"]
pub struct CSCTL8_SPEC;
impl crate::RegisterSpec for CSCTL8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl8::R](R) reader structure"]
impl crate::Readable for CSCTL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl8::W](W) writer structure"]
impl crate::Writable for CSCTL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL8 to value 0"]
impl crate::Resettable for CSCTL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
