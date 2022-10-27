#[doc = "Register `CPCTL0` reader"]
pub struct R(crate::R<CPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPCTL0` writer"]
pub struct W(crate::W<CPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPCTL0_SPEC>;
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
impl From<crate::W<CPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPPSEL` reader - Channel input selected for the V+ terminal"]
pub type CPPSEL_R = crate::FieldReader<u8, CPPSEL_A>;
#[doc = "Channel input selected for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPPSEL_A {
    #[doc = "0: select external input source"]
    CPPSEL_0 = 0,
    #[doc = "1: select external input source"]
    CPPSEL_1 = 1,
    #[doc = "2: select external input source"]
    CPPSEL_2 = 2,
    #[doc = "3: select external input source"]
    CPPSEL_3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    CPPSEL_4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    CPPSEL_5 = 5,
    #[doc = "6: 6-bit DAC"]
    CPPSEL_6 = 6,
    #[doc = "7: Reserved"]
    CPPSEL_7 = 7,
}
impl From<CPPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPPSEL_A) -> Self {
        variant as _
    }
}
impl CPPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPSEL_A {
        match self.bits {
            0 => CPPSEL_A::CPPSEL_0,
            1 => CPPSEL_A::CPPSEL_1,
            2 => CPPSEL_A::CPPSEL_2,
            3 => CPPSEL_A::CPPSEL_3,
            4 => CPPSEL_A::CPPSEL_4,
            5 => CPPSEL_A::CPPSEL_5,
            6 => CPPSEL_A::CPPSEL_6,
            7 => CPPSEL_A::CPPSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPPSEL_0`"]
    #[inline(always)]
    pub fn is_cppsel_0(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_0
    }
    #[doc = "Checks if the value of the field is `CPPSEL_1`"]
    #[inline(always)]
    pub fn is_cppsel_1(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_1
    }
    #[doc = "Checks if the value of the field is `CPPSEL_2`"]
    #[inline(always)]
    pub fn is_cppsel_2(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_2
    }
    #[doc = "Checks if the value of the field is `CPPSEL_3`"]
    #[inline(always)]
    pub fn is_cppsel_3(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_3
    }
    #[doc = "Checks if the value of the field is `CPPSEL_4`"]
    #[inline(always)]
    pub fn is_cppsel_4(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_4
    }
    #[doc = "Checks if the value of the field is `CPPSEL_5`"]
    #[inline(always)]
    pub fn is_cppsel_5(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_5
    }
    #[doc = "Checks if the value of the field is `CPPSEL_6`"]
    #[inline(always)]
    pub fn is_cppsel_6(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_6
    }
    #[doc = "Checks if the value of the field is `CPPSEL_7`"]
    #[inline(always)]
    pub fn is_cppsel_7(&self) -> bool {
        *self == CPPSEL_A::CPPSEL_7
    }
}
#[doc = "Field `CPPSEL` writer - Channel input selected for the V+ terminal"]
pub type CPPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CPCTL0_SPEC, u8, CPPSEL_A, 3, O>;
impl<'a, const O: u8> CPPSEL_W<'a, O> {
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_0(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_1(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_2(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_3(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_4(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_5(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cppsel_6(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cppsel_7(self) -> &'a mut W {
        self.variant(CPPSEL_A::CPPSEL_7)
    }
}
#[doc = "Field `CPPEN` reader - Channel input enable for the V+ terminal"]
pub type CPPEN_R = crate::BitReader<CPPEN_A>;
#[doc = "Channel input enable for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPEN_A {
    #[doc = "0: Selected analog input channel for V+ terminal is disabled."]
    CPPEN_0 = 0,
    #[doc = "1: Selected analog input channel for V+ terminal is enabled."]
    CPPEN_1 = 1,
}
impl From<CPPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPEN_A {
        match self.bits {
            false => CPPEN_A::CPPEN_0,
            true => CPPEN_A::CPPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPPEN_0`"]
    #[inline(always)]
    pub fn is_cppen_0(&self) -> bool {
        *self == CPPEN_A::CPPEN_0
    }
    #[doc = "Checks if the value of the field is `CPPEN_1`"]
    #[inline(always)]
    pub fn is_cppen_1(&self) -> bool {
        *self == CPPEN_A::CPPEN_1
    }
}
#[doc = "Field `CPPEN` writer - Channel input enable for the V+ terminal"]
pub type CPPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CPCTL0_SPEC, CPPEN_A, O>;
impl<'a, const O: u8> CPPEN_W<'a, O> {
    #[doc = "Selected analog input channel for V+ terminal is disabled."]
    #[inline(always)]
    pub fn cppen_0(self) -> &'a mut W {
        self.variant(CPPEN_A::CPPEN_0)
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled."]
    #[inline(always)]
    pub fn cppen_1(self) -> &'a mut W {
        self.variant(CPPEN_A::CPPEN_1)
    }
}
#[doc = "Field `CPNSEL` reader - Channel input selected for the - terminal"]
pub type CPNSEL_R = crate::FieldReader<u8, CPNSEL_A>;
#[doc = "Channel input selected for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPNSEL_A {
    #[doc = "0: select external input source"]
    CPNSEL_0 = 0,
    #[doc = "1: select external input source"]
    CPNSEL_1 = 1,
    #[doc = "2: select external input source"]
    CPNSEL_2 = 2,
    #[doc = "3: select external input source"]
    CPNSEL_3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    CPNSEL_4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    CPNSEL_5 = 5,
    #[doc = "6: 6-bit DAC"]
    CPNSEL_6 = 6,
    #[doc = "7: Reserved"]
    CPNSEL_7 = 7,
}
impl From<CPNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CPNSEL_A) -> Self {
        variant as _
    }
}
impl CPNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPNSEL_A {
        match self.bits {
            0 => CPNSEL_A::CPNSEL_0,
            1 => CPNSEL_A::CPNSEL_1,
            2 => CPNSEL_A::CPNSEL_2,
            3 => CPNSEL_A::CPNSEL_3,
            4 => CPNSEL_A::CPNSEL_4,
            5 => CPNSEL_A::CPNSEL_5,
            6 => CPNSEL_A::CPNSEL_6,
            7 => CPNSEL_A::CPNSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPNSEL_0`"]
    #[inline(always)]
    pub fn is_cpnsel_0(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_0
    }
    #[doc = "Checks if the value of the field is `CPNSEL_1`"]
    #[inline(always)]
    pub fn is_cpnsel_1(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_1
    }
    #[doc = "Checks if the value of the field is `CPNSEL_2`"]
    #[inline(always)]
    pub fn is_cpnsel_2(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_2
    }
    #[doc = "Checks if the value of the field is `CPNSEL_3`"]
    #[inline(always)]
    pub fn is_cpnsel_3(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_3
    }
    #[doc = "Checks if the value of the field is `CPNSEL_4`"]
    #[inline(always)]
    pub fn is_cpnsel_4(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_4
    }
    #[doc = "Checks if the value of the field is `CPNSEL_5`"]
    #[inline(always)]
    pub fn is_cpnsel_5(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_5
    }
    #[doc = "Checks if the value of the field is `CPNSEL_6`"]
    #[inline(always)]
    pub fn is_cpnsel_6(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_6
    }
    #[doc = "Checks if the value of the field is `CPNSEL_7`"]
    #[inline(always)]
    pub fn is_cpnsel_7(&self) -> bool {
        *self == CPNSEL_A::CPNSEL_7
    }
}
#[doc = "Field `CPNSEL` writer - Channel input selected for the - terminal"]
pub type CPNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CPCTL0_SPEC, u8, CPNSEL_A, 3, O>;
impl<'a, const O: u8> CPNSEL_W<'a, O> {
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_0(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_1(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_2(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_3(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_4(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_5(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cpnsel_6(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cpnsel_7(self) -> &'a mut W {
        self.variant(CPNSEL_A::CPNSEL_7)
    }
}
#[doc = "Field `CPNEN` reader - Channel input enable for the - terminal"]
pub type CPNEN_R = crate::BitReader<CPNEN_A>;
#[doc = "Channel input enable for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPNEN_A {
    #[doc = "0: Selected analog input channel for V- terminal is disabled."]
    CPNEN_0 = 0,
    #[doc = "1: Selected analog input channel for V- terminal is enabled."]
    CPNEN_1 = 1,
}
impl From<CPNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPNEN_A {
        match self.bits {
            false => CPNEN_A::CPNEN_0,
            true => CPNEN_A::CPNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPNEN_0`"]
    #[inline(always)]
    pub fn is_cpnen_0(&self) -> bool {
        *self == CPNEN_A::CPNEN_0
    }
    #[doc = "Checks if the value of the field is `CPNEN_1`"]
    #[inline(always)]
    pub fn is_cpnen_1(&self) -> bool {
        *self == CPNEN_A::CPNEN_1
    }
}
#[doc = "Field `CPNEN` writer - Channel input enable for the - terminal"]
pub type CPNEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CPCTL0_SPEC, CPNEN_A, O>;
impl<'a, const O: u8> CPNEN_W<'a, O> {
    #[doc = "Selected analog input channel for V- terminal is disabled."]
    #[inline(always)]
    pub fn cpnen_0(self) -> &'a mut W {
        self.variant(CPNEN_A::CPNEN_0)
    }
    #[doc = "Selected analog input channel for V- terminal is enabled."]
    #[inline(always)]
    pub fn cpnen_1(self) -> &'a mut W {
        self.variant(CPNEN_A::CPNEN_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&self) -> CPPSEL_R {
        CPPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&self) -> CPPEN_R {
        CPPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&self) -> CPNSEL_R {
        CPNSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&self) -> CPNEN_R {
        CPNEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&mut self) -> CPPSEL_W<0> {
        CPPSEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&mut self) -> CPPEN_W<4> {
        CPPEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&mut self) -> CPNSEL_W<8> {
        CPNSEL_W::new(self)
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&mut self) -> CPNEN_W<12> {
        CPNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpctl0](index.html) module"]
pub struct CPCTL0_SPEC;
impl crate::RegisterSpec for CPCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cpctl0::R](R) reader structure"]
impl crate::Readable for CPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpctl0::W](W) writer structure"]
impl crate::Writable for CPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPCTL0 to value 0"]
impl crate::Resettable for CPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
