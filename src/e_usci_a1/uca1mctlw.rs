#[doc = "Register `UCA1MCTLW` reader"]
pub struct R(crate::R<UCA1MCTLW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1MCTLW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1MCTLW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1MCTLW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1MCTLW` writer"]
pub struct W(crate::W<UCA1MCTLW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1MCTLW_SPEC>;
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
impl From<crate::W<UCA1MCTLW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1MCTLW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOS16` reader - Oversampling mode enabled"]
pub type UCOS16_R = crate::BitReader<UCOS16_A>;
#[doc = "Oversampling mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCOS16_A {
    #[doc = "0: Disabled"]
    UCOS16_0 = 0,
    #[doc = "1: Enabled"]
    UCOS16_1 = 1,
}
impl From<UCOS16_A> for bool {
    #[inline(always)]
    fn from(variant: UCOS16_A) -> Self {
        variant as u8 != 0
    }
}
impl UCOS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOS16_A {
        match self.bits {
            false => UCOS16_A::UCOS16_0,
            true => UCOS16_A::UCOS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOS16_0`"]
    #[inline(always)]
    pub fn is_ucos16_0(&self) -> bool {
        *self == UCOS16_A::UCOS16_0
    }
    #[doc = "Checks if the value of the field is `UCOS16_1`"]
    #[inline(always)]
    pub fn is_ucos16_1(&self) -> bool {
        *self == UCOS16_A::UCOS16_1
    }
}
#[doc = "Field `UCOS16` writer - Oversampling mode enabled"]
pub type UCOS16_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1MCTLW_SPEC, UCOS16_A, O>;
impl<'a, const O: u8> UCOS16_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ucos16_0(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ucos16_1(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_1)
    }
}
#[doc = "Field `UCBRF` reader - First modulation stage select"]
pub type UCBRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCBRF` writer - First modulation stage select"]
pub type UCBRF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCA1MCTLW_SPEC, u8, u8, 4, O>;
#[doc = "Field `UCBRS` reader - Second modulation stage select"]
pub type UCBRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCBRS` writer - Second modulation stage select"]
pub type UCBRS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCA1MCTLW_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W<0> {
        UCOS16_W::new(self)
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W<4> {
        UCBRF_W::new(self)
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UCBRS_W<8> {
        UCBRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1mctlw](index.html) module"]
pub struct UCA1MCTLW_SPEC;
impl crate::RegisterSpec for UCA1MCTLW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca1mctlw::R](R) reader structure"]
impl crate::Readable for UCA1MCTLW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1mctlw::W](W) writer structure"]
impl crate::Writable for UCA1MCTLW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1MCTLW to value 0"]
impl crate::Resettable for UCA1MCTLW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
