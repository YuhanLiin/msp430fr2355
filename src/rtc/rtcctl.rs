#[doc = "Reader of register RTCCTL"]
pub type R = crate::R<u16, super::RTCCTL>;
#[doc = "Writer for register RTCCTL"]
pub type W = crate::W<u16, super::RTCCTL>;
#[doc = "Register RTCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCIFG_A {
    #[doc = "0: No interrupt pending"]
    RTCIFG_0,
    #[doc = "1: Interrupt pending"]
    RTCIFG_1,
}
impl From<RTCIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCIFG_A) -> Self {
        match variant {
            RTCIFG_A::RTCIFG_0 => false,
            RTCIFG_A::RTCIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `RTCIFG`"]
pub type RTCIFG_R = crate::R<bool, RTCIFG_A>;
impl RTCIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCIFG_A {
        match self.bits {
            false => RTCIFG_A::RTCIFG_0,
            true => RTCIFG_A::RTCIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCIFG_0`"]
    #[inline(always)]
    pub fn is_rtcifg_0(&self) -> bool {
        *self == RTCIFG_A::RTCIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCIFG_1`"]
    #[inline(always)]
    pub fn is_rtcifg_1(&self) -> bool {
        *self == RTCIFG_A::RTCIFG_1
    }
}
#[doc = "Real-time interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCIE_A {
    #[doc = "0: Interrupt disabled"]
    RTCIE_0,
    #[doc = "1: Interrupt enabled"]
    RTCIE_1,
}
impl From<RTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCIE_A) -> Self {
        match variant {
            RTCIE_A::RTCIE_0 => false,
            RTCIE_A::RTCIE_1 => true,
        }
    }
}
#[doc = "Reader of field `RTCIE`"]
pub type RTCIE_R = crate::R<bool, RTCIE_A>;
impl RTCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCIE_A {
        match self.bits {
            false => RTCIE_A::RTCIE_0,
            true => RTCIE_A::RTCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCIE_0`"]
    #[inline(always)]
    pub fn is_rtcie_0(&self) -> bool {
        *self == RTCIE_A::RTCIE_0
    }
    #[doc = "Checks if the value of the field is `RTCIE_1`"]
    #[inline(always)]
    pub fn is_rtcie_1(&self) -> bool {
        *self == RTCIE_A::RTCIE_1
    }
}
#[doc = "Write proxy for field `RTCIE`"]
pub struct RTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn rtcie_0(self) -> &'a mut W {
        self.variant(RTCIE_A::RTCIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn rtcie_1(self) -> &'a mut W {
        self.variant(RTCIE_A::RTCIE_1)
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
#[doc = "Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSR_A {
    #[doc = "0: Write 0 has no effect"]
    RTCSR_0,
    #[doc = "1: Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    RTCSR_1,
}
impl From<RTCSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTCSR_A) -> Self {
        match variant {
            RTCSR_A::RTCSR_0 => false,
            RTCSR_A::RTCSR_1 => true,
        }
    }
}
#[doc = "Reader of field `RTCSR`"]
pub type RTCSR_R = crate::R<bool, RTCSR_A>;
impl RTCSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSR_A {
        match self.bits {
            false => RTCSR_A::RTCSR_0,
            true => RTCSR_A::RTCSR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCSR_0`"]
    #[inline(always)]
    pub fn is_rtcsr_0(&self) -> bool {
        *self == RTCSR_A::RTCSR_0
    }
    #[doc = "Checks if the value of the field is `RTCSR_1`"]
    #[inline(always)]
    pub fn is_rtcsr_1(&self) -> bool {
        *self == RTCSR_A::RTCSR_1
    }
}
#[doc = "Write proxy for field `RTCSR`"]
pub struct RTCSR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr_0(self) -> &'a mut W {
        self.variant(RTCSR_A::RTCSR_0)
    }
    #[doc = "Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    #[inline(always)]
    pub fn rtcsr_1(self) -> &'a mut W {
        self.variant(RTCSR_A::RTCSR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Real-time clock pre-divider select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCPS_A {
    #[doc = "0: /1"]
    _1,
    #[doc = "1: /10"]
    _10,
    #[doc = "2: /100"]
    _100,
    #[doc = "3: /1000"]
    _1000,
    #[doc = "4: /16"]
    _16,
    #[doc = "5: /64"]
    _64,
    #[doc = "6: /256"]
    _256,
    #[doc = "7: /1024"]
    _1024,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        match variant {
            RTCPS_A::_1 => 0,
            RTCPS_A::_10 => 1,
            RTCPS_A::_100 => 2,
            RTCPS_A::_1000 => 3,
            RTCPS_A::_16 => 4,
            RTCPS_A::_64 => 5,
            RTCPS_A::_256 => 6,
            RTCPS_A::_1024 => 7,
        }
    }
}
#[doc = "Reader of field `RTCPS`"]
pub type RTCPS_R = crate::R<u8, RTCPS_A>;
impl RTCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            0 => RTCPS_A::_1,
            1 => RTCPS_A::_10,
            2 => RTCPS_A::_100,
            3 => RTCPS_A::_1000,
            4 => RTCPS_A::_16,
            5 => RTCPS_A::_64,
            6 => RTCPS_A::_256,
            7 => RTCPS_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCPS_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTCPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RTCPS_A::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RTCPS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == RTCPS_A::_16
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RTCPS_A::_64
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RTCPS_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == RTCPS_A::_1024
    }
}
#[doc = "Write proxy for field `RTCPS`"]
pub struct RTCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCPS_A::_1)
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTCPS_A::_10)
    }
    #[doc = "/100"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RTCPS_A::_100)
    }
    #[doc = "/1000"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(RTCPS_A::_1000)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(RTCPS_A::_16)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RTCPS_A::_64)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RTCPS_A::_256)
    }
    #[doc = "/1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(RTCPS_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSS_A {
    #[doc = "0: Disabled"]
    DISABLED,
    #[doc = "1: SMCLK"]
    SMCLK,
    #[doc = "2: XT1CLK"]
    XT1CLK,
    #[doc = "3: VLOCLK"]
    VLOCLK,
}
impl From<RTCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSS_A) -> Self {
        match variant {
            RTCSS_A::DISABLED => 0,
            RTCSS_A::SMCLK => 1,
            RTCSS_A::XT1CLK => 2,
            RTCSS_A::VLOCLK => 3,
        }
    }
}
#[doc = "Reader of field `RTCSS`"]
pub type RTCSS_R = crate::R<u8, RTCSS_A>;
impl RTCSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSS_A {
        match self.bits {
            0 => RTCSS_A::DISABLED,
            1 => RTCSS_A::SMCLK,
            2 => RTCSS_A::XT1CLK,
            3 => RTCSS_A::VLOCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCSS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == RTCSS_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == RTCSS_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == RTCSS_A::VLOCLK
    }
}
#[doc = "Write proxy for field `RTCSS`"]
pub struct RTCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCSS_A::DISABLED)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(RTCSS_A::SMCLK)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(RTCSS_A::XT1CLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(RTCSS_A::VLOCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register."]
    #[inline(always)]
    pub fn rtcifg(&self) -> RTCIFG_R {
        RTCIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RTCSR_R {
        RTCSR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&self) -> RTCSS_R {
        RTCSS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W {
        RTCIE_W { w: self }
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RTCSR_W {
        RTCSR_W { w: self }
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W {
        RTCPS_W { w: self }
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RTCSS_W {
        RTCSS_W { w: self }
    }
}
