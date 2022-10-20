#[doc = "Register `CPINT` reader"]
pub struct R(crate::R<CPINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPINT` writer"]
pub struct W(crate::W<CPINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPINT_SPEC>;
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
impl From<crate::W<CPINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPIFG` reader - Comparator output interrupt flag"]
pub type CPIFG_R = crate::BitReader<CPIFG_A>;
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPIFG_A {
    #[doc = "0: No interrupt pending."]
    CPIFG_0 = 0,
    #[doc = "1: Output interrupt pending."]
    CPIFG_1 = 1,
}
impl From<CPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CPIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl CPIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIFG_A {
        match self.bits {
            false => CPIFG_A::CPIFG_0,
            true => CPIFG_A::CPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIFG_0`"]
    #[inline(always)]
    pub fn is_cpifg_0(&self) -> bool {
        *self == CPIFG_A::CPIFG_0
    }
    #[doc = "Checks if the value of the field is `CPIFG_1`"]
    #[inline(always)]
    pub fn is_cpifg_1(&self) -> bool {
        *self == CPIFG_A::CPIFG_1
    }
}
#[doc = "Field `CPIFG` writer - Comparator output interrupt flag"]
pub type CPIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CPINT_SPEC, CPIFG_A, O>;
impl<'a, const O: u8> CPIFG_W<'a, O> {
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpifg_0(self) -> &'a mut W {
        self.variant(CPIFG_A::CPIFG_0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpifg_1(self) -> &'a mut W {
        self.variant(CPIFG_A::CPIFG_1)
    }
}
#[doc = "Field `CPIIFG` reader - Comparator output inverted interrupt flag"]
pub type CPIIFG_R = crate::BitReader<CPIIFG_A>;
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPIIFG_A {
    #[doc = "0: No interrupt pending."]
    CPIIFG_0 = 0,
    #[doc = "1: Output interrupt pending."]
    CPIIFG_1 = 1,
}
impl From<CPIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CPIIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl CPIIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIIFG_A {
        match self.bits {
            false => CPIIFG_A::CPIIFG_0,
            true => CPIIFG_A::CPIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIIFG_0`"]
    #[inline(always)]
    pub fn is_cpiifg_0(&self) -> bool {
        *self == CPIIFG_A::CPIIFG_0
    }
    #[doc = "Checks if the value of the field is `CPIIFG_1`"]
    #[inline(always)]
    pub fn is_cpiifg_1(&self) -> bool {
        *self == CPIIFG_A::CPIIFG_1
    }
}
#[doc = "Field `CPIIFG` writer - Comparator output inverted interrupt flag"]
pub type CPIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CPINT_SPEC, CPIIFG_A, O>;
impl<'a, const O: u8> CPIIFG_W<'a, O> {
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_0(self) -> &'a mut W {
        self.variant(CPIIFG_A::CPIIFG_0)
    }
    #[doc = "Output interrupt pending."]
    #[inline(always)]
    pub fn cpiifg_1(self) -> &'a mut W {
        self.variant(CPIIFG_A::CPIIFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&self) -> CPIFG_R {
        CPIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&self) -> CPIIFG_R {
        CPIIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn cpifg(&mut self) -> CPIFG_W<0> {
        CPIFG_W::new(self)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn cpiifg(&mut self) -> CPIIFG_W<1> {
        CPIIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpint](index.html) module"]
pub struct CPINT_SPEC;
impl crate::RegisterSpec for CPINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cpint::R](R) reader structure"]
impl crate::Readable for CPINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpint::W](W) writer structure"]
impl crate::Writable for CPINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPINT to value 0"]
impl crate::Resettable for CPINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
