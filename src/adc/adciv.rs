#[doc = "Register `ADCIV` reader"]
pub struct R(crate::R<ADCIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIV` writer"]
pub struct W(crate::W<ADCIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIV_SPEC>;
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
impl From<crate::W<ADCIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCIV` reader - interrupt vector value"]
pub type ADCIV_R = crate::FieldReader<u16, ADCIV_A>;
#[doc = "interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ADCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    ADCOVIFG = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    ADCTOVIFG = 4,
    #[doc = "6: Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    ADCHIIFG = 6,
    #[doc = "8: Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    ADCLOIFG = 8,
    #[doc = "10: nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    ADCINIFG = 10,
    #[doc = "12: Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    ADCIFG0 = 12,
}
impl From<ADCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: ADCIV_A) -> Self {
        variant as _
    }
}
impl ADCIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCIV_A> {
        match self.bits {
            0 => Some(ADCIV_A::NONE),
            2 => Some(ADCIV_A::ADCOVIFG),
            4 => Some(ADCIV_A::ADCTOVIFG),
            6 => Some(ADCIV_A::ADCHIIFG),
            8 => Some(ADCIV_A::ADCLOIFG),
            10 => Some(ADCIV_A::ADCINIFG),
            12 => Some(ADCIV_A::ADCIFG0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `ADCOVIFG`"]
    #[inline(always)]
    pub fn is_adcovifg(&self) -> bool {
        *self == ADCIV_A::ADCOVIFG
    }
    #[doc = "Checks if the value of the field is `ADCTOVIFG`"]
    #[inline(always)]
    pub fn is_adctovifg(&self) -> bool {
        *self == ADCIV_A::ADCTOVIFG
    }
    #[doc = "Checks if the value of the field is `ADCHIIFG`"]
    #[inline(always)]
    pub fn is_adchiifg(&self) -> bool {
        *self == ADCIV_A::ADCHIIFG
    }
    #[doc = "Checks if the value of the field is `ADCLOIFG`"]
    #[inline(always)]
    pub fn is_adcloifg(&self) -> bool {
        *self == ADCIV_A::ADCLOIFG
    }
    #[doc = "Checks if the value of the field is `ADCINIFG`"]
    #[inline(always)]
    pub fn is_adcinifg(&self) -> bool {
        *self == ADCIV_A::ADCINIFG
    }
    #[doc = "Checks if the value of the field is `ADCIFG0`"]
    #[inline(always)]
    pub fn is_adcifg0(&self) -> bool {
        *self == ADCIV_A::ADCIFG0
    }
}
#[doc = "Field `ADCIV` writer - interrupt vector value"]
pub type ADCIV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADCIV_SPEC, u16, ADCIV_A, 16, O>;
impl<'a, const O: u8> ADCIV_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADCIV_A::NONE)
    }
    #[doc = "Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adcovifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCOVIFG)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn adctovifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCTOVIFG)
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn adchiifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCHIIFG)
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn adcloifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCLOIFG)
    }
    #[doc = "nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn adcinifg(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCINIFG)
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn adcifg0(self) -> &'a mut W {
        self.variant(ADCIV_A::ADCIFG0)
    }
}
impl R {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&self) -> ADCIV_R {
        ADCIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&mut self) -> ADCIV_W<0> {
        ADCIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adciv](index.html) module"]
pub struct ADCIV_SPEC;
impl crate::RegisterSpec for ADCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adciv::R](R) reader structure"]
impl crate::Readable for ADCIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adciv::W](W) writer structure"]
impl crate::Writable for ADCIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIV to value 0"]
impl crate::Resettable for ADCIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
