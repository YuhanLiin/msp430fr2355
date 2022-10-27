#[doc = "Register `UCA1IE` reader"]
pub struct R(crate::R<UCA1IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IE` writer"]
pub struct W(crate::W<UCA1IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IE_SPEC>;
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
impl From<crate::W<UCA1IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE` reader - Receive interrupt enable"]
pub type UCRXIE_R = crate::BitReader<UCRXIE_A>;
#[doc = "Receive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE_1 = 1,
}
impl From<UCRXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE_A {
        match self.bits {
            false => UCRXIE_A::UCRXIE_0,
            true => UCRXIE_A::UCRXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE_0`"]
    #[inline(always)]
    pub fn is_ucrxie_0(&self) -> bool {
        *self == UCRXIE_A::UCRXIE_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE_1`"]
    #[inline(always)]
    pub fn is_ucrxie_1(&self) -> bool {
        *self == UCRXIE_A::UCRXIE_1
    }
}
#[doc = "Field `UCRXIE` writer - Receive interrupt enable"]
pub type UCRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IE_SPEC, UCRXIE_A, O>;
impl<'a, const O: u8> UCRXIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie_0(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie_1(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_1)
    }
}
#[doc = "Field `UCTXIE` reader - Transmit interrupt enable"]
pub type UCTXIE_R = crate::BitReader<UCTXIE_A>;
#[doc = "Transmit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE_1 = 1,
}
impl From<UCTXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE_A {
        match self.bits {
            false => UCTXIE_A::UCTXIE_0,
            true => UCTXIE_A::UCTXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE_0`"]
    #[inline(always)]
    pub fn is_uctxie_0(&self) -> bool {
        *self == UCTXIE_A::UCTXIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE_1`"]
    #[inline(always)]
    pub fn is_uctxie_1(&self) -> bool {
        *self == UCTXIE_A::UCTXIE_1
    }
}
#[doc = "Field `UCTXIE` writer - Transmit interrupt enable"]
pub type UCTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IE_SPEC, UCTXIE_A, O>;
impl<'a, const O: u8> UCTXIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie_0(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie_1(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_1)
    }
}
#[doc = "Field `UCSTTIE` reader - Start bit interrupt enable"]
pub type UCSTTIE_R = crate::BitReader<UCSTTIE_A>;
#[doc = "Start bit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCSTTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCSTTIE_1 = 1,
}
impl From<UCSTTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIE_A {
        match self.bits {
            false => UCSTTIE_A::UCSTTIE_0,
            true => UCSTTIE_A::UCSTTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_0`"]
    #[inline(always)]
    pub fn is_ucsttie_0(&self) -> bool {
        *self == UCSTTIE_A::UCSTTIE_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_1`"]
    #[inline(always)]
    pub fn is_ucsttie_1(&self) -> bool {
        *self == UCSTTIE_A::UCSTTIE_1
    }
}
#[doc = "Field `UCSTTIE` writer - Start bit interrupt enable"]
pub type UCSTTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IE_SPEC, UCSTTIE_A, O>;
impl<'a, const O: u8> UCSTTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucsttie_0(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucsttie_1(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_1)
    }
}
#[doc = "Field `UCTXCPTIE` reader - Transmit complete interrupt enable"]
pub type UCTXCPTIE_R = crate::BitReader<UCTXCPTIE_A>;
#[doc = "Transmit complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXCPTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXCPTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXCPTIE_1 = 1,
}
impl From<UCTXCPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXCPTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXCPTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXCPTIE_A {
        match self.bits {
            false => UCTXCPTIE_A::UCTXCPTIE_0,
            true => UCTXCPTIE_A::UCTXCPTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_0`"]
    #[inline(always)]
    pub fn is_uctxcptie_0(&self) -> bool {
        *self == UCTXCPTIE_A::UCTXCPTIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_1`"]
    #[inline(always)]
    pub fn is_uctxcptie_1(&self) -> bool {
        *self == UCTXCPTIE_A::UCTXCPTIE_1
    }
}
#[doc = "Field `UCTXCPTIE` writer - Transmit complete interrupt enable"]
pub type UCTXCPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IE_SPEC, UCTXCPTIE_A, O>;
impl<'a, const O: u8> UCTXCPTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxcptie_0(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxcptie_1(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&self) -> UCTXCPTIE_R {
        UCTXCPTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W<0> {
        UCRXIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W<1> {
        UCTXIE_W::new(self)
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<2> {
        UCSTTIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&mut self) -> UCTXCPTIE_W<3> {
        UCTXCPTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie](index.html) module"]
pub struct UCA1IE_SPEC;
impl crate::RegisterSpec for UCA1IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca1ie::R](R) reader structure"]
impl crate::Readable for UCA1IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1ie::W](W) writer structure"]
impl crate::Writable for UCA1IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1IE to value 0"]
impl crate::Resettable for UCA1IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
