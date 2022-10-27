#[doc = "Register `SYSCFG1` reader"]
pub struct R(crate::R<SYSCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG1` writer"]
pub struct W(crate::W<SYSCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG1_SPEC>;
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
impl From<crate::W<SYSCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: Infrared function disabled"]
    IREN_0 = 0,
    #[doc = "1: Infrared function enabled"]
    IREN_1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::IREN_0,
            true => IREN_A::IREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREN_0`"]
    #[inline(always)]
    pub fn is_iren_0(&self) -> bool {
        *self == IREN_A::IREN_0
    }
    #[doc = "Checks if the value of the field is `IREN_1`"]
    #[inline(always)]
    pub fn is_iren_1(&self) -> bool {
        *self == IREN_A::IREN_1
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "Infrared function disabled"]
    #[inline(always)]
    pub fn iren_0(self) -> &'a mut W {
        self.variant(IREN_A::IREN_0)
    }
    #[doc = "Infrared function enabled"]
    #[inline(always)]
    pub fn iren_1(self) -> &'a mut W {
        self.variant(IREN_A::IREN_1)
    }
}
#[doc = "Field `IRPSEL` reader - Infrared polarity select"]
pub type IRPSEL_R = crate::BitReader<IRPSEL_A>;
#[doc = "Infrared polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRPSEL_A {
    #[doc = "0: Normal polarity"]
    IRPSEL_0 = 0,
    #[doc = "1: Inverted polarity"]
    IRPSEL_1 = 1,
}
impl From<IRPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IRPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPSEL_A {
        match self.bits {
            false => IRPSEL_A::IRPSEL_0,
            true => IRPSEL_A::IRPSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRPSEL_0`"]
    #[inline(always)]
    pub fn is_irpsel_0(&self) -> bool {
        *self == IRPSEL_A::IRPSEL_0
    }
    #[doc = "Checks if the value of the field is `IRPSEL_1`"]
    #[inline(always)]
    pub fn is_irpsel_1(&self) -> bool {
        *self == IRPSEL_A::IRPSEL_1
    }
}
#[doc = "Field `IRPSEL` writer - Infrared polarity select"]
pub type IRPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, IRPSEL_A, O>;
impl<'a, const O: u8> IRPSEL_W<'a, O> {
    #[doc = "Normal polarity"]
    #[inline(always)]
    pub fn irpsel_0(self) -> &'a mut W {
        self.variant(IRPSEL_A::IRPSEL_0)
    }
    #[doc = "Inverted polarity"]
    #[inline(always)]
    pub fn irpsel_1(self) -> &'a mut W {
        self.variant(IRPSEL_A::IRPSEL_1)
    }
}
#[doc = "Field `IRMSEL` reader - Infrared mode select"]
pub type IRMSEL_R = crate::BitReader<IRMSEL_A>;
#[doc = "Infrared mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRMSEL_A {
    #[doc = "0: FSK mode"]
    IRMSEL_0 = 0,
    #[doc = "1: ASK mode"]
    IRMSEL_1 = 1,
}
impl From<IRMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IRMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMSEL_A {
        match self.bits {
            false => IRMSEL_A::IRMSEL_0,
            true => IRMSEL_A::IRMSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRMSEL_0`"]
    #[inline(always)]
    pub fn is_irmsel_0(&self) -> bool {
        *self == IRMSEL_A::IRMSEL_0
    }
    #[doc = "Checks if the value of the field is `IRMSEL_1`"]
    #[inline(always)]
    pub fn is_irmsel_1(&self) -> bool {
        *self == IRMSEL_A::IRMSEL_1
    }
}
#[doc = "Field `IRMSEL` writer - Infrared mode select"]
pub type IRMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, IRMSEL_A, O>;
impl<'a, const O: u8> IRMSEL_W<'a, O> {
    #[doc = "FSK mode"]
    #[inline(always)]
    pub fn irmsel_0(self) -> &'a mut W {
        self.variant(IRMSEL_A::IRMSEL_0)
    }
    #[doc = "ASK mode"]
    #[inline(always)]
    pub fn irmsel_1(self) -> &'a mut W {
        self.variant(IRMSEL_A::IRMSEL_1)
    }
}
#[doc = "Field `IRDSSEL` reader - Infrared data source select"]
pub type IRDSSEL_R = crate::BitReader<IRDSSEL_A>;
#[doc = "Infrared data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDSSEL_A {
    #[doc = "0: From hardware peripherals upon device configuration"]
    IRDSSEL_0 = 0,
    #[doc = "1: From IRDATA bit"]
    IRDSSEL_1 = 1,
}
impl From<IRDSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IRDSSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IRDSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDSSEL_A {
        match self.bits {
            false => IRDSSEL_A::IRDSSEL_0,
            true => IRDSSEL_A::IRDSSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRDSSEL_0`"]
    #[inline(always)]
    pub fn is_irdssel_0(&self) -> bool {
        *self == IRDSSEL_A::IRDSSEL_0
    }
    #[doc = "Checks if the value of the field is `IRDSSEL_1`"]
    #[inline(always)]
    pub fn is_irdssel_1(&self) -> bool {
        *self == IRDSSEL_A::IRDSSEL_1
    }
}
#[doc = "Field `IRDSSEL` writer - Infrared data source select"]
pub type IRDSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, IRDSSEL_A, O>;
impl<'a, const O: u8> IRDSSEL_W<'a, O> {
    #[doc = "From hardware peripherals upon device configuration"]
    #[inline(always)]
    pub fn irdssel_0(self) -> &'a mut W {
        self.variant(IRDSSEL_A::IRDSSEL_0)
    }
    #[doc = "From IRDATA bit"]
    #[inline(always)]
    pub fn irdssel_1(self) -> &'a mut W {
        self.variant(IRDSSEL_A::IRDSSEL_1)
    }
}
#[doc = "Field `IRDATA` reader - Infrared data"]
pub type IRDATA_R = crate::BitReader<IRDATA_A>;
#[doc = "Infrared data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDATA_A {
    #[doc = "0: Infrared data logic 0"]
    IRDATA_0 = 0,
    #[doc = "1: Infrared data logic 1"]
    IRDATA_1 = 1,
}
impl From<IRDATA_A> for bool {
    #[inline(always)]
    fn from(variant: IRDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl IRDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDATA_A {
        match self.bits {
            false => IRDATA_A::IRDATA_0,
            true => IRDATA_A::IRDATA_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRDATA_0`"]
    #[inline(always)]
    pub fn is_irdata_0(&self) -> bool {
        *self == IRDATA_A::IRDATA_0
    }
    #[doc = "Checks if the value of the field is `IRDATA_1`"]
    #[inline(always)]
    pub fn is_irdata_1(&self) -> bool {
        *self == IRDATA_A::IRDATA_1
    }
}
#[doc = "Field `IRDATA` writer - Infrared data"]
pub type IRDATA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, IRDATA_A, O>;
impl<'a, const O: u8> IRDATA_W<'a, O> {
    #[doc = "Infrared data logic 0"]
    #[inline(always)]
    pub fn irdata_0(self) -> &'a mut W {
        self.variant(IRDATA_A::IRDATA_0)
    }
    #[doc = "Infrared data logic 1"]
    #[inline(always)]
    pub fn irdata_1(self) -> &'a mut W {
        self.variant(IRDATA_A::IRDATA_1)
    }
}
#[doc = "Field `SYNCSEL` reader - Captivate Conversion triggered Source Selection"]
pub type SYNCSEL_R = crate::FieldReader<u8, SYNCSEL_A>;
#[doc = "Captivate Conversion triggered Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSEL_A {
    #[doc = "0: External source is selected"]
    SYNCSEL_0 = 0,
    #[doc = "1: ADC as the source is selected"]
    SYNCSEL_1 = 1,
    #[doc = "2: internal source is selected"]
    SYNCSEL_2 = 2,
    #[doc = "3: Reserved"]
    SYNCSEL_3 = 3,
}
impl From<SYNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSEL_A) -> Self {
        variant as _
    }
}
impl SYNCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSEL_A {
        match self.bits {
            0 => SYNCSEL_A::SYNCSEL_0,
            1 => SYNCSEL_A::SYNCSEL_1,
            2 => SYNCSEL_A::SYNCSEL_2,
            3 => SYNCSEL_A::SYNCSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_0`"]
    #[inline(always)]
    pub fn is_syncsel_0(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_0
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_1`"]
    #[inline(always)]
    pub fn is_syncsel_1(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_1
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_2`"]
    #[inline(always)]
    pub fn is_syncsel_2(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_2
    }
    #[doc = "Checks if the value of the field is `SYNCSEL_3`"]
    #[inline(always)]
    pub fn is_syncsel_3(&self) -> bool {
        *self == SYNCSEL_A::SYNCSEL_3
    }
}
#[doc = "Field `SYNCSEL` writer - Captivate Conversion triggered Source Selection"]
pub type SYNCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SYSCFG1_SPEC, u8, SYNCSEL_A, 2, O>;
impl<'a, const O: u8> SYNCSEL_W<'a, O> {
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn syncsel_0(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_0)
    }
    #[doc = "ADC as the source is selected"]
    #[inline(always)]
    pub fn syncsel_1(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_1)
    }
    #[doc = "internal source is selected"]
    #[inline(always)]
    pub fn syncsel_2(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn syncsel_3(self) -> &'a mut W {
        self.variant(SYNCSEL_A::SYNCSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IRPSEL_R {
        IRPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IRMSEL_R {
        IRMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IRDSSEL_R {
        IRDSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&self) -> IRDATA_R {
        IRDATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&mut self) -> IRPSEL_W<1> {
        IRPSEL_W::new(self)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&mut self) -> IRMSEL_W<2> {
        IRMSEL_W::new(self)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&mut self) -> IRDSSEL_W<3> {
        IRDSSEL_W::new(self)
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&mut self) -> IRDATA_W<4> {
        IRDATA_W::new(self)
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W<6> {
        SYNCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg1](index.html) module"]
pub struct SYSCFG1_SPEC;
impl crate::RegisterSpec for SYSCFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg1::R](R) reader structure"]
impl crate::Readable for SYSCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg1::W](W) writer structure"]
impl crate::Writable for SYSCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for SYSCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
