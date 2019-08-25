#[doc = "Reader of register CP1CTL1"]
pub type R = crate::R<u16, super::CP1CTL1>;
#[doc = "Writer for register CP1CTL1"]
pub type W = crate::W<u16, super::CP1CTL1>;
#[doc = "Register CP1CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CP1CTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator output value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOUT_A {}
impl From<CPOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOUT_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `CPOUT`"]
pub type CPOUT_R = crate::R<bool, CPOUT_A>;
impl CPOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CPOUT_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Comparator output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPINV_A {
    #[doc = "0: Comparator output is non-inverted"]
    CPINV_0,
    #[doc = "1: Comparator output is inverted"]
    CPINV_1,
}
impl From<CPINV_A> for bool {
    #[inline(always)]
    fn from(variant: CPINV_A) -> Self {
        match variant {
            CPINV_A::CPINV_0 => false,
            CPINV_A::CPINV_1 => true,
        }
    }
}
#[doc = "Reader of field `CPINV`"]
pub type CPINV_R = crate::R<bool, CPINV_A>;
impl CPINV_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPINV`"]
pub struct CPINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Interrupt edge select for CEIIFG and CEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIES_A {
    #[doc = "0: Rising edge for CPIFG, falling edge for CPIIFG"]
    CPIES_0,
    #[doc = "1: Falling edge for CPIFG, rising edge for CPIIFG"]
    CPIES_1,
}
impl From<CPIES_A> for bool {
    #[inline(always)]
    fn from(variant: CPIES_A) -> Self {
        match variant {
            CPIES_A::CPIES_0 => false,
            CPIES_A::CPIES_1 => true,
        }
    }
}
#[doc = "Reader of field `CPIES`"]
pub type CPIES_R = crate::R<bool, CPIES_A>;
impl CPIES_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPIES`"]
pub struct CPIES_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPFLT_A {
    #[doc = "0: Comparator output is not filtered"]
    CPFLT_0,
    #[doc = "1: Comparator output is filtered"]
    CPFLT_1,
}
impl From<CPFLT_A> for bool {
    #[inline(always)]
    fn from(variant: CPFLT_A) -> Self {
        match variant {
            CPFLT_A::CPFLT_0 => false,
            CPFLT_A::CPFLT_1 => true,
        }
    }
}
#[doc = "Reader of field `CPFLT`"]
pub type CPFLT_R = crate::R<bool, CPFLT_A>;
impl CPFLT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPFLT`"]
pub struct CPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPFLT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Analog Filter Delay selection. These bits are used to select the analog filter delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPFLTDLY_A {
    #[doc = "0: Typical filter delay of 450ns"]
    CPFLTDLY_0,
    #[doc = "1: Typical filter delay of 900ns"]
    CPFLTDLY_1,
    #[doc = "2: Typical filter delay of 1800ns"]
    CPFLTDLY_2,
    #[doc = "3: Typical filter delay of 3600ns"]
    CPFLTDLY_3,
}
impl From<CPFLTDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CPFLTDLY_A) -> Self {
        match variant {
            CPFLTDLY_A::CPFLTDLY_0 => 0,
            CPFLTDLY_A::CPFLTDLY_1 => 1,
            CPFLTDLY_A::CPFLTDLY_2 => 2,
            CPFLTDLY_A::CPFLTDLY_3 => 3,
        }
    }
}
#[doc = "Reader of field `CPFLTDLY`"]
pub type CPFLTDLY_R = crate::R<u8, CPFLTDLY_A>;
impl CPFLTDLY_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPFLTDLY`"]
pub struct CPFLTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CPFLTDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPFLTDLY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Power mode selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPMSEL_A {
    #[doc = "0: High-power & High speed mode (500nA)"]
    CPMSEL_0,
    #[doc = "1: Low-power & Low speed mode (10nA)"]
    CPMSEL_1,
}
impl From<CPMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CPMSEL_A) -> Self {
        match variant {
            CPMSEL_A::CPMSEL_0 => false,
            CPMSEL_A::CPMSEL_1 => true,
        }
    }
}
#[doc = "Reader of field `CPMSEL`"]
pub type CPMSEL_R = crate::R<bool, CPMSEL_A>;
impl CPMSEL_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPMSEL`"]
pub struct CPMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPEN_A {
    #[doc = "0: Comparator is disabled"]
    CPEN_0,
    #[doc = "1: Comparator is enabled"]
    CPEN_1,
}
impl From<CPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPEN_A) -> Self {
        match variant {
            CPEN_A::CPEN_0 => false,
            CPEN_A::CPEN_1 => true,
        }
    }
}
#[doc = "Reader of field `CPEN`"]
pub type CPEN_R = crate::R<bool, CPEN_A>;
impl CPEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPEN`"]
pub struct CPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Programable Hysteresis mode. These bits are used to select the Hysteresis mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHSEL_A {
    #[doc = "0: disable"]
    CPHSEL_0,
    #[doc = "1: 10mV"]
    CPHSEL_1,
    #[doc = "2: 20mV"]
    CPHSEL_2,
    #[doc = "3: 30mV"]
    CPHSEL_3,
}
impl From<CPHSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPHSEL_A) -> Self {
        match variant {
            CPHSEL_A::CPHSEL_0 => 0,
            CPHSEL_A::CPHSEL_1 => 1,
            CPHSEL_A::CPHSEL_2 => 2,
            CPHSEL_A::CPHSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `CPHSEL`"]
pub type CPHSEL_R = crate::R<u8, CPHSEL_A>;
impl CPHSEL_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPHSEL`"]
pub struct CPHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Comparator interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIE_A {
    #[doc = "0: Interrupt output is disabled"]
    CPIE_0,
    #[doc = "1: Interrupt output is enabled"]
    CPIE_1,
}
impl From<CPIE_A> for bool {
    #[inline(always)]
    fn from(variant: CPIE_A) -> Self {
        match variant {
            CPIE_A::CPIE_0 => false,
            CPIE_A::CPIE_1 => true,
        }
    }
}
#[doc = "Reader of field `CPIE`"]
pub type CPIE_R = crate::R<bool, CPIE_A>;
impl CPIE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPIE`"]
pub struct CPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Comparator inverted interrupt output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIIE_A {
    #[doc = "0: Interrupt inverted output is disabled"]
    CPIIE_0,
    #[doc = "1: Interrupt inverted output is enabled"]
    CPIIE_1,
}
impl From<CPIIE_A> for bool {
    #[inline(always)]
    fn from(variant: CPIIE_A) -> Self {
        match variant {
            CPIIE_A::CPIIE_0 => false,
            CPIIE_A::CPIIE_1 => true,
        }
    }
}
#[doc = "Reader of field `CPIIE`"]
pub type CPIIE_R = crate::R<bool, CPIIE_A>;
impl CPIIE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CPIIE`"]
pub struct CPIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output value"]
    #[inline(always)]
    pub fn cpout(&self) -> CPOUT_R {
        CPOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&self) -> CPINV_R {
        CPINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&self) -> CPIES_R {
        CPIES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&self) -> CPFLT_R {
        CPFLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&self) -> CPFLTDLY_R {
        CPFLTDLY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&self) -> CPMSEL_R {
        CPMSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&self) -> CPEN_R {
        CPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&self) -> CPHSEL_R {
        CPHSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&self) -> CPIE_R {
        CPIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&self) -> CPIIE_R {
        CPIIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator output polarity"]
    #[inline(always)]
    pub fn cpinv(&mut self) -> CPINV_W {
        CPINV_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt edge select for CEIIFG and CEIFG"]
    #[inline(always)]
    pub fn cpies(&mut self) -> CPIES_W {
        CPIES_W { w: self }
    }
    #[doc = "Bit 5 - Analog Output Low Pass filter Selection. Changing CPFLT might set interrupt flag."]
    #[inline(always)]
    pub fn cpflt(&mut self) -> CPFLT_W {
        CPFLT_W { w: self }
    }
    #[doc = "Bits 6:7 - Analog Filter Delay selection. These bits are used to select the analog filter delay"]
    #[inline(always)]
    pub fn cpfltdly(&mut self) -> CPFLTDLY_W {
        CPFLTDLY_W { w: self }
    }
    #[doc = "Bit 8 - Power mode selection."]
    #[inline(always)]
    pub fn cpmsel(&mut self) -> CPMSEL_W {
        CPMSEL_W { w: self }
    }
    #[doc = "Bit 9 - Comparator enable/disable. This bit is used to disable/enable the comparator. When the comparator is disabled, the Comparator consumes no power."]
    #[inline(always)]
    pub fn cpen(&mut self) -> CPEN_W {
        CPEN_W { w: self }
    }
    #[doc = "Bits 10:11 - Programable Hysteresis mode. These bits are used to select the Hysteresis mode."]
    #[inline(always)]
    pub fn cphsel(&mut self) -> CPHSEL_W {
        CPHSEL_W { w: self }
    }
    #[doc = "Bit 14 - Comparator interrupt output enable bit"]
    #[inline(always)]
    pub fn cpie(&mut self) -> CPIE_W {
        CPIE_W { w: self }
    }
    #[doc = "Bit 15 - Comparator inverted interrupt output enable bit"]
    #[inline(always)]
    pub fn cpiie(&mut self) -> CPIIE_W {
        CPIIE_W { w: self }
    }
}
