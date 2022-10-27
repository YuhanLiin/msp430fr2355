#[doc = "Register `RTCIV` reader"]
pub struct R(crate::R<RTCIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCIV` writer"]
pub struct W(crate::W<RTCIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCIV_SPEC>;
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
impl From<crate::W<RTCIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCIV` reader - Real-time clock interrupt vector value"]
pub type RTCIV_R = crate::FieldReader<u16, RTCIV_A>;
#[doc = "Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum RTCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: upt Source: RTC Counter Overflow; Interrupt Flag: RTCIFG"]
    RTCIFG = 2,
}
impl From<RTCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: RTCIV_A) -> Self {
        variant as _
    }
}
impl RTCIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCIV_A> {
        match self.bits {
            0 => Some(RTCIV_A::NONE),
            2 => Some(RTCIV_A::RTCIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RTCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `RTCIFG`"]
    #[inline(always)]
    pub fn is_rtcifg(&self) -> bool {
        *self == RTCIV_A::RTCIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RTCIV_R {
        RTCIV_R::new(self.bits)
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
#[doc = "Real-Time Clock Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](index.html) module"]
pub struct RTCIV_SPEC;
impl crate::RegisterSpec for RTCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtciv::R](R) reader structure"]
impl crate::Readable for RTCIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtciv::W](W) writer structure"]
impl crate::Writable for RTCIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RTCIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
