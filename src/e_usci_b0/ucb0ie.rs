#[doc = "Register `UCB0IE` reader"]
pub struct R(crate::R<UCB0IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IE` writer"]
pub struct W(crate::W<UCB0IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IE_SPEC>;
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
impl From<crate::W<UCB0IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE0` reader - Receive interrupt enable 0"]
pub type UCRXIE0_R = crate::BitReader<UCRXIE0_A>;
#[doc = "Receive interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIE0_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE0_1 = 1,
}
impl From<UCRXIE0_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE0_A {
        match self.bits {
            false => UCRXIE0_A::UCRXIE0_0,
            true => UCRXIE0_A::UCRXIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE0_0`"]
    #[inline(always)]
    pub fn is_ucrxie0_0(&self) -> bool {
        *self == UCRXIE0_A::UCRXIE0_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE0_1`"]
    #[inline(always)]
    pub fn is_ucrxie0_1(&self) -> bool {
        *self == UCRXIE0_A::UCRXIE0_1
    }
}
#[doc = "Field `UCRXIE0` writer - Receive interrupt enable 0"]
pub type UCRXIE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCRXIE0_A, O>;
impl<'a, const O: u8> UCRXIE0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie0_0(self) -> &'a mut W {
        self.variant(UCRXIE0_A::UCRXIE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie0_1(self) -> &'a mut W {
        self.variant(UCRXIE0_A::UCRXIE0_1)
    }
}
#[doc = "Field `UCTXIE0` reader - Transmit interrupt enable 0"]
pub type UCTXIE0_R = crate::BitReader<UCTXIE0_A>;
#[doc = "Transmit interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIE0_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE0_1 = 1,
}
impl From<UCTXIE0_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE0_A {
        match self.bits {
            false => UCTXIE0_A::UCTXIE0_0,
            true => UCTXIE0_A::UCTXIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE0_0`"]
    #[inline(always)]
    pub fn is_uctxie0_0(&self) -> bool {
        *self == UCTXIE0_A::UCTXIE0_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE0_1`"]
    #[inline(always)]
    pub fn is_uctxie0_1(&self) -> bool {
        *self == UCTXIE0_A::UCTXIE0_1
    }
}
#[doc = "Field `UCTXIE0` writer - Transmit interrupt enable 0"]
pub type UCTXIE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCTXIE0_A, O>;
impl<'a, const O: u8> UCTXIE0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie0_0(self) -> &'a mut W {
        self.variant(UCTXIE0_A::UCTXIE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie0_1(self) -> &'a mut W {
        self.variant(UCTXIE0_A::UCTXIE0_1)
    }
}
#[doc = "Field `UCSTTIE` reader - START condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader<UCSTTIE_A>;
#[doc = "START condition interrupt enable\n\nValue on reset: 0"]
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
#[doc = "Field `UCSTTIE` writer - START condition interrupt enable"]
pub type UCSTTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCSTTIE_A, O>;
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
#[doc = "Field `UCSTPIE` reader - STOP condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader<UCSTPIE_A>;
#[doc = "STOP condition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTPIE_A {
    #[doc = "0: Interrupt disabled"]
    UCSTPIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCSTPIE_1 = 1,
}
impl From<UCSTPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTPIE_A {
        match self.bits {
            false => UCSTPIE_A::UCSTPIE_0,
            true => UCSTPIE_A::UCSTPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTPIE_0`"]
    #[inline(always)]
    pub fn is_ucstpie_0(&self) -> bool {
        *self == UCSTPIE_A::UCSTPIE_0
    }
    #[doc = "Checks if the value of the field is `UCSTPIE_1`"]
    #[inline(always)]
    pub fn is_ucstpie_1(&self) -> bool {
        *self == UCSTPIE_A::UCSTPIE_1
    }
}
#[doc = "Field `UCSTPIE` writer - STOP condition interrupt enable"]
pub type UCSTPIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCSTPIE_A, O>;
impl<'a, const O: u8> UCSTPIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucstpie_0(self) -> &'a mut W {
        self.variant(UCSTPIE_A::UCSTPIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucstpie_1(self) -> &'a mut W {
        self.variant(UCSTPIE_A::UCSTPIE_1)
    }
}
#[doc = "Field `UCALIE` reader - Arbitration lost interrupt enable"]
pub type UCALIE_R = crate::BitReader<UCALIE_A>;
#[doc = "Arbitration lost interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCALIE_A {
    #[doc = "0: Interrupt disabled"]
    UCALIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCALIE_1 = 1,
}
impl From<UCALIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCALIE_A {
        match self.bits {
            false => UCALIE_A::UCALIE_0,
            true => UCALIE_A::UCALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCALIE_0`"]
    #[inline(always)]
    pub fn is_ucalie_0(&self) -> bool {
        *self == UCALIE_A::UCALIE_0
    }
    #[doc = "Checks if the value of the field is `UCALIE_1`"]
    #[inline(always)]
    pub fn is_ucalie_1(&self) -> bool {
        *self == UCALIE_A::UCALIE_1
    }
}
#[doc = "Field `UCALIE` writer - Arbitration lost interrupt enable"]
pub type UCALIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCALIE_A, O>;
impl<'a, const O: u8> UCALIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucalie_0(self) -> &'a mut W {
        self.variant(UCALIE_A::UCALIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucalie_1(self) -> &'a mut W {
        self.variant(UCALIE_A::UCALIE_1)
    }
}
#[doc = "Field `UCNACKIE` reader - Not-acknowledge interrupt enable"]
pub type UCNACKIE_R = crate::BitReader<UCNACKIE_A>;
#[doc = "Not-acknowledge interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCNACKIE_A {
    #[doc = "0: Interrupt disabled"]
    UCNACKIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCNACKIE_1 = 1,
}
impl From<UCNACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCNACKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCNACKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCNACKIE_A {
        match self.bits {
            false => UCNACKIE_A::UCNACKIE_0,
            true => UCNACKIE_A::UCNACKIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCNACKIE_0`"]
    #[inline(always)]
    pub fn is_ucnackie_0(&self) -> bool {
        *self == UCNACKIE_A::UCNACKIE_0
    }
    #[doc = "Checks if the value of the field is `UCNACKIE_1`"]
    #[inline(always)]
    pub fn is_ucnackie_1(&self) -> bool {
        *self == UCNACKIE_A::UCNACKIE_1
    }
}
#[doc = "Field `UCNACKIE` writer - Not-acknowledge interrupt enable"]
pub type UCNACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCNACKIE_A, O>;
impl<'a, const O: u8> UCNACKIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucnackie_0(self) -> &'a mut W {
        self.variant(UCNACKIE_A::UCNACKIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucnackie_1(self) -> &'a mut W {
        self.variant(UCNACKIE_A::UCNACKIE_1)
    }
}
#[doc = "Field `UCBCNTIE` reader - Byte counter interrupt enable"]
pub type UCBCNTIE_R = crate::BitReader<UCBCNTIE_A>;
#[doc = "Byte counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBCNTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCBCNTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCBCNTIE_1 = 1,
}
impl From<UCBCNTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBCNTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBCNTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBCNTIE_A {
        match self.bits {
            false => UCBCNTIE_A::UCBCNTIE_0,
            true => UCBCNTIE_A::UCBCNTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBCNTIE_0`"]
    #[inline(always)]
    pub fn is_ucbcntie_0(&self) -> bool {
        *self == UCBCNTIE_A::UCBCNTIE_0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIE_1`"]
    #[inline(always)]
    pub fn is_ucbcntie_1(&self) -> bool {
        *self == UCBCNTIE_A::UCBCNTIE_1
    }
}
#[doc = "Field `UCBCNTIE` writer - Byte counter interrupt enable"]
pub type UCBCNTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCBCNTIE_A, O>;
impl<'a, const O: u8> UCBCNTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbcntie_0(self) -> &'a mut W {
        self.variant(UCBCNTIE_A::UCBCNTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbcntie_1(self) -> &'a mut W {
        self.variant(UCBCNTIE_A::UCBCNTIE_1)
    }
}
#[doc = "Field `UCCLTOIE` reader - Clock low timeout interrupt enable"]
pub type UCCLTOIE_R = crate::BitReader<UCCLTOIE_A>;
#[doc = "Clock low timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCCLTOIE_A {
    #[doc = "0: Interrupt disabled"]
    UCCLTOIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCCLTOIE_1 = 1,
}
impl From<UCCLTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCCLTOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCCLTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCLTOIE_A {
        match self.bits {
            false => UCCLTOIE_A::UCCLTOIE_0,
            true => UCCLTOIE_A::UCCLTOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTOIE_0`"]
    #[inline(always)]
    pub fn is_uccltoie_0(&self) -> bool {
        *self == UCCLTOIE_A::UCCLTOIE_0
    }
    #[doc = "Checks if the value of the field is `UCCLTOIE_1`"]
    #[inline(always)]
    pub fn is_uccltoie_1(&self) -> bool {
        *self == UCCLTOIE_A::UCCLTOIE_1
    }
}
#[doc = "Field `UCCLTOIE` writer - Clock low timeout interrupt enable"]
pub type UCCLTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCCLTOIE_A, O>;
impl<'a, const O: u8> UCCLTOIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uccltoie_0(self) -> &'a mut W {
        self.variant(UCCLTOIE_A::UCCLTOIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uccltoie_1(self) -> &'a mut W {
        self.variant(UCCLTOIE_A::UCCLTOIE_1)
    }
}
#[doc = "Field `UCRXIE1` reader - Receive interrupt enable 1"]
pub type UCRXIE1_R = crate::BitReader<UCRXIE1_A>;
#[doc = "Receive interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIE1_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE1_1 = 1,
}
impl From<UCRXIE1_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE1_A {
        match self.bits {
            false => UCRXIE1_A::UCRXIE1_0,
            true => UCRXIE1_A::UCRXIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE1_0`"]
    #[inline(always)]
    pub fn is_ucrxie1_0(&self) -> bool {
        *self == UCRXIE1_A::UCRXIE1_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE1_1`"]
    #[inline(always)]
    pub fn is_ucrxie1_1(&self) -> bool {
        *self == UCRXIE1_A::UCRXIE1_1
    }
}
#[doc = "Field `UCRXIE1` writer - Receive interrupt enable 1"]
pub type UCRXIE1_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCRXIE1_A, O>;
impl<'a, const O: u8> UCRXIE1_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie1_0(self) -> &'a mut W {
        self.variant(UCRXIE1_A::UCRXIE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie1_1(self) -> &'a mut W {
        self.variant(UCRXIE1_A::UCRXIE1_1)
    }
}
#[doc = "Field `UCTXIE1` reader - Transmit interrupt enable 1"]
pub type UCTXIE1_R = crate::BitReader<UCTXIE1_A>;
#[doc = "Transmit interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIE1_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE1_1 = 1,
}
impl From<UCTXIE1_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE1_A {
        match self.bits {
            false => UCTXIE1_A::UCTXIE1_0,
            true => UCTXIE1_A::UCTXIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE1_0`"]
    #[inline(always)]
    pub fn is_uctxie1_0(&self) -> bool {
        *self == UCTXIE1_A::UCTXIE1_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE1_1`"]
    #[inline(always)]
    pub fn is_uctxie1_1(&self) -> bool {
        *self == UCTXIE1_A::UCTXIE1_1
    }
}
#[doc = "Field `UCTXIE1` writer - Transmit interrupt enable 1"]
pub type UCTXIE1_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCTXIE1_A, O>;
impl<'a, const O: u8> UCTXIE1_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie1_0(self) -> &'a mut W {
        self.variant(UCTXIE1_A::UCTXIE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie1_1(self) -> &'a mut W {
        self.variant(UCTXIE1_A::UCTXIE1_1)
    }
}
#[doc = "Field `UCRXIE2` reader - Receive interrupt enable 2"]
pub type UCRXIE2_R = crate::BitReader<UCRXIE2_A>;
#[doc = "Receive interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIE2_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE2_1 = 1,
}
impl From<UCRXIE2_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE2_A {
        match self.bits {
            false => UCRXIE2_A::UCRXIE2_0,
            true => UCRXIE2_A::UCRXIE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE2_0`"]
    #[inline(always)]
    pub fn is_ucrxie2_0(&self) -> bool {
        *self == UCRXIE2_A::UCRXIE2_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE2_1`"]
    #[inline(always)]
    pub fn is_ucrxie2_1(&self) -> bool {
        *self == UCRXIE2_A::UCRXIE2_1
    }
}
#[doc = "Field `UCRXIE2` writer - Receive interrupt enable 2"]
pub type UCRXIE2_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCRXIE2_A, O>;
impl<'a, const O: u8> UCRXIE2_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie2_0(self) -> &'a mut W {
        self.variant(UCRXIE2_A::UCRXIE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie2_1(self) -> &'a mut W {
        self.variant(UCRXIE2_A::UCRXIE2_1)
    }
}
#[doc = "Field `UCTXIE2` reader - Transmit interrupt enable 2"]
pub type UCTXIE2_R = crate::BitReader<UCTXIE2_A>;
#[doc = "Transmit interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIE2_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE2_1 = 1,
}
impl From<UCTXIE2_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE2_A {
        match self.bits {
            false => UCTXIE2_A::UCTXIE2_0,
            true => UCTXIE2_A::UCTXIE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE2_0`"]
    #[inline(always)]
    pub fn is_uctxie2_0(&self) -> bool {
        *self == UCTXIE2_A::UCTXIE2_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE2_1`"]
    #[inline(always)]
    pub fn is_uctxie2_1(&self) -> bool {
        *self == UCTXIE2_A::UCTXIE2_1
    }
}
#[doc = "Field `UCTXIE2` writer - Transmit interrupt enable 2"]
pub type UCTXIE2_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCTXIE2_A, O>;
impl<'a, const O: u8> UCTXIE2_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie2_0(self) -> &'a mut W {
        self.variant(UCTXIE2_A::UCTXIE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie2_1(self) -> &'a mut W {
        self.variant(UCTXIE2_A::UCTXIE2_1)
    }
}
#[doc = "Field `UCRXIE3` reader - Receive interrupt enable 3"]
pub type UCRXIE3_R = crate::BitReader<UCRXIE3_A>;
#[doc = "Receive interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIE3_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE3_1 = 1,
}
impl From<UCRXIE3_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE3_A {
        match self.bits {
            false => UCRXIE3_A::UCRXIE3_0,
            true => UCRXIE3_A::UCRXIE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE3_0`"]
    #[inline(always)]
    pub fn is_ucrxie3_0(&self) -> bool {
        *self == UCRXIE3_A::UCRXIE3_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE3_1`"]
    #[inline(always)]
    pub fn is_ucrxie3_1(&self) -> bool {
        *self == UCRXIE3_A::UCRXIE3_1
    }
}
#[doc = "Field `UCRXIE3` writer - Receive interrupt enable 3"]
pub type UCRXIE3_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCRXIE3_A, O>;
impl<'a, const O: u8> UCRXIE3_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie3_0(self) -> &'a mut W {
        self.variant(UCRXIE3_A::UCRXIE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie3_1(self) -> &'a mut W {
        self.variant(UCRXIE3_A::UCRXIE3_1)
    }
}
#[doc = "Field `UCTXIE3` reader - Transmit interrupt enable 3"]
pub type UCTXIE3_R = crate::BitReader<UCTXIE3_A>;
#[doc = "Transmit interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIE3_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE3_1 = 1,
}
impl From<UCTXIE3_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE3_A {
        match self.bits {
            false => UCTXIE3_A::UCTXIE3_0,
            true => UCTXIE3_A::UCTXIE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE3_0`"]
    #[inline(always)]
    pub fn is_uctxie3_0(&self) -> bool {
        *self == UCTXIE3_A::UCTXIE3_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE3_1`"]
    #[inline(always)]
    pub fn is_uctxie3_1(&self) -> bool {
        *self == UCTXIE3_A::UCTXIE3_1
    }
}
#[doc = "Field `UCTXIE3` writer - Transmit interrupt enable 3"]
pub type UCTXIE3_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCTXIE3_A, O>;
impl<'a, const O: u8> UCTXIE3_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie3_0(self) -> &'a mut W {
        self.variant(UCTXIE3_A::UCTXIE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie3_1(self) -> &'a mut W {
        self.variant(UCTXIE3_A::UCTXIE3_1)
    }
}
#[doc = "Field `UCBIT9IE` reader - Bit position 9 interrupt enable"]
pub type UCBIT9IE_R = crate::BitReader<UCBIT9IE_A>;
#[doc = "Bit position 9 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBIT9IE_A {
    #[doc = "0: Interrupt disabled"]
    UCBIT9IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCBIT9IE_1 = 1,
}
impl From<UCBIT9IE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBIT9IE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBIT9IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBIT9IE_A {
        match self.bits {
            false => UCBIT9IE_A::UCBIT9IE_0,
            true => UCBIT9IE_A::UCBIT9IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBIT9IE_0`"]
    #[inline(always)]
    pub fn is_ucbit9ie_0(&self) -> bool {
        *self == UCBIT9IE_A::UCBIT9IE_0
    }
    #[doc = "Checks if the value of the field is `UCBIT9IE_1`"]
    #[inline(always)]
    pub fn is_ucbit9ie_1(&self) -> bool {
        *self == UCBIT9IE_A::UCBIT9IE_1
    }
}
#[doc = "Field `UCBIT9IE` writer - Bit position 9 interrupt enable"]
pub type UCBIT9IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB0IE_SPEC, UCBIT9IE_A, O>;
impl<'a, const O: u8> UCBIT9IE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucbit9ie_0(self) -> &'a mut W {
        self.variant(UCBIT9IE_A::UCBIT9IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucbit9ie_1(self) -> &'a mut W {
        self.variant(UCBIT9IE_A::UCBIT9IE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W<0> {
        UCRXIE0_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> UCTXIE0_W<1> {
        UCTXIE0_W::new(self)
    }
    #[doc = "Bit 2 - START condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<2> {
        UCSTTIE_W::new(self)
    }
    #[doc = "Bit 3 - STOP condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<3> {
        UCSTPIE_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W<4> {
        UCALIE_W::new(self)
    }
    #[doc = "Bit 5 - Not-acknowledge interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<5> {
        UCNACKIE_W::new(self)
    }
    #[doc = "Bit 6 - Byte counter interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W<6> {
        UCBCNTIE_W::new(self)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W<7> {
        UCCLTOIE_W::new(self)
    }
    #[doc = "Bit 8 - Receive interrupt enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W<8> {
        UCRXIE1_W::new(self)
    }
    #[doc = "Bit 9 - Transmit interrupt enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> UCTXIE1_W<9> {
        UCTXIE1_W::new(self)
    }
    #[doc = "Bit 10 - Receive interrupt enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W<10> {
        UCRXIE2_W::new(self)
    }
    #[doc = "Bit 11 - Transmit interrupt enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> UCTXIE2_W<11> {
        UCTXIE2_W::new(self)
    }
    #[doc = "Bit 12 - Receive interrupt enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W<12> {
        UCRXIE3_W::new(self)
    }
    #[doc = "Bit 13 - Transmit interrupt enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> UCTXIE3_W<13> {
        UCTXIE3_W::new(self)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt enable"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W<14> {
        UCBIT9IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie](index.html) module"]
pub struct UCB0IE_SPEC;
impl crate::RegisterSpec for UCB0IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ie::R](R) reader structure"]
impl crate::Readable for UCB0IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ie::W](W) writer structure"]
impl crate::Writable for UCB0IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IE to value 0"]
impl crate::Resettable for UCB0IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
