#[doc = "Register `CP1CTL1` reader"]
pub struct R(crate::R<CP1CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP1CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP1CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP1CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CP1CTL1` writer"]
pub struct W(crate::W<CP1CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CP1CTL1_SPEC>;
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
impl From<crate::W<CP1CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CP1CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOUT` reader - Comparator output value"]
pub type CPOUT_R = crate::BitReader<bool>;
#[doc = "Field `CPINV` reader - Comparator output polarity"]
pub type CPINV_R = crate::BitReader<CPINV_A>;
#[doc = "Comparator output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPINV_A {
    #[doc = "0: Comparator output is non-inverted"]
    CPINV_0 = 0,
    #[doc = "1: Comparator output is inverted"]
    CPINV_1 = 1,
}
impl From<CPINV_A> for bool {
    #[inline(always)]
    fn from(variant: CPINV_A) -> Self {
        variant as u8 != 0
    }
}
impl CPINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPINV_A {
        match self.bits {
            false => CPINV_A::CPINV_0,
            true => CPINV_A::CPINV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPINV_0`"]
    #[inline(always)]
    pub fn is_cpinv_0(&self) -> bool {
        *self == CPINV_A::CPINV_0
    }
    #[doc = "Checks if the value of the field is `CPINV_1`"]
    #[inline(always)]
    pub fn is_cpinv_1(&self) -> bool {
        *self == CPINV_A::CPINV_1
    }
}
#[doc = "Field `CPINV` writer - Comparator output polarity"]
pub type CPINV_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPINV_A, O>;
impl<'a, const O: u8> CPINV_W<'a, O> {
    #[doc = "Comparator output is non-inverted"]
    #[inline(always)]
    pub fn cpinv_0(self) -> &'a mut W {
        self.variant(CPINV_A::CPINV_0)
    }
    #[doc = "Comparator output is inverted"]
    #[inline(always)]
    pub fn cpinv_1(self) -> &'a mut W {
        self.variant(CPINV_A::CPINV_1)
    }
}
#[doc = "Field `CPIES` reader - Interrupt edge select for CEIIFG and CEIFG"]
pub type CPIES_R = crate::BitReader<CPIES_A>;
#[doc = "Interrupt edge select for CEIIFG and CEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPIES_A {
    #[doc = "0: Rising edge for CPIFG, falling edge for CPIIFG"]
    CPIES_0 = 0,
    #[doc = "1: Falling edge for CPIFG, rising edge for CPIIFG"]
    CPIES_1 = 1,
}
impl From<CPIES_A> for bool {
    #[inline(always)]
    fn from(variant: CPIES_A) -> Self {
        variant as u8 != 0
    }
}
impl CPIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIES_A {
        match self.bits {
            false => CPIES_A::CPIES_0,
            true => CPIES_A::CPIES_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIES_0`"]
    #[inline(always)]
    pub fn is_cpies_0(&self) -> bool {
        *self == CPIES_A::CPIES_0
    }
    #[doc = "Checks if the value of the field is `CPIES_1`"]
    #[inline(always)]
    pub fn is_cpies_1(&self) -> bool {
        *self == CPIES_A::CPIES_1
    }
}
#[doc = "Field `CPIES` writer - Interrupt edge select for CEIIFG and CEIFG"]
pub type CPIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPIES_A, O>;
impl<'a, const O: u8> CPIES_W<'a, O> {
    #[doc = "Rising edge for CPIFG, falling edge for CPIIFG"]
    #[inline(always)]
    pub fn cpies_0(self) -> &'a mut W {
        self.variant(CPIES_A::CPIES_0)
    }
    #[doc = "Falling edge for CPIFG, rising edge for CPIIFG"]
    #[inline(always)]
    pub fn cpies_1(self) -> &'a mut W {
        self.variant(CPIES_A::CPIES_1)
    }
}
#[doc = "Field `CPFLT` reader - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
pub type CPFLT_R = crate::BitReader<CPFLT_A>;
#[doc = "Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPFLT_A {
    #[doc = "0: Comparator output is not filtered"]
    CPFLT_0 = 0,
    #[doc = "1: Comparator output is filtered"]
    CPFLT_1 = 1,
}
impl From<CPFLT_A> for bool {
    #[inline(always)]
    fn from(variant: CPFLT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPFLT_A {
        match self.bits {
            false => CPFLT_A::CPFLT_0,
            true => CPFLT_A::CPFLT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPFLT_0`"]
    #[inline(always)]
    pub fn is_cpflt_0(&self) -> bool {
        *self == CPFLT_A::CPFLT_0
    }
    #[doc = "Checks if the value of the field is `CPFLT_1`"]
    #[inline(always)]
    pub fn is_cpflt_1(&self) -> bool {
        *self == CPFLT_A::CPFLT_1
    }
}
#[doc = "Field `CPFLT` writer - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
pub type CPFLT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPFLT_A, O>;
impl<'a, const O: u8> CPFLT_W<'a, O> {
    #[doc = "Comparator output is not filtered"]
    #[inline(always)]
    pub fn cpflt_0(self) -> &'a mut W {
        self.variant(CPFLT_A::CPFLT_0)
    }
    #[doc = "Comparator output is filtered"]
    #[inline(always)]
    pub fn cpflt_1(self) -> &'a mut W {
        self.variant(CPFLT_A::CPFLT_1)
    }
}
#[doc = "Field `CPFLTDLY` reader - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
pub type CPFLTDLY_R = crate::FieldReader<u8, CPFLTDLY_A>;
#[doc = "Analog Filter Delay selection. These bits are used to select the analog filter delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPFLTDLY_A {
    #[doc = "0: Typical filter delay of 450ns"]
    CPFLTDLY_0 = 0,
    #[doc = "1: Typical filter delay of 900ns"]
    CPFLTDLY_1 = 1,
    #[doc = "2: Typical filter delay of 1800ns"]
    CPFLTDLY_2 = 2,
    #[doc = "3: Typical filter delay of 3600ns"]
    CPFLTDLY_3 = 3,
}
impl From<CPFLTDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CPFLTDLY_A) -> Self {
        variant as _
    }
}
impl CPFLTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPFLTDLY_A {
        match self.bits {
            0 => CPFLTDLY_A::CPFLTDLY_0,
            1 => CPFLTDLY_A::CPFLTDLY_1,
            2 => CPFLTDLY_A::CPFLTDLY_2,
            3 => CPFLTDLY_A::CPFLTDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPFLTDLY_0`"]
    #[inline(always)]
    pub fn is_cpfltdly_0(&self) -> bool {
        *self == CPFLTDLY_A::CPFLTDLY_0
    }
    #[doc = "Checks if the value of the field is `CPFLTDLY_1`"]
    #[inline(always)]
    pub fn is_cpfltdly_1(&self) -> bool {
        *self == CPFLTDLY_A::CPFLTDLY_1
    }
    #[doc = "Checks if the value of the field is `CPFLTDLY_2`"]
    #[inline(always)]
    pub fn is_cpfltdly_2(&self) -> bool {
        *self == CPFLTDLY_A::CPFLTDLY_2
    }
    #[doc = "Checks if the value of the field is `CPFLTDLY_3`"]
    #[inline(always)]
    pub fn is_cpfltdly_3(&self) -> bool {
        *self == CPFLTDLY_A::CPFLTDLY_3
    }
}
#[doc = "Field `CPFLTDLY` writer - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
pub type CPFLTDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CP1CTL1_SPEC, u8, CPFLTDLY_A, 2, O>;
impl<'a, const O: u8> CPFLTDLY_W<'a, O> {
    #[doc = "Typical filter delay of 450ns"]
    #[inline(always)]
    pub fn cpfltdly_0(self) -> &'a mut W {
        self.variant(CPFLTDLY_A::CPFLTDLY_0)
    }
    #[doc = "Typical filter delay of 900ns"]
    #[inline(always)]
    pub fn cpfltdly_1(self) -> &'a mut W {
        self.variant(CPFLTDLY_A::CPFLTDLY_1)
    }
    #[doc = "Typical filter delay of 1800ns"]
    #[inline(always)]
    pub fn cpfltdly_2(self) -> &'a mut W {
        self.variant(CPFLTDLY_A::CPFLTDLY_2)
    }
    #[doc = "Typical filter delay of 3600ns"]
    #[inline(always)]
    pub fn cpfltdly_3(self) -> &'a mut W {
        self.variant(CPFLTDLY_A::CPFLTDLY_3)
    }
}
#[doc = "Field `CPMSEL` reader - Power mode selection."]
pub type CPMSEL_R = crate::BitReader<CPMSEL_A>;
#[doc = "Power mode selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPMSEL_A {
    #[doc = "0: High-power & High speed mode (500nA)"]
    CPMSEL_0 = 0,
    #[doc = "1: Low-power & Low speed mode (10nA)"]
    CPMSEL_1 = 1,
}
impl From<CPMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CPMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPMSEL_A {
        match self.bits {
            false => CPMSEL_A::CPMSEL_0,
            true => CPMSEL_A::CPMSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPMSEL_0`"]
    #[inline(always)]
    pub fn is_cpmsel_0(&self) -> bool {
        *self == CPMSEL_A::CPMSEL_0
    }
    #[doc = "Checks if the value of the field is `CPMSEL_1`"]
    #[inline(always)]
    pub fn is_cpmsel_1(&self) -> bool {
        *self == CPMSEL_A::CPMSEL_1
    }
}
#[doc = "Field `CPMSEL` writer - Power mode selection."]
pub type CPMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPMSEL_A, O>;
impl<'a, const O: u8> CPMSEL_W<'a, O> {
    #[doc = "High-power & High speed mode (500nA)"]
    #[inline(always)]
    pub fn cpmsel_0(self) -> &'a mut W {
        self.variant(CPMSEL_A::CPMSEL_0)
    }
    #[doc = "Low-power & Low speed mode (10nA)"]
    #[inline(always)]
    pub fn cpmsel_1(self) -> &'a mut W {
        self.variant(CPMSEL_A::CPMSEL_1)
    }
}
#[doc = "Field `CPEN` reader - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
pub type CPEN_R = crate::BitReader<CPEN_A>;
#[doc = "Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPEN_A {
    #[doc = "0: Comparator is disabled"]
    CPEN_0 = 0,
    #[doc = "1: Comparator is enabled"]
    CPEN_1 = 1,
}
impl From<CPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPEN_A {
        match self.bits {
            false => CPEN_A::CPEN_0,
            true => CPEN_A::CPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPEN_0`"]
    #[inline(always)]
    pub fn is_cpen_0(&self) -> bool {
        *self == CPEN_A::CPEN_0
    }
    #[doc = "Checks if the value of the field is `CPEN_1`"]
    #[inline(always)]
    pub fn is_cpen_1(&self) -> bool {
        *self == CPEN_A::CPEN_1
    }
}
#[doc = "Field `CPEN` writer - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
pub type CPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPEN_A, O>;
impl<'a, const O: u8> CPEN_W<'a, O> {
    #[doc = "Comparator is disabled"]
    #[inline(always)]
    pub fn cpen_0(self) -> &'a mut W {
        self.variant(CPEN_A::CPEN_0)
    }
    #[doc = "Comparator is enabled"]
    #[inline(always)]
    pub fn cpen_1(self) -> &'a mut W {
        self.variant(CPEN_A::CPEN_1)
    }
}
#[doc = "Field `CPHSEL` reader - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
pub type CPHSEL_R = crate::FieldReader<u8, CPHSEL_A>;
#[doc = "Programable Hysteresis mode. These bits are used to select the Hysteresis mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPHSEL_A {
    #[doc = "0: disable"]
    CPHSEL_0 = 0,
    #[doc = "1: 10mV"]
    CPHSEL_1 = 1,
    #[doc = "2: 20mV"]
    CPHSEL_2 = 2,
    #[doc = "3: 30mV"]
    CPHSEL_3 = 3,
}
impl From<CPHSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPHSEL_A) -> Self {
        variant as _
    }
}
impl CPHSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHSEL_A {
        match self.bits {
            0 => CPHSEL_A::CPHSEL_0,
            1 => CPHSEL_A::CPHSEL_1,
            2 => CPHSEL_A::CPHSEL_2,
            3 => CPHSEL_A::CPHSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPHSEL_0`"]
    #[inline(always)]
    pub fn is_cphsel_0(&self) -> bool {
        *self == CPHSEL_A::CPHSEL_0
    }
    #[doc = "Checks if the value of the field is `CPHSEL_1`"]
    #[inline(always)]
    pub fn is_cphsel_1(&self) -> bool {
        *self == CPHSEL_A::CPHSEL_1
    }
    #[doc = "Checks if the value of the field is `CPHSEL_2`"]
    #[inline(always)]
    pub fn is_cphsel_2(&self) -> bool {
        *self == CPHSEL_A::CPHSEL_2
    }
    #[doc = "Checks if the value of the field is `CPHSEL_3`"]
    #[inline(always)]
    pub fn is_cphsel_3(&self) -> bool {
        *self == CPHSEL_A::CPHSEL_3
    }
}
#[doc = "Field `CPHSEL` writer - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
pub type CPHSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CP1CTL1_SPEC, u8, CPHSEL_A, 2, O>;
impl<'a, const O: u8> CPHSEL_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn cphsel_0(self) -> &'a mut W {
        self.variant(CPHSEL_A::CPHSEL_0)
    }
    #[doc = "10mV"]
    #[inline(always)]
    pub fn cphsel_1(self) -> &'a mut W {
        self.variant(CPHSEL_A::CPHSEL_1)
    }
    #[doc = "20mV"]
    #[inline(always)]
    pub fn cphsel_2(self) -> &'a mut W {
        self.variant(CPHSEL_A::CPHSEL_2)
    }
    #[doc = "30mV"]
    #[inline(always)]
    pub fn cphsel_3(self) -> &'a mut W {
        self.variant(CPHSEL_A::CPHSEL_3)
    }
}
#[doc = "Field `CPIE` reader - Comparator interrupt output enable bit"]
pub type CPIE_R = crate::BitReader<CPIE_A>;
#[doc = "Comparator interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPIE_A {
    #[doc = "0: Interrupt output is disabled"]
    CPIE_0 = 0,
    #[doc = "1: Interrupt output is enabled"]
    CPIE_1 = 1,
}
impl From<CPIE_A> for bool {
    #[inline(always)]
    fn from(variant: CPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIE_A {
        match self.bits {
            false => CPIE_A::CPIE_0,
            true => CPIE_A::CPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIE_0`"]
    #[inline(always)]
    pub fn is_cpie_0(&self) -> bool {
        *self == CPIE_A::CPIE_0
    }
    #[doc = "Checks if the value of the field is `CPIE_1`"]
    #[inline(always)]
    pub fn is_cpie_1(&self) -> bool {
        *self == CPIE_A::CPIE_1
    }
}
#[doc = "Field `CPIE` writer - Comparator interrupt output enable bit"]
pub type CPIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPIE_A, O>;
impl<'a, const O: u8> CPIE_W<'a, O> {
    #[doc = "Interrupt output is disabled"]
    #[inline(always)]
    pub fn cpie_0(self) -> &'a mut W {
        self.variant(CPIE_A::CPIE_0)
    }
    #[doc = "Interrupt output is enabled"]
    #[inline(always)]
    pub fn cpie_1(self) -> &'a mut W {
        self.variant(CPIE_A::CPIE_1)
    }
}
#[doc = "Field `CPIIE` reader - Comparator inverted interrupt output enable bit"]
pub type CPIIE_R = crate::BitReader<CPIIE_A>;
#[doc = "Comparator inverted interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPIIE_A {
    #[doc = "0: Interrupt inverted output is disabled"]
    CPIIE_0 = 0,
    #[doc = "1: Interrupt inverted output is enabled"]
    CPIIE_1 = 1,
}
impl From<CPIIE_A> for bool {
    #[inline(always)]
    fn from(variant: CPIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPIIE_A {
        match self.bits {
            false => CPIIE_A::CPIIE_0,
            true => CPIIE_A::CPIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPIIE_0`"]
    #[inline(always)]
    pub fn is_cpiie_0(&self) -> bool {
        *self == CPIIE_A::CPIIE_0
    }
    #[doc = "Checks if the value of the field is `CPIIE_1`"]
    #[inline(always)]
    pub fn is_cpiie_1(&self) -> bool {
        *self == CPIIE_A::CPIIE_1
    }
}
#[doc = "Field `CPIIE` writer - Comparator inverted interrupt output enable bit"]
pub type CPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1CTL1_SPEC, CPIIE_A, O>;
impl<'a, const O: u8> CPIIE_W<'a, O> {
    #[doc = "Interrupt inverted output is disabled"]
    #[inline(always)]
    pub fn cpiie_0(self) -> &'a mut W {
        self.variant(CPIIE_A::CPIIE_0)
    }
    #[doc = "Interrupt inverted output is enabled"]
    #[inline(always)]
    pub fn cpiie_1(self) -> &'a mut W {
        self.variant(CPIIE_A::CPIIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn cpout(&self) -> CPOUT_R {
        CPOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&self) -> CPINV_R {
        CPINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&self) -> CPIES_R {
        CPIES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&self) -> CPFLT_R {
        CPFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&self) -> CPFLTDLY_R {
        CPFLTDLY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&self) -> CPMSEL_R {
        CPMSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&self) -> CPEN_R {
        CPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&self) -> CPHSEL_R {
        CPHSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&self) -> CPIE_R {
        CPIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&self) -> CPIIE_R {
        CPIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&mut self) -> CPINV_W<1> {
        CPINV_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&mut self) -> CPIES_W<4> {
        CPIES_W::new(self)
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&mut self) -> CPFLT_W<5> {
        CPFLT_W::new(self)
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&mut self) -> CPFLTDLY_W<6> {
        CPFLTDLY_W::new(self)
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&mut self) -> CPMSEL_W<8> {
        CPMSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&mut self) -> CPEN_W<9> {
        CPEN_W::new(self)
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&mut self) -> CPHSEL_W<10> {
        CPHSEL_W::new(self)
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&mut self) -> CPIE_W<14> {
        CPIE_W::new(self)
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&mut self) -> CPIIE_W<15> {
        CPIIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1ctl1](index.html) module"]
pub struct CP1CTL1_SPEC;
impl crate::RegisterSpec for CP1CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cp1ctl1::R](R) reader structure"]
impl crate::Readable for CP1CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cp1ctl1::W](W) writer structure"]
impl crate::Writable for CP1CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CP1CTL1 to value 0"]
impl crate::Resettable for CP1CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
