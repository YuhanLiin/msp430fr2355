#[doc = "Register `RTCCTL` reader"]
pub struct R(crate::R<RTCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL` writer"]
pub struct W(crate::W<RTCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL_SPEC>;
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
impl From<crate::W<RTCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCIFG` reader - Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register."]
pub type RTCIFG_R = crate::BitReader<RTCIFG_A>;
#[doc = "Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCIFG_A {
    #[doc = "0: No interrupt pending"]
    RTCIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    RTCIFG_1 = 1,
}
impl From<RTCIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCIFG_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RTCIE` reader - Real-time interrupt enable"]
pub type RTCIE_R = crate::BitReader<RTCIE_A>;
#[doc = "Real-time interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCIE_A {
    #[doc = "0: Interrupt disabled"]
    RTCIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    RTCIE_1 = 1,
}
impl From<RTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCIE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RTCIE` writer - Real-time interrupt enable"]
pub type RTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL_SPEC, RTCIE_A, O>;
impl<'a, const O: u8> RTCIE_W<'a, O> {
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
}
#[doc = "Field `RTCSR` reader - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
pub type RTCSR_R = crate::BitReader<RTCSR_A>;
#[doc = "Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCSR_A {
    #[doc = "0: Write 0 has no effect"]
    RTCSR_0 = 0,
    #[doc = "1: Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    RTCSR_1 = 1,
}
impl From<RTCSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTCSR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCSR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RTCSR` writer - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
pub type RTCSR_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL_SPEC, RTCSR_A, O>;
impl<'a, const O: u8> RTCSR_W<'a, O> {
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
}
#[doc = "Field `RTCPS` reader - Real-time clock pre-divider select"]
pub type RTCPS_R = crate::FieldReader<u8, RTCPS_A>;
#[doc = "Real-time clock pre-divider select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPS_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /10"]
    _10 = 1,
    #[doc = "2: /100"]
    _100 = 2,
    #[doc = "3: /1000"]
    _1000 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /64"]
    _64 = 5,
    #[doc = "6: /256"]
    _256 = 6,
    #[doc = "7: /1024"]
    _1024 = 7,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as _
    }
}
impl RTCPS_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RTCPS` writer - Real-time clock pre-divider select"]
pub type RTCPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, RTCCTL_SPEC, u8, RTCPS_A, 3, O>;
impl<'a, const O: u8> RTCPS_W<'a, O> {
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
}
#[doc = "Field `RTCSS` reader - Real-time clock source select"]
pub type RTCSS_R = crate::FieldReader<u8, RTCSS_A>;
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSS_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: SMCLK"]
    SMCLK = 1,
    #[doc = "2: XT1CLK"]
    XT1CLK = 2,
    #[doc = "3: VLOCLK"]
    VLOCLK = 3,
}
impl From<RTCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSS_A) -> Self {
        variant as _
    }
}
impl RTCSS_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RTCSS` writer - Real-time clock source select"]
pub type RTCSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, RTCCTL_SPEC, u8, RTCSS_A, 2, O>;
impl<'a, const O: u8> RTCSS_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register."]
    #[inline(always)]
    pub fn rtcifg(&self) -> RTCIFG_R {
        RTCIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RTCSR_R {
        RTCSR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&self) -> RTCSS_R {
        RTCSS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W<1> {
        RTCIE_W::new(self)
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RTCSR_W<6> {
        RTCSR_W::new(self)
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W<8> {
        RTCPS_W::new(self)
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RTCSS_W<12> {
        RTCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCTL0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl](index.html) module"]
pub struct RTCCTL_SPEC;
impl crate::RegisterSpec for RTCCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl::R](R) reader structure"]
impl crate::Readable for RTCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl::W](W) writer structure"]
impl crate::Writable for RTCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RTCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
