#[doc = "Reader of register ADCCTL0"]
pub type R = crate::R<u16, super::ADCCTL0>;
#[doc = "Writer for register ADCCTL0"]
pub type W = crate::W<u16, super::ADCCTL0>;
#[doc = "Register ADCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSC_A {
    #[doc = "0: No sample-and-conversion-start"]
    ADCSC_0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    ADCSC_1 = 1,
}
impl From<ADCSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADCSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCSC`"]
pub type ADCSC_R = crate::R<bool, ADCSC_A>;
impl ADCSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSC_A {
        match self.bits {
            false => ADCSC_A::ADCSC_0,
            true => ADCSC_A::ADCSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSC_0`"]
    #[inline(always)]
    pub fn is_adcsc_0(&self) -> bool {
        *self == ADCSC_A::ADCSC_0
    }
    #[doc = "Checks if the value of the field is `ADCSC_1`"]
    #[inline(always)]
    pub fn is_adcsc_1(&self) -> bool {
        *self == ADCSC_A::ADCSC_1
    }
}
#[doc = "Write proxy for field `ADCSC`"]
pub struct ADCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adcsc_0(self) -> &'a mut W {
        self.variant(ADCSC_A::ADCSC_0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adcsc_1(self) -> &'a mut W {
        self.variant(ADCSC_A::ADCSC_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCENC_A {
    #[doc = "0: ADC disabled"]
    ADCENC_0 = 0,
    #[doc = "1: ADC enabled"]
    ADCENC_1 = 1,
}
impl From<ADCENC_A> for bool {
    #[inline(always)]
    fn from(variant: ADCENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCENC`"]
pub type ADCENC_R = crate::R<bool, ADCENC_A>;
impl ADCENC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCENC_A {
        match self.bits {
            false => ADCENC_A::ADCENC_0,
            true => ADCENC_A::ADCENC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCENC_0`"]
    #[inline(always)]
    pub fn is_adcenc_0(&self) -> bool {
        *self == ADCENC_A::ADCENC_0
    }
    #[doc = "Checks if the value of the field is `ADCENC_1`"]
    #[inline(always)]
    pub fn is_adcenc_1(&self) -> bool {
        *self == ADCENC_A::ADCENC_1
    }
}
#[doc = "Write proxy for field `ADCENC`"]
pub struct ADCENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCENC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn adcenc_0(self) -> &'a mut W {
        self.variant(ADCENC_A::ADCENC_0)
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn adcenc_1(self) -> &'a mut W {
        self.variant(ADCENC_A::ADCENC_1)
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
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCON_A {
    #[doc = "0: ADC off"]
    ADCON_0 = 0,
    #[doc = "1: ADC on"]
    ADCON_1 = 1,
}
impl From<ADCON_A> for bool {
    #[inline(always)]
    fn from(variant: ADCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCON`"]
pub type ADCON_R = crate::R<bool, ADCON_A>;
impl ADCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCON_A {
        match self.bits {
            false => ADCON_A::ADCON_0,
            true => ADCON_A::ADCON_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCON_0`"]
    #[inline(always)]
    pub fn is_adcon_0(&self) -> bool {
        *self == ADCON_A::ADCON_0
    }
    #[doc = "Checks if the value of the field is `ADCON_1`"]
    #[inline(always)]
    pub fn is_adcon_1(&self) -> bool {
        *self == ADCON_A::ADCON_1
    }
}
#[doc = "Write proxy for field `ADCON`"]
pub struct ADCON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC off"]
    #[inline(always)]
    pub fn adcon_0(self) -> &'a mut W {
        self.variant(ADCON_A::ADCON_0)
    }
    #[doc = "ADC on"]
    #[inline(always)]
    pub fn adcon_1(self) -> &'a mut W {
        self.variant(ADCON_A::ADCON_1)
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
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMSC_A {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    ADCMSC_0 = 0,
    #[doc = "1: The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    ADCMSC_1 = 1,
}
impl From<ADCMSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADCMSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCMSC`"]
pub type ADCMSC_R = crate::R<bool, ADCMSC_A>;
impl ADCMSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMSC_A {
        match self.bits {
            false => ADCMSC_A::ADCMSC_0,
            true => ADCMSC_A::ADCMSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCMSC_0`"]
    #[inline(always)]
    pub fn is_adcmsc_0(&self) -> bool {
        *self == ADCMSC_A::ADCMSC_0
    }
    #[doc = "Checks if the value of the field is `ADCMSC_1`"]
    #[inline(always)]
    pub fn is_adcmsc_1(&self) -> bool {
        *self == ADCMSC_A::ADCMSC_1
    }
}
#[doc = "Write proxy for field `ADCMSC`"]
pub struct ADCMSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn adcmsc_0(self) -> &'a mut W {
        self.variant(ADCMSC_A::ADCMSC_0)
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn adcmsc_1(self) -> &'a mut W {
        self.variant(ADCMSC_A::ADCMSC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSHT_A {
    #[doc = "0: 4 ADCCLK cycles"]
    ADCSHT_0 = 0,
    #[doc = "1: 8 ADCCLK cycles"]
    ADCSHT_1 = 1,
    #[doc = "2: 16 ADCCLK cycles"]
    ADCSHT_2 = 2,
    #[doc = "3: 32 ADCCLK cycles"]
    ADCSHT_3 = 3,
    #[doc = "4: 64 ADCCLK cycles"]
    ADCSHT_4 = 4,
    #[doc = "5: 96 ADCCLK cycles"]
    ADCSHT_5 = 5,
    #[doc = "6: 128 ADCCLK cycles"]
    ADCSHT_6 = 6,
    #[doc = "7: 192 ADCCLK cycles"]
    ADCSHT_7 = 7,
    #[doc = "8: 256 ADCCLK cycles"]
    ADCSHT_8 = 8,
    #[doc = "9: 384 ADCCLK cycles"]
    ADCSHT_9 = 9,
    #[doc = "10: 512 ADCCLK cycles"]
    ADCSHT_10 = 10,
    #[doc = "11: 768 ADCCLK cycles"]
    ADCSHT_11 = 11,
    #[doc = "12: 1024 ADCCLK cycles"]
    ADCSHT_12 = 12,
    #[doc = "13: 1024 ADCCLK cycles"]
    ADCSHT_13 = 13,
    #[doc = "14: 1024 ADCCLK cycles"]
    ADCSHT_14 = 14,
    #[doc = "15: 1024 ADCCLK cycles"]
    ADCSHT_15 = 15,
}
impl From<ADCSHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSHT`"]
pub type ADCSHT_R = crate::R<u8, ADCSHT_A>;
impl ADCSHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSHT_A {
        match self.bits {
            0 => ADCSHT_A::ADCSHT_0,
            1 => ADCSHT_A::ADCSHT_1,
            2 => ADCSHT_A::ADCSHT_2,
            3 => ADCSHT_A::ADCSHT_3,
            4 => ADCSHT_A::ADCSHT_4,
            5 => ADCSHT_A::ADCSHT_5,
            6 => ADCSHT_A::ADCSHT_6,
            7 => ADCSHT_A::ADCSHT_7,
            8 => ADCSHT_A::ADCSHT_8,
            9 => ADCSHT_A::ADCSHT_9,
            10 => ADCSHT_A::ADCSHT_10,
            11 => ADCSHT_A::ADCSHT_11,
            12 => ADCSHT_A::ADCSHT_12,
            13 => ADCSHT_A::ADCSHT_13,
            14 => ADCSHT_A::ADCSHT_14,
            15 => ADCSHT_A::ADCSHT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHT_0`"]
    #[inline(always)]
    pub fn is_adcsht_0(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_0
    }
    #[doc = "Checks if the value of the field is `ADCSHT_1`"]
    #[inline(always)]
    pub fn is_adcsht_1(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_1
    }
    #[doc = "Checks if the value of the field is `ADCSHT_2`"]
    #[inline(always)]
    pub fn is_adcsht_2(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_2
    }
    #[doc = "Checks if the value of the field is `ADCSHT_3`"]
    #[inline(always)]
    pub fn is_adcsht_3(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_3
    }
    #[doc = "Checks if the value of the field is `ADCSHT_4`"]
    #[inline(always)]
    pub fn is_adcsht_4(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_4
    }
    #[doc = "Checks if the value of the field is `ADCSHT_5`"]
    #[inline(always)]
    pub fn is_adcsht_5(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_5
    }
    #[doc = "Checks if the value of the field is `ADCSHT_6`"]
    #[inline(always)]
    pub fn is_adcsht_6(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_6
    }
    #[doc = "Checks if the value of the field is `ADCSHT_7`"]
    #[inline(always)]
    pub fn is_adcsht_7(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_7
    }
    #[doc = "Checks if the value of the field is `ADCSHT_8`"]
    #[inline(always)]
    pub fn is_adcsht_8(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_8
    }
    #[doc = "Checks if the value of the field is `ADCSHT_9`"]
    #[inline(always)]
    pub fn is_adcsht_9(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_9
    }
    #[doc = "Checks if the value of the field is `ADCSHT_10`"]
    #[inline(always)]
    pub fn is_adcsht_10(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_10
    }
    #[doc = "Checks if the value of the field is `ADCSHT_11`"]
    #[inline(always)]
    pub fn is_adcsht_11(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_11
    }
    #[doc = "Checks if the value of the field is `ADCSHT_12`"]
    #[inline(always)]
    pub fn is_adcsht_12(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_12
    }
    #[doc = "Checks if the value of the field is `ADCSHT_13`"]
    #[inline(always)]
    pub fn is_adcsht_13(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_13
    }
    #[doc = "Checks if the value of the field is `ADCSHT_14`"]
    #[inline(always)]
    pub fn is_adcsht_14(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_14
    }
    #[doc = "Checks if the value of the field is `ADCSHT_15`"]
    #[inline(always)]
    pub fn is_adcsht_15(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_15
    }
}
#[doc = "Write proxy for field `ADCSHT`"]
pub struct ADCSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSHT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_0(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_0)
    }
    #[doc = "8 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_1(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_1)
    }
    #[doc = "16 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_2(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_2)
    }
    #[doc = "32 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_3(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_3)
    }
    #[doc = "64 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_4(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_4)
    }
    #[doc = "96 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_5(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_5)
    }
    #[doc = "128 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_6(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_6)
    }
    #[doc = "192 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_7(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_7)
    }
    #[doc = "256 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_8(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_8)
    }
    #[doc = "384 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_9(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_9)
    }
    #[doc = "512 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_10(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_10)
    }
    #[doc = "768 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_11(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_11)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_12(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_12)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_13(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_13)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_14(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_14)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_15(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adcsc(&self) -> ADCSC_R {
        ADCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adcenc(&self) -> ADCENC_R {
        ADCENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcmsc(&self) -> ADCMSC_R {
        ADCMSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcsht(&self) -> ADCSHT_R {
        ADCSHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adcsc(&mut self) -> ADCSC_W {
        ADCSC_W { w: self }
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adcenc(&mut self) -> ADCENC_W {
        ADCENC_W { w: self }
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adcon(&mut self) -> ADCON_W {
        ADCON_W { w: self }
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcmsc(&mut self) -> ADCMSC_W {
        ADCMSC_W { w: self }
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcsht(&mut self) -> ADCSHT_W {
        ADCSHT_W { w: self }
    }
}
