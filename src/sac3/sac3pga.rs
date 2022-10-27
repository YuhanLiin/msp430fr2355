#[doc = "Register `SAC3PGA` reader"]
pub struct R(crate::R<SAC3PGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC3PGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC3PGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC3PGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC3PGA` writer"]
pub struct W(crate::W<SAC3PGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC3PGA_SPEC>;
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
impl From<crate::W<SAC3PGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC3PGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - SAC PGA Mode Selection"]
pub type MSEL_R = crate::FieldReader<u8, MSEL_A>;
#[doc = "SAC PGA Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Inverting PGA mode (external pad IN- is selected)"]
    MSEL_0 = 0,
    #[doc = "1: Buffer mode (floating is selected )"]
    MSEL_1 = 1,
    #[doc = "2: Non-inverting mode"]
    MSEL_2 = 2,
    #[doc = "3: Cascade OA Inverting mode"]
    MSEL_3 = 3,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
}
#[doc = "Field `MSEL` writer - SAC PGA Mode Selection"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, SAC3PGA_SPEC, u8, MSEL_A, 2, O>;
impl<'a, const O: u8> MSEL_W<'a, O> {
    #[doc = "Inverting PGA mode (external pad IN- is selected)"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "Buffer mode (floating is selected )"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "Non-inverting mode"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "Cascade OA Inverting mode"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
}
#[doc = "Field `GAIN` reader - SAC PGA Gain configuration"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - SAC PGA Gain configuration"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SAC3PGA_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<0> {
        MSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W<4> {
        GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAC PGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3pga](index.html) module"]
pub struct SAC3PGA_SPEC;
impl crate::RegisterSpec for SAC3PGA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sac3pga::R](R) reader structure"]
impl crate::Readable for SAC3PGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac3pga::W](W) writer structure"]
impl crate::Writable for SAC3PGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC3PGA to value 0"]
impl crate::Resettable for SAC3PGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
