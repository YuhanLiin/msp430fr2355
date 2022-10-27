#[doc = "Register `UCA1IRCTL` reader"]
pub struct R(crate::R<UCA1IRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRCTL` writer"]
pub struct W(crate::W<UCA1IRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRCTL_SPEC>;
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
impl From<crate::W<UCA1IRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIREN` reader - IrDA encoder/decoder enable"]
pub type UCIREN_R = crate::BitReader<UCIREN_A>;
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCIREN_A {
    #[doc = "0: IrDA encoder/decoder disabled"]
    UCIREN_0 = 0,
    #[doc = "1: IrDA encoder/decoder enabled"]
    UCIREN_1 = 1,
}
impl From<UCIREN_A> for bool {
    #[inline(always)]
    fn from(variant: UCIREN_A) -> Self {
        variant as u8 != 0
    }
}
impl UCIREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIREN_A {
        match self.bits {
            false => UCIREN_A::UCIREN_0,
            true => UCIREN_A::UCIREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIREN_0`"]
    #[inline(always)]
    pub fn is_uciren_0(&self) -> bool {
        *self == UCIREN_A::UCIREN_0
    }
    #[doc = "Checks if the value of the field is `UCIREN_1`"]
    #[inline(always)]
    pub fn is_uciren_1(&self) -> bool {
        *self == UCIREN_A::UCIREN_1
    }
}
#[doc = "Field `UCIREN` writer - IrDA encoder/decoder enable"]
pub type UCIREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IRCTL_SPEC, UCIREN_A, O>;
impl<'a, const O: u8> UCIREN_W<'a, O> {
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn uciren_0(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_0)
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn uciren_1(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_1)
    }
}
#[doc = "Field `UCIRTXCLK` reader - IrDA transmit pulse clock select"]
pub type UCIRTXCLK_R = crate::BitReader<UCIRTXCLK_A>;
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCIRTXCLK_A {
    #[doc = "0: BRCLK"]
    UCIRTXCLK_0 = 0,
    #[doc = "1: BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    UCIRTXCLK_1 = 1,
}
impl From<UCIRTXCLK_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRTXCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCIRTXCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRTXCLK_A {
        match self.bits {
            false => UCIRTXCLK_A::UCIRTXCLK_0,
            true => UCIRTXCLK_A::UCIRTXCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_0`"]
    #[inline(always)]
    pub fn is_ucirtxclk_0(&self) -> bool {
        *self == UCIRTXCLK_A::UCIRTXCLK_0
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_1`"]
    #[inline(always)]
    pub fn is_ucirtxclk_1(&self) -> bool {
        *self == UCIRTXCLK_A::UCIRTXCLK_1
    }
}
#[doc = "Field `UCIRTXCLK` writer - IrDA transmit pulse clock select"]
pub type UCIRTXCLK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IRCTL_SPEC, UCIRTXCLK_A, O>;
impl<'a, const O: u8> UCIRTXCLK_W<'a, O> {
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn ucirtxclk_0(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_0)
    }
    #[doc = "BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    #[inline(always)]
    pub fn ucirtxclk_1(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_1)
    }
}
#[doc = "Field `UCIRTXPL` reader - Transmit pulse length"]
pub type UCIRTXPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCIRTXPL` writer - Transmit pulse length"]
pub type UCIRTXPL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCA1IRCTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `UCIRRXFE` reader - IrDA receive filter enabled"]
pub type UCIRRXFE_R = crate::BitReader<UCIRRXFE_A>;
#[doc = "IrDA receive filter enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCIRRXFE_A {
    #[doc = "0: Receive filter disabled"]
    UCIRRXFE_0 = 0,
    #[doc = "1: Receive filter enabled"]
    UCIRRXFE_1 = 1,
}
impl From<UCIRRXFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCIRRXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXFE_A {
        match self.bits {
            false => UCIRRXFE_A::UCIRRXFE_0,
            true => UCIRRXFE_A::UCIRRXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_0`"]
    #[inline(always)]
    pub fn is_ucirrxfe_0(&self) -> bool {
        *self == UCIRRXFE_A::UCIRRXFE_0
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_1`"]
    #[inline(always)]
    pub fn is_ucirrxfe_1(&self) -> bool {
        *self == UCIRRXFE_A::UCIRRXFE_1
    }
}
#[doc = "Field `UCIRRXFE` writer - IrDA receive filter enabled"]
pub type UCIRRXFE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IRCTL_SPEC, UCIRRXFE_A, O>;
impl<'a, const O: u8> UCIRRXFE_W<'a, O> {
    #[doc = "Receive filter disabled"]
    #[inline(always)]
    pub fn ucirrxfe_0(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_0)
    }
    #[doc = "Receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe_1(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_1)
    }
}
#[doc = "Field `UCIRRXPL` reader - IrDA receive input UCAxRXD polarity"]
pub type UCIRRXPL_R = crate::BitReader<UCIRRXPL_A>;
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCIRRXPL_A {
    #[doc = "0: IrDA transceiver delivers a high pulse when a light pulse is seen"]
    HIGH = 0,
    #[doc = "1: IrDA transceiver delivers a low pulse when a light pulse is seen"]
    LOW = 1,
}
impl From<UCIRRXPL_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXPL_A) -> Self {
        variant as u8 != 0
    }
}
impl UCIRRXPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXPL_A {
        match self.bits {
            false => UCIRRXPL_A::HIGH,
            true => UCIRRXPL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == UCIRRXPL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == UCIRRXPL_A::LOW
    }
}
#[doc = "Field `UCIRRXPL` writer - IrDA receive input UCAxRXD polarity"]
pub type UCIRRXPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA1IRCTL_SPEC, UCIRRXPL_A, O>;
impl<'a, const O: u8> UCIRRXPL_W<'a, O> {
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::HIGH)
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::LOW)
    }
}
#[doc = "Field `UCIRRXFL` reader - Receive filter length"]
pub type UCIRRXFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCIRRXFL` writer - Receive filter length"]
pub type UCIRRXFL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCA1IRCTL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UCIRRXFL_R {
        UCIRRXFL_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W<0> {
        UCIREN_W::new(self)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W<1> {
        UCIRTXCLK_W::new(self)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W<2> {
        UCIRTXPL_W::new(self)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W<8> {
        UCIRRXFE_W::new(self)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W<9> {
        UCIRRXPL_W::new(self)
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UCIRRXFL_W<10> {
        UCIRRXFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irctl](index.html) module"]
pub struct UCA1IRCTL_SPEC;
impl crate::RegisterSpec for UCA1IRCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca1irctl::R](R) reader structure"]
impl crate::Readable for UCA1IRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irctl::W](W) writer structure"]
impl crate::Writable for UCA1IRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1IRCTL to value 0"]
impl crate::Resettable for UCA1IRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
