#[doc = "Register `CP1IV` reader"]
pub struct R(crate::R<CP1IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP1IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP1IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP1IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CP1IV` writer"]
pub struct W(crate::W<CP1IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CP1IV_SPEC>;
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
impl From<crate::W<CP1IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CP1IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPIV` reader - Comparator interrupt vector word register"]
pub type CPIV_R = crate::FieldReader<u16, CPIV_A>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CPIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: CPIFG"]
    CPIFG = 2,
    #[doc = "4: CPIIFG"]
    CPIIFG = 4,
}
impl From<CPIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CPIV_A) -> Self {
        variant as _
    }
}
impl CPIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPIV_A> {
        match self.bits {
            0 => Some(CPIV_A::NONE),
            2 => Some(CPIV_A::CPIFG),
            4 => Some(CPIV_A::CPIIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CPIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `CPIFG`"]
    #[inline(always)]
    pub fn is_cpifg(&self) -> bool {
        *self == CPIV_A::CPIFG
    }
    #[doc = "Checks if the value of the field is `CPIIFG`"]
    #[inline(always)]
    pub fn is_cpiifg(&self) -> bool {
        *self == CPIV_A::CPIIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn cpiv(&self) -> CPIV_R {
        CPIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1iv](index.html) module"]
pub struct CP1IV_SPEC;
impl crate::RegisterSpec for CP1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cp1iv::R](R) reader structure"]
impl crate::Readable for CP1IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cp1iv::W](W) writer structure"]
impl crate::Writable for CP1IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CP1IV to value 0"]
impl crate::Resettable for CP1IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
