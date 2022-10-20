#[doc = "Register `TB3EX0` reader"]
pub struct R(crate::R<TB3EX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TB3EX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TB3EX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TB3EX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TB3EX0` writer"]
pub struct W(crate::W<TB3EX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TB3EX0_SPEC>;
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
impl From<crate::W<TB3EX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TB3EX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBIDEX` reader - Input divider expansion"]
pub type TBIDEX_R = crate::FieldReader<u8, TBIDEX_A>;
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBIDEX_A {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<TBIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TBIDEX_A) -> Self {
        variant as _
    }
}
impl TBIDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIDEX_A {
        match self.bits {
            0 => TBIDEX_A::_1,
            1 => TBIDEX_A::_2,
            2 => TBIDEX_A::_3,
            3 => TBIDEX_A::_4,
            4 => TBIDEX_A::_5,
            5 => TBIDEX_A::_6,
            6 => TBIDEX_A::_7,
            7 => TBIDEX_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBIDEX_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TBIDEX_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TBIDEX_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == TBIDEX_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == TBIDEX_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == TBIDEX_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == TBIDEX_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == TBIDEX_A::_8
    }
}
#[doc = "Field `TBIDEX` writer - Input divider expansion"]
pub type TBIDEX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TB3EX0_SPEC, u8, TBIDEX_A, 3, O>;
impl<'a, const O: u8> TBIDEX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBIDEX_A::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TBIDEX_A::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TBIDEX_A::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(TBIDEX_A::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(TBIDEX_A::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(TBIDEX_A::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(TBIDEX_A::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(TBIDEX_A::_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn tbidex(&self) -> TBIDEX_R {
        TBIDEX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn tbidex(&mut self) -> TBIDEX_W<0> {
        TBIDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb3ex0](index.html) module"]
pub struct TB3EX0_SPEC;
impl crate::RegisterSpec for TB3EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tb3ex0::R](R) reader structure"]
impl crate::Readable for TB3EX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tb3ex0::W](W) writer structure"]
impl crate::Writable for TB3EX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TB3EX0 to value 0"]
impl crate::Resettable for TB3EX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
