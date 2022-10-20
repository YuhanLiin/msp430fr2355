#[doc = "Register `ADCIE` reader"]
pub struct R(crate::R<ADCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIE` writer"]
pub struct W(crate::W<ADCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIE_SPEC>;
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
impl From<crate::W<ADCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCIE0` reader - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
pub type ADCIE0_R = crate::BitReader<ADCIE0_A>;
#[doc = "Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCIE0_A {
    #[doc = "0: 0b = Interrupt disabled"]
    ADCIE0_0 = 0,
    #[doc = "1: 1b = Interrupt enabled"]
    ADCIE0_1 = 1,
}
impl From<ADCIE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIE0_A {
        match self.bits {
            false => ADCIE0_A::ADCIE0_0,
            true => ADCIE0_A::ADCIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCIE0_0`"]
    #[inline(always)]
    pub fn is_adcie0_0(&self) -> bool {
        *self == ADCIE0_A::ADCIE0_0
    }
    #[doc = "Checks if the value of the field is `ADCIE0_1`"]
    #[inline(always)]
    pub fn is_adcie0_1(&self) -> bool {
        *self == ADCIE0_A::ADCIE0_1
    }
}
#[doc = "Field `ADCIE0` writer - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
pub type ADCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCIE0_A, O>;
impl<'a, const O: u8> ADCIE0_W<'a, O> {
    #[doc = "0b = Interrupt disabled"]
    #[inline(always)]
    pub fn adcie0_0(self) -> &'a mut W {
        self.variant(ADCIE0_A::ADCIE0_0)
    }
    #[doc = "1b = Interrupt enabled"]
    #[inline(always)]
    pub fn adcie0_1(self) -> &'a mut W {
        self.variant(ADCIE0_A::ADCIE0_1)
    }
}
#[doc = "Field `ADCINIE` reader - Interrupt enable for the inside of window interrupt of the window comparator."]
pub type ADCINIE_R = crate::BitReader<ADCINIE_A>;
#[doc = "Interrupt enable for the inside of window interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCINIE_A {
    #[doc = "0: 0b = Inside of window interrupt disabled"]
    ADCINIE_0 = 0,
    #[doc = "1: 1b = Inside of window interrupt enabled"]
    ADCINIE_1 = 1,
}
impl From<ADCINIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCINIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCINIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINIE_A {
        match self.bits {
            false => ADCINIE_A::ADCINIE_0,
            true => ADCINIE_A::ADCINIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCINIE_0`"]
    #[inline(always)]
    pub fn is_adcinie_0(&self) -> bool {
        *self == ADCINIE_A::ADCINIE_0
    }
    #[doc = "Checks if the value of the field is `ADCINIE_1`"]
    #[inline(always)]
    pub fn is_adcinie_1(&self) -> bool {
        *self == ADCINIE_A::ADCINIE_1
    }
}
#[doc = "Field `ADCINIE` writer - Interrupt enable for the inside of window interrupt of the window comparator."]
pub type ADCINIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCINIE_A, O>;
impl<'a, const O: u8> ADCINIE_W<'a, O> {
    #[doc = "0b = Inside of window interrupt disabled"]
    #[inline(always)]
    pub fn adcinie_0(self) -> &'a mut W {
        self.variant(ADCINIE_A::ADCINIE_0)
    }
    #[doc = "1b = Inside of window interrupt enabled"]
    #[inline(always)]
    pub fn adcinie_1(self) -> &'a mut W {
        self.variant(ADCINIE_A::ADCINIE_1)
    }
}
#[doc = "Field `ADCLOIE` reader - Interrupt enable for the below lower threshold interrupt of the window comparator."]
pub type ADCLOIE_R = crate::BitReader<ADCLOIE_A>;
#[doc = "Interrupt enable for the below lower threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCLOIE_A {
    #[doc = "0: 0b = Below lower threshold interrupt disabled"]
    ADCLOIE_0 = 0,
    #[doc = "1: 1b = Below lower threshold interrupt enabled"]
    ADCLOIE_1 = 1,
}
impl From<ADCLOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCLOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCLOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCLOIE_A {
        match self.bits {
            false => ADCLOIE_A::ADCLOIE_0,
            true => ADCLOIE_A::ADCLOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCLOIE_0`"]
    #[inline(always)]
    pub fn is_adcloie_0(&self) -> bool {
        *self == ADCLOIE_A::ADCLOIE_0
    }
    #[doc = "Checks if the value of the field is `ADCLOIE_1`"]
    #[inline(always)]
    pub fn is_adcloie_1(&self) -> bool {
        *self == ADCLOIE_A::ADCLOIE_1
    }
}
#[doc = "Field `ADCLOIE` writer - Interrupt enable for the below lower threshold interrupt of the window comparator."]
pub type ADCLOIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCLOIE_A, O>;
impl<'a, const O: u8> ADCLOIE_W<'a, O> {
    #[doc = "0b = Below lower threshold interrupt disabled"]
    #[inline(always)]
    pub fn adcloie_0(self) -> &'a mut W {
        self.variant(ADCLOIE_A::ADCLOIE_0)
    }
    #[doc = "1b = Below lower threshold interrupt enabled"]
    #[inline(always)]
    pub fn adcloie_1(self) -> &'a mut W {
        self.variant(ADCLOIE_A::ADCLOIE_1)
    }
}
#[doc = "Field `ADCHIIE` reader - Interrupt enable for the above upper threshold interrupt of the window comparator."]
pub type ADCHIIE_R = crate::BitReader<ADCHIIE_A>;
#[doc = "Interrupt enable for the above upper threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCHIIE_A {
    #[doc = "0: 0b = Above upper threshold interrupt disabled"]
    ADCHIIE_0 = 0,
    #[doc = "1: 1b = Above upper threshold interrupt enabled"]
    ADCHIIE_1 = 1,
}
impl From<ADCHIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCHIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHIIE_A {
        match self.bits {
            false => ADCHIIE_A::ADCHIIE_0,
            true => ADCHIIE_A::ADCHIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCHIIE_0`"]
    #[inline(always)]
    pub fn is_adchiie_0(&self) -> bool {
        *self == ADCHIIE_A::ADCHIIE_0
    }
    #[doc = "Checks if the value of the field is `ADCHIIE_1`"]
    #[inline(always)]
    pub fn is_adchiie_1(&self) -> bool {
        *self == ADCHIIE_A::ADCHIIE_1
    }
}
#[doc = "Field `ADCHIIE` writer - Interrupt enable for the above upper threshold interrupt of the window comparator."]
pub type ADCHIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCHIIE_A, O>;
impl<'a, const O: u8> ADCHIIE_W<'a, O> {
    #[doc = "0b = Above upper threshold interrupt disabled"]
    #[inline(always)]
    pub fn adchiie_0(self) -> &'a mut W {
        self.variant(ADCHIIE_A::ADCHIIE_0)
    }
    #[doc = "1b = Above upper threshold interrupt enabled"]
    #[inline(always)]
    pub fn adchiie_1(self) -> &'a mut W {
        self.variant(ADCHIIE_A::ADCHIIE_1)
    }
}
#[doc = "Field `ADCOVIE` reader - ADCMEM0 overflow interrupt enable."]
pub type ADCOVIE_R = crate::BitReader<ADCOVIE_A>;
#[doc = "ADCMEM0 overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCOVIE_A {
    #[doc = "0: 0b = Overflow interrupt disabled"]
    ADCOVIE_0 = 0,
    #[doc = "1: 1b = Overflow interrupt enabled"]
    ADCOVIE_1 = 1,
}
impl From<ADCOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCOVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCOVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCOVIE_A {
        match self.bits {
            false => ADCOVIE_A::ADCOVIE_0,
            true => ADCOVIE_A::ADCOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIE_0`"]
    #[inline(always)]
    pub fn is_adcovie_0(&self) -> bool {
        *self == ADCOVIE_A::ADCOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADCOVIE_1`"]
    #[inline(always)]
    pub fn is_adcovie_1(&self) -> bool {
        *self == ADCOVIE_A::ADCOVIE_1
    }
}
#[doc = "Field `ADCOVIE` writer - ADCMEM0 overflow interrupt enable."]
pub type ADCOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCOVIE_A, O>;
impl<'a, const O: u8> ADCOVIE_W<'a, O> {
    #[doc = "0b = Overflow interrupt disabled"]
    #[inline(always)]
    pub fn adcovie_0(self) -> &'a mut W {
        self.variant(ADCOVIE_A::ADCOVIE_0)
    }
    #[doc = "1b = Overflow interrupt enabled"]
    #[inline(always)]
    pub fn adcovie_1(self) -> &'a mut W {
        self.variant(ADCOVIE_A::ADCOVIE_1)
    }
}
#[doc = "Field `ADCTOVIE` reader - ADC conversion-time-overflow interrupt enable."]
pub type ADCTOVIE_R = crate::BitReader<ADCTOVIE_A>;
#[doc = "ADC conversion-time-overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCTOVIE_A {
    #[doc = "0: 0b = Conversion time overflow interrupt disabled"]
    ADCTOVIE_0 = 0,
    #[doc = "1: 1b = Conversion time overflow interrupt enabled"]
    ADCTOVIE_1 = 1,
}
impl From<ADCTOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTOVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCTOVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTOVIE_A {
        match self.bits {
            false => ADCTOVIE_A::ADCTOVIE_0,
            true => ADCTOVIE_A::ADCTOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCTOVIE_0`"]
    #[inline(always)]
    pub fn is_adctovie_0(&self) -> bool {
        *self == ADCTOVIE_A::ADCTOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADCTOVIE_1`"]
    #[inline(always)]
    pub fn is_adctovie_1(&self) -> bool {
        *self == ADCTOVIE_A::ADCTOVIE_1
    }
}
#[doc = "Field `ADCTOVIE` writer - ADC conversion-time-overflow interrupt enable."]
pub type ADCTOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, ADCTOVIE_A, O>;
impl<'a, const O: u8> ADCTOVIE_W<'a, O> {
    #[doc = "0b = Conversion time overflow interrupt disabled"]
    #[inline(always)]
    pub fn adctovie_0(self) -> &'a mut W {
        self.variant(ADCTOVIE_A::ADCTOVIE_0)
    }
    #[doc = "1b = Conversion time overflow interrupt enabled"]
    #[inline(always)]
    pub fn adctovie_1(self) -> &'a mut W {
        self.variant(ADCTOVIE_A::ADCTOVIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&self) -> ADCIE0_R {
        ADCIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&self) -> ADCINIE_R {
        ADCINIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&self) -> ADCLOIE_R {
        ADCLOIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&self) -> ADCHIIE_R {
        ADCHIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&self) -> ADCOVIE_R {
        ADCOVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&self) -> ADCTOVIE_R {
        ADCTOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&mut self) -> ADCIE0_W<0> {
        ADCIE0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&mut self) -> ADCINIE_W<1> {
        ADCINIE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&mut self) -> ADCLOIE_W<2> {
        ADCLOIE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&mut self) -> ADCHIIE_W<3> {
        ADCHIIE_W::new(self)
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&mut self) -> ADCOVIE_W<4> {
        ADCOVIE_W::new(self)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&mut self) -> ADCTOVIE_W<5> {
        ADCTOVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcie](index.html) module"]
pub struct ADCIE_SPEC;
impl crate::RegisterSpec for ADCIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcie::R](R) reader structure"]
impl crate::Readable for ADCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcie::W](W) writer structure"]
impl crate::Writable for ADCIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIE to value 0"]
impl crate::Resettable for ADCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
