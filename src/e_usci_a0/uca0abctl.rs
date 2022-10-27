#[doc = "Register `UCA0ABCTL` reader"]
pub struct R(crate::R<UCA0ABCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0ABCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0ABCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0ABCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0ABCTL` writer"]
pub struct W(crate::W<UCA0ABCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0ABCTL_SPEC>;
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
impl From<crate::W<UCA0ABCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0ABCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCABDEN` reader - Automatic baud-rate detect enable"]
pub type UCABDEN_R = crate::BitReader<UCABDEN_A>;
#[doc = "Automatic baud-rate detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCABDEN_A {
    #[doc = "0: Baud-rate detection disabled. Length of break and synch field is not measured."]
    UCABDEN_0 = 0,
    #[doc = "1: Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    UCABDEN_1 = 1,
}
impl From<UCABDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCABDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UCABDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCABDEN_A {
        match self.bits {
            false => UCABDEN_A::UCABDEN_0,
            true => UCABDEN_A::UCABDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCABDEN_0`"]
    #[inline(always)]
    pub fn is_ucabden_0(&self) -> bool {
        *self == UCABDEN_A::UCABDEN_0
    }
    #[doc = "Checks if the value of the field is `UCABDEN_1`"]
    #[inline(always)]
    pub fn is_ucabden_1(&self) -> bool {
        *self == UCABDEN_A::UCABDEN_1
    }
}
#[doc = "Field `UCABDEN` writer - Automatic baud-rate detect enable"]
pub type UCABDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0ABCTL_SPEC, UCABDEN_A, O>;
impl<'a, const O: u8> UCABDEN_W<'a, O> {
    #[doc = "Baud-rate detection disabled. Length of break and synch field is not measured."]
    #[inline(always)]
    pub fn ucabden_0(self) -> &'a mut W {
        self.variant(UCABDEN_A::UCABDEN_0)
    }
    #[doc = "Baud-rate detection enabled. Length of break and synch field is measured and baud-rate settings are changed accordingly."]
    #[inline(always)]
    pub fn ucabden_1(self) -> &'a mut W {
        self.variant(UCABDEN_A::UCABDEN_1)
    }
}
#[doc = "Field `UCBTOE` reader - Break time out error"]
pub type UCBTOE_R = crate::BitReader<UCBTOE_A>;
#[doc = "Break time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBTOE_A {
    #[doc = "0: No error"]
    UCBTOE_0 = 0,
    #[doc = "1: Length of break field exceeded 22 bit times"]
    UCBTOE_1 = 1,
}
impl From<UCBTOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBTOE_A {
        match self.bits {
            false => UCBTOE_A::UCBTOE_0,
            true => UCBTOE_A::UCBTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBTOE_0`"]
    #[inline(always)]
    pub fn is_ucbtoe_0(&self) -> bool {
        *self == UCBTOE_A::UCBTOE_0
    }
    #[doc = "Checks if the value of the field is `UCBTOE_1`"]
    #[inline(always)]
    pub fn is_ucbtoe_1(&self) -> bool {
        *self == UCBTOE_A::UCBTOE_1
    }
}
#[doc = "Field `UCBTOE` writer - Break time out error"]
pub type UCBTOE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0ABCTL_SPEC, UCBTOE_A, O>;
impl<'a, const O: u8> UCBTOE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucbtoe_0(self) -> &'a mut W {
        self.variant(UCBTOE_A::UCBTOE_0)
    }
    #[doc = "Length of break field exceeded 22 bit times"]
    #[inline(always)]
    pub fn ucbtoe_1(self) -> &'a mut W {
        self.variant(UCBTOE_A::UCBTOE_1)
    }
}
#[doc = "Field `UCSTOE` reader - Synch field time out error"]
pub type UCSTOE_R = crate::BitReader<UCSTOE_A>;
#[doc = "Synch field time out error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTOE_A {
    #[doc = "0: No error"]
    UCSTOE_0 = 0,
    #[doc = "1: Length of synch field exceeded measurable time"]
    UCSTOE_1 = 1,
}
impl From<UCSTOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTOE_A {
        match self.bits {
            false => UCSTOE_A::UCSTOE_0,
            true => UCSTOE_A::UCSTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTOE_0`"]
    #[inline(always)]
    pub fn is_ucstoe_0(&self) -> bool {
        *self == UCSTOE_A::UCSTOE_0
    }
    #[doc = "Checks if the value of the field is `UCSTOE_1`"]
    #[inline(always)]
    pub fn is_ucstoe_1(&self) -> bool {
        *self == UCSTOE_A::UCSTOE_1
    }
}
#[doc = "Field `UCSTOE` writer - Synch field time out error"]
pub type UCSTOE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0ABCTL_SPEC, UCSTOE_A, O>;
impl<'a, const O: u8> UCSTOE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucstoe_0(self) -> &'a mut W {
        self.variant(UCSTOE_A::UCSTOE_0)
    }
    #[doc = "Length of synch field exceeded measurable time"]
    #[inline(always)]
    pub fn ucstoe_1(self) -> &'a mut W {
        self.variant(UCSTOE_A::UCSTOE_1)
    }
}
#[doc = "Field `UCDELIM` reader - Break/synch delimiter length"]
pub type UCDELIM_R = crate::FieldReader<u8, UCDELIM_A>;
#[doc = "Break/synch delimiter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCDELIM_A {
    #[doc = "0: 1 bit time"]
    UCDELIM_0 = 0,
    #[doc = "1: 2 bit times"]
    UCDELIM_1 = 1,
    #[doc = "2: 3 bit times"]
    UCDELIM_2 = 2,
    #[doc = "3: 4 bit times"]
    UCDELIM_3 = 3,
}
impl From<UCDELIM_A> for u8 {
    #[inline(always)]
    fn from(variant: UCDELIM_A) -> Self {
        variant as _
    }
}
impl UCDELIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCDELIM_A {
        match self.bits {
            0 => UCDELIM_A::UCDELIM_0,
            1 => UCDELIM_A::UCDELIM_1,
            2 => UCDELIM_A::UCDELIM_2,
            3 => UCDELIM_A::UCDELIM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCDELIM_0`"]
    #[inline(always)]
    pub fn is_ucdelim_0(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_0
    }
    #[doc = "Checks if the value of the field is `UCDELIM_1`"]
    #[inline(always)]
    pub fn is_ucdelim_1(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_1
    }
    #[doc = "Checks if the value of the field is `UCDELIM_2`"]
    #[inline(always)]
    pub fn is_ucdelim_2(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_2
    }
    #[doc = "Checks if the value of the field is `UCDELIM_3`"]
    #[inline(always)]
    pub fn is_ucdelim_3(&self) -> bool {
        *self == UCDELIM_A::UCDELIM_3
    }
}
#[doc = "Field `UCDELIM` writer - Break/synch delimiter length"]
pub type UCDELIM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCA0ABCTL_SPEC, u8, UCDELIM_A, 2, O>;
impl<'a, const O: u8> UCDELIM_W<'a, O> {
    #[doc = "1 bit time"]
    #[inline(always)]
    pub fn ucdelim_0(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_0)
    }
    #[doc = "2 bit times"]
    #[inline(always)]
    pub fn ucdelim_1(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_1)
    }
    #[doc = "3 bit times"]
    #[inline(always)]
    pub fn ucdelim_2(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_2)
    }
    #[doc = "4 bit times"]
    #[inline(always)]
    pub fn ucdelim_3(self) -> &'a mut W {
        self.variant(UCDELIM_A::UCDELIM_3)
    }
}
impl R {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&self) -> UCDELIM_R {
        UCDELIM_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud-rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W<0> {
        UCABDEN_W::new(self)
    }
    #[doc = "Bit 2 - Break time out error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W<2> {
        UCBTOE_W::new(self)
    }
    #[doc = "Bit 3 - Synch field time out error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W<3> {
        UCSTOE_W::new(self)
    }
    #[doc = "Bits 4:5 - Break/synch delimiter length"]
    #[inline(always)]
    pub fn ucdelim(&mut self) -> UCDELIM_W<4> {
        UCDELIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0abctl](index.html) module"]
pub struct UCA0ABCTL_SPEC;
impl crate::RegisterSpec for UCA0ABCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0abctl::R](R) reader structure"]
impl crate::Readable for UCA0ABCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0abctl::W](W) writer structure"]
impl crate::Writable for UCA0ABCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for UCA0ABCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
