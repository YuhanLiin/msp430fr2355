#[doc = "Register `UCA0CTLW1` reader"]
pub struct R(crate::R<UCA0CTLW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0CTLW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0CTLW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0CTLW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0CTLW1` writer"]
pub struct W(crate::W<UCA0CTLW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0CTLW1_SPEC>;
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
impl From<crate::W<UCA0CTLW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0CTLW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCGLIT` reader - Deglitch time"]
pub type UCGLIT_R = crate::FieldReader<u8, UCGLIT_A>;
#[doc = "Deglitch time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: Approximately 2 ns (equivalent of 1 delay element)"]
    UCGLIT_0 = 0,
    #[doc = "1: Approximately 50 ns"]
    UCGLIT_1 = 1,
    #[doc = "2: Approximately 100 ns"]
    UCGLIT_2 = 2,
    #[doc = "3: Approximately 200 ns"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
impl UCGLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_3
    }
}
#[doc = "Field `UCGLIT` writer - Deglitch time"]
pub type UCGLIT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCA0CTLW1_SPEC, u8, UCGLIT_A, 2, O>;
impl<'a, const O: u8> UCGLIT_W<'a, O> {
    #[doc = "Approximately 2 ns (equivalent of 1 delay element)"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "Approximately 50 ns"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "Approximately 100 ns"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "Approximately 200 ns"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W<0> {
        UCGLIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctlw1](index.html) module"]
pub struct UCA0CTLW1_SPEC;
impl crate::RegisterSpec for UCA0CTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0ctlw1::R](R) reader structure"]
impl crate::Readable for UCA0CTLW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ctlw1::W](W) writer structure"]
impl crate::Writable for UCA0CTLW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0CTLW1 to value 0"]
impl crate::Resettable for UCA0CTLW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
