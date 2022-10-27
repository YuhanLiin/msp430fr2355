#[doc = "Register `UCB1IFG` reader"]
pub struct R(crate::R<UCB1IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1IFG` writer"]
pub struct W(crate::W<UCB1IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1IFG_SPEC>;
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
impl From<crate::W<UCB1IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG0` reader - eUSCI_B receive interrupt flag 0"]
pub type UCRXIFG0_R = crate::BitReader<UCRXIFG0_A>;
#[doc = "eUSCI_B receive interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIFG0_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG0_1 = 1,
}
impl From<UCRXIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG0_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG0_A {
        match self.bits {
            false => UCRXIFG0_A::UCRXIFG0_0,
            true => UCRXIFG0_A::UCRXIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0_0`"]
    #[inline(always)]
    pub fn is_ucrxifg0_0(&self) -> bool {
        *self == UCRXIFG0_A::UCRXIFG0_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0_1`"]
    #[inline(always)]
    pub fn is_ucrxifg0_1(&self) -> bool {
        *self == UCRXIFG0_A::UCRXIFG0_1
    }
}
#[doc = "Field `UCRXIFG0` writer - eUSCI_B receive interrupt flag 0"]
pub type UCRXIFG0_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCRXIFG0_A, O>;
impl<'a, const O: u8> UCRXIFG0_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_0(self) -> &'a mut W {
        self.variant(UCRXIFG0_A::UCRXIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_1(self) -> &'a mut W {
        self.variant(UCRXIFG0_A::UCRXIFG0_1)
    }
}
#[doc = "Field `UCTXIFG0` reader - eUSCI_B transmit interrupt flag 0"]
pub type UCTXIFG0_R = crate::BitReader<UCTXIFG0_A>;
#[doc = "eUSCI_B transmit interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIFG0_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG0_1 = 1,
}
impl From<UCTXIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG0_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG0_A {
        match self.bits {
            false => UCTXIFG0_A::UCTXIFG0_0,
            true => UCTXIFG0_A::UCTXIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0_0`"]
    #[inline(always)]
    pub fn is_uctxifg0_0(&self) -> bool {
        *self == UCTXIFG0_A::UCTXIFG0_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0_1`"]
    #[inline(always)]
    pub fn is_uctxifg0_1(&self) -> bool {
        *self == UCTXIFG0_A::UCTXIFG0_1
    }
}
#[doc = "Field `UCTXIFG0` writer - eUSCI_B transmit interrupt flag 0"]
pub type UCTXIFG0_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCTXIFG0_A, O>;
impl<'a, const O: u8> UCTXIFG0_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_0(self) -> &'a mut W {
        self.variant(UCTXIFG0_A::UCTXIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_1(self) -> &'a mut W {
        self.variant(UCTXIFG0_A::UCTXIFG0_1)
    }
}
#[doc = "Field `UCSTTIFG` reader - START condition interrupt flag"]
pub type UCSTTIFG_R = crate::BitReader<UCSTTIFG_A>;
#[doc = "START condition interrupt flag\n\nValue on reset: 0"]
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
#[doc = "Field `UCSTTIFG` writer - START condition interrupt flag"]
pub type UCSTTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCSTTIFG_A, O>;
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
#[doc = "Field `UCSTPIFG` reader - STOP condition interrupt flag"]
pub type UCSTPIFG_R = crate::BitReader<UCSTPIFG_A>;
#[doc = "STOP condition interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTPIFG_A {
    #[doc = "0: No interrupt pending"]
    UCSTPIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCSTPIFG_1 = 1,
}
impl From<UCSTPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTPIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTPIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTPIFG_A {
        match self.bits {
            false => UCSTPIFG_A::UCSTPIFG_0,
            true => UCSTPIFG_A::UCSTPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG_0`"]
    #[inline(always)]
    pub fn is_ucstpifg_0(&self) -> bool {
        *self == UCSTPIFG_A::UCSTPIFG_0
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG_1`"]
    #[inline(always)]
    pub fn is_ucstpifg_1(&self) -> bool {
        *self == UCSTPIFG_A::UCSTPIFG_1
    }
}
#[doc = "Field `UCSTPIFG` writer - STOP condition interrupt flag"]
pub type UCSTPIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCSTPIFG_A, O>;
impl<'a, const O: u8> UCSTPIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_0(self) -> &'a mut W {
        self.variant(UCSTPIFG_A::UCSTPIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_1(self) -> &'a mut W {
        self.variant(UCSTPIFG_A::UCSTPIFG_1)
    }
}
#[doc = "Field `UCALIFG` reader - Arbitration lost interrupt flag"]
pub type UCALIFG_R = crate::BitReader<UCALIFG_A>;
#[doc = "Arbitration lost interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCALIFG_A {
    #[doc = "0: No interrupt pending"]
    UCALIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCALIFG_1 = 1,
}
impl From<UCALIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCALIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCALIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCALIFG_A {
        match self.bits {
            false => UCALIFG_A::UCALIFG_0,
            true => UCALIFG_A::UCALIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCALIFG_0`"]
    #[inline(always)]
    pub fn is_ucalifg_0(&self) -> bool {
        *self == UCALIFG_A::UCALIFG_0
    }
    #[doc = "Checks if the value of the field is `UCALIFG_1`"]
    #[inline(always)]
    pub fn is_ucalifg_1(&self) -> bool {
        *self == UCALIFG_A::UCALIFG_1
    }
}
#[doc = "Field `UCALIFG` writer - Arbitration lost interrupt flag"]
pub type UCALIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCALIFG_A, O>;
impl<'a, const O: u8> UCALIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_0(self) -> &'a mut W {
        self.variant(UCALIFG_A::UCALIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_1(self) -> &'a mut W {
        self.variant(UCALIFG_A::UCALIFG_1)
    }
}
#[doc = "Field `UCNACKIFG` reader - Not-acknowledge received interrupt flag"]
pub type UCNACKIFG_R = crate::BitReader<UCNACKIFG_A>;
#[doc = "Not-acknowledge received interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCNACKIFG_A {
    #[doc = "0: No interrupt pending"]
    UCNACKIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCNACKIFG_1 = 1,
}
impl From<UCNACKIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCNACKIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCNACKIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCNACKIFG_A {
        match self.bits {
            false => UCNACKIFG_A::UCNACKIFG_0,
            true => UCNACKIFG_A::UCNACKIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG_0`"]
    #[inline(always)]
    pub fn is_ucnackifg_0(&self) -> bool {
        *self == UCNACKIFG_A::UCNACKIFG_0
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG_1`"]
    #[inline(always)]
    pub fn is_ucnackifg_1(&self) -> bool {
        *self == UCNACKIFG_A::UCNACKIFG_1
    }
}
#[doc = "Field `UCNACKIFG` writer - Not-acknowledge received interrupt flag"]
pub type UCNACKIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCNACKIFG_A, O>;
impl<'a, const O: u8> UCNACKIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_0(self) -> &'a mut W {
        self.variant(UCNACKIFG_A::UCNACKIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_1(self) -> &'a mut W {
        self.variant(UCNACKIFG_A::UCNACKIFG_1)
    }
}
#[doc = "Field `UCBCNTIFG` reader - Byte counter interrupt flag"]
pub type UCBCNTIFG_R = crate::BitReader<UCBCNTIFG_A>;
#[doc = "Byte counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBCNTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCBCNTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCBCNTIFG_1 = 1,
}
impl From<UCBCNTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCBCNTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBCNTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBCNTIFG_A {
        match self.bits {
            false => UCBCNTIFG_A::UCBCNTIFG_0,
            true => UCBCNTIFG_A::UCBCNTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG_0`"]
    #[inline(always)]
    pub fn is_ucbcntifg_0(&self) -> bool {
        *self == UCBCNTIFG_A::UCBCNTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG_1`"]
    #[inline(always)]
    pub fn is_ucbcntifg_1(&self) -> bool {
        *self == UCBCNTIFG_A::UCBCNTIFG_1
    }
}
#[doc = "Field `UCBCNTIFG` writer - Byte counter interrupt flag"]
pub type UCBCNTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCBCNTIFG_A, O>;
impl<'a, const O: u8> UCBCNTIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_0(self) -> &'a mut W {
        self.variant(UCBCNTIFG_A::UCBCNTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_1(self) -> &'a mut W {
        self.variant(UCBCNTIFG_A::UCBCNTIFG_1)
    }
}
#[doc = "Field `UCCLTOIFG` reader - Clock low timeout interrupt flag"]
pub type UCCLTOIFG_R = crate::BitReader<UCCLTOIFG_A>;
#[doc = "Clock low timeout interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCCLTOIFG_A {
    #[doc = "0: No interrupt pending"]
    UCCLTOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCCLTOIFG_1 = 1,
}
impl From<UCCLTOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCCLTOIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCCLTOIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCLTOIFG_A {
        match self.bits {
            false => UCCLTOIFG_A::UCCLTOIFG_0,
            true => UCCLTOIFG_A::UCCLTOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG_0`"]
    #[inline(always)]
    pub fn is_uccltoifg_0(&self) -> bool {
        *self == UCCLTOIFG_A::UCCLTOIFG_0
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG_1`"]
    #[inline(always)]
    pub fn is_uccltoifg_1(&self) -> bool {
        *self == UCCLTOIFG_A::UCCLTOIFG_1
    }
}
#[doc = "Field `UCCLTOIFG` writer - Clock low timeout interrupt flag"]
pub type UCCLTOIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCCLTOIFG_A, O>;
impl<'a, const O: u8> UCCLTOIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_0(self) -> &'a mut W {
        self.variant(UCCLTOIFG_A::UCCLTOIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_1(self) -> &'a mut W {
        self.variant(UCCLTOIFG_A::UCCLTOIFG_1)
    }
}
#[doc = "Field `UCRXIFG1` reader - eUSCI_B receive interrupt flag 1"]
pub type UCRXIFG1_R = crate::BitReader<UCRXIFG1_A>;
#[doc = "eUSCI_B receive interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIFG1_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG1_1 = 1,
}
impl From<UCRXIFG1_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG1_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIFG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG1_A {
        match self.bits {
            false => UCRXIFG1_A::UCRXIFG1_0,
            true => UCRXIFG1_A::UCRXIFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1_0`"]
    #[inline(always)]
    pub fn is_ucrxifg1_0(&self) -> bool {
        *self == UCRXIFG1_A::UCRXIFG1_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1_1`"]
    #[inline(always)]
    pub fn is_ucrxifg1_1(&self) -> bool {
        *self == UCRXIFG1_A::UCRXIFG1_1
    }
}
#[doc = "Field `UCRXIFG1` writer - eUSCI_B receive interrupt flag 1"]
pub type UCRXIFG1_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCRXIFG1_A, O>;
impl<'a, const O: u8> UCRXIFG1_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_0(self) -> &'a mut W {
        self.variant(UCRXIFG1_A::UCRXIFG1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_1(self) -> &'a mut W {
        self.variant(UCRXIFG1_A::UCRXIFG1_1)
    }
}
#[doc = "Field `UCTXIFG1` reader - eUSCI_B transmit interrupt flag 1"]
pub type UCTXIFG1_R = crate::BitReader<UCTXIFG1_A>;
#[doc = "eUSCI_B transmit interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIFG1_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG1_1 = 1,
}
impl From<UCTXIFG1_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG1_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIFG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG1_A {
        match self.bits {
            false => UCTXIFG1_A::UCTXIFG1_0,
            true => UCTXIFG1_A::UCTXIFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1_0`"]
    #[inline(always)]
    pub fn is_uctxifg1_0(&self) -> bool {
        *self == UCTXIFG1_A::UCTXIFG1_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1_1`"]
    #[inline(always)]
    pub fn is_uctxifg1_1(&self) -> bool {
        *self == UCTXIFG1_A::UCTXIFG1_1
    }
}
#[doc = "Field `UCTXIFG1` writer - eUSCI_B transmit interrupt flag 1"]
pub type UCTXIFG1_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCTXIFG1_A, O>;
impl<'a, const O: u8> UCTXIFG1_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_0(self) -> &'a mut W {
        self.variant(UCTXIFG1_A::UCTXIFG1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_1(self) -> &'a mut W {
        self.variant(UCTXIFG1_A::UCTXIFG1_1)
    }
}
#[doc = "Field `UCRXIFG2` reader - eUSCI_B receive interrupt flag 2"]
pub type UCRXIFG2_R = crate::BitReader<UCRXIFG2_A>;
#[doc = "eUSCI_B receive interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIFG2_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG2_1 = 1,
}
impl From<UCRXIFG2_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG2_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIFG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG2_A {
        match self.bits {
            false => UCRXIFG2_A::UCRXIFG2_0,
            true => UCRXIFG2_A::UCRXIFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2_0`"]
    #[inline(always)]
    pub fn is_ucrxifg2_0(&self) -> bool {
        *self == UCRXIFG2_A::UCRXIFG2_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2_1`"]
    #[inline(always)]
    pub fn is_ucrxifg2_1(&self) -> bool {
        *self == UCRXIFG2_A::UCRXIFG2_1
    }
}
#[doc = "Field `UCRXIFG2` writer - eUSCI_B receive interrupt flag 2"]
pub type UCRXIFG2_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCRXIFG2_A, O>;
impl<'a, const O: u8> UCRXIFG2_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_0(self) -> &'a mut W {
        self.variant(UCRXIFG2_A::UCRXIFG2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_1(self) -> &'a mut W {
        self.variant(UCRXIFG2_A::UCRXIFG2_1)
    }
}
#[doc = "Field `UCTXIFG2` reader - eUSCI_B transmit interrupt flag 2"]
pub type UCTXIFG2_R = crate::BitReader<UCTXIFG2_A>;
#[doc = "eUSCI_B transmit interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIFG2_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG2_1 = 1,
}
impl From<UCTXIFG2_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG2_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIFG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG2_A {
        match self.bits {
            false => UCTXIFG2_A::UCTXIFG2_0,
            true => UCTXIFG2_A::UCTXIFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2_0`"]
    #[inline(always)]
    pub fn is_uctxifg2_0(&self) -> bool {
        *self == UCTXIFG2_A::UCTXIFG2_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2_1`"]
    #[inline(always)]
    pub fn is_uctxifg2_1(&self) -> bool {
        *self == UCTXIFG2_A::UCTXIFG2_1
    }
}
#[doc = "Field `UCTXIFG2` writer - eUSCI_B transmit interrupt flag 2"]
pub type UCTXIFG2_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCTXIFG2_A, O>;
impl<'a, const O: u8> UCTXIFG2_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_0(self) -> &'a mut W {
        self.variant(UCTXIFG2_A::UCTXIFG2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_1(self) -> &'a mut W {
        self.variant(UCTXIFG2_A::UCTXIFG2_1)
    }
}
#[doc = "Field `UCRXIFG3` reader - eUSCI_B receive interrupt flag 3"]
pub type UCRXIFG3_R = crate::BitReader<UCRXIFG3_A>;
#[doc = "eUSCI_B receive interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXIFG3_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG3_1 = 1,
}
impl From<UCRXIFG3_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG3_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXIFG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG3_A {
        match self.bits {
            false => UCRXIFG3_A::UCRXIFG3_0,
            true => UCRXIFG3_A::UCRXIFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3_0`"]
    #[inline(always)]
    pub fn is_ucrxifg3_0(&self) -> bool {
        *self == UCRXIFG3_A::UCRXIFG3_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3_1`"]
    #[inline(always)]
    pub fn is_ucrxifg3_1(&self) -> bool {
        *self == UCRXIFG3_A::UCRXIFG3_1
    }
}
#[doc = "Field `UCRXIFG3` writer - eUSCI_B receive interrupt flag 3"]
pub type UCRXIFG3_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCRXIFG3_A, O>;
impl<'a, const O: u8> UCRXIFG3_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_0(self) -> &'a mut W {
        self.variant(UCRXIFG3_A::UCRXIFG3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_1(self) -> &'a mut W {
        self.variant(UCRXIFG3_A::UCRXIFG3_1)
    }
}
#[doc = "Field `UCTXIFG3` reader - eUSCI_B transmit interrupt flag 3"]
pub type UCTXIFG3_R = crate::BitReader<UCTXIFG3_A>;
#[doc = "eUSCI_B transmit interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCTXIFG3_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG3_1 = 1,
}
impl From<UCTXIFG3_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG3_A) -> Self {
        variant as u8 != 0
    }
}
impl UCTXIFG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG3_A {
        match self.bits {
            false => UCTXIFG3_A::UCTXIFG3_0,
            true => UCTXIFG3_A::UCTXIFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3_0`"]
    #[inline(always)]
    pub fn is_uctxifg3_0(&self) -> bool {
        *self == UCTXIFG3_A::UCTXIFG3_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3_1`"]
    #[inline(always)]
    pub fn is_uctxifg3_1(&self) -> bool {
        *self == UCTXIFG3_A::UCTXIFG3_1
    }
}
#[doc = "Field `UCTXIFG3` writer - eUSCI_B transmit interrupt flag 3"]
pub type UCTXIFG3_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCTXIFG3_A, O>;
impl<'a, const O: u8> UCTXIFG3_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_0(self) -> &'a mut W {
        self.variant(UCTXIFG3_A::UCTXIFG3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_1(self) -> &'a mut W {
        self.variant(UCTXIFG3_A::UCTXIFG3_1)
    }
}
#[doc = "Field `UCBIT9IFG` reader - Bit position 9 interrupt flag"]
pub type UCBIT9IFG_R = crate::BitReader<UCBIT9IFG_A>;
#[doc = "Bit position 9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBIT9IFG_A {
    #[doc = "0: No interrupt pending"]
    UCBIT9IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCBIT9IFG_1 = 1,
}
impl From<UCBIT9IFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCBIT9IFG_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBIT9IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBIT9IFG_A {
        match self.bits {
            false => UCBIT9IFG_A::UCBIT9IFG_0,
            true => UCBIT9IFG_A::UCBIT9IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG_0`"]
    #[inline(always)]
    pub fn is_ucbit9ifg_0(&self) -> bool {
        *self == UCBIT9IFG_A::UCBIT9IFG_0
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG_1`"]
    #[inline(always)]
    pub fn is_ucbit9ifg_1(&self) -> bool {
        *self == UCBIT9IFG_A::UCBIT9IFG_1
    }
}
#[doc = "Field `UCBIT9IFG` writer - Bit position 9 interrupt flag"]
pub type UCBIT9IFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1IFG_SPEC, UCBIT9IFG_A, O>;
impl<'a, const O: u8> UCBIT9IFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_0(self) -> &'a mut W {
        self.variant(UCBIT9IFG_A::UCBIT9IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_1(self) -> &'a mut W {
        self.variant(UCBIT9IFG_A::UCBIT9IFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> UCRXIFG0_R {
        UCRXIFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> UCTXIFG0_R {
        UCTXIFG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UCBCNTIFG_R {
        UCBCNTIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UCCLTOIFG_R {
        UCCLTOIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> UCRXIFG1_R {
        UCRXIFG1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> UCTXIFG1_R {
        UCTXIFG1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> UCRXIFG2_R {
        UCRXIFG2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> UCTXIFG2_R {
        UCTXIFG2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> UCRXIFG3_R {
        UCRXIFG3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> UCTXIFG3_R {
        UCTXIFG3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> UCBIT9IFG_R {
        UCBIT9IFG_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> UCRXIFG0_W<0> {
        UCRXIFG0_W::new(self)
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> UCTXIFG0_W<1> {
        UCTXIFG0_W::new(self)
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W<2> {
        UCSTTIFG_W::new(self)
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W<3> {
        UCSTPIFG_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W<4> {
        UCALIFG_W::new(self)
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W<5> {
        UCNACKIFG_W::new(self)
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UCBCNTIFG_W<6> {
        UCBCNTIFG_W::new(self)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UCCLTOIFG_W<7> {
        UCCLTOIFG_W::new(self)
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> UCRXIFG1_W<8> {
        UCRXIFG1_W::new(self)
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> UCTXIFG1_W<9> {
        UCTXIFG1_W::new(self)
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> UCRXIFG2_W<10> {
        UCRXIFG2_W::new(self)
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> UCTXIFG2_W<11> {
        UCTXIFG2_W::new(self)
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> UCRXIFG3_W<12> {
        UCRXIFG3_W::new(self)
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> UCTXIFG3_W<13> {
        UCTXIFG3_W::new(self)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> UCBIT9IFG_W<14> {
        UCBIT9IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg](index.html) module"]
pub struct UCB1IFG_SPEC;
impl crate::RegisterSpec for UCB1IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1ifg::R](R) reader structure"]
impl crate::Readable for UCB1IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1ifg::W](W) writer structure"]
impl crate::Writable for UCB1IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1IFG to value 0"]
impl crate::Resettable for UCB1IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
