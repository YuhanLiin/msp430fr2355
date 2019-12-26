#[doc = "Reader of register ADCCTL1"]
pub type R = crate::R<u16, super::ADCCTL1>;
#[doc = "Writer for register ADCCTL1"]
pub type W = crate::W<u16, super::ADCCTL1>;
#[doc = "Register ADCCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCBUSY_A {
    #[doc = "0: No operation is active."]
    ADCBUSY_0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active."]
    ADCBUSY_1 = 1,
}
impl From<ADCBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ADCBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCBUSY`"]
pub type ADCBUSY_R = crate::R<bool, ADCBUSY_A>;
impl ADCBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCBUSY_A {
        match self.bits {
            false => ADCBUSY_A::ADCBUSY_0,
            true => ADCBUSY_A::ADCBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCBUSY_0`"]
    #[inline(always)]
    pub fn is_adcbusy_0(&self) -> bool {
        *self == ADCBUSY_A::ADCBUSY_0
    }
    #[doc = "Checks if the value of the field is `ADCBUSY_1`"]
    #[inline(always)]
    pub fn is_adcbusy_1(&self) -> bool {
        *self == ADCBUSY_A::ADCBUSY_1
    }
}
#[doc = "conversion sequence mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCCONSEQ_A {
    #[doc = "0: Single-channel, single-conversion"]
    ADCCONSEQ_0 = 0,
    #[doc = "1: Sequence-of-channels"]
    ADCCONSEQ_1 = 1,
    #[doc = "2: Repeat-single-channel"]
    ADCCONSEQ_2 = 2,
    #[doc = "3: Repeat-sequence-of-channels"]
    ADCCONSEQ_3 = 3,
}
impl From<ADCCONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCCONSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCCONSEQ`"]
pub type ADCCONSEQ_R = crate::R<u8, ADCCONSEQ_A>;
impl ADCCONSEQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCONSEQ_A {
        match self.bits {
            0 => ADCCONSEQ_A::ADCCONSEQ_0,
            1 => ADCCONSEQ_A::ADCCONSEQ_1,
            2 => ADCCONSEQ_A::ADCCONSEQ_2,
            3 => ADCCONSEQ_A::ADCCONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_0`"]
    #[inline(always)]
    pub fn is_adcconseq_0(&self) -> bool {
        *self == ADCCONSEQ_A::ADCCONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_1`"]
    #[inline(always)]
    pub fn is_adcconseq_1(&self) -> bool {
        *self == ADCCONSEQ_A::ADCCONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_2`"]
    #[inline(always)]
    pub fn is_adcconseq_2(&self) -> bool {
        *self == ADCCONSEQ_A::ADCCONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_3`"]
    #[inline(always)]
    pub fn is_adcconseq_3(&self) -> bool {
        *self == ADCCONSEQ_A::ADCCONSEQ_3
    }
}
#[doc = "Write proxy for field `ADCCONSEQ`"]
pub struct ADCCONSEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCONSEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCONSEQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn adcconseq_0(self) -> &'a mut W {
        self.variant(ADCCONSEQ_A::ADCCONSEQ_0)
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn adcconseq_1(self) -> &'a mut W {
        self.variant(ADCCONSEQ_A::ADCCONSEQ_1)
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn adcconseq_2(self) -> &'a mut W {
        self.variant(ADCCONSEQ_A::ADCCONSEQ_2)
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn adcconseq_3(self) -> &'a mut W {
        self.variant(ADCCONSEQ_A::ADCCONSEQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u16) & 0x03) << 1);
        self.w
    }
}
#[doc = "clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSSEL_A {
    #[doc = "0: ADCOSC (MODOSC)"]
    ADCSSEL_0 = 0,
    #[doc = "1: ACLK"]
    ADCSSEL_1 = 1,
    #[doc = "2: MCLK"]
    ADCSSEL_2 = 2,
    #[doc = "3: SMCLK"]
    ADCSSEL_3 = 3,
}
impl From<ADCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSSEL`"]
pub type ADCSSEL_R = crate::R<u8, ADCSSEL_A>;
impl ADCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSSEL_A {
        match self.bits {
            0 => ADCSSEL_A::ADCSSEL_0,
            1 => ADCSSEL_A::ADCSSEL_1,
            2 => ADCSSEL_A::ADCSSEL_2,
            3 => ADCSSEL_A::ADCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_0`"]
    #[inline(always)]
    pub fn is_adcssel_0(&self) -> bool {
        *self == ADCSSEL_A::ADCSSEL_0
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_1`"]
    #[inline(always)]
    pub fn is_adcssel_1(&self) -> bool {
        *self == ADCSSEL_A::ADCSSEL_1
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_2`"]
    #[inline(always)]
    pub fn is_adcssel_2(&self) -> bool {
        *self == ADCSSEL_A::ADCSSEL_2
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_3`"]
    #[inline(always)]
    pub fn is_adcssel_3(&self) -> bool {
        *self == ADCSSEL_A::ADCSSEL_3
    }
}
#[doc = "Write proxy for field `ADCSSEL`"]
pub struct ADCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADCOSC (MODOSC)"]
    #[inline(always)]
    pub fn adcssel_0(self) -> &'a mut W {
        self.variant(ADCSSEL_A::ADCSSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adcssel_1(self) -> &'a mut W {
        self.variant(ADCSSEL_A::ADCSSEL_1)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adcssel_2(self) -> &'a mut W {
        self.variant(ADCSSEL_A::ADCSSEL_2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adcssel_3(self) -> &'a mut W {
        self.variant(ADCSSEL_A::ADCSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u16) & 0x03) << 3);
        self.w
    }
}
#[doc = "clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCDIV_A {
    #[doc = "0: /1"]
    ADCDIV_0 = 0,
    #[doc = "1: /2"]
    ADCDIV_1 = 1,
    #[doc = "2: /3"]
    ADCDIV_2 = 2,
    #[doc = "3: /4"]
    ADCDIV_3 = 3,
    #[doc = "4: /5"]
    ADCDIV_4 = 4,
    #[doc = "5: /6"]
    ADCDIV_5 = 5,
    #[doc = "6: /7"]
    ADCDIV_6 = 6,
    #[doc = "7: /8"]
    ADCDIV_7 = 7,
}
impl From<ADCDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCDIV`"]
pub type ADCDIV_R = crate::R<u8, ADCDIV_A>;
impl ADCDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCDIV_A {
        match self.bits {
            0 => ADCDIV_A::ADCDIV_0,
            1 => ADCDIV_A::ADCDIV_1,
            2 => ADCDIV_A::ADCDIV_2,
            3 => ADCDIV_A::ADCDIV_3,
            4 => ADCDIV_A::ADCDIV_4,
            5 => ADCDIV_A::ADCDIV_5,
            6 => ADCDIV_A::ADCDIV_6,
            7 => ADCDIV_A::ADCDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCDIV_0`"]
    #[inline(always)]
    pub fn is_adcdiv_0(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_0
    }
    #[doc = "Checks if the value of the field is `ADCDIV_1`"]
    #[inline(always)]
    pub fn is_adcdiv_1(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_1
    }
    #[doc = "Checks if the value of the field is `ADCDIV_2`"]
    #[inline(always)]
    pub fn is_adcdiv_2(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_2
    }
    #[doc = "Checks if the value of the field is `ADCDIV_3`"]
    #[inline(always)]
    pub fn is_adcdiv_3(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_3
    }
    #[doc = "Checks if the value of the field is `ADCDIV_4`"]
    #[inline(always)]
    pub fn is_adcdiv_4(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_4
    }
    #[doc = "Checks if the value of the field is `ADCDIV_5`"]
    #[inline(always)]
    pub fn is_adcdiv_5(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_5
    }
    #[doc = "Checks if the value of the field is `ADCDIV_6`"]
    #[inline(always)]
    pub fn is_adcdiv_6(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_6
    }
    #[doc = "Checks if the value of the field is `ADCDIV_7`"]
    #[inline(always)]
    pub fn is_adcdiv_7(&self) -> bool {
        *self == ADCDIV_A::ADCDIV_7
    }
}
#[doc = "Write proxy for field `ADCDIV`"]
pub struct ADCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn adcdiv_0(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn adcdiv_1(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn adcdiv_2(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn adcdiv_3(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn adcdiv_4(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn adcdiv_5(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_5)
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn adcdiv_6(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn adcdiv_7(self) -> &'a mut W {
        self.variant(ADCDIV_A::ADCDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "invert signal sample-and-hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCISSH_A {
    #[doc = "0: The sample-input signal is not inverted."]
    ADCISSH_0 = 0,
    #[doc = "1: The sample-input signal is inverted."]
    ADCISSH_1 = 1,
}
impl From<ADCISSH_A> for bool {
    #[inline(always)]
    fn from(variant: ADCISSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCISSH`"]
pub type ADCISSH_R = crate::R<bool, ADCISSH_A>;
impl ADCISSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCISSH_A {
        match self.bits {
            false => ADCISSH_A::ADCISSH_0,
            true => ADCISSH_A::ADCISSH_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCISSH_0`"]
    #[inline(always)]
    pub fn is_adcissh_0(&self) -> bool {
        *self == ADCISSH_A::ADCISSH_0
    }
    #[doc = "Checks if the value of the field is `ADCISSH_1`"]
    #[inline(always)]
    pub fn is_adcissh_1(&self) -> bool {
        *self == ADCISSH_A::ADCISSH_1
    }
}
#[doc = "Write proxy for field `ADCISSH`"]
pub struct ADCISSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCISSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCISSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn adcissh_0(self) -> &'a mut W {
        self.variant(ADCISSH_A::ADCISSH_0)
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn adcissh_1(self) -> &'a mut W {
        self.variant(ADCISSH_A::ADCISSH_1)
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
#[doc = "sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSHP_A {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal."]
    ADCSHP_0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer."]
    ADCSHP_1 = 1,
}
impl From<ADCSHP_A> for bool {
    #[inline(always)]
    fn from(variant: ADCSHP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCSHP`"]
pub type ADCSHP_R = crate::R<bool, ADCSHP_A>;
impl ADCSHP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSHP_A {
        match self.bits {
            false => ADCSHP_A::ADCSHP_0,
            true => ADCSHP_A::ADCSHP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHP_0`"]
    #[inline(always)]
    pub fn is_adcshp_0(&self) -> bool {
        *self == ADCSHP_A::ADCSHP_0
    }
    #[doc = "Checks if the value of the field is `ADCSHP_1`"]
    #[inline(always)]
    pub fn is_adcshp_1(&self) -> bool {
        *self == ADCSHP_A::ADCSHP_1
    }
}
#[doc = "Write proxy for field `ADCSHP`"]
pub struct ADCSHP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSHP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSHP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn adcshp_0(self) -> &'a mut W {
        self.variant(ADCSHP_A::ADCSHP_0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn adcshp_1(self) -> &'a mut W {
        self.variant(ADCSHP_A::ADCSHP_1)
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
#[doc = "sample-and-hold source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSHS_A {
    #[doc = "0: ADCSC bit"]
    ADCSHS_0 = 0,
    #[doc = "1: see the device-specific data sheet for source"]
    ADCSHS_1 = 1,
    #[doc = "2: see the device-specific data sheet for source"]
    ADCSHS_2 = 2,
    #[doc = "3: see the device-specific data sheet for source"]
    ADCSHS_3 = 3,
}
impl From<ADCSHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSHS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSHS`"]
pub type ADCSHS_R = crate::R<u8, ADCSHS_A>;
impl ADCSHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSHS_A {
        match self.bits {
            0 => ADCSHS_A::ADCSHS_0,
            1 => ADCSHS_A::ADCSHS_1,
            2 => ADCSHS_A::ADCSHS_2,
            3 => ADCSHS_A::ADCSHS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHS_0`"]
    #[inline(always)]
    pub fn is_adcshs_0(&self) -> bool {
        *self == ADCSHS_A::ADCSHS_0
    }
    #[doc = "Checks if the value of the field is `ADCSHS_1`"]
    #[inline(always)]
    pub fn is_adcshs_1(&self) -> bool {
        *self == ADCSHS_A::ADCSHS_1
    }
    #[doc = "Checks if the value of the field is `ADCSHS_2`"]
    #[inline(always)]
    pub fn is_adcshs_2(&self) -> bool {
        *self == ADCSHS_A::ADCSHS_2
    }
    #[doc = "Checks if the value of the field is `ADCSHS_3`"]
    #[inline(always)]
    pub fn is_adcshs_3(&self) -> bool {
        *self == ADCSHS_A::ADCSHS_3
    }
}
#[doc = "Write proxy for field `ADCSHS`"]
pub struct ADCSHS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSHS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADCSC bit"]
    #[inline(always)]
    pub fn adcshs_0(self) -> &'a mut W {
        self.variant(ADCSHS_A::ADCSHS_0)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adcshs_1(self) -> &'a mut W {
        self.variant(ADCSHS_A::ADCSHS_1)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adcshs_2(self) -> &'a mut W {
        self.variant(ADCSHS_A::ADCSHS_2)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adcshs_3(self) -> &'a mut W {
        self.variant(ADCSHS_A::ADCSHS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC busy"]
    #[inline(always)]
    pub fn adcbusy(&self) -> ADCBUSY_R {
        ADCBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adcconseq(&self) -> ADCCONSEQ_R {
        ADCCONSEQ_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adcssel(&self) -> ADCSSEL_R {
        ADCSSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adcdiv(&self) -> ADCDIV_R {
        ADCDIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adcissh(&self) -> ADCISSH_R {
        ADCISSH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adcshp(&self) -> ADCSHP_R {
        ADCSHP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adcshs(&self) -> ADCSHS_R {
        ADCSHS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adcconseq(&mut self) -> ADCCONSEQ_W {
        ADCCONSEQ_W { w: self }
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adcssel(&mut self) -> ADCSSEL_W {
        ADCSSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> ADCDIV_W {
        ADCDIV_W { w: self }
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adcissh(&mut self) -> ADCISSH_W {
        ADCISSH_W { w: self }
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adcshp(&mut self) -> ADCSHP_W {
        ADCSHP_W { w: self }
    }
    #[doc = "Bits 10:11 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adcshs(&mut self) -> ADCSHS_W {
        ADCSHS_W { w: self }
    }
}
