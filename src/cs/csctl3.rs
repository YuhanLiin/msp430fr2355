#[doc = "Register `CSCTL3` reader"]
pub struct R(crate::R<CSCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL3` writer"]
pub struct W(crate::W<CSCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL3_SPEC>;
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
impl From<crate::W<CSCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLREFDIV` reader - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
pub type FLLREFDIV_R = crate::FieldReader<u8, FLLREFDIV_A>;
#[doc = "FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLREFDIV_A {
    #[doc = "0: fFLLREFCLK / 1"]
    _1 = 0,
    #[doc = "1: fFLLREFCLK / 32"]
    _32 = 1,
    #[doc = "2: fFLLREFCLK / 64"]
    _64 = 2,
    #[doc = "3: fFLLREFCLK / 128"]
    _128 = 3,
    #[doc = "4: fFLLREFCLK / 256"]
    _256 = 4,
    #[doc = "5: fFLLREFCLK / 512"]
    _512 = 5,
    #[doc = "6: fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    FLLREFDIV_6 = 6,
    #[doc = "7: fFLLREFCLK / 768(only available in 24MHz clock system)"]
    FLLREFDIV_7 = 7,
}
impl From<FLLREFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLREFDIV_A) -> Self {
        variant as _
    }
}
impl FLLREFDIV_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLLREFDIV` writer - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
pub type FLLREFDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL3_SPEC, u8, FLLREFDIV_A, 3, O>;
impl<'a, const O: u8> FLLREFDIV_W<'a, O> {
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
}
#[doc = "Field `SELREF` reader - FLL reference select. These bits select the FLL reference clock source."]
pub type SELREF_R = crate::FieldReader<u8, SELREF_A>;
#[doc = "FLL reference select. These bits select the FLL reference clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELREF_A {
    #[doc = "0: XT1CLK"]
    XT1CLK = 0,
    #[doc = "1: REFOCLK"]
    REFOCLK = 1,
    #[doc = "2: served for future use"]
    SELREF_2 = 2,
    #[doc = "3: served for future use"]
    SELREF_3 = 3,
}
impl From<SELREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELREF_A) -> Self {
        variant as _
    }
}
impl SELREF_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SELREF` writer - FLL reference select. These bits select the FLL reference clock source."]
pub type SELREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL3_SPEC, u8, SELREF_A, 2, O>;
impl<'a, const O: u8> SELREF_W<'a, O> {
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
}
#[doc = "Field `REFOLP` reader - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
pub type REFOLP_R = crate::BitReader<REFOLP_A>;
#[doc = "REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFOLP_A {
    #[doc = "0: REFO Low Power Disabled (High Power Mode)"]
    REFOLP_0 = 0,
    #[doc = "1: REFO Low Power Enabled"]
    REFOLP_1 = 1,
}
impl From<REFOLP_A> for bool {
    #[inline(always)]
    fn from(variant: REFOLP_A) -> Self {
        variant as u8 != 0
    }
}
impl REFOLP_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `REFOLP` writer - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
pub type REFOLP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL3_SPEC, REFOLP_A, O>;
impl<'a, const O: u8> REFOLP_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FLLREFDIV_R {
        FLLREFDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&self) -> REFOLP_R {
        REFOLP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FLLREFDIV_W<0> {
        FLLREFDIV_W::new(self)
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&mut self) -> SELREF_W<4> {
        SELREF_W::new(self)
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&mut self) -> REFOLP_W<7> {
        REFOLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl3](index.html) module"]
pub struct CSCTL3_SPEC;
impl crate::RegisterSpec for CSCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl3::R](R) reader structure"]
impl crate::Readable for CSCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl3::W](W) writer structure"]
impl crate::Writable for CSCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL3 to value 0"]
impl crate::Resettable for CSCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
