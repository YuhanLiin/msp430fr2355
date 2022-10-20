#[doc = "Register `SYSJMBC` reader"]
pub struct R(crate::R<SYSJMBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBC` writer"]
pub struct W(crate::W<SYSJMBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBC_SPEC>;
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
impl From<crate::W<SYSJMBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JMBIN0FG` reader - Incoming JTAG Mailbox 0 flag"]
pub type JMBIN0FG_R = crate::BitReader<JMBIN0FG_A>;
#[doc = "Incoming JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBIN0FG_A {
    #[doc = "0: JMBI0 has no new data"]
    JMBIN0FG_0 = 0,
    #[doc = "1: JMBI0 has new data available"]
    JMBIN0FG_1 = 1,
}
impl From<JMBIN0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN0FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBIN0FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN0FG_A {
        match self.bits {
            false => JMBIN0FG_A::JMBIN0FG_0,
            true => JMBIN0FG_A::JMBIN0FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBIN0FG_0`"]
    #[inline(always)]
    pub fn is_jmbin0fg_0(&self) -> bool {
        *self == JMBIN0FG_A::JMBIN0FG_0
    }
    #[doc = "Checks if the value of the field is `JMBIN0FG_1`"]
    #[inline(always)]
    pub fn is_jmbin0fg_1(&self) -> bool {
        *self == JMBIN0FG_A::JMBIN0FG_1
    }
}
#[doc = "Field `JMBIN0FG` writer - Incoming JTAG Mailbox 0 flag"]
pub type JMBIN0FG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBIN0FG_A, O>;
impl<'a, const O: u8> JMBIN0FG_W<'a, O> {
    #[doc = "JMBI0 has no new data"]
    #[inline(always)]
    pub fn jmbin0fg_0(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::JMBIN0FG_0)
    }
    #[doc = "JMBI0 has new data available"]
    #[inline(always)]
    pub fn jmbin0fg_1(self) -> &'a mut W {
        self.variant(JMBIN0FG_A::JMBIN0FG_1)
    }
}
#[doc = "Field `JMBIN1FG` reader - Incoming JTAG Mailbox 1 flag"]
pub type JMBIN1FG_R = crate::BitReader<JMBIN1FG_A>;
#[doc = "Incoming JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBIN1FG_A {
    #[doc = "0: JMBI1 has no new data"]
    JMBIN1FG_0 = 0,
    #[doc = "1: JMBI1 has new data available"]
    JMBIN1FG_1 = 1,
}
impl From<JMBIN1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBIN1FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBIN1FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBIN1FG_A {
        match self.bits {
            false => JMBIN1FG_A::JMBIN1FG_0,
            true => JMBIN1FG_A::JMBIN1FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBIN1FG_0`"]
    #[inline(always)]
    pub fn is_jmbin1fg_0(&self) -> bool {
        *self == JMBIN1FG_A::JMBIN1FG_0
    }
    #[doc = "Checks if the value of the field is `JMBIN1FG_1`"]
    #[inline(always)]
    pub fn is_jmbin1fg_1(&self) -> bool {
        *self == JMBIN1FG_A::JMBIN1FG_1
    }
}
#[doc = "Field `JMBIN1FG` writer - Incoming JTAG Mailbox 1 flag"]
pub type JMBIN1FG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBIN1FG_A, O>;
impl<'a, const O: u8> JMBIN1FG_W<'a, O> {
    #[doc = "JMBI1 has no new data"]
    #[inline(always)]
    pub fn jmbin1fg_0(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::JMBIN1FG_0)
    }
    #[doc = "JMBI1 has new data available"]
    #[inline(always)]
    pub fn jmbin1fg_1(self) -> &'a mut W {
        self.variant(JMBIN1FG_A::JMBIN1FG_1)
    }
}
#[doc = "Field `JMBOUT0FG` reader - Outgoing JTAG Mailbox 0 flag"]
pub type JMBOUT0FG_R = crate::BitReader<JMBOUT0FG_A>;
#[doc = "Outgoing JTAG Mailbox 0 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUT0FG_A {
    #[doc = "0: JMBO0 is not ready to receive new data"]
    JMBOUT0FG_0 = 0,
    #[doc = "1: JMBO0 is ready to receive new data"]
    JMBOUT0FG_1 = 1,
}
impl From<JMBOUT0FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT0FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUT0FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT0FG_A {
        match self.bits {
            false => JMBOUT0FG_A::JMBOUT0FG_0,
            true => JMBOUT0FG_A::JMBOUT0FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUT0FG_0`"]
    #[inline(always)]
    pub fn is_jmbout0fg_0(&self) -> bool {
        *self == JMBOUT0FG_A::JMBOUT0FG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUT0FG_1`"]
    #[inline(always)]
    pub fn is_jmbout0fg_1(&self) -> bool {
        *self == JMBOUT0FG_A::JMBOUT0FG_1
    }
}
#[doc = "Field `JMBOUT1FG` reader - Outgoing JTAG Mailbox 1 flag"]
pub type JMBOUT1FG_R = crate::BitReader<JMBOUT1FG_A>;
#[doc = "Outgoing JTAG Mailbox 1 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBOUT1FG_A {
    #[doc = "0: JMBO1 is not ready to receive new data"]
    JMBOUT1FG_0 = 0,
    #[doc = "1: JMBO1 is ready to receive new data"]
    JMBOUT1FG_1 = 1,
}
impl From<JMBOUT1FG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUT1FG_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBOUT1FG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUT1FG_A {
        match self.bits {
            false => JMBOUT1FG_A::JMBOUT1FG_0,
            true => JMBOUT1FG_A::JMBOUT1FG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUT1FG_0`"]
    #[inline(always)]
    pub fn is_jmbout1fg_0(&self) -> bool {
        *self == JMBOUT1FG_A::JMBOUT1FG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUT1FG_1`"]
    #[inline(always)]
    pub fn is_jmbout1fg_1(&self) -> bool {
        *self == JMBOUT1FG_A::JMBOUT1FG_1
    }
}
#[doc = "Field `JMBMODE` reader - Operation mode of JMB"]
pub type JMBMODE_R = crate::BitReader<JMBMODE_A>;
#[doc = "Operation mode of JMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBMODE_A {
    #[doc = "0: 16-bit transfers using JMBO0 and JMBI0 only"]
    _16BIT = 0,
    #[doc = "1: 32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    _32BIT = 1,
}
impl From<JMBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: JMBMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBMODE_A {
        match self.bits {
            false => JMBMODE_A::_16BIT,
            true => JMBMODE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == JMBMODE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == JMBMODE_A::_32BIT
    }
}
#[doc = "Field `JMBMODE` writer - Operation mode of JMB"]
pub type JMBMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBMODE_A, O>;
impl<'a, const O: u8> JMBMODE_W<'a, O> {
    #[doc = "16-bit transfers using JMBO0 and JMBI0 only"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_16BIT)
    }
    #[doc = "32-bit transfers using JMBO0 with JMBO1 and JMBI0 with JMBI1"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(JMBMODE_A::_32BIT)
    }
}
#[doc = "Field `JMBCLR0OFF` reader - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type JMBCLR0OFF_R = crate::BitReader<JMBCLR0OFF_A>;
#[doc = "Incoming JTAG Mailbox 0 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBCLR0OFF_A {
    #[doc = "0: JMBIN0FG cleared on read of JMB0IN register"]
    JMBCLR0OFF_0 = 0,
    #[doc = "1: JMBIN0FG cleared by software"]
    JMBCLR0OFF_1 = 1,
}
impl From<JMBCLR0OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR0OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBCLR0OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR0OFF_A {
        match self.bits {
            false => JMBCLR0OFF_A::JMBCLR0OFF_0,
            true => JMBCLR0OFF_A::JMBCLR0OFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBCLR0OFF_0`"]
    #[inline(always)]
    pub fn is_jmbclr0off_0(&self) -> bool {
        *self == JMBCLR0OFF_A::JMBCLR0OFF_0
    }
    #[doc = "Checks if the value of the field is `JMBCLR0OFF_1`"]
    #[inline(always)]
    pub fn is_jmbclr0off_1(&self) -> bool {
        *self == JMBCLR0OFF_A::JMBCLR0OFF_1
    }
}
#[doc = "Field `JMBCLR0OFF` writer - Incoming JTAG Mailbox 0 flag auto-clear disable"]
pub type JMBCLR0OFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBCLR0OFF_A, O>;
impl<'a, const O: u8> JMBCLR0OFF_W<'a, O> {
    #[doc = "JMBIN0FG cleared on read of JMB0IN register"]
    #[inline(always)]
    pub fn jmbclr0off_0(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::JMBCLR0OFF_0)
    }
    #[doc = "JMBIN0FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr0off_1(self) -> &'a mut W {
        self.variant(JMBCLR0OFF_A::JMBCLR0OFF_1)
    }
}
#[doc = "Field `JMBCLR1OFF` reader - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type JMBCLR1OFF_R = crate::BitReader<JMBCLR1OFF_A>;
#[doc = "Incoming JTAG Mailbox 1 flag auto-clear disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JMBCLR1OFF_A {
    #[doc = "0: JMBIN1FG cleared on read of JMB1IN register"]
    JMBCLR1OFF_0 = 0,
    #[doc = "1: JMBIN1FG cleared by software"]
    JMBCLR1OFF_1 = 1,
}
impl From<JMBCLR1OFF_A> for bool {
    #[inline(always)]
    fn from(variant: JMBCLR1OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl JMBCLR1OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBCLR1OFF_A {
        match self.bits {
            false => JMBCLR1OFF_A::JMBCLR1OFF_0,
            true => JMBCLR1OFF_A::JMBCLR1OFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBCLR1OFF_0`"]
    #[inline(always)]
    pub fn is_jmbclr1off_0(&self) -> bool {
        *self == JMBCLR1OFF_A::JMBCLR1OFF_0
    }
    #[doc = "Checks if the value of the field is `JMBCLR1OFF_1`"]
    #[inline(always)]
    pub fn is_jmbclr1off_1(&self) -> bool {
        *self == JMBCLR1OFF_A::JMBCLR1OFF_1
    }
}
#[doc = "Field `JMBCLR1OFF` writer - Incoming JTAG Mailbox 1 flag auto-clear disable"]
pub type JMBCLR1OFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, JMBCLR1OFF_A, O>;
impl<'a, const O: u8> JMBCLR1OFF_W<'a, O> {
    #[doc = "JMBIN1FG cleared on read of JMB1IN register"]
    #[inline(always)]
    pub fn jmbclr1off_0(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::JMBCLR1OFF_0)
    }
    #[doc = "JMBIN1FG cleared by software"]
    #[inline(always)]
    pub fn jmbclr1off_1(self) -> &'a mut W {
        self.variant(JMBCLR1OFF_A::JMBCLR1OFF_1)
    }
}
impl R {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Outgoing JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Outgoing JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incoming JTAG Mailbox 0 flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W<0> {
        JMBIN0FG_W::new(self)
    }
    #[doc = "Bit 1 - Incoming JTAG Mailbox 1 flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W<1> {
        JMBIN1FG_W::new(self)
    }
    #[doc = "Bit 4 - Operation mode of JMB"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JMBMODE_W<4> {
        JMBMODE_W::new(self)
    }
    #[doc = "Bit 6 - Incoming JTAG Mailbox 0 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W<6> {
        JMBCLR0OFF_W::new(self)
    }
    #[doc = "Bit 7 - Incoming JTAG Mailbox 1 flag auto-clear disable"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W<7> {
        JMBCLR1OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG Mailbox Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbc](index.html) module"]
pub struct SYSJMBC_SPEC;
impl crate::RegisterSpec for SYSJMBC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbc::R](R) reader structure"]
impl crate::Readable for SYSJMBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](W) writer structure"]
impl crate::Writable for SYSJMBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SYSJMBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
