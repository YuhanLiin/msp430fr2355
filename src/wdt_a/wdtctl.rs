#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
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
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIS` reader - Watchdog timer interval select"]
pub type WDTIS_R = crate::FieldReader<u8, WDTIS_A>;
#[doc = "Watchdog timer interval select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTIS_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WDTIS` writer - Watchdog timer interval select"]
pub type WDTIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WDTCTL_SPEC, u8, WDTIS_A, 3, O>;
impl<'a, const O: u8> WDTIS_W<'a, O> {
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
}
#[doc = "Field `WDTCNTCL` reader - Watchdog timer counter clear"]
pub type WDTCNTCL_R = crate::BitReader<WDTCNTCL_A>;
#[doc = "Watchdog timer counter clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTCNTCL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WDTCNTCL` writer - Watchdog timer counter clear"]
pub type WDTCNTCL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, WDTCNTCL_A, O>;
impl<'a, const O: u8> WDTCNTCL_W<'a, O> {
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
}
#[doc = "Field `WDTTMSEL` reader - Watchdog timer mode select"]
pub type WDTTMSEL_R = crate::BitReader<WDTTMSEL_A>;
#[doc = "Watchdog timer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTTMSEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WDTTMSEL` writer - Watchdog timer mode select"]
pub type WDTTMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, WDTTMSEL_A, O>;
impl<'a, const O: u8> WDTTMSEL_W<'a, O> {
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
}
#[doc = "Field `WDTSSEL` reader - Watchdog timer clock source select"]
pub type WDTSSEL_R = crate::FieldReader<u8, WDTSSEL_A>;
#[doc = "Watchdog timer clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTSSEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WDTSSEL` writer - Watchdog timer clock source select"]
pub type WDTSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, WDTCTL_SPEC, u8, WDTSSEL_A, 2, O>;
impl<'a, const O: u8> WDTSSEL_W<'a, O> {
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
}
#[doc = "Field `WDTHOLD` reader - Watchdog timer hold"]
pub type WDTHOLD_R = crate::BitReader<WDTHOLD_A>;
#[doc = "Watchdog timer hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDTHOLD_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `WDTHOLD` writer - Watchdog timer hold"]
pub type WDTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, WDTHOLD_A, O>;
impl<'a, const O: u8> WDTHOLD_W<'a, O> {
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
}
#[doc = "Field `WDTPW` reader - Watchdog timer password"]
pub type WDTPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTPW` writer - Watchdog timer password"]
pub type WDTPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WDTCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn wdtis(&mut self) -> WDTIS_W<0> {
        WDTIS_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<3> {
        WDTCNTCL_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<4> {
        WDTTMSEL_W::new(self)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<5> {
        WDTSSEL_W::new(self)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W<7> {
        WDTHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W<8> {
        WDTPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
