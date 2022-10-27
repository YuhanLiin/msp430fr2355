#[doc = "Register `SAC1IV` reader"]
pub struct R(crate::R<SAC1IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC1IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC1IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC1IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC1IV` writer"]
pub struct W(crate::W<SAC1IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC1IV_SPEC>;
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
impl From<crate::W<SAC1IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC1IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SACIV1` reader - SAC Interrupt Vector Register"]
pub type SACIV1_R = crate::FieldReader<u16, SACIV1_A>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SACIV1_A {
    #[doc = "0: No interrupt pending"]
    SACIV_0 = 0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    SACIV_2 = 2,
    #[doc = "4: DAC channel update interrupt flag"]
    SACIV_4 = 4,
}
impl From<SACIV1_A> for u16 {
    #[inline(always)]
    fn from(variant: SACIV1_A) -> Self {
        variant as _
    }
}
impl SACIV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SACIV1_A> {
        match self.bits {
            0 => Some(SACIV1_A::SACIV_0),
            2 => Some(SACIV1_A::SACIV_2),
            4 => Some(SACIV1_A::SACIV_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SACIV_0`"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == SACIV1_A::SACIV_0
    }
    #[doc = "Checks if the value of the field is `SACIV_2`"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == SACIV1_A::SACIV_2
    }
    #[doc = "Checks if the value of the field is `SACIV_4`"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == SACIV1_A::SACIV_4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv1(&self) -> SACIV1_R {
        SACIV1_R::new(self.bits)
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
#[doc = "SAC Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1iv](index.html) module"]
pub struct SAC1IV_SPEC;
impl crate::RegisterSpec for SAC1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac1iv::R](R) reader structure"]
impl crate::Readable for SAC1IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac1iv::W](W) writer structure"]
impl crate::Writable for SAC1IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC1IV to value 0"]
impl crate::Resettable for SAC1IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
