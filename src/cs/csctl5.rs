#[doc = "Register `CSCTL5` reader"]
pub struct R(crate::R<CSCTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL5` writer"]
pub struct W(crate::W<CSCTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL5_SPEC>;
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
impl From<crate::W<CSCTL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVM` reader - MCLK source divider"]
pub type DIVM_R = crate::FieldReader<u8, DIVM_A>;
#[doc = "MCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /32"]
    _32 = 5,
    #[doc = "6: /64"]
    _64 = 6,
    #[doc = "7: /128"]
    _128 = 7,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::_1,
            1 => DIVM_A::_2,
            2 => DIVM_A::_4,
            3 => DIVM_A::_8,
            4 => DIVM_A::_16,
            5 => DIVM_A::_32,
            6 => DIVM_A::_64,
            7 => DIVM_A::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVM_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DIVM_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DIVM_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DIVM_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DIVM_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DIVM_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == DIVM_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DIVM_A::_128
    }
}
#[doc = "Field `DIVM` writer - MCLK source divider"]
pub type DIVM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL5_SPEC, u8, DIVM_A, 3, O>;
impl<'a, const O: u8> DIVM_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVM_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DIVM_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DIVM_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DIVM_A::_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DIVM_A::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DIVM_A::_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(DIVM_A::_64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DIVM_A::_128)
    }
}
#[doc = "Field `DIVS` reader - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
pub type DIVS_R = crate::FieldReader<u8, DIVS_A>;
#[doc = "SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
impl DIVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::_1,
            1 => DIVS_A::_2,
            2 => DIVS_A::_4,
            3 => DIVS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIVS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DIVS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DIVS_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DIVS_A::_8
    }
}
#[doc = "Field `DIVS` writer - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
pub type DIVS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL5_SPEC, u8, DIVS_A, 2, O>;
impl<'a, const O: u8> DIVS_W<'a, O> {
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIVS_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DIVS_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DIVS_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DIVS_A::_8)
    }
}
#[doc = "Field `SMCLKOFF` reader - SMCLK off. This bit turns off SMCLK clock"]
pub type SMCLKOFF_R = crate::BitReader<SMCLKOFF_A>;
#[doc = "SMCLK off. This bit turns off SMCLK clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMCLKOFF_A {
    #[doc = "0: SMCLK on"]
    SMCLKOFF_0 = 0,
    #[doc = "1: SMCLK off"]
    SMCLKOFF_1 = 1,
}
impl From<SMCLKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SMCLKOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKOFF_A {
        match self.bits {
            false => SMCLKOFF_A::SMCLKOFF_0,
            true => SMCLKOFF_A::SMCLKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_0`"]
    #[inline(always)]
    pub fn is_smclkoff_0(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_0
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_1`"]
    #[inline(always)]
    pub fn is_smclkoff_1(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_1
    }
}
#[doc = "Field `SMCLKOFF` writer - SMCLK off. This bit turns off SMCLK clock"]
pub type SMCLKOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL5_SPEC, SMCLKOFF_A, O>;
impl<'a, const O: u8> SMCLKOFF_W<'a, O> {
    #[doc = "SMCLK on"]
    #[inline(always)]
    pub fn smclkoff_0(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_0)
    }
    #[doc = "SMCLK off"]
    #[inline(always)]
    pub fn smclkoff_1(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_1)
    }
}
#[doc = "Field `VLOAUTOOFF` reader - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
pub type VLOAUTOOFF_R = crate::BitReader<VLOAUTOOFF_A>;
#[doc = "VLO automatic off enable. This bit turns off VLO, if VLO is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLOAUTOOFF_A {
    #[doc = "0: VLO always on"]
    VLOAUTOOFF_0 = 0,
    #[doc = "1: VLO automatically turned off if not used(default)"]
    VLOAUTOOFF_1 = 1,
}
impl From<VLOAUTOOFF_A> for bool {
    #[inline(always)]
    fn from(variant: VLOAUTOOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl VLOAUTOOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLOAUTOOFF_A {
        match self.bits {
            false => VLOAUTOOFF_A::VLOAUTOOFF_0,
            true => VLOAUTOOFF_A::VLOAUTOOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLOAUTOOFF_0`"]
    #[inline(always)]
    pub fn is_vloautooff_0(&self) -> bool {
        *self == VLOAUTOOFF_A::VLOAUTOOFF_0
    }
    #[doc = "Checks if the value of the field is `VLOAUTOOFF_1`"]
    #[inline(always)]
    pub fn is_vloautooff_1(&self) -> bool {
        *self == VLOAUTOOFF_A::VLOAUTOOFF_1
    }
}
#[doc = "Field `VLOAUTOOFF` writer - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
pub type VLOAUTOOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL5_SPEC, VLOAUTOOFF_A, O>;
impl<'a, const O: u8> VLOAUTOOFF_W<'a, O> {
    #[doc = "VLO always on"]
    #[inline(always)]
    pub fn vloautooff_0(self) -> &'a mut W {
        self.variant(VLOAUTOOFF_A::VLOAUTOOFF_0)
    }
    #[doc = "VLO automatically turned off if not used(default)"]
    #[inline(always)]
    pub fn vloautooff_1(self) -> &'a mut W {
        self.variant(VLOAUTOOFF_A::VLOAUTOOFF_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - SMCLK off. This bit turns off SMCLK clock"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
    #[inline(always)]
    pub fn vloautooff(&self) -> VLOAUTOOFF_R {
        VLOAUTOOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W<0> {
        DIVM_W::new(self)
    }
    #[doc = "Bits 4:5 - SMCLK source divider. SMCLK directly derives from MCLK. SMCLK frequency is the combination of DIVM and DIVS out of selected clock source."]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W<4> {
        DIVS_W::new(self)
    }
    #[doc = "Bit 8 - SMCLK off. This bit turns off SMCLK clock"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W<8> {
        SMCLKOFF_W::new(self)
    }
    #[doc = "Bit 12 - VLO automatic off enable. This bit turns off VLO, if VLO is not used."]
    #[inline(always)]
    pub fn vloautooff(&mut self) -> VLOAUTOOFF_W<12> {
        VLOAUTOOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl5](index.html) module"]
pub struct CSCTL5_SPEC;
impl crate::RegisterSpec for CSCTL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl5::R](R) reader structure"]
impl crate::Readable for CSCTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl5::W](W) writer structure"]
impl crate::Writable for CSCTL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for CSCTL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
