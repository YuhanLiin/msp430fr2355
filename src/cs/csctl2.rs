#[doc = "Register `CSCTL2` reader"]
pub struct R(crate::R<CSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL2` writer"]
pub struct W(crate::W<CSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL2_SPEC>;
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
impl From<crate::W<CSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLN` reader - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
pub type FLLN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLLN` writer - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
pub type FLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL2_SPEC, u16, u16, 10, O>;
#[doc = "Field `FLLD` reader - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
pub type FLLD_R = crate::FieldReader<u8, FLLD_A>;
#[doc = "FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLD_A {
    #[doc = "0: fDCOCLK / 1"]
    _1 = 0,
    #[doc = "1: fDCOCLK / 2"]
    _2 = 1,
    #[doc = "2: fDCOCLK / 4"]
    _4 = 2,
    #[doc = "3: fDCOCLK / 8"]
    _8 = 3,
    #[doc = "4: fDCOCLK / 16"]
    _16 = 4,
    #[doc = "5: fDCOCLK / 32"]
    _32 = 5,
    #[doc = "6: fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    FLLD_6 = 6,
    #[doc = "7: fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    FLLD_7 = 7,
}
impl From<FLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLD_A) -> Self {
        variant as _
    }
}
impl FLLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLD_A {
        match self.bits {
            0 => FLLD_A::_1,
            1 => FLLD_A::_2,
            2 => FLLD_A::_4,
            3 => FLLD_A::_8,
            4 => FLLD_A::_16,
            5 => FLLD_A::_32,
            6 => FLLD_A::FLLD_6,
            7 => FLLD_A::FLLD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLD_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == FLLD_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FLLD_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == FLLD_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == FLLD_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == FLLD_A::_32
    }
    #[doc = "Checks if the value of the field is `FLLD_6`"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        *self == FLLD_A::FLLD_6
    }
    #[doc = "Checks if the value of the field is `FLLD_7`"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        *self == FLLD_A::FLLD_7
    }
}
#[doc = "Field `FLLD` writer - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
pub type FLLD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL2_SPEC, u8, FLLD_A, 3, O>;
impl<'a, const O: u8> FLLD_W<'a, O> {
    #[doc = "fDCOCLK / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLD_A::_1)
    }
    #[doc = "fDCOCLK / 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FLLD_A::_2)
    }
    #[doc = "fDCOCLK / 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FLLD_A::_4)
    }
    #[doc = "fDCOCLK / 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(FLLD_A::_8)
    }
    #[doc = "fDCOCLK / 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FLLD_A::_16)
    }
    #[doc = "fDCOCLK / 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FLLD_A::_32)
    }
    #[doc = "fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_6)
    }
    #[doc = "fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut W {
        self.variant(FLLD_A::FLLD_7)
    }
}
impl R {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&self) -> FLLN_R {
        FLLN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&self) -> FLLD_R {
        FLLD_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&mut self) -> FLLN_W<0> {
        FLLN_W::new(self)
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&mut self) -> FLLD_W<12> {
        FLLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock System Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl2](index.html) module"]
pub struct CSCTL2_SPEC;
impl crate::RegisterSpec for CSCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl2::R](R) reader structure"]
impl crate::Readable for CSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl2::W](W) writer structure"]
impl crate::Writable for CSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL2 to value 0"]
impl crate::Resettable for CSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
