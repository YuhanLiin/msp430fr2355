#[doc = "Register `SFRIE1` reader"]
pub struct R(crate::R<SFRIE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRIE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRIE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRIE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRIE1` writer"]
pub struct W(crate::W<SFRIE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRIE1_SPEC>;
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
impl From<crate::W<SFRIE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRIE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIE` reader - Watchdog timer interrupt enable"]
pub type WDTIE_R = crate::BitReader<WDTIE_A>;
#[doc = "Watchdog timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<WDTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIE_A {
        match self.bits {
            false => WDTIE_A::DISABLE,
            true => WDTIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDTIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDTIE_A::ENABLE
    }
}
#[doc = "Field `WDTIE` writer - Watchdog timer interrupt enable"]
pub type WDTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, WDTIE_A, O>;
impl<'a, const O: u8> WDTIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTIE_A::ENABLE)
    }
}
#[doc = "Field `OFIE` reader - Oscillator fault interrupt enable"]
pub type OFIE_R = crate::BitReader<OFIE_A>;
#[doc = "Oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<OFIE_A> for bool {
    #[inline(always)]
    fn from(variant: OFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFIE_A {
        match self.bits {
            false => OFIE_A::DISABLE,
            true => OFIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OFIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OFIE_A::ENABLE
    }
}
#[doc = "Field `OFIE` writer - Oscillator fault interrupt enable"]
pub type OFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, OFIE_A, O>;
impl<'a, const O: u8> OFIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OFIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OFIE_A::ENABLE)
    }
}
#[doc = "Field `VMAIE` reader - Vacant memory access interrupt enable"]
pub type VMAIE_R = crate::BitReader<VMAIE_A>;
#[doc = "Vacant memory access interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMAIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<VMAIE_A> for bool {
    #[inline(always)]
    fn from(variant: VMAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl VMAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMAIE_A {
        match self.bits {
            false => VMAIE_A::DISABLE,
            true => VMAIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VMAIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VMAIE_A::ENABLE
    }
}
#[doc = "Field `VMAIE` writer - Vacant memory access interrupt enable"]
pub type VMAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, VMAIE_A, O>;
impl<'a, const O: u8> VMAIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VMAIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VMAIE_A::ENABLE)
    }
}
#[doc = "Field `NMIIE` reader - NMI pin interrupt enable"]
pub type NMIIE_R = crate::BitReader<NMIIE_A>;
#[doc = "NMI pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<NMIIE_A> for bool {
    #[inline(always)]
    fn from(variant: NMIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIIE_A {
        match self.bits {
            false => NMIIE_A::DISABLE,
            true => NMIIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NMIIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NMIIE_A::ENABLE
    }
}
#[doc = "Field `NMIIE` writer - NMI pin interrupt enable"]
pub type NMIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, NMIIE_A, O>;
impl<'a, const O: u8> NMIIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NMIIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NMIIE_A::ENABLE)
    }
}
#[doc = "Field `JMBINIE` reader - JTAG mailbox input interrupt enable"]
pub type JMBINIE_R = crate::BitReader<JMBINIE_A>;
#[doc = "JTAG mailbox input interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBINIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<JMBINIE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBINIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBINIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBINIE_A {
        match self.bits {
            false => JMBINIE_A::DISABLE,
            true => JMBINIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JMBINIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JMBINIE_A::ENABLE
    }
}
#[doc = "Field `JMBINIE` writer - JTAG mailbox input interrupt enable"]
pub type JMBINIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, JMBINIE_A, O>;
impl<'a, const O: u8> JMBINIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JMBINIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JMBINIE_A::ENABLE)
    }
}
#[doc = "Field `JMBOUTIE` reader - JTAG mailbox output interrupt enable"]
pub type JMBOUTIE_R = crate::BitReader<JMBOUTIE_A>;
#[doc = "JTAG mailbox output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUTIE_A {
    #[doc = "0: Interrupts disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupts enabled"]
    ENABLE = 1,
}
impl From<JMBOUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUTIE_A {
        match self.bits {
            false => JMBOUTIE_A::DISABLE,
            true => JMBOUTIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JMBOUTIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JMBOUTIE_A::ENABLE
    }
}
#[doc = "Field `JMBOUTIE` writer - JTAG mailbox output interrupt enable"]
pub type JMBOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIE1_SPEC, JMBOUTIE_A, O>;
impl<'a, const O: u8> JMBOUTIE_W<'a, O> {
    #[doc = "Interrupts disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JMBOUTIE_A::DISABLE)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JMBOUTIE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W<0> {
        WDTIE_W::new(self)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W<1> {
        OFIE_W::new(self)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VMAIE_W<3> {
        VMAIE_W::new(self)
    }
    #[doc = "Bit 4 - NMI pin interrupt enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W<4> {
        NMIIE_W::new(self)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JMBINIE_W<6> {
        JMBINIE_W::new(self)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W<7> {
        JMBOUTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrie1](index.html) module"]
pub struct SFRIE1_SPEC;
impl crate::RegisterSpec for SFRIE1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrie1::R](R) reader structure"]
impl crate::Readable for SFRIE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrie1::W](W) writer structure"]
impl crate::Writable for SFRIE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for SFRIE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
