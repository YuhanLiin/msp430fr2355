#[doc = "Register `CSCTL4` reader"]
pub struct R(crate::R<CSCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL4` writer"]
pub struct W(crate::W<CSCTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL4_SPEC>;
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
impl From<crate::W<CSCTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELMS` reader - Selects the MCLK and SMCLK source"]
pub type SELMS_R = crate::FieldReader<u8, SELMS_A>;
#[doc = "Selects the MCLK and SMCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELMS_A {
    #[doc = "0: DCOCLKDIV"]
    DCOCLKDIV = 0,
    #[doc = "1: REFOCLK"]
    REFOCLK = 1,
    #[doc = "2: XT1CLK"]
    XT1CLK = 2,
    #[doc = "3: VLOCLK"]
    VLOCLK = 3,
    #[doc = "4: Reserved for future use"]
    SELMS_4 = 4,
    #[doc = "5: Reserved for future use"]
    SELMS_5 = 5,
    #[doc = "6: Reserved for future use"]
    SELMS_6 = 6,
    #[doc = "7: Reserved for future use"]
    SELMS_7 = 7,
}
impl From<SELMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMS_A) -> Self {
        variant as _
    }
}
impl SELMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMS_A {
        match self.bits {
            0 => SELMS_A::DCOCLKDIV,
            1 => SELMS_A::REFOCLK,
            2 => SELMS_A::XT1CLK,
            3 => SELMS_A::VLOCLK,
            4 => SELMS_A::SELMS_4,
            5 => SELMS_A::SELMS_5,
            6 => SELMS_A::SELMS_6,
            7 => SELMS_A::SELMS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOCLKDIV`"]
    #[inline(always)]
    pub fn is_dcoclkdiv(&self) -> bool {
        *self == SELMS_A::DCOCLKDIV
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELMS_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELMS_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELMS_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `SELMS_4`"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == SELMS_A::SELMS_4
    }
    #[doc = "Checks if the value of the field is `SELMS_5`"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == SELMS_A::SELMS_5
    }
    #[doc = "Checks if the value of the field is `SELMS_6`"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == SELMS_A::SELMS_6
    }
    #[doc = "Checks if the value of the field is `SELMS_7`"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == SELMS_A::SELMS_7
    }
}
#[doc = "Field `SELMS` writer - Selects the MCLK and SMCLK source"]
pub type SELMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL4_SPEC, u8, SELMS_A, 3, O>;
impl<'a, const O: u8> SELMS_W<'a, O> {
    #[doc = "DCOCLKDIV"]
    #[inline(always)]
    pub fn dcoclkdiv(self) -> &'a mut W {
        self.variant(SELMS_A::DCOCLKDIV)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELMS_A::REFOCLK)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELMS_A::XT1CLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELMS_A::VLOCLK)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_4)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_5)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_6)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut W {
        self.variant(SELMS_A::SELMS_7)
    }
}
#[doc = "Field `SELA` reader - Selects the ACLK source"]
pub type SELA_R = crate::FieldReader<u8, SELA_A>;
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELA_A {
    #[doc = "0: XT1CLK with divider (must be no more than 40 kHz)"]
    XT1CLK = 0,
    #[doc = "1: REFO (internal 32-kHz clock source)"]
    REFOCLK = 1,
    #[doc = "2: VLO (internal 10-kHz clock source)"]
    VLOCLK = 2,
}
impl From<SELA_A> for u8 {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        variant as _
    }
}
impl SELA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELA_A {
        match self.bits {
            0 => SELA_A::XT1CLK,
            1 => SELA_A::REFOCLK,
            2 => SELA_A::VLOCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELA_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELA_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELA_A::VLOCLK
    }
}
#[doc = "Field `SELA` writer - Selects the ACLK source"]
pub type SELA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL4_SPEC, u8, SELA_A, 2, O>;
impl<'a, const O: u8> SELA_W<'a, O> {
    #[doc = "XT1CLK with divider (must be no more than 40 kHz)"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELA_A::XT1CLK)
    }
    #[doc = "REFO (internal 32-kHz clock source)"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELA_A::REFOCLK)
    }
    #[doc = "VLO (internal 10-kHz clock source)"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELA_A::VLOCLK)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&self) -> SELMS_R {
        SELMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&mut self) -> SELMS_W<0> {
        SELMS_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W<8> {
        SELA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl4](index.html) module"]
pub struct CSCTL4_SPEC;
impl crate::RegisterSpec for CSCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl4::R](R) reader structure"]
impl crate::Readable for CSCTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl4::W](W) writer structure"]
impl crate::Writable for CSCTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for CSCTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
