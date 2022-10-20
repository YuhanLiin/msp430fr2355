#[doc = "Register `SFRRPCR` reader"]
pub struct R(crate::R<SFRRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRRPCR` writer"]
pub struct W(crate::W<SFRRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRRPCR_SPEC>;
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
impl From<crate::W<SFRRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSNMI` reader - NMI select"]
pub type SYSNMI_R = crate::BitReader<SYSNMI_A>;
#[doc = "NMI select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSNMI_A {
    #[doc = "0: Reset function"]
    RESET = 0,
    #[doc = "1: NMI function"]
    NMI = 1,
}
impl From<SYSNMI_A> for bool {
    #[inline(always)]
    fn from(variant: SYSNMI_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSNMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSNMI_A {
        match self.bits {
            false => SYSNMI_A::RESET,
            true => SYSNMI_A::NMI,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSNMI_A::RESET
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == SYSNMI_A::NMI
    }
}
#[doc = "Field `SYSNMI` writer - NMI select"]
pub type SYSNMI_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, SYSNMI_A, O>;
impl<'a, const O: u8> SYSNMI_W<'a, O> {
    #[doc = "Reset function"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSNMI_A::RESET)
    }
    #[doc = "NMI function"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(SYSNMI_A::NMI)
    }
}
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub type SYSNMIIES_R = crate::BitReader<SYSNMIIES_A>;
#[doc = "NMI edge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSNMIIES_A {
    #[doc = "0: NMI on rising edge"]
    RISING = 0,
    #[doc = "1: NMI on falling edge"]
    FALLING = 1,
}
impl From<SYSNMIIES_A> for bool {
    #[inline(always)]
    fn from(variant: SYSNMIIES_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSNMIIES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSNMIIES_A {
        match self.bits {
            false => SYSNMIIES_A::RISING,
            true => SYSNMIIES_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == SYSNMIIES_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == SYSNMIIES_A::FALLING
    }
}
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub type SYSNMIIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, SYSNMIIES_A, O>;
impl<'a, const O: u8> SYSNMIIES_W<'a, O> {
    #[doc = "NMI on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(SYSNMIIES_A::RISING)
    }
    #[doc = "NMI on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(SYSNMIIES_A::FALLING)
    }
}
#[doc = "Field `SYSRSTUP` reader - Reset resistor pin pullup or pulldown"]
pub type SYSRSTUP_R = crate::BitReader<SYSRSTUP_A>;
#[doc = "Reset resistor pin pullup or pulldown\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRSTUP_A {
    #[doc = "0: Pulldown is selected"]
    PULLDOWN = 0,
    #[doc = "1: Pullup is selected"]
    PULLUP = 1,
}
impl From<SYSRSTUP_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRSTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTUP_A {
        match self.bits {
            false => SYSRSTUP_A::PULLDOWN,
            true => SYSRSTUP_A::PULLUP,
        }
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == SYSRSTUP_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == SYSRSTUP_A::PULLUP
    }
}
#[doc = "Field `SYSRSTUP` writer - Reset resistor pin pullup or pulldown"]
pub type SYSRSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, SYSRSTUP_A, O>;
impl<'a, const O: u8> SYSRSTUP_W<'a, O> {
    #[doc = "Pulldown is selected"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(SYSRSTUP_A::PULLDOWN)
    }
    #[doc = "Pullup is selected"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(SYSRSTUP_A::PULLUP)
    }
}
#[doc = "Field `SYSRSTRE` reader - Reset pin resistor enable"]
pub type SYSRSTRE_R = crate::BitReader<SYSRSTRE_A>;
#[doc = "Reset pin resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRSTRE_A {
    #[doc = "0: Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    DISABLE = 0,
    #[doc = "1: Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    ENABLE = 1,
}
impl From<SYSRSTRE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRSTRE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRSTRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRSTRE_A {
        match self.bits {
            false => SYSRSTRE_A::DISABLE,
            true => SYSRSTRE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYSRSTRE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYSRSTRE_A::ENABLE
    }
}
#[doc = "Field `SYSRSTRE` writer - Reset pin resistor enable"]
pub type SYSRSTRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, SYSRSTRE_A, O>;
impl<'a, const O: u8> SYSRSTRE_W<'a, O> {
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSRSTRE_A::DISABLE)
    }
    #[doc = "Pullup or pulldown resistor at the RST/NMI pin is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSRSTRE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SYSNMI_W<0> {
        SYSNMI_W::new(self)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W<1> {
        SYSNMIIES_W::new(self)
    }
    #[doc = "Bit 2 - Reset resistor pin pullup or pulldown"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W<2> {
        SYSRSTUP_W::new(self)
    }
    #[doc = "Bit 3 - Reset pin resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W<3> {
        SYSRSTRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Pin Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrrpcr](index.html) module"]
pub struct SFRRPCR_SPEC;
impl crate::RegisterSpec for SFRRPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrrpcr::R](R) reader structure"]
impl crate::Readable for SFRRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrrpcr::W](W) writer structure"]
impl crate::Writable for SFRRPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SFRRPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
