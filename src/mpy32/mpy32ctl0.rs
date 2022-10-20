#[doc = "Register `MPY32CTL0` reader"]
pub struct R(crate::R<MPY32CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPY32CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPY32CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPY32CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPY32CTL0` writer"]
pub struct W(crate::W<MPY32CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPY32CTL0_SPEC>;
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
impl From<crate::W<MPY32CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPY32CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPYC` reader - Carry of the multiplier"]
pub type MPYC_R = crate::BitReader<MPYC_A>;
#[doc = "Carry of the multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYC_A {
    #[doc = "0: No carry for result."]
    MPYC_0 = 0,
    #[doc = "1: Result has a carry."]
    MPYC_1 = 1,
}
impl From<MPYC_A> for bool {
    #[inline(always)]
    fn from(variant: MPYC_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYC_A {
        match self.bits {
            false => MPYC_A::MPYC_0,
            true => MPYC_A::MPYC_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYC_0`"]
    #[inline(always)]
    pub fn is_mpyc_0(&self) -> bool {
        *self == MPYC_A::MPYC_0
    }
    #[doc = "Checks if the value of the field is `MPYC_1`"]
    #[inline(always)]
    pub fn is_mpyc_1(&self) -> bool {
        *self == MPYC_A::MPYC_1
    }
}
#[doc = "Field `MPYC` writer - Carry of the multiplier"]
pub type MPYC_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYC_A, O>;
impl<'a, const O: u8> MPYC_W<'a, O> {
    #[doc = "No carry for result."]
    #[inline(always)]
    pub fn mpyc_0(self) -> &'a mut W {
        self.variant(MPYC_A::MPYC_0)
    }
    #[doc = "Result has a carry."]
    #[inline(always)]
    pub fn mpyc_1(self) -> &'a mut W {
        self.variant(MPYC_A::MPYC_1)
    }
}
#[doc = "Field `MPYFRAC` reader - Fractional mode."]
pub type MPYFRAC_R = crate::BitReader<MPYFRAC_A>;
#[doc = "Fractional mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYFRAC_A {
    #[doc = "0: Fractional mode disabled."]
    DISABLE = 0,
    #[doc = "1: Fractional mode enabled."]
    ENABLE = 1,
}
impl From<MPYFRAC_A> for bool {
    #[inline(always)]
    fn from(variant: MPYFRAC_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYFRAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYFRAC_A {
        match self.bits {
            false => MPYFRAC_A::DISABLE,
            true => MPYFRAC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPYFRAC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPYFRAC_A::ENABLE
    }
}
#[doc = "Field `MPYFRAC` writer - Fractional mode."]
pub type MPYFRAC_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYFRAC_A, O>;
impl<'a, const O: u8> MPYFRAC_W<'a, O> {
    #[doc = "Fractional mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPYFRAC_A::DISABLE)
    }
    #[doc = "Fractional mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPYFRAC_A::ENABLE)
    }
}
#[doc = "Field `MPYSAT` reader - Saturation mode"]
pub type MPYSAT_R = crate::BitReader<MPYSAT_A>;
#[doc = "Saturation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYSAT_A {
    #[doc = "0: Saturation mode disabled."]
    DISABLE = 0,
    #[doc = "1: Saturation mode enabled."]
    ENABLE = 1,
}
impl From<MPYSAT_A> for bool {
    #[inline(always)]
    fn from(variant: MPYSAT_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYSAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYSAT_A {
        match self.bits {
            false => MPYSAT_A::DISABLE,
            true => MPYSAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPYSAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPYSAT_A::ENABLE
    }
}
#[doc = "Field `MPYSAT` writer - Saturation mode"]
pub type MPYSAT_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYSAT_A, O>;
impl<'a, const O: u8> MPYSAT_W<'a, O> {
    #[doc = "Saturation mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPYSAT_A::DISABLE)
    }
    #[doc = "Saturation mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPYSAT_A::ENABLE)
    }
}
#[doc = "Field `MPYM` reader - Multiplier mode"]
pub type MPYM_R = crate::FieldReader<u8, MPYM_A>;
#[doc = "Multiplier mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPYM_A {
    #[doc = "0: MPY Multiply"]
    MPY = 0,
    #[doc = "1: MPYS Signed multiply"]
    MPYS = 1,
    #[doc = "2: MAC Multiply accumulate"]
    MAC = 2,
    #[doc = "3: MACS Signed multiply accumulate"]
    MACS = 3,
}
impl From<MPYM_A> for u8 {
    #[inline(always)]
    fn from(variant: MPYM_A) -> Self {
        variant as _
    }
}
impl MPYM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYM_A {
        match self.bits {
            0 => MPYM_A::MPY,
            1 => MPYM_A::MPYS,
            2 => MPYM_A::MAC,
            3 => MPYM_A::MACS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPY`"]
    #[inline(always)]
    pub fn is_mpy(&self) -> bool {
        *self == MPYM_A::MPY
    }
    #[doc = "Checks if the value of the field is `MPYS`"]
    #[inline(always)]
    pub fn is_mpys(&self) -> bool {
        *self == MPYM_A::MPYS
    }
    #[doc = "Checks if the value of the field is `MAC`"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == MPYM_A::MAC
    }
    #[doc = "Checks if the value of the field is `MACS`"]
    #[inline(always)]
    pub fn is_macs(&self) -> bool {
        *self == MPYM_A::MACS
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode"]
pub type MPYM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, MPY32CTL0_SPEC, u8, MPYM_A, 2, O>;
impl<'a, const O: u8> MPYM_W<'a, O> {
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn mpy(self) -> &'a mut W {
        self.variant(MPYM_A::MPY)
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn mpys(self) -> &'a mut W {
        self.variant(MPYM_A::MPYS)
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn mac(self) -> &'a mut W {
        self.variant(MPYM_A::MAC)
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn macs(self) -> &'a mut W {
        self.variant(MPYM_A::MACS)
    }
}
#[doc = "Field `MPYOP1_32` reader - Multiplier bit width of operand 1"]
pub type MPYOP1_32_R = crate::BitReader<MPYOP1_32_A>;
#[doc = "Multiplier bit width of operand 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYOP1_32_A {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<MPYOP1_32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYOP1_32_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYOP1_32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYOP1_32_A {
        match self.bits {
            false => MPYOP1_32_A::_16,
            true => MPYOP1_32_A::_32,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == MPYOP1_32_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == MPYOP1_32_A::_32
    }
}
#[doc = "Field `MPYOP1_32` writer - Multiplier bit width of operand 1"]
pub type MPYOP1_32_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYOP1_32_A, O>;
impl<'a, const O: u8> MPYOP1_32_W<'a, O> {
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(MPYOP1_32_A::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(MPYOP1_32_A::_32)
    }
}
#[doc = "Field `MPYOP2_32` reader - Multiplier bit width of operand 2"]
pub type MPYOP2_32_R = crate::BitReader<MPYOP2_32_A>;
#[doc = "Multiplier bit width of operand 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYOP2_32_A {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<MPYOP2_32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYOP2_32_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYOP2_32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYOP2_32_A {
        match self.bits {
            false => MPYOP2_32_A::_16,
            true => MPYOP2_32_A::_32,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == MPYOP2_32_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == MPYOP2_32_A::_32
    }
}
#[doc = "Field `MPYOP2_32` writer - Multiplier bit width of operand 2"]
pub type MPYOP2_32_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYOP2_32_A, O>;
impl<'a, const O: u8> MPYOP2_32_W<'a, O> {
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(MPYOP2_32_A::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(MPYOP2_32_A::_32)
    }
}
#[doc = "Field `MPYDLYWRTEN` reader - Delayed write enable."]
pub type MPYDLYWRTEN_R = crate::BitReader<MPYDLYWRTEN_A>;
#[doc = "Delayed write enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYDLYWRTEN_A {
    #[doc = "0: Writes are not delayed."]
    MPYDLYWRTEN_0 = 0,
    #[doc = "1: Writes are delayed."]
    MPYDLYWRTEN_1 = 1,
}
impl From<MPYDLYWRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MPYDLYWRTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYDLYWRTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYDLYWRTEN_A {
        match self.bits {
            false => MPYDLYWRTEN_A::MPYDLYWRTEN_0,
            true => MPYDLYWRTEN_A::MPYDLYWRTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYDLYWRTEN_0`"]
    #[inline(always)]
    pub fn is_mpydlywrten_0(&self) -> bool {
        *self == MPYDLYWRTEN_A::MPYDLYWRTEN_0
    }
    #[doc = "Checks if the value of the field is `MPYDLYWRTEN_1`"]
    #[inline(always)]
    pub fn is_mpydlywrten_1(&self) -> bool {
        *self == MPYDLYWRTEN_A::MPYDLYWRTEN_1
    }
}
#[doc = "Field `MPYDLYWRTEN` writer - Delayed write enable."]
pub type MPYDLYWRTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYDLYWRTEN_A, O>;
impl<'a, const O: u8> MPYDLYWRTEN_W<'a, O> {
    #[doc = "Writes are not delayed."]
    #[inline(always)]
    pub fn mpydlywrten_0(self) -> &'a mut W {
        self.variant(MPYDLYWRTEN_A::MPYDLYWRTEN_0)
    }
    #[doc = "Writes are delayed."]
    #[inline(always)]
    pub fn mpydlywrten_1(self) -> &'a mut W {
        self.variant(MPYDLYWRTEN_A::MPYDLYWRTEN_1)
    }
}
#[doc = "Field `MPYDLY32` reader - Delayed write mode."]
pub type MPYDLY32_R = crate::BitReader<MPYDLY32_A>;
#[doc = "Delayed write mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPYDLY32_A {
    #[doc = "0: Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    MPYDLY32_0 = 0,
    #[doc = "1: Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    MPYDLY32_1 = 1,
}
impl From<MPYDLY32_A> for bool {
    #[inline(always)]
    fn from(variant: MPYDLY32_A) -> Self {
        variant as u8 != 0
    }
}
impl MPYDLY32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYDLY32_A {
        match self.bits {
            false => MPYDLY32_A::MPYDLY32_0,
            true => MPYDLY32_A::MPYDLY32_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPYDLY32_0`"]
    #[inline(always)]
    pub fn is_mpydly32_0(&self) -> bool {
        *self == MPYDLY32_A::MPYDLY32_0
    }
    #[doc = "Checks if the value of the field is `MPYDLY32_1`"]
    #[inline(always)]
    pub fn is_mpydly32_1(&self) -> bool {
        *self == MPYDLY32_A::MPYDLY32_1
    }
}
#[doc = "Field `MPYDLY32` writer - Delayed write mode."]
pub type MPYDLY32_W<'a, const O: u8> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, MPYDLY32_A, O>;
impl<'a, const O: u8> MPYDLY32_W<'a, O> {
    #[doc = "Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    #[inline(always)]
    pub fn mpydly32_0(self) -> &'a mut W {
        self.variant(MPYDLY32_A::MPYDLY32_0)
    }
    #[doc = "Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    #[inline(always)]
    pub fn mpydly32_1(self) -> &'a mut W {
        self.variant(MPYDLY32_A::MPYDLY32_1)
    }
}
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MPYC_R {
        MPYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MPYFRAC_R {
        MPYFRAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MPYSAT_R {
        MPYSAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&self) -> MPYM_R {
        MPYM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&self) -> MPYOP1_32_R {
        MPYOP1_32_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&self) -> MPYOP2_32_R {
        MPYOP2_32_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MPYDLYWRTEN_R {
        MPYDLYWRTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&self) -> MPYDLY32_R {
        MPYDLY32_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MPYC_W<0> {
        MPYC_W::new(self)
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MPYFRAC_W<2> {
        MPYFRAC_W::new(self)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MPYSAT_W<3> {
        MPYSAT_W::new(self)
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MPYM_W<4> {
        MPYM_W::new(self)
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&mut self) -> MPYOP1_32_W<6> {
        MPYOP1_32_W::new(self)
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&mut self) -> MPYOP2_32_W<7> {
        MPYOP2_32_W::new(self)
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MPYDLYWRTEN_W<8> {
        MPYDLYWRTEN_W::new(self)
    }
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> MPYDLY32_W<9> {
        MPYDLY32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPY32 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32ctl0](index.html) module"]
pub struct MPY32CTL0_SPEC;
impl crate::RegisterSpec for MPY32CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpy32ctl0::R](R) reader structure"]
impl crate::Readable for MPY32CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpy32ctl0::W](W) writer structure"]
impl crate::Writable for MPY32CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPY32CTL0 to value 0"]
impl crate::Resettable for MPY32CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
