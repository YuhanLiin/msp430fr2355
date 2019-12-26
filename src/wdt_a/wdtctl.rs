#[doc = "Reader of register WDTCTL"]
pub type R = crate::R<u16, super::WDTCTL>;
#[doc = "Writer for register WDTCTL"]
pub type W = crate::W<u16, super::WDTCTL>;
#[doc = "Register WDTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog timer interval select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTIS_A {
    #[doc = "0: Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    _2G = 0,
    #[doc = "1: Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    _128M = 1,
    #[doc = "2: Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    _8192K = 2,
    #[doc = "3: Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    _512K = 3,
    #[doc = "4: Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    _32K = 4,
    #[doc = "5: Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    _8192 = 5,
    #[doc = "6: Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    _512 = 6,
    #[doc = "7: Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    _64 = 7,
}
impl From<WDTIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDTIS`"]
pub type WDTIS_R = crate::R<u8, WDTIS_A>;
impl WDTIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIS_A {
        match self.bits {
            0 => WDTIS_A::_2G,
            1 => WDTIS_A::_128M,
            2 => WDTIS_A::_8192K,
            3 => WDTIS_A::_512K,
            4 => WDTIS_A::_32K,
            5 => WDTIS_A::_8192,
            6 => WDTIS_A::_512,
            7 => WDTIS_A::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2G`"]
    #[inline(always)]
    pub fn is_2g(&self) -> bool {
        *self == WDTIS_A::_2G
    }
    #[doc = "Checks if the value of the field is `_128M`"]
    #[inline(always)]
    pub fn is_128m(&self) -> bool {
        *self == WDTIS_A::_128M
    }
    #[doc = "Checks if the value of the field is `_8192K`"]
    #[inline(always)]
    pub fn is_8192k(&self) -> bool {
        *self == WDTIS_A::_8192K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == WDTIS_A::_512K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == WDTIS_A::_32K
    }
    #[doc = "Checks if the value of the field is `_8192`"]
    #[inline(always)]
    pub fn is_8192(&self) -> bool {
        *self == WDTIS_A::_8192
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == WDTIS_A::_512
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == WDTIS_A::_64
    }
}
#[doc = "Write proxy for field `WDTIS`"]
pub struct WDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _2g(self) -> &'a mut W {
        self.variant(WDTIS_A::_2G)
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _128m(self) -> &'a mut W {
        self.variant(WDTIS_A::_128M)
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192k(self) -> &'a mut W {
        self.variant(WDTIS_A::_8192K)
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut W {
        self.variant(WDTIS_A::_512K)
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(WDTIS_A::_32K)
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192(self) -> &'a mut W {
        self.variant(WDTIS_A::_8192)
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(WDTIS_A::_512)
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(WDTIS_A::_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Watchdog timer counter clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCNTCL_A {
    #[doc = "0: No action"]
    WDTCNTCL_0 = 0,
    #[doc = "1: WDTCNT = 0000h"]
    WDTCNTCL_1 = 1,
}
impl From<WDTCNTCL_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCNTCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTCNTCL`"]
pub type WDTCNTCL_R = crate::R<bool, WDTCNTCL_A>;
impl WDTCNTCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTCNTCL_A {
        match self.bits {
            false => WDTCNTCL_A::WDTCNTCL_0,
            true => WDTCNTCL_A::WDTCNTCL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTCNTCL_0`"]
    #[inline(always)]
    pub fn is_wdtcntcl_0(&self) -> bool {
        *self == WDTCNTCL_A::WDTCNTCL_0
    }
    #[doc = "Checks if the value of the field is `WDTCNTCL_1`"]
    #[inline(always)]
    pub fn is_wdtcntcl_1(&self) -> bool {
        *self == WDTCNTCL_A::WDTCNTCL_1
    }
}
#[doc = "Write proxy for field `WDTCNTCL`"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCNTCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn wdtcntcl_0(self) -> &'a mut W {
        self.variant(WDTCNTCL_A::WDTCNTCL_0)
    }
    #[doc = "WDTCNT = 0000h"]
    #[inline(always)]
    pub fn wdtcntcl_1(self) -> &'a mut W {
        self.variant(WDTCNTCL_A::WDTCNTCL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Watchdog timer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTTMSEL_A {
    #[doc = "0: Watchdog mode"]
    WDTTMSEL_0 = 0,
    #[doc = "1: Interval timer mode"]
    WDTTMSEL_1 = 1,
}
impl From<WDTTMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDTTMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTTMSEL`"]
pub type WDTTMSEL_R = crate::R<bool, WDTTMSEL_A>;
impl WDTTMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTTMSEL_A {
        match self.bits {
            false => WDTTMSEL_A::WDTTMSEL_0,
            true => WDTTMSEL_A::WDTTMSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTTMSEL_0`"]
    #[inline(always)]
    pub fn is_wdttmsel_0(&self) -> bool {
        *self == WDTTMSEL_A::WDTTMSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTTMSEL_1`"]
    #[inline(always)]
    pub fn is_wdttmsel_1(&self) -> bool {
        *self == WDTTMSEL_A::WDTTMSEL_1
    }
}
#[doc = "Write proxy for field `WDTTMSEL`"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTTMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog mode"]
    #[inline(always)]
    pub fn wdttmsel_0(self) -> &'a mut W {
        self.variant(WDTTMSEL_A::WDTTMSEL_0)
    }
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn wdttmsel_1(self) -> &'a mut W {
        self.variant(WDTTMSEL_A::WDTTMSEL_1)
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
#[doc = "Watchdog timer clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: SMCLK"]
    SMCLK = 0,
    #[doc = "1: ACLK"]
    ACLK = 1,
    #[doc = "2: VLOCLK"]
    VLOCLK = 2,
    #[doc = "3: BCLK"]
    BCLK = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDTSSEL`"]
pub type WDTSSEL_R = crate::R<u8, WDTSSEL_A>;
impl WDTSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::SMCLK,
            1 => WDTSSEL_A::ACLK,
            2 => WDTSSEL_A::VLOCLK,
            3 => WDTSSEL_A::BCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == WDTSSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == WDTSSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == WDTSSEL_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `BCLK`"]
    #[inline(always)]
    pub fn is_bclk(&self) -> bool {
        *self == WDTSSEL_A::BCLK
    }
}
#[doc = "Write proxy for field `WDTSSEL`"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::SMCLK)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::ACLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::VLOCLK)
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn bclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::BCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "Watchdog timer hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTHOLD_A {
    #[doc = "0: Watchdog timer is not stopped"]
    UNHOLD = 0,
    #[doc = "1: Watchdog timer is stopped"]
    HOLD = 1,
}
impl From<WDTHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTHOLD`"]
pub type WDTHOLD_R = crate::R<bool, WDTHOLD_A>;
impl WDTHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTHOLD_A {
        match self.bits {
            false => WDTHOLD_A::UNHOLD,
            true => WDTHOLD_A::HOLD,
        }
    }
    #[doc = "Checks if the value of the field is `UNHOLD`"]
    #[inline(always)]
    pub fn is_unhold(&self) -> bool {
        *self == WDTHOLD_A::UNHOLD
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == WDTHOLD_A::HOLD
    }
}
#[doc = "Write proxy for field `WDTHOLD`"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn unhold(self) -> &'a mut W {
        self.variant(WDTHOLD_A::UNHOLD)
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(WDTHOLD_A::HOLD)
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
#[doc = "Reader of field `WDTPW`"]
pub type WDTPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDTPW`"]
pub struct WDTPW_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W {
        WDTIS_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W {
        WDTPW_W { w: self }
    }
}
