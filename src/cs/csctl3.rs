#[doc = "Reader of register CSCTL3"]
pub type R = crate::R<u16, super::CSCTL3>;
#[doc = "Writer for register CSCTL3"]
pub type W = crate::W<u16, super::CSCTL3>;
#[doc = "Register CSCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLREFDIV_A {
    #[doc = "0: fFLLREFCLK / 1"]
    _1,
    #[doc = "1: fFLLREFCLK / 32"]
    _32,
    #[doc = "2: fFLLREFCLK / 64"]
    _64,
    #[doc = "3: fFLLREFCLK / 128"]
    _128,
    #[doc = "4: fFLLREFCLK / 256"]
    _256,
    #[doc = "5: fFLLREFCLK / 512"]
    _512,
    #[doc = "6: fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    FLLREFDIV_6,
    #[doc = "7: fFLLREFCLK / 768(only available in 24MHz clock system)"]
    FLLREFDIV_7,
}
impl From<FLLREFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLREFDIV_A) -> Self {
        match variant {
            FLLREFDIV_A::_1 => 0,
            FLLREFDIV_A::_32 => 1,
            FLLREFDIV_A::_64 => 2,
            FLLREFDIV_A::_128 => 3,
            FLLREFDIV_A::_256 => 4,
            FLLREFDIV_A::_512 => 5,
            FLLREFDIV_A::FLLREFDIV_6 => 6,
            FLLREFDIV_A::FLLREFDIV_7 => 7,
        }
    }
}
#[doc = "Reader of field `FLLREFDIV`"]
pub type FLLREFDIV_R = crate::R<u8, FLLREFDIV_A>;
impl FLLREFDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLREFDIV_A {
        match self.bits {
            0 => FLLREFDIV_A::_1,
            1 => FLLREFDIV_A::_32,
            2 => FLLREFDIV_A::_64,
            3 => FLLREFDIV_A::_128,
            4 => FLLREFDIV_A::_256,
            5 => FLLREFDIV_A::_512,
            6 => FLLREFDIV_A::FLLREFDIV_6,
            7 => FLLREFDIV_A::FLLREFDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLREFDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == FLLREFDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == FLLREFDIV_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == FLLREFDIV_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == FLLREFDIV_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == FLLREFDIV_A::_512
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_6`"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_6
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_7`"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_7
    }
}
#[doc = "Write proxy for field `FLLREFDIV`"]
pub struct FLLREFDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLREFDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLREFDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_1)
    }
    #[doc = "fFLLREFCLK / 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_32)
    }
    #[doc = "fFLLREFCLK / 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_64)
    }
    #[doc = "fFLLREFCLK / 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_128)
    }
    #[doc = "fFLLREFCLK / 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_256)
    }
    #[doc = "fFLLREFCLK / 512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::_512)
    }
    #[doc = "fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_6)
    }
    #[doc = "fFLLREFCLK / 768(only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "FLL reference select. These bits select the FLL reference clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELREF_A {
    #[doc = "0: XT1CLK"]
    XT1CLK,
    #[doc = "1: REFOCLK"]
    REFOCLK,
    #[doc = "2: served for future use"]
    SELREF_2,
    #[doc = "3: served for future use"]
    SELREF_3,
}
impl From<SELREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELREF_A) -> Self {
        match variant {
            SELREF_A::XT1CLK => 0,
            SELREF_A::REFOCLK => 1,
            SELREF_A::SELREF_2 => 2,
            SELREF_A::SELREF_3 => 3,
        }
    }
}
#[doc = "Reader of field `SELREF`"]
pub type SELREF_R = crate::R<u8, SELREF_A>;
impl SELREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELREF_A {
        match self.bits {
            0 => SELREF_A::XT1CLK,
            1 => SELREF_A::REFOCLK,
            2 => SELREF_A::SELREF_2,
            3 => SELREF_A::SELREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELREF_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELREF_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `SELREF_2`"]
    #[inline(always)]
    pub fn is_selref_2(&self) -> bool {
        *self == SELREF_A::SELREF_2
    }
    #[doc = "Checks if the value of the field is `SELREF_3`"]
    #[inline(always)]
    pub fn is_selref_3(&self) -> bool {
        *self == SELREF_A::SELREF_3
    }
}
#[doc = "Write proxy for field `SELREF`"]
pub struct SELREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SELREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELREF_A::XT1CLK)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELREF_A::REFOCLK)
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn selref_2(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_2)
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn selref_3(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOLP_A {
    #[doc = "0: REFO Low Power Disabled (High Power Mode)"]
    REFOLP_0,
    #[doc = "1: REFO Low Power Enabled"]
    REFOLP_1,
}
impl From<REFOLP_A> for bool {
    #[inline(always)]
    fn from(variant: REFOLP_A) -> Self {
        match variant {
            REFOLP_A::REFOLP_0 => false,
            REFOLP_A::REFOLP_1 => true,
        }
    }
}
#[doc = "Reader of field `REFOLP`"]
pub type REFOLP_R = crate::R<bool, REFOLP_A>;
impl REFOLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOLP_A {
        match self.bits {
            false => REFOLP_A::REFOLP_0,
            true => REFOLP_A::REFOLP_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOLP_0`"]
    #[inline(always)]
    pub fn is_refolp_0(&self) -> bool {
        *self == REFOLP_A::REFOLP_0
    }
    #[doc = "Checks if the value of the field is `REFOLP_1`"]
    #[inline(always)]
    pub fn is_refolp_1(&self) -> bool {
        *self == REFOLP_A::REFOLP_1
    }
}
#[doc = "Write proxy for field `REFOLP`"]
pub struct REFOLP_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFOLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "REFO Low Power Disabled (High Power Mode)"]
    #[inline(always)]
    pub fn refolp_0(self) -> &'a mut W {
        self.variant(REFOLP_A::REFOLP_0)
    }
    #[doc = "REFO Low Power Enabled"]
    #[inline(always)]
    pub fn refolp_1(self) -> &'a mut W {
        self.variant(REFOLP_A::REFOLP_1)
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
impl R {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FLLREFDIV_R {
        FLLREFDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&self) -> REFOLP_R {
        REFOLP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FLLREFDIV_W {
        FLLREFDIV_W { w: self }
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&mut self) -> SELREF_W {
        SELREF_W { w: self }
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&mut self) -> REFOLP_W {
        REFOLP_W { w: self }
    }
}
