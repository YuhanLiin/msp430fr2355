#[doc = "Register `ADCIFG` reader"]
pub struct R(crate::R<ADCIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIFG` writer"]
pub struct W(crate::W<ADCIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIFG_SPEC>;
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
impl From<crate::W<ADCIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCIFG0` reader - ADCMEM0 interrupt flag"]
pub type ADCIFG0_R = crate::BitReader<ADCIFG0_A>;
#[doc = "ADCMEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCIFG0_A {
    #[doc = "0: No interrupt pending"]
    ADCIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCIFG0_1 = 1,
}
impl From<ADCIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIFG0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCIFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIFG0_A {
        match self.bits {
            false => ADCIFG0_A::ADCIFG0_0,
            true => ADCIFG0_A::ADCIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCIFG0_0`"]
    #[inline(always)]
    pub fn is_adcifg0_0(&self) -> bool {
        *self == ADCIFG0_A::ADCIFG0_0
    }
    #[doc = "Checks if the value of the field is `ADCIFG0_1`"]
    #[inline(always)]
    pub fn is_adcifg0_1(&self) -> bool {
        *self == ADCIFG0_A::ADCIFG0_1
    }
}
#[doc = "Field `ADCIFG0` writer - ADCMEM0 interrupt flag"]
pub type ADCIFG0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCIFG0_A, O>;
impl<'a, const O: u8> ADCIFG0_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_0(self) -> &'a mut W {
        self.variant(ADCIFG0_A::ADCIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_1(self) -> &'a mut W {
        self.variant(ADCIFG0_A::ADCIFG0_1)
    }
}
#[doc = "Field `ADCINIFG` reader - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
pub type ADCINIFG_R = crate::BitReader<ADCINIFG_A>;
#[doc = "The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCINIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCINIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCINIFG_1 = 1,
}
impl From<ADCINIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCINIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCINIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCINIFG_A {
        match self.bits {
            false => ADCINIFG_A::ADCINIFG_0,
            true => ADCINIFG_A::ADCINIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCINIFG_0`"]
    #[inline(always)]
    pub fn is_adcinifg_0(&self) -> bool {
        *self == ADCINIFG_A::ADCINIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCINIFG_1`"]
    #[inline(always)]
    pub fn is_adcinifg_1(&self) -> bool {
        *self == ADCINIFG_A::ADCINIFG_1
    }
}
#[doc = "Field `ADCINIFG` writer - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
pub type ADCINIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCINIFG_A, O>;
impl<'a, const O: u8> ADCINIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_0(self) -> &'a mut W {
        self.variant(ADCINIFG_A::ADCINIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_1(self) -> &'a mut W {
        self.variant(ADCINIFG_A::ADCINIFG_1)
    }
}
#[doc = "Field `ADCLOIFG` reader - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
pub type ADCLOIFG_R = crate::BitReader<ADCLOIFG_A>;
#[doc = "The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCLOIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCLOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCLOIFG_1 = 1,
}
impl From<ADCLOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCLOIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCLOIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCLOIFG_A {
        match self.bits {
            false => ADCLOIFG_A::ADCLOIFG_0,
            true => ADCLOIFG_A::ADCLOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG_0`"]
    #[inline(always)]
    pub fn is_adcloifg_0(&self) -> bool {
        *self == ADCLOIFG_A::ADCLOIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG_1`"]
    #[inline(always)]
    pub fn is_adcloifg_1(&self) -> bool {
        *self == ADCLOIFG_A::ADCLOIFG_1
    }
}
#[doc = "Field `ADCLOIFG` writer - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
pub type ADCLOIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCLOIFG_A, O>;
impl<'a, const O: u8> ADCLOIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_0(self) -> &'a mut W {
        self.variant(ADCLOIFG_A::ADCLOIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_1(self) -> &'a mut W {
        self.variant(ADCLOIFG_A::ADCLOIFG_1)
    }
}
#[doc = "Field `ADCHIIFG` reader - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
pub type ADCHIIFG_R = crate::BitReader<ADCHIIFG_A>;
#[doc = "The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCHIIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCHIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCHIIFG_1 = 1,
}
impl From<ADCHIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHIIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCHIIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHIIFG_A {
        match self.bits {
            false => ADCHIIFG_A::ADCHIIFG_0,
            true => ADCHIIFG_A::ADCHIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG_0`"]
    #[inline(always)]
    pub fn is_adchiifg_0(&self) -> bool {
        *self == ADCHIIFG_A::ADCHIIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG_1`"]
    #[inline(always)]
    pub fn is_adchiifg_1(&self) -> bool {
        *self == ADCHIIFG_A::ADCHIIFG_1
    }
}
#[doc = "Field `ADCHIIFG` writer - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
pub type ADCHIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCHIIFG_A, O>;
impl<'a, const O: u8> ADCHIIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_0(self) -> &'a mut W {
        self.variant(ADCHIIFG_A::ADCHIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_1(self) -> &'a mut W {
        self.variant(ADCHIIFG_A::ADCHIIFG_1)
    }
}
#[doc = "Field `ADCOVIFG` reader - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
pub type ADCOVIFG_R = crate::BitReader<ADCOVIFG_A>;
#[doc = "The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCOVIFG_1 = 1,
}
impl From<ADCOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCOVIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCOVIFG_A {
        match self.bits {
            false => ADCOVIFG_A::ADCOVIFG_0,
            true => ADCOVIFG_A::ADCOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_0`"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == ADCOVIFG_A::ADCOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_1`"]
    #[inline(always)]
    pub fn is_adcovifg_1(&self) -> bool {
        *self == ADCOVIFG_A::ADCOVIFG_1
    }
}
#[doc = "Field `ADCOVIFG` writer - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
pub type ADCOVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCOVIFG_A, O>;
impl<'a, const O: u8> ADCOVIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut W {
        self.variant(ADCOVIFG_A::ADCOVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_1(self) -> &'a mut W {
        self.variant(ADCOVIFG_A::ADCOVIFG_1)
    }
}
#[doc = "Field `ADCTOVIFG` reader - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
pub type ADCTOVIFG_R = crate::BitReader<ADCTOVIFG_A>;
#[doc = "The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCTOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADCOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADCTOVIFG_1 = 1,
}
impl From<ADCTOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADCTOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCTOVIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTOVIFG_A {
        match self.bits {
            false => ADCTOVIFG_A::ADCOVIFG_0,
            true => ADCTOVIFG_A::ADCTOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG_0`"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == ADCTOVIFG_A::ADCOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADCTOVIFG_1`"]
    #[inline(always)]
    pub fn is_adctovifg_1(&self) -> bool {
        *self == ADCTOVIFG_A::ADCTOVIFG_1
    }
}
#[doc = "Field `ADCTOVIFG` writer - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
pub type ADCTOVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, ADCTOVIFG_A, O>;
impl<'a, const O: u8> ADCTOVIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut W {
        self.variant(ADCTOVIFG_A::ADCOVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adctovifg_1(self) -> &'a mut W {
        self.variant(ADCTOVIFG_A::ADCTOVIFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> ADCIFG0_R {
        ADCIFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&self) -> ADCINIFG_R {
        ADCINIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&self) -> ADCLOIFG_R {
        ADCLOIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&self) -> ADCHIIFG_R {
        ADCHIIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&self) -> ADCOVIFG_R {
        ADCOVIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&self) -> ADCTOVIFG_R {
        ADCTOVIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> ADCIFG0_W<0> {
        ADCIFG0_W::new(self)
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> ADCINIFG_W<1> {
        ADCINIFG_W::new(self)
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> ADCLOIFG_W<2> {
        ADCLOIFG_W::new(self)
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> ADCHIIFG_W<3> {
        ADCHIIFG_W::new(self)
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> ADCOVIFG_W<4> {
        ADCOVIFG_W::new(self)
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> ADCTOVIFG_W<5> {
        ADCTOVIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcifg](index.html) module"]
pub struct ADCIFG_SPEC;
impl crate::RegisterSpec for ADCIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcifg::R](R) reader structure"]
impl crate::Readable for ADCIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcifg::W](W) writer structure"]
impl crate::Writable for ADCIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIFG to value 0"]
impl crate::Resettable for ADCIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
