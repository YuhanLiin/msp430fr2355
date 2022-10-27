#[doc = "Register `FRCTL0` reader"]
pub struct R(crate::R<FRCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCTL0` writer"]
pub struct W(crate::W<FRCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCTL0_SPEC>;
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
impl From<crate::W<FRCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWAITS` reader - Wait state numbers"]
pub type NWAITS_R = crate::FieldReader<u8, NWAITS_A>;
#[doc = "Wait state numbers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NWAITS_A {
    #[doc = "0: FRAM wait states: 0"]
    NWAITS_0 = 0,
    #[doc = "1: FRAM wait states: 1"]
    NWAITS_1 = 1,
    #[doc = "2: FRAM wait states: 2"]
    NWAITS_2 = 2,
    #[doc = "3: FRAM wait states: 3"]
    NWAITS_3 = 3,
    #[doc = "4: FRAM wait states: 4"]
    NWAITS_4 = 4,
    #[doc = "5: FRAM wait states: 5"]
    NWAITS_5 = 5,
    #[doc = "6: FRAM wait states: 6"]
    NWAITS_6 = 6,
    #[doc = "7: FRAM wait states: 7"]
    NWAITS_7 = 7,
}
impl From<NWAITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NWAITS_A) -> Self {
        variant as _
    }
}
impl NWAITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NWAITS_A {
        match self.bits {
            0 => NWAITS_A::NWAITS_0,
            1 => NWAITS_A::NWAITS_1,
            2 => NWAITS_A::NWAITS_2,
            3 => NWAITS_A::NWAITS_3,
            4 => NWAITS_A::NWAITS_4,
            5 => NWAITS_A::NWAITS_5,
            6 => NWAITS_A::NWAITS_6,
            7 => NWAITS_A::NWAITS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NWAITS_0`"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == NWAITS_A::NWAITS_0
    }
    #[doc = "Checks if the value of the field is `NWAITS_1`"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == NWAITS_A::NWAITS_1
    }
    #[doc = "Checks if the value of the field is `NWAITS_2`"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == NWAITS_A::NWAITS_2
    }
    #[doc = "Checks if the value of the field is `NWAITS_3`"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == NWAITS_A::NWAITS_3
    }
    #[doc = "Checks if the value of the field is `NWAITS_4`"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == NWAITS_A::NWAITS_4
    }
    #[doc = "Checks if the value of the field is `NWAITS_5`"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == NWAITS_A::NWAITS_5
    }
    #[doc = "Checks if the value of the field is `NWAITS_6`"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == NWAITS_A::NWAITS_6
    }
    #[doc = "Checks if the value of the field is `NWAITS_7`"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == NWAITS_A::NWAITS_7
    }
}
#[doc = "Field `NWAITS` writer - Wait state numbers"]
pub type NWAITS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, FRCTL0_SPEC, u8, NWAITS_A, 3, O>;
impl<'a, const O: u8> NWAITS_W<'a, O> {
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_0)
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_1)
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_2)
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_3)
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_4)
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_5)
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_6)
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_7)
    }
}
#[doc = "Field `FRCTLPW` reader - FRCTLPW password"]
pub type FRCTLPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRCTLPW` writer - FRCTLPW password"]
pub type FRCTLPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FRCTL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&self) -> NWAITS_R {
        NWAITS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FRCTLPW_R {
        FRCTLPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&mut self) -> NWAITS_W<4> {
        NWAITS_W::new(self)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FRCTLPW_W<8> {
        FRCTLPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRAM Controller Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frctl0](index.html) module"]
pub struct FRCTL0_SPEC;
impl crate::RegisterSpec for FRCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [frctl0::R](R) reader structure"]
impl crate::Readable for FRCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frctl0::W](W) writer structure"]
impl crate::Writable for FRCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRCTL0 to value 0"]
impl crate::Resettable for FRCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
