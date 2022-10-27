#[doc = "Register `ADCCTL2` reader"]
pub struct R(crate::R<ADCCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTL2` writer"]
pub struct W(crate::W<ADCCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTL2_SPEC>;
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
impl From<crate::W<ADCCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSR` reader - ADC sampling rate."]
pub type ADCSR_R = crate::BitReader<bool>;
#[doc = "Field `ADCSR` writer - ADC sampling rate."]
pub type ADCSR_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL2_SPEC, bool, O>;
#[doc = "Field `ADCDF` reader - data read-back format"]
pub type ADCDF_R = crate::BitReader<ADCDF_A>;
#[doc = "data read-back format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCDF_A {
    #[doc = "0: Binary unsigned. Theoretically the analog input voltage V(REF) results in 0000h, the analog input voltage +V(REF) results in 03FFh."]
    ADCDF_0 = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically the analog input voltage V(REF) results in 8000h, the analog input voltage +V(REF) results in 7FC0h."]
    ADCDF_1 = 1,
}
impl From<ADCDF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCDF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCDF_A {
        match self.bits {
            false => ADCDF_A::ADCDF_0,
            true => ADCDF_A::ADCDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCDF_0`"]
    #[inline(always)]
    pub fn is_adcdf_0(&self) -> bool {
        *self == ADCDF_A::ADCDF_0
    }
    #[doc = "Checks if the value of the field is `ADCDF_1`"]
    #[inline(always)]
    pub fn is_adcdf_1(&self) -> bool {
        *self == ADCDF_A::ADCDF_1
    }
}
#[doc = "Field `ADCDF` writer - data read-back format"]
pub type ADCDF_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL2_SPEC, ADCDF_A, O>;
impl<'a, const O: u8> ADCDF_W<'a, O> {
    #[doc = "Binary unsigned. Theoretically the analog input voltage V(REF) results in 0000h, the analog input voltage +V(REF) results in 03FFh."]
    #[inline(always)]
    pub fn adcdf_0(self) -> &'a mut W {
        self.variant(ADCDF_A::ADCDF_0)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically the analog input voltage V(REF) results in 8000h, the analog input voltage +V(REF) results in 7FC0h."]
    #[inline(always)]
    pub fn adcdf_1(self) -> &'a mut W {
        self.variant(ADCDF_A::ADCDF_1)
    }
}
#[doc = "Field `ADCRES` reader - resolution"]
pub type ADCRES_R = crate::FieldReader<u8, ADCRES_A>;
#[doc = "resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCRES_A {
    #[doc = "0: 8 bit"]
    ADCRES_0 = 0,
    #[doc = "1: 10 bit"]
    ADCRES_1 = 1,
    #[doc = "2: 12 bit"]
    ADCRES_2 = 2,
    #[doc = "3: Reserved"]
    ADCRES_3 = 3,
}
impl From<ADCRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCRES_A) -> Self {
        variant as _
    }
}
impl ADCRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCRES_A {
        match self.bits {
            0 => ADCRES_A::ADCRES_0,
            1 => ADCRES_A::ADCRES_1,
            2 => ADCRES_A::ADCRES_2,
            3 => ADCRES_A::ADCRES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCRES_0`"]
    #[inline(always)]
    pub fn is_adcres_0(&self) -> bool {
        *self == ADCRES_A::ADCRES_0
    }
    #[doc = "Checks if the value of the field is `ADCRES_1`"]
    #[inline(always)]
    pub fn is_adcres_1(&self) -> bool {
        *self == ADCRES_A::ADCRES_1
    }
    #[doc = "Checks if the value of the field is `ADCRES_2`"]
    #[inline(always)]
    pub fn is_adcres_2(&self) -> bool {
        *self == ADCRES_A::ADCRES_2
    }
    #[doc = "Checks if the value of the field is `ADCRES_3`"]
    #[inline(always)]
    pub fn is_adcres_3(&self) -> bool {
        *self == ADCRES_A::ADCRES_3
    }
}
#[doc = "Field `ADCRES` writer - resolution"]
pub type ADCRES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCCTL2_SPEC, u8, ADCRES_A, 2, O>;
impl<'a, const O: u8> ADCRES_W<'a, O> {
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn adcres_0(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_0)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn adcres_1(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_1)
    }
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn adcres_2(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_3(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_3)
    }
}
#[doc = "Field `ADCPDIV` reader - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
pub type ADCPDIV_R = crate::FieldReader<u8, ADCPDIV_A>;
#[doc = "ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPDIV_A {
    #[doc = "0: Predivide by 1"]
    _1 = 0,
    #[doc = "1: Predivide by 4"]
    _4 = 1,
    #[doc = "2: Predivide by 64"]
    _64 = 2,
    #[doc = "3: Reserved"]
    ADCPDIV_3 = 3,
}
impl From<ADCPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPDIV_A) -> Self {
        variant as _
    }
}
impl ADCPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPDIV_A {
        match self.bits {
            0 => ADCPDIV_A::_1,
            1 => ADCPDIV_A::_4,
            2 => ADCPDIV_A::_64,
            3 => ADCPDIV_A::ADCPDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADCPDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == ADCPDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == ADCPDIV_A::_64
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_3`"]
    #[inline(always)]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_3
    }
}
#[doc = "Field `ADCPDIV` writer - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
pub type ADCPDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCCTL2_SPEC, u8, ADCPDIV_A, 2, O>;
impl<'a, const O: u8> ADCPDIV_W<'a, O> {
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCPDIV_A::_1)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ADCPDIV_A::_4)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(ADCPDIV_A::_64)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcpdiv_3(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_3)
    }
}
impl R {
    #[doc = "Bit 2 - ADC sampling rate."]
    #[inline(always)]
    pub fn adcsr(&self) -> ADCSR_R {
        ADCSR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adcdf(&self) -> ADCDF_R {
        ADCDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> ADCRES_R {
        ADCRES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
    #[inline(always)]
    pub fn adcpdiv(&self) -> ADCPDIV_R {
        ADCPDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC sampling rate."]
    #[inline(always)]
    pub fn adcsr(&mut self) -> ADCSR_W<2> {
        ADCSR_W::new(self)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adcdf(&mut self) -> ADCDF_W<3> {
        ADCDF_W::new(self)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adcres(&mut self) -> ADCRES_W<4> {
        ADCRES_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
    #[inline(always)]
    pub fn adcpdiv(&mut self) -> ADCPDIV_W<8> {
        ADCPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl2](index.html) module"]
pub struct ADCCTL2_SPEC;
impl crate::RegisterSpec for ADCCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcctl2::R](R) reader structure"]
impl crate::Readable for ADCCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctl2::W](W) writer structure"]
impl crate::Writable for ADCCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for ADCCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
