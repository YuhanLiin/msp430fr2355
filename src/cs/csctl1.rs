#[doc = "Register `CSCTL1` reader"]
pub struct R(crate::R<CSCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL1` writer"]
pub struct W(crate::W<CSCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL1_SPEC>;
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
impl From<crate::W<CSCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMOD` reader - Modulation. This bit enables/disables the modulation."]
pub type DISMOD_R = crate::BitReader<DISMOD_A>;
#[doc = "Modulation. This bit enables/disables the modulation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISMOD_A {
    #[doc = "0: Modulation enabled"]
    DISMOD_0 = 0,
    #[doc = "1: Modulation disabled"]
    DISMOD_1 = 1,
}
impl From<DISMOD_A> for bool {
    #[inline(always)]
    fn from(variant: DISMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl DISMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISMOD_A {
        match self.bits {
            false => DISMOD_A::DISMOD_0,
            true => DISMOD_A::DISMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISMOD_0`"]
    #[inline(always)]
    pub fn is_dismod_0(&self) -> bool {
        *self == DISMOD_A::DISMOD_0
    }
    #[doc = "Checks if the value of the field is `DISMOD_1`"]
    #[inline(always)]
    pub fn is_dismod_1(&self) -> bool {
        *self == DISMOD_A::DISMOD_1
    }
}
#[doc = "Field `DISMOD` writer - Modulation. This bit enables/disables the modulation."]
pub type DISMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL1_SPEC, DISMOD_A, O>;
impl<'a, const O: u8> DISMOD_W<'a, O> {
    #[doc = "Modulation enabled"]
    #[inline(always)]
    pub fn dismod_0(self) -> &'a mut W {
        self.variant(DISMOD_A::DISMOD_0)
    }
    #[doc = "Modulation disabled"]
    #[inline(always)]
    pub fn dismod_1(self) -> &'a mut W {
        self.variant(DISMOD_A::DISMOD_1)
    }
}
#[doc = "Field `DCORSEL` reader - DCO Range Select"]
pub type DCORSEL_R = crate::FieldReader<u8, DCORSEL_A>;
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: 1 MHz"]
    DCORSEL_0 = 0,
    #[doc = "1: 2 MHz"]
    DCORSEL_1 = 1,
    #[doc = "2: 4 MHz"]
    DCORSEL_2 = 2,
    #[doc = "3: 8 MHz"]
    DCORSEL_3 = 3,
    #[doc = "4: 12 MHz"]
    DCORSEL_4 = 4,
    #[doc = "5: 16 MHz"]
    DCORSEL_5 = 5,
    #[doc = "6: 20 MHz(Only avaliable in 24MHz clock system)"]
    DCORSEL_6 = 6,
    #[doc = "7: 24 MHz(Only avaliable in 24MHz clock system)"]
    DCORSEL_7 = 7,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
impl DCORSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORSEL_A {
        match self.bits {
            0 => DCORSEL_A::DCORSEL_0,
            1 => DCORSEL_A::DCORSEL_1,
            2 => DCORSEL_A::DCORSEL_2,
            3 => DCORSEL_A::DCORSEL_3,
            4 => DCORSEL_A::DCORSEL_4,
            5 => DCORSEL_A::DCORSEL_5,
            6 => DCORSEL_A::DCORSEL_6,
            7 => DCORSEL_A::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
    #[doc = "Checks if the value of the field is `DCORSEL_6`"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_6
    }
    #[doc = "Checks if the value of the field is `DCORSEL_7`"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_7
    }
}
#[doc = "Field `DCORSEL` writer - DCO Range Select"]
pub type DCORSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL1_SPEC, u8, DCORSEL_A, 3, O>;
impl<'a, const O: u8> DCORSEL_W<'a, O> {
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = "20 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_6)
    }
    #[doc = "24 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_7)
    }
}
#[doc = "Field `DCOFTRIM` reader - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DCOFTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCOFTRIM` writer - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DCOFTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DCOFTRIMEN` reader - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
pub type DCOFTRIMEN_R = crate::BitReader<DCOFTRIMEN_A>;
#[doc = "DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOFTRIMEN_A {
    #[doc = "0: Disable frequency trim"]
    DCOFTRIMEN_0 = 0,
    #[doc = "1: Enable frequency trim"]
    DCOFTRIMEN_1 = 1,
}
impl From<DCOFTRIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCOFTRIMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOFTRIMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFTRIMEN_A {
        match self.bits {
            false => DCOFTRIMEN_A::DCOFTRIMEN_0,
            true => DCOFTRIMEN_A::DCOFTRIMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOFTRIMEN_0`"]
    #[inline(always)]
    pub fn is_dcoftrimen_0(&self) -> bool {
        *self == DCOFTRIMEN_A::DCOFTRIMEN_0
    }
    #[doc = "Checks if the value of the field is `DCOFTRIMEN_1`"]
    #[inline(always)]
    pub fn is_dcoftrimen_1(&self) -> bool {
        *self == DCOFTRIMEN_A::DCOFTRIMEN_1
    }
}
#[doc = "Field `DCOFTRIMEN` writer - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
pub type DCOFTRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL1_SPEC, DCOFTRIMEN_A, O>;
impl<'a, const O: u8> DCOFTRIMEN_W<'a, O> {
    #[doc = "Disable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_0(self) -> &'a mut W {
        self.variant(DCOFTRIMEN_A::DCOFTRIMEN_0)
    }
    #[doc = "Enable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_1(self) -> &'a mut W {
        self.variant(DCOFTRIMEN_A::DCOFTRIMEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DCOFTRIM_R {
        DCOFTRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DCOFTRIMEN_R {
        DCOFTRIMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&mut self) -> DISMOD_W<0> {
        DISMOD_W::new(self)
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W<1> {
        DCORSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DCOFTRIM_W<4> {
        DCOFTRIM_W::new(self)
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DCOFTRIMEN_W<7> {
        DCOFTRIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl1](index.html) module"]
pub struct CSCTL1_SPEC;
impl crate::RegisterSpec for CSCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl1::R](R) reader structure"]
impl crate::Readable for CSCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl1::W](W) writer structure"]
impl crate::Writable for CSCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL1 to value 0"]
impl crate::Resettable for CSCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
