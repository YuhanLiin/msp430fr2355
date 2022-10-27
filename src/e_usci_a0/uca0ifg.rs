#[doc = "Register `UCA0IFG` reader"]
pub struct R(crate::R<UCA0IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IFG` writer"]
pub struct W(crate::W<UCA0IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IFG_SPEC>;
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
impl From<crate::W<UCA0IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - Receive interrupt flag"]
pub type UCRXIFG_R = crate::BitReader<UCRXIFG_A>;
#[doc = "Receive interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG_1 = 1,
}
impl From<UCRXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG_A {
        match self.bits {
            false => UCRXIFG_A::UCRXIFG_0,
            true => UCRXIFG_A::UCRXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_0`"]
    #[inline(always)]
    pub fn is_ucrxifg_0(&self) -> bool {
        *self == UCRXIFG_A::UCRXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_1`"]
    #[inline(always)]
    pub fn is_ucrxifg_1(&self) -> bool {
        *self == UCRXIFG_A::UCRXIFG_1
    }
}
#[doc = "Field `UCRXIFG` writer - Receive interrupt flag"]
pub type UCRXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0IFG_SPEC, UCRXIFG_A, O>;
impl<'a, const O: u8> UCRXIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_0(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_1(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_1)
    }
}
#[doc = "Field `UCTXIFG` reader - Transmit interrupt flag"]
pub type UCTXIFG_R = crate::BitReader<UCTXIFG_A>;
#[doc = "Transmit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG_1 = 1,
}
impl From<UCTXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG_A {
        match self.bits {
            false => UCTXIFG_A::UCTXIFG_0,
            true => UCTXIFG_A::UCTXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_0`"]
    #[inline(always)]
    pub fn is_uctxifg_0(&self) -> bool {
        *self == UCTXIFG_A::UCTXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_1`"]
    #[inline(always)]
    pub fn is_uctxifg_1(&self) -> bool {
        *self == UCTXIFG_A::UCTXIFG_1
    }
}
#[doc = "Field `UCTXIFG` writer - Transmit interrupt flag"]
pub type UCTXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0IFG_SPEC, UCTXIFG_A, O>;
impl<'a, const O: u8> UCTXIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_0(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_1(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_1)
    }
}
#[doc = "Field `UCSTTIFG` reader - Start bit interrupt flag"]
pub type UCSTTIFG_R = crate::BitReader<UCSTTIFG_A>;
#[doc = "Start bit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCSTTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCSTTIFG_1 = 1,
}
impl From<UCSTTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIFG_A {
        match self.bits {
            false => UCSTTIFG_A::UCSTTIFG_0,
            true => UCSTTIFG_A::UCSTTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_0`"]
    #[inline(always)]
    pub fn is_ucsttifg_0(&self) -> bool {
        *self == UCSTTIFG_A::UCSTTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_1`"]
    #[inline(always)]
    pub fn is_ucsttifg_1(&self) -> bool {
        *self == UCSTTIFG_A::UCSTTIFG_1
    }
}
#[doc = "Field `UCSTTIFG` writer - Start bit interrupt flag"]
pub type UCSTTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0IFG_SPEC, UCSTTIFG_A, O>;
impl<'a, const O: u8> UCSTTIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_0(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_1(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_1)
    }
}
#[doc = "Field `UCTXCPTIFG` reader - Transmit ready interrupt enable"]
pub type UCTXCPTIFG_R = crate::BitReader<UCTXCPTIFG_A>;
#[doc = "Transmit ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXCPTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCTXCPTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXCPTIFG_1 = 1,
}
impl From<UCTXCPTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXCPTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXCPTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXCPTIFG_A {
        match self.bits {
            false => UCTXCPTIFG_A::UCTXCPTIFG_0,
            true => UCTXCPTIFG_A::UCTXCPTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG_0`"]
    #[inline(always)]
    pub fn is_uctxcptifg_0(&self) -> bool {
        *self == UCTXCPTIFG_A::UCTXCPTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG_1`"]
    #[inline(always)]
    pub fn is_uctxcptifg_1(&self) -> bool {
        *self == UCTXCPTIFG_A::UCTXCPTIFG_1
    }
}
#[doc = "Field `UCTXCPTIFG` writer - Transmit ready interrupt enable"]
pub type UCTXCPTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0IFG_SPEC, UCTXCPTIFG_A, O>;
impl<'a, const O: u8> UCTXCPTIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_0(self) -> &'a mut W {
        self.variant(UCTXCPTIFG_A::UCTXCPTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_1(self) -> &'a mut W {
        self.variant(UCTXCPTIFG_A::UCTXCPTIFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start bit interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit ready interrupt enable"]
    #[inline(always)]
    pub fn uctxcptifg(&self) -> UCTXCPTIFG_R {
        UCTXCPTIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W<0> {
        UCRXIFG_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W<1> {
        UCTXIFG_W::new(self)
    }
    #[doc = "Bit 2 - Start bit interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W<2> {
        UCSTTIFG_W::new(self)
    }
    #[doc = "Bit 3 - Transmit ready interrupt enable"]
    #[inline(always)]
    pub fn uctxcptifg(&mut self) -> UCTXCPTIFG_W<3> {
        UCTXCPTIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ifg](index.html) module"]
pub struct UCA0IFG_SPEC;
impl crate::RegisterSpec for UCA0IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0ifg::R](R) reader structure"]
impl crate::Readable for UCA0IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ifg::W](W) writer structure"]
impl crate::Writable for UCA0IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IFG to value 0"]
impl crate::Resettable for UCA0IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
