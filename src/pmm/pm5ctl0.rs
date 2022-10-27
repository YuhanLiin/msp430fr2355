#[doc = "Register `PM5CTL0` reader"]
pub struct R(crate::R<PM5CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PM5CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PM5CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PM5CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PM5CTL0` writer"]
pub struct W(crate::W<PM5CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PM5CTL0_SPEC>;
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
impl From<crate::W<PM5CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PM5CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKLPM5` reader - LPMx.5 Lock Bit"]
pub type LOCKLPM5_R = crate::BitReader<LOCKLPM5_A>;
#[doc = "LPMx.5 Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKLPM5_A {
    #[doc = "0: LPMx.5 configuration is not locked and defaults to its reset condition."]
    LOCKLPM5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    LOCKLPM5_1 = 1,
}
impl From<LOCKLPM5_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKLPM5_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKLPM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKLPM5_A {
        match self.bits {
            false => LOCKLPM5_A::LOCKLPM5_0,
            true => LOCKLPM5_A::LOCKLPM5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_0`"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_0
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_1`"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_1
    }
}
#[doc = "Field `LOCKLPM5` writer - LPMx.5 Lock Bit"]
pub type LOCKLPM5_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, LOCKLPM5_A, O>;
impl<'a, const O: u8> LOCKLPM5_W<'a, O> {
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_0)
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_1)
    }
}
#[doc = "Field `LPM5SW` reader - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
pub type LPM5SW_R = crate::BitReader<LPM5SW_A>;
#[doc = "Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM5SW_A {
    #[doc = "0: LPMx.5 switch disconnected"]
    LPM5SW_0 = 0,
    #[doc = "1: LPMx.5 switch connected"]
    LPM5SW_1 = 1,
}
impl From<LPM5SW_A> for bool {
    #[inline(always)]
    fn from(variant: LPM5SW_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM5SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM5SW_A {
        match self.bits {
            false => LPM5SW_A::LPM5SW_0,
            true => LPM5SW_A::LPM5SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM5SW_0`"]
    #[inline(always)]
    pub fn is_lpm5sw_0(&self) -> bool {
        *self == LPM5SW_A::LPM5SW_0
    }
    #[doc = "Checks if the value of the field is `LPM5SW_1`"]
    #[inline(always)]
    pub fn is_lpm5sw_1(&self) -> bool {
        *self == LPM5SW_A::LPM5SW_1
    }
}
#[doc = "Field `LPM5SW` writer - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
pub type LPM5SW_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, LPM5SW_A, O>;
impl<'a, const O: u8> LPM5SW_W<'a, O> {
    #[doc = "LPMx.5 switch disconnected"]
    #[inline(always)]
    pub fn lpm5sw_0(self) -> &'a mut W {
        self.variant(LPM5SW_A::LPM5SW_0)
    }
    #[doc = "LPMx.5 switch connected"]
    #[inline(always)]
    pub fn lpm5sw_1(self) -> &'a mut W {
        self.variant(LPM5SW_A::LPM5SW_1)
    }
}
#[doc = "Field `LPM5SM` reader - Specifies the operation mode of the LPM3.5 switch."]
pub type LPM5SM_R = crate::BitReader<LPM5SM_A>;
#[doc = "Specifies the operation mode of the LPM3.5 switch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM5SM_A {
    #[doc = "0: Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    LPM5SM_0 = 0,
    #[doc = "1: Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    LPM5SM_1 = 1,
}
impl From<LPM5SM_A> for bool {
    #[inline(always)]
    fn from(variant: LPM5SM_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM5SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM5SM_A {
        match self.bits {
            false => LPM5SM_A::LPM5SM_0,
            true => LPM5SM_A::LPM5SM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM5SM_0`"]
    #[inline(always)]
    pub fn is_lpm5sm_0(&self) -> bool {
        *self == LPM5SM_A::LPM5SM_0
    }
    #[doc = "Checks if the value of the field is `LPM5SM_1`"]
    #[inline(always)]
    pub fn is_lpm5sm_1(&self) -> bool {
        *self == LPM5SM_A::LPM5SM_1
    }
}
#[doc = "Field `LPM5SM` writer - Specifies the operation mode of the LPM3.5 switch."]
pub type LPM5SM_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, LPM5SM_A, O>;
impl<'a, const O: u8> LPM5SM_W<'a, O> {
    #[doc = "Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    #[inline(always)]
    pub fn lpm5sm_0(self) -> &'a mut W {
        self.variant(LPM5SM_A::LPM5SM_0)
    }
    #[doc = "Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    #[inline(always)]
    pub fn lpm5sm_1(self) -> &'a mut W {
        self.variant(LPM5SM_A::LPM5SM_1)
    }
}
impl R {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&self) -> LPM5SW_R {
        LPM5SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&self) -> LPM5SM_R {
        LPM5SM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W<0> {
        LOCKLPM5_W::new(self)
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> LPM5SW_W<4> {
        LPM5SW_W::new(self)
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> LPM5SM_W<5> {
        LPM5SM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power mode 5 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm5ctl0](index.html) module"]
pub struct PM5CTL0_SPEC;
impl crate::RegisterSpec for PM5CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pm5ctl0::R](R) reader structure"]
impl crate::Readable for PM5CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](W) writer structure"]
impl crate::Writable for PM5CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for PM5CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
