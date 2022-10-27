#[doc = "Register `GCCTL1` reader"]
pub struct R(crate::R<GCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL1` writer"]
pub struct W(crate::W<GCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL1_SPEC>;
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
impl From<crate::W<GCCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error detection flag"]
pub type CBDIFG_R = crate::BitReader<CBDIFG_A>;
#[doc = "FRAM correctable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBDIFG_A {
    #[doc = "0: No interrupt is pending"]
    CBDIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    CBDIFG_1 = 1,
}
impl From<CBDIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CBDIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl CBDIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBDIFG_A {
        match self.bits {
            false => CBDIFG_A::CBDIFG_0,
            true => CBDIFG_A::CBDIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CBDIFG_0`"]
    #[inline(always)]
    pub fn is_cbdifg_0(&self) -> bool {
        *self == CBDIFG_A::CBDIFG_0
    }
    #[doc = "Checks if the value of the field is `CBDIFG_1`"]
    #[inline(always)]
    pub fn is_cbdifg_1(&self) -> bool {
        *self == CBDIFG_A::CBDIFG_1
    }
}
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error detection flag"]
pub type CBDIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, CBDIFG_A, O>;
impl<'a, const O: u8> CBDIFG_W<'a, O> {
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn cbdifg_0(self) -> &'a mut W {
        self.variant(CBDIFG_A::CBDIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV if it is the highest pending interrupt."]
    #[inline(always)]
    pub fn cbdifg_1(self) -> &'a mut W {
        self.variant(CBDIFG_A::CBDIFG_1)
    }
}
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error detection flag"]
pub type UBDIFG_R = crate::BitReader<UBDIFG_A>;
#[doc = "FRAM uncorrectable bit error detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UBDIFG_A {
    #[doc = "0: No interrupt pending."]
    UBDIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    UBDIFG_1 = 1,
}
impl From<UBDIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UBDIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UBDIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDIFG_A {
        match self.bits {
            false => UBDIFG_A::UBDIFG_0,
            true => UBDIFG_A::UBDIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDIFG_0`"]
    #[inline(always)]
    pub fn is_ubdifg_0(&self) -> bool {
        *self == UBDIFG_A::UBDIFG_0
    }
    #[doc = "Checks if the value of the field is `UBDIFG_1`"]
    #[inline(always)]
    pub fn is_ubdifg_1(&self) -> bool {
        *self == UBDIFG_A::UBDIFG_1
    }
}
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error detection flag"]
pub type UBDIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, UBDIFG_A, O>;
impl<'a, const O: u8> UBDIFG_W<'a, O> {
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn ubdifg_0(self) -> &'a mut W {
        self.variant(UBDIFG_A::UBDIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn ubdifg_1(self) -> &'a mut W {
        self.variant(UBDIFG_A::UBDIFG_1)
    }
}
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub type ACCTEIFG_R = crate::BitReader<ACCTEIFG_A>;
#[doc = "Access time error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCTEIFG_A {
    #[doc = "0: No interrupt pending."]
    ACCTEIFG_0 = 0,
    #[doc = "1: Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    ACCTEIFG_1 = 1,
}
impl From<ACCTEIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ACCTEIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCTEIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCTEIFG_A {
        match self.bits {
            false => ACCTEIFG_A::ACCTEIFG_0,
            true => ACCTEIFG_A::ACCTEIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG_0`"]
    #[inline(always)]
    pub fn is_accteifg_0(&self) -> bool {
        *self == ACCTEIFG_A::ACCTEIFG_0
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG_1`"]
    #[inline(always)]
    pub fn is_accteifg_1(&self) -> bool {
        *self == ACCTEIFG_A::ACCTEIFG_1
    }
}
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub type ACCTEIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, ACCTEIFG_A, O>;
impl<'a, const O: u8> ACCTEIFG_W<'a, O> {
    #[doc = "No interrupt pending."]
    #[inline(always)]
    pub fn accteifg_0(self) -> &'a mut W {
        self.variant(ACCTEIFG_A::ACCTEIFG_0)
    }
    #[doc = "Interrupt pending. Can be cleared by writing '0' or by reading SYSSNIV when it is the highest pending interrupt."]
    #[inline(always)]
    pub fn accteifg_1(self) -> &'a mut W {
        self.variant(ACCTEIFG_A::ACCTEIFG_1)
    }
}
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error detection flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CBDIFG_W<1> {
        CBDIFG_W::new(self)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error detection flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UBDIFG_W<2> {
        UBDIFG_W::new(self)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> ACCTEIFG_W<3> {
        ACCTEIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl1](index.html) module"]
pub struct GCCTL1_SPEC;
impl crate::RegisterSpec for GCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl1::R](R) reader structure"]
impl crate::Readable for GCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl1::W](W) writer structure"]
impl crate::Writable for GCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for GCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
