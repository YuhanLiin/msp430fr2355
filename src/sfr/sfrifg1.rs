#[doc = "Register `SFRIFG1` reader"]
pub struct R(crate::R<SFRIFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRIFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRIFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRIFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRIFG1` writer"]
pub struct W(crate::W<SFRIFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRIFG1_SPEC>;
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
impl From<crate::W<SFRIFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRIFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIFG` reader - Watchdog timer interrupt flag"]
pub type WDTIFG_R = crate::BitReader<WDTIFG_A>;
#[doc = "Watchdog timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTIFG_A {
    #[doc = "0: No interrupt pending"]
    WDTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    WDTIFG_1 = 1,
}
impl From<WDTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: WDTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIFG_A {
        match self.bits {
            false => WDTIFG_A::WDTIFG_0,
            true => WDTIFG_A::WDTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTIFG_0`"]
    #[inline(always)]
    pub fn is_wdtifg_0(&self) -> bool {
        *self == WDTIFG_A::WDTIFG_0
    }
    #[doc = "Checks if the value of the field is `WDTIFG_1`"]
    #[inline(always)]
    pub fn is_wdtifg_1(&self) -> bool {
        *self == WDTIFG_A::WDTIFG_1
    }
}
#[doc = "Field `WDTIFG` writer - Watchdog timer interrupt flag"]
pub type WDTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, WDTIFG_A, O>;
impl<'a, const O: u8> WDTIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_0(self) -> &'a mut W {
        self.variant(WDTIFG_A::WDTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_1(self) -> &'a mut W {
        self.variant(WDTIFG_A::WDTIFG_1)
    }
}
#[doc = "Field `OFIFG` reader - Oscillator fault interrupt flag"]
pub type OFIFG_R = crate::BitReader<OFIFG_A>;
#[doc = "Oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFIFG_A {
    #[doc = "0: No interrupt pending"]
    OFIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    OFIFG_1 = 1,
}
impl From<OFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: OFIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl OFIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFIFG_A {
        match self.bits {
            false => OFIFG_A::OFIFG_0,
            true => OFIFG_A::OFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFIFG_0`"]
    #[inline(always)]
    pub fn is_ofifg_0(&self) -> bool {
        *self == OFIFG_A::OFIFG_0
    }
    #[doc = "Checks if the value of the field is `OFIFG_1`"]
    #[inline(always)]
    pub fn is_ofifg_1(&self) -> bool {
        *self == OFIFG_A::OFIFG_1
    }
}
#[doc = "Field `OFIFG` writer - Oscillator fault interrupt flag"]
pub type OFIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, OFIFG_A, O>;
impl<'a, const O: u8> OFIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ofifg_0(self) -> &'a mut W {
        self.variant(OFIFG_A::OFIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ofifg_1(self) -> &'a mut W {
        self.variant(OFIFG_A::OFIFG_1)
    }
}
#[doc = "Field `VMAIFG` reader - Vacant memory access interrupt flag"]
pub type VMAIFG_R = crate::BitReader<VMAIFG_A>;
#[doc = "Vacant memory access interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMAIFG_A {
    #[doc = "0: No interrupt pending"]
    VMAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    VMAIFG_1 = 1,
}
impl From<VMAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: VMAIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl VMAIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMAIFG_A {
        match self.bits {
            false => VMAIFG_A::VMAIFG_0,
            true => VMAIFG_A::VMAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VMAIFG_0`"]
    #[inline(always)]
    pub fn is_vmaifg_0(&self) -> bool {
        *self == VMAIFG_A::VMAIFG_0
    }
    #[doc = "Checks if the value of the field is `VMAIFG_1`"]
    #[inline(always)]
    pub fn is_vmaifg_1(&self) -> bool {
        *self == VMAIFG_A::VMAIFG_1
    }
}
#[doc = "Field `VMAIFG` writer - Vacant memory access interrupt flag"]
pub type VMAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, VMAIFG_A, O>;
impl<'a, const O: u8> VMAIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_0(self) -> &'a mut W {
        self.variant(VMAIFG_A::VMAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_1(self) -> &'a mut W {
        self.variant(VMAIFG_A::VMAIFG_1)
    }
}
#[doc = "Field `NMIIFG` reader - NMI pin interrupt flag"]
pub type NMIIFG_R = crate::BitReader<NMIIFG_A>;
#[doc = "NMI pin interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIIFG_A {
    #[doc = "0: No interrupt pending"]
    NMIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    NMIIFG_1 = 1,
}
impl From<NMIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: NMIIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIIFG_A {
        match self.bits {
            false => NMIIFG_A::NMIIFG_0,
            true => NMIIFG_A::NMIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIIFG_0`"]
    #[inline(always)]
    pub fn is_nmiifg_0(&self) -> bool {
        *self == NMIIFG_A::NMIIFG_0
    }
    #[doc = "Checks if the value of the field is `NMIIFG_1`"]
    #[inline(always)]
    pub fn is_nmiifg_1(&self) -> bool {
        *self == NMIIFG_A::NMIIFG_1
    }
}
#[doc = "Field `NMIIFG` writer - NMI pin interrupt flag"]
pub type NMIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, NMIIFG_A, O>;
impl<'a, const O: u8> NMIIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_0(self) -> &'a mut W {
        self.variant(NMIIFG_A::NMIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_1(self) -> &'a mut W {
        self.variant(NMIIFG_A::NMIIFG_1)
    }
}
#[doc = "Field `JMBINIFG` reader - JTAG mailbox input interrupt flag"]
pub type JMBINIFG_R = crate::BitReader<JMBINIFG_A>;
#[doc = "JTAG mailbox input interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBINIFG_A {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    JMBINIFG_0 = 0,
    #[doc = "1: Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    JMBINIFG_1 = 1,
}
impl From<JMBINIFG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBINIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBINIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBINIFG_A {
        match self.bits {
            false => JMBINIFG_A::JMBINIFG_0,
            true => JMBINIFG_A::JMBINIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBINIFG_0`"]
    #[inline(always)]
    pub fn is_jmbinifg_0(&self) -> bool {
        *self == JMBINIFG_A::JMBINIFG_0
    }
    #[doc = "Checks if the value of the field is `JMBINIFG_1`"]
    #[inline(always)]
    pub fn is_jmbinifg_1(&self) -> bool {
        *self == JMBINIFG_A::JMBINIFG_1
    }
}
#[doc = "Field `JMBINIFG` writer - JTAG mailbox input interrupt flag"]
pub type JMBINIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, JMBINIFG_A, O>;
impl<'a, const O: u8> JMBINIFG_W<'a, O> {
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    #[inline(always)]
    pub fn jmbinifg_0(self) -> &'a mut W {
        self.variant(JMBINIFG_A::JMBINIFG_0)
    }
    #[doc = "Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    #[inline(always)]
    pub fn jmbinifg_1(self) -> &'a mut W {
        self.variant(JMBINIFG_A::JMBINIFG_1)
    }
}
#[doc = "Field `JMBOUTIFG` reader - JTAG mailbox output interrupt flag"]
pub type JMBOUTIFG_R = crate::BitReader<JMBOUTIFG_A>;
#[doc = "JTAG mailbox output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUTIFG_A {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    JMBOUTIFG_0 = 0,
    #[doc = "1: Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    JMBOUTIFG_1 = 1,
}
impl From<JMBOUTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUTIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUTIFG_A {
        match self.bits {
            false => JMBOUTIFG_A::JMBOUTIFG_0,
            true => JMBOUTIFG_A::JMBOUTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG_0`"]
    #[inline(always)]
    pub fn is_jmboutifg_0(&self) -> bool {
        *self == JMBOUTIFG_A::JMBOUTIFG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG_1`"]
    #[inline(always)]
    pub fn is_jmboutifg_1(&self) -> bool {
        *self == JMBOUTIFG_A::JMBOUTIFG_1
    }
}
#[doc = "Field `JMBOUTIFG` writer - JTAG mailbox output interrupt flag"]
pub type JMBOUTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, JMBOUTIFG_A, O>;
impl<'a, const O: u8> JMBOUTIFG_W<'a, O> {
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    #[inline(always)]
    pub fn jmboutifg_0(self) -> &'a mut W {
        self.variant(JMBOUTIFG_A::JMBOUTIFG_0)
    }
    #[doc = "Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    #[inline(always)]
    pub fn jmboutifg_1(self) -> &'a mut W {
        self.variant(JMBOUTIFG_A::JMBOUTIFG_1)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VMAIFG_R {
        VMAIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JMBINIFG_R {
        JMBINIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JMBOUTIFG_R {
        JMBOUTIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W<0> {
        WDTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W<1> {
        OFIFG_W::new(self)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VMAIFG_W<3> {
        VMAIFG_W::new(self)
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W<4> {
        NMIIFG_W::new(self)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JMBINIFG_W<6> {
        JMBINIFG_W::new(self)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JMBOUTIFG_W<7> {
        JMBOUTIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrifg1](index.html) module"]
pub struct SFRIFG1_SPEC;
impl crate::RegisterSpec for SFRIFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrifg1::R](R) reader structure"]
impl crate::Readable for SFRIFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrifg1::W](W) writer structure"]
impl crate::Writable for SFRIFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRIFG1 to value 0"]
impl crate::Resettable for SFRIFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
