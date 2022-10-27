#[doc = "Register `SYSCFG2` reader"]
pub struct R(crate::R<SYSCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG2` writer"]
pub struct W(crate::W<SYSCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG2_SPEC>;
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
impl From<crate::W<SYSCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCPCTL0` reader - ADC input A0 pin select"]
pub type ADCPCTL0_R = crate::BitReader<ADCPCTL0_A>;
#[doc = "ADC input A0 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL0_A {
    #[doc = "0: ADC input A0 disabled"]
    ADCPCTL0_0 = 0,
    #[doc = "1: ADC input A0 enabled"]
    ADCPCTL0_1 = 1,
}
impl From<ADCPCTL0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL0_A {
        match self.bits {
            false => ADCPCTL0_A::ADCPCTL0_0,
            true => ADCPCTL0_A::ADCPCTL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL0_0`"]
    #[inline(always)]
    pub fn is_adcpctl0_0(&self) -> bool {
        *self == ADCPCTL0_A::ADCPCTL0_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL0_1`"]
    #[inline(always)]
    pub fn is_adcpctl0_1(&self) -> bool {
        *self == ADCPCTL0_A::ADCPCTL0_1
    }
}
#[doc = "Field `ADCPCTL0` writer - ADC input A0 pin select"]
pub type ADCPCTL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL0_A, O>;
impl<'a, const O: u8> ADCPCTL0_W<'a, O> {
    #[doc = "ADC input A0 disabled"]
    #[inline(always)]
    pub fn adcpctl0_0(self) -> &'a mut W {
        self.variant(ADCPCTL0_A::ADCPCTL0_0)
    }
    #[doc = "ADC input A0 enabled"]
    #[inline(always)]
    pub fn adcpctl0_1(self) -> &'a mut W {
        self.variant(ADCPCTL0_A::ADCPCTL0_1)
    }
}
#[doc = "Field `ADCPCTL1` reader - ADC input A1 pin select"]
pub type ADCPCTL1_R = crate::BitReader<ADCPCTL1_A>;
#[doc = "ADC input A1 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL1_A {
    #[doc = "0: ADC input A1 disabled"]
    ADCPCTL1_0 = 0,
    #[doc = "1: ADC input A1 enabled"]
    ADCPCTL1_1 = 1,
}
impl From<ADCPCTL1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL1_A {
        match self.bits {
            false => ADCPCTL1_A::ADCPCTL1_0,
            true => ADCPCTL1_A::ADCPCTL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL1_0`"]
    #[inline(always)]
    pub fn is_adcpctl1_0(&self) -> bool {
        *self == ADCPCTL1_A::ADCPCTL1_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL1_1`"]
    #[inline(always)]
    pub fn is_adcpctl1_1(&self) -> bool {
        *self == ADCPCTL1_A::ADCPCTL1_1
    }
}
#[doc = "Field `ADCPCTL1` writer - ADC input A1 pin select"]
pub type ADCPCTL1_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL1_A, O>;
impl<'a, const O: u8> ADCPCTL1_W<'a, O> {
    #[doc = "ADC input A1 disabled"]
    #[inline(always)]
    pub fn adcpctl1_0(self) -> &'a mut W {
        self.variant(ADCPCTL1_A::ADCPCTL1_0)
    }
    #[doc = "ADC input A1 enabled"]
    #[inline(always)]
    pub fn adcpctl1_1(self) -> &'a mut W {
        self.variant(ADCPCTL1_A::ADCPCTL1_1)
    }
}
#[doc = "Field `ADCPCTL2` reader - ADC input A2 pin select"]
pub type ADCPCTL2_R = crate::BitReader<ADCPCTL2_A>;
#[doc = "ADC input A2 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL2_A {
    #[doc = "0: ADC input A2 disabled"]
    ADCPCTL2_0 = 0,
    #[doc = "1: ADC input A2 enabled"]
    ADCPCTL2_1 = 1,
}
impl From<ADCPCTL2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL2_A {
        match self.bits {
            false => ADCPCTL2_A::ADCPCTL2_0,
            true => ADCPCTL2_A::ADCPCTL2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL2_0`"]
    #[inline(always)]
    pub fn is_adcpctl2_0(&self) -> bool {
        *self == ADCPCTL2_A::ADCPCTL2_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL2_1`"]
    #[inline(always)]
    pub fn is_adcpctl2_1(&self) -> bool {
        *self == ADCPCTL2_A::ADCPCTL2_1
    }
}
#[doc = "Field `ADCPCTL2` writer - ADC input A2 pin select"]
pub type ADCPCTL2_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL2_A, O>;
impl<'a, const O: u8> ADCPCTL2_W<'a, O> {
    #[doc = "ADC input A2 disabled"]
    #[inline(always)]
    pub fn adcpctl2_0(self) -> &'a mut W {
        self.variant(ADCPCTL2_A::ADCPCTL2_0)
    }
    #[doc = "ADC input A2 enabled"]
    #[inline(always)]
    pub fn adcpctl2_1(self) -> &'a mut W {
        self.variant(ADCPCTL2_A::ADCPCTL2_1)
    }
}
#[doc = "Field `ADCPCTL3` reader - ADC input A3 pin select"]
pub type ADCPCTL3_R = crate::BitReader<ADCPCTL3_A>;
#[doc = "ADC input A3 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL3_A {
    #[doc = "0: ADC input A3 disabled"]
    ADCPCTL3_0 = 0,
    #[doc = "1: ADC input A3 enabled"]
    ADCPCTL3_1 = 1,
}
impl From<ADCPCTL3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL3_A {
        match self.bits {
            false => ADCPCTL3_A::ADCPCTL3_0,
            true => ADCPCTL3_A::ADCPCTL3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL3_0`"]
    #[inline(always)]
    pub fn is_adcpctl3_0(&self) -> bool {
        *self == ADCPCTL3_A::ADCPCTL3_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL3_1`"]
    #[inline(always)]
    pub fn is_adcpctl3_1(&self) -> bool {
        *self == ADCPCTL3_A::ADCPCTL3_1
    }
}
#[doc = "Field `ADCPCTL3` writer - ADC input A3 pin select"]
pub type ADCPCTL3_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL3_A, O>;
impl<'a, const O: u8> ADCPCTL3_W<'a, O> {
    #[doc = "ADC input A3 disabled"]
    #[inline(always)]
    pub fn adcpctl3_0(self) -> &'a mut W {
        self.variant(ADCPCTL3_A::ADCPCTL3_0)
    }
    #[doc = "ADC input A3 enabled"]
    #[inline(always)]
    pub fn adcpctl3_1(self) -> &'a mut W {
        self.variant(ADCPCTL3_A::ADCPCTL3_1)
    }
}
#[doc = "Field `ADCPCTL4` reader - ADC input A4 pin select"]
pub type ADCPCTL4_R = crate::BitReader<ADCPCTL4_A>;
#[doc = "ADC input A4 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL4_A {
    #[doc = "0: ADC input A4 disabled"]
    ADCPCTL4_0 = 0,
    #[doc = "1: ADC input A4 enabled"]
    ADCPCTL4_1 = 1,
}
impl From<ADCPCTL4_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL4_A {
        match self.bits {
            false => ADCPCTL4_A::ADCPCTL4_0,
            true => ADCPCTL4_A::ADCPCTL4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL4_0`"]
    #[inline(always)]
    pub fn is_adcpctl4_0(&self) -> bool {
        *self == ADCPCTL4_A::ADCPCTL4_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL4_1`"]
    #[inline(always)]
    pub fn is_adcpctl4_1(&self) -> bool {
        *self == ADCPCTL4_A::ADCPCTL4_1
    }
}
#[doc = "Field `ADCPCTL4` writer - ADC input A4 pin select"]
pub type ADCPCTL4_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL4_A, O>;
impl<'a, const O: u8> ADCPCTL4_W<'a, O> {
    #[doc = "ADC input A4 disabled"]
    #[inline(always)]
    pub fn adcpctl4_0(self) -> &'a mut W {
        self.variant(ADCPCTL4_A::ADCPCTL4_0)
    }
    #[doc = "ADC input A4 enabled"]
    #[inline(always)]
    pub fn adcpctl4_1(self) -> &'a mut W {
        self.variant(ADCPCTL4_A::ADCPCTL4_1)
    }
}
#[doc = "Field `ADCPCTL5` reader - ADC input A5 pin select"]
pub type ADCPCTL5_R = crate::BitReader<ADCPCTL5_A>;
#[doc = "ADC input A5 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL5_A {
    #[doc = "0: ADC input A5 disabled"]
    ADCPCTL5_0 = 0,
    #[doc = "1: ADC input A5 enabled"]
    ADCPCTL5_1 = 1,
}
impl From<ADCPCTL5_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL5_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL5_A {
        match self.bits {
            false => ADCPCTL5_A::ADCPCTL5_0,
            true => ADCPCTL5_A::ADCPCTL5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL5_0`"]
    #[inline(always)]
    pub fn is_adcpctl5_0(&self) -> bool {
        *self == ADCPCTL5_A::ADCPCTL5_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL5_1`"]
    #[inline(always)]
    pub fn is_adcpctl5_1(&self) -> bool {
        *self == ADCPCTL5_A::ADCPCTL5_1
    }
}
#[doc = "Field `ADCPCTL5` writer - ADC input A5 pin select"]
pub type ADCPCTL5_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL5_A, O>;
impl<'a, const O: u8> ADCPCTL5_W<'a, O> {
    #[doc = "ADC input A5 disabled"]
    #[inline(always)]
    pub fn adcpctl5_0(self) -> &'a mut W {
        self.variant(ADCPCTL5_A::ADCPCTL5_0)
    }
    #[doc = "ADC input A5 enabled"]
    #[inline(always)]
    pub fn adcpctl5_1(self) -> &'a mut W {
        self.variant(ADCPCTL5_A::ADCPCTL5_1)
    }
}
#[doc = "Field `ADCPCTL6` reader - ADC input A6 pin select"]
pub type ADCPCTL6_R = crate::BitReader<ADCPCTL6_A>;
#[doc = "ADC input A6 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL6_A {
    #[doc = "0: ADC input A6 disabled"]
    ADCPCTL6_0 = 0,
    #[doc = "1: ADC input A6 enabled"]
    ADCPCTL6_1 = 1,
}
impl From<ADCPCTL6_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL6_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL6_A {
        match self.bits {
            false => ADCPCTL6_A::ADCPCTL6_0,
            true => ADCPCTL6_A::ADCPCTL6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL6_0`"]
    #[inline(always)]
    pub fn is_adcpctl6_0(&self) -> bool {
        *self == ADCPCTL6_A::ADCPCTL6_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL6_1`"]
    #[inline(always)]
    pub fn is_adcpctl6_1(&self) -> bool {
        *self == ADCPCTL6_A::ADCPCTL6_1
    }
}
#[doc = "Field `ADCPCTL6` writer - ADC input A6 pin select"]
pub type ADCPCTL6_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL6_A, O>;
impl<'a, const O: u8> ADCPCTL6_W<'a, O> {
    #[doc = "ADC input A6 disabled"]
    #[inline(always)]
    pub fn adcpctl6_0(self) -> &'a mut W {
        self.variant(ADCPCTL6_A::ADCPCTL6_0)
    }
    #[doc = "ADC input A6 enabled"]
    #[inline(always)]
    pub fn adcpctl6_1(self) -> &'a mut W {
        self.variant(ADCPCTL6_A::ADCPCTL6_1)
    }
}
#[doc = "Field `ADCPCTL7` reader - ADC input A7 pin select"]
pub type ADCPCTL7_R = crate::BitReader<ADCPCTL7_A>;
#[doc = "ADC input A7 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL7_A {
    #[doc = "0: ADC input A7 disabled"]
    ADCPCTL7_0 = 0,
    #[doc = "1: ADC input A7 enabled"]
    ADCPCTL7_1 = 1,
}
impl From<ADCPCTL7_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL7_A {
        match self.bits {
            false => ADCPCTL7_A::ADCPCTL7_0,
            true => ADCPCTL7_A::ADCPCTL7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL7_0`"]
    #[inline(always)]
    pub fn is_adcpctl7_0(&self) -> bool {
        *self == ADCPCTL7_A::ADCPCTL7_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL7_1`"]
    #[inline(always)]
    pub fn is_adcpctl7_1(&self) -> bool {
        *self == ADCPCTL7_A::ADCPCTL7_1
    }
}
#[doc = "Field `ADCPCTL7` writer - ADC input A7 pin select"]
pub type ADCPCTL7_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL7_A, O>;
impl<'a, const O: u8> ADCPCTL7_W<'a, O> {
    #[doc = "ADC input A7 disabled"]
    #[inline(always)]
    pub fn adcpctl7_0(self) -> &'a mut W {
        self.variant(ADCPCTL7_A::ADCPCTL7_0)
    }
    #[doc = "ADC input A7 enabled"]
    #[inline(always)]
    pub fn adcpctl7_1(self) -> &'a mut W {
        self.variant(ADCPCTL7_A::ADCPCTL7_1)
    }
}
#[doc = "Field `ADCPCTL8` reader - ADC input A8 pin select"]
pub type ADCPCTL8_R = crate::BitReader<ADCPCTL8_A>;
#[doc = "ADC input A8 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL8_A {
    #[doc = "0: ADC input A8 disabled"]
    ADCPCTL8_0 = 0,
    #[doc = "1: ADC input A8 enabled"]
    ADCPCTL8_1 = 1,
}
impl From<ADCPCTL8_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL8_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL8_A {
        match self.bits {
            false => ADCPCTL8_A::ADCPCTL8_0,
            true => ADCPCTL8_A::ADCPCTL8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL8_0`"]
    #[inline(always)]
    pub fn is_adcpctl8_0(&self) -> bool {
        *self == ADCPCTL8_A::ADCPCTL8_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL8_1`"]
    #[inline(always)]
    pub fn is_adcpctl8_1(&self) -> bool {
        *self == ADCPCTL8_A::ADCPCTL8_1
    }
}
#[doc = "Field `ADCPCTL8` writer - ADC input A8 pin select"]
pub type ADCPCTL8_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL8_A, O>;
impl<'a, const O: u8> ADCPCTL8_W<'a, O> {
    #[doc = "ADC input A8 disabled"]
    #[inline(always)]
    pub fn adcpctl8_0(self) -> &'a mut W {
        self.variant(ADCPCTL8_A::ADCPCTL8_0)
    }
    #[doc = "ADC input A8 enabled"]
    #[inline(always)]
    pub fn adcpctl8_1(self) -> &'a mut W {
        self.variant(ADCPCTL8_A::ADCPCTL8_1)
    }
}
#[doc = "Field `ADCPCTL9` reader - ADC input A9 pin select"]
pub type ADCPCTL9_R = crate::BitReader<ADCPCTL9_A>;
#[doc = "ADC input A9 pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCPCTL9_A {
    #[doc = "0: ADC input A9 disabled"]
    ADCPCTL9_0 = 0,
    #[doc = "1: ADC input A9 enabled"]
    ADCPCTL9_1 = 1,
}
impl From<ADCPCTL9_A> for bool {
    #[inline(always)]
    fn from(variant: ADCPCTL9_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCPCTL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPCTL9_A {
        match self.bits {
            false => ADCPCTL9_A::ADCPCTL9_0,
            true => ADCPCTL9_A::ADCPCTL9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCPCTL9_0`"]
    #[inline(always)]
    pub fn is_adcpctl9_0(&self) -> bool {
        *self == ADCPCTL9_A::ADCPCTL9_0
    }
    #[doc = "Checks if the value of the field is `ADCPCTL9_1`"]
    #[inline(always)]
    pub fn is_adcpctl9_1(&self) -> bool {
        *self == ADCPCTL9_A::ADCPCTL9_1
    }
}
#[doc = "Field `ADCPCTL9` writer - ADC input A9 pin select"]
pub type ADCPCTL9_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, ADCPCTL9_A, O>;
impl<'a, const O: u8> ADCPCTL9_W<'a, O> {
    #[doc = "ADC input A9 disabled"]
    #[inline(always)]
    pub fn adcpctl9_0(self) -> &'a mut W {
        self.variant(ADCPCTL9_A::ADCPCTL9_0)
    }
    #[doc = "ADC input A9 enabled"]
    #[inline(always)]
    pub fn adcpctl9_1(self) -> &'a mut W {
        self.variant(ADCPCTL9_A::ADCPCTL9_1)
    }
}
#[doc = "Field `RTCCKSEL` reader - RTC clock selection"]
pub type RTCCKSEL_R = crate::BitReader<RTCCKSEL_A>;
#[doc = "RTC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCCKSEL_A {
    #[doc = "0: SMCLK is selected"]
    RTC_SMCLK = 0,
    #[doc = "1: ACLK is selected"]
    RTC_ACLK = 1,
}
impl From<RTCCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCKSEL_A {
        match self.bits {
            false => RTCCKSEL_A::RTC_SMCLK,
            true => RTCCKSEL_A::RTC_ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_SMCLK`"]
    #[inline(always)]
    pub fn is_rtc_smclk(&self) -> bool {
        *self == RTCCKSEL_A::RTC_SMCLK
    }
    #[doc = "Checks if the value of the field is `RTC_ACLK`"]
    #[inline(always)]
    pub fn is_rtc_aclk(&self) -> bool {
        *self == RTCCKSEL_A::RTC_ACLK
    }
}
#[doc = "Field `RTCCKSEL` writer - RTC clock selection"]
pub type RTCCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, RTCCKSEL_A, O>;
impl<'a, const O: u8> RTCCKSEL_W<'a, O> {
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn rtc_smclk(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTC_SMCLK)
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn rtc_aclk(self) -> &'a mut W {
        self.variant(RTCCKSEL_A::RTC_ACLK)
    }
}
#[doc = "Field `USCIBRMP` reader - eUSCIB Remapping source selection , please refer to device specific for details"]
pub type USCIBRMP_R = crate::BitReader<USCIBRMP_A>;
#[doc = "eUSCIB Remapping source selection , please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCIBRMP_A {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    USCIBRMP_0 = 0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    USCIBRMP_1 = 1,
}
impl From<USCIBRMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIBRMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USCIBRMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIBRMP_A {
        match self.bits {
            false => USCIBRMP_A::USCIBRMP_0,
            true => USCIBRMP_A::USCIBRMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIBRMP_0`"]
    #[inline(always)]
    pub fn is_uscibrmp_0(&self) -> bool {
        *self == USCIBRMP_A::USCIBRMP_0
    }
    #[doc = "Checks if the value of the field is `USCIBRMP_1`"]
    #[inline(always)]
    pub fn is_uscibrmp_1(&self) -> bool {
        *self == USCIBRMP_A::USCIBRMP_1
    }
}
#[doc = "Field `USCIBRMP` writer - eUSCIB Remapping source selection , please refer to device specific for details"]
pub type USCIBRMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, USCIBRMP_A, O>;
impl<'a, const O: u8> USCIBRMP_W<'a, O> {
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_0(self) -> &'a mut W {
        self.variant(USCIBRMP_A::USCIBRMP_0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp_1(self) -> &'a mut W {
        self.variant(USCIBRMP_A::USCIBRMP_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> ADCPCTL0_R {
        ADCPCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> ADCPCTL1_R {
        ADCPCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> ADCPCTL2_R {
        ADCPCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> ADCPCTL3_R {
        ADCPCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> ADCPCTL4_R {
        ADCPCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> ADCPCTL5_R {
        ADCPCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> ADCPCTL6_R {
        ADCPCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> ADCPCTL7_R {
        ADCPCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&self) -> ADCPCTL8_R {
        ADCPCTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&self) -> ADCPCTL9_R {
        ADCPCTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RTCCKSEL_R {
        RTCCKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&self) -> USCIBRMP_R {
        USCIBRMP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> ADCPCTL0_W<0> {
        ADCPCTL0_W::new(self)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> ADCPCTL1_W<1> {
        ADCPCTL1_W::new(self)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> ADCPCTL2_W<2> {
        ADCPCTL2_W::new(self)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> ADCPCTL3_W<3> {
        ADCPCTL3_W::new(self)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> ADCPCTL4_W<4> {
        ADCPCTL4_W::new(self)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> ADCPCTL5_W<5> {
        ADCPCTL5_W::new(self)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> ADCPCTL6_W<6> {
        ADCPCTL6_W::new(self)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> ADCPCTL7_W<7> {
        ADCPCTL7_W::new(self)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&mut self) -> ADCPCTL8_W<8> {
        ADCPCTL8_W::new(self)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&mut self) -> ADCPCTL9_W<9> {
        ADCPCTL9_W::new(self)
    }
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&mut self) -> RTCCKSEL_W<10> {
        RTCCKSEL_W::new(self)
    }
    #[doc = "Bit 11 - eUSCIB Remapping source selection , please refer to device specific for details"]
    #[inline(always)]
    pub fn uscibrmp(&mut self) -> USCIBRMP_W<11> {
        USCIBRMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg2](index.html) module"]
pub struct SYSCFG2_SPEC;
impl crate::RegisterSpec for SYSCFG2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg2::R](R) reader structure"]
impl crate::Readable for SYSCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg2::W](W) writer structure"]
impl crate::Writable for SYSCFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for SYSCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
