#[doc = "Register `SYSUNIV` reader"]
pub struct R(crate::R<SYSUNIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSUNIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSUNIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSUNIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSUNIV` writer"]
pub struct W(crate::W<SYSUNIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSUNIV_SPEC>;
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
impl From<crate::W<SYSUNIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSUNIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSUNIV` reader - User NMI vector"]
pub type SYSUNIV_R = crate::FieldReader<u16, SYSUNIV_A>;
#[doc = "User NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSUNIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: NMIIFG NMI pin"]
    NMIIFG = 2,
    #[doc = "4: OFIFG oscillator fault"]
    OFIFG = 4,
}
impl From<SYSUNIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSUNIV_A) -> Self {
        variant as _
    }
}
impl SYSUNIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSUNIV_A> {
        match self.bits {
            0 => Some(SYSUNIV_A::NONE),
            2 => Some(SYSUNIV_A::NMIIFG),
            4 => Some(SYSUNIV_A::OFIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSUNIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `NMIIFG`"]
    #[inline(always)]
    pub fn is_nmiifg(&self) -> bool {
        *self == SYSUNIV_A::NMIIFG
    }
    #[doc = "Checks if the value of the field is `OFIFG`"]
    #[inline(always)]
    pub fn is_ofifg(&self) -> bool {
        *self == SYSUNIV_A::OFIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&self) -> SYSUNIV_R {
        SYSUNIV_R::new(self.bits)
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
#[doc = "User NMI Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysuniv](index.html) module"]
pub struct SYSUNIV_SPEC;
impl crate::RegisterSpec for SYSUNIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysuniv::R](R) reader structure"]
impl crate::Readable for SYSUNIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysuniv::W](W) writer structure"]
impl crate::Writable for SYSUNIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSUNIV to value 0"]
impl crate::Resettable for SYSUNIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
