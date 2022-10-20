#[doc = "Register `SYSCTL` reader"]
pub struct R(crate::R<SYSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTL` writer"]
pub struct W(crate::W<SYSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTL_SPEC>;
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
impl From<crate::W<SYSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRIVECT` reader - RAM-based interrupt vectors"]
pub type SYSRIVECT_R = crate::BitReader<SYSRIVECT_A>;
#[doc = "RAM-based interrupt vectors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRIVECT_A {
    #[doc = "0: Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh"]
    FRAM = 0,
    #[doc = "1: Interrupt vectors generated with end address TOP of RAM, when RAM available"]
    RAM = 1,
}
impl From<SYSRIVECT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRIVECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRIVECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRIVECT_A {
        match self.bits {
            false => SYSRIVECT_A::FRAM,
            true => SYSRIVECT_A::RAM,
        }
    }
    #[doc = "Checks if the value of the field is `FRAM`"]
    #[inline(always)]
    pub fn is_fram(&self) -> bool {
        *self == SYSRIVECT_A::FRAM
    }
    #[doc = "Checks if the value of the field is `RAM`"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == SYSRIVECT_A::RAM
    }
}
#[doc = "Field `SYSRIVECT` writer - RAM-based interrupt vectors"]
pub type SYSRIVECT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCTL_SPEC, SYSRIVECT_A, O>;
impl<'a, const O: u8> SYSRIVECT_W<'a, O> {
    #[doc = "Interrupt vectors generated with end address TOP of lower 64K FRAM FFFFh"]
    #[inline(always)]
    pub fn fram(self) -> &'a mut W {
        self.variant(SYSRIVECT_A::FRAM)
    }
    #[doc = "Interrupt vectors generated with end address TOP of RAM, when RAM available"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut W {
        self.variant(SYSRIVECT_A::RAM)
    }
}
#[doc = "Field `SYSPMMPE` reader - PMM access protect"]
pub type SYSPMMPE_R = crate::BitReader<SYSPMMPE_A>;
#[doc = "PMM access protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPMMPE_A {
    #[doc = "0: Access from anywhere in memory"]
    SYSPMMPE_0 = 0,
    #[doc = "1: Access only from the BSL segments"]
    SYSPMMPE_1 = 1,
}
impl From<SYSPMMPE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPMMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSPMMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPMMPE_A {
        match self.bits {
            false => SYSPMMPE_A::SYSPMMPE_0,
            true => SYSPMMPE_A::SYSPMMPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSPMMPE_0`"]
    #[inline(always)]
    pub fn is_syspmmpe_0(&self) -> bool {
        *self == SYSPMMPE_A::SYSPMMPE_0
    }
    #[doc = "Checks if the value of the field is `SYSPMMPE_1`"]
    #[inline(always)]
    pub fn is_syspmmpe_1(&self) -> bool {
        *self == SYSPMMPE_A::SYSPMMPE_1
    }
}
#[doc = "Field `SYSPMMPE` writer - PMM access protect"]
pub type SYSPMMPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCTL_SPEC, SYSPMMPE_A, O>;
impl<'a, const O: u8> SYSPMMPE_W<'a, O> {
    #[doc = "Access from anywhere in memory"]
    #[inline(always)]
    pub fn syspmmpe_0(self) -> &'a mut W {
        self.variant(SYSPMMPE_A::SYSPMMPE_0)
    }
    #[doc = "Access only from the BSL segments"]
    #[inline(always)]
    pub fn syspmmpe_1(self) -> &'a mut W {
        self.variant(SYSPMMPE_A::SYSPMMPE_1)
    }
}
#[doc = "Field `SYSBSLIND` reader - BSL entry indication"]
pub type SYSBSLIND_R = crate::BitReader<SYSBSLIND_A>;
#[doc = "BSL entry indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSBSLIND_A {
    #[doc = "0: No BSL entry sequence detected"]
    SYSBSLIND_0 = 0,
    #[doc = "1: BSL entry sequence detected"]
    SYSBSLIND_1 = 1,
}
impl From<SYSBSLIND_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLIND_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSBSLIND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLIND_A {
        match self.bits {
            false => SYSBSLIND_A::SYSBSLIND_0,
            true => SYSBSLIND_A::SYSBSLIND_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSBSLIND_0`"]
    #[inline(always)]
    pub fn is_sysbslind_0(&self) -> bool {
        *self == SYSBSLIND_A::SYSBSLIND_0
    }
    #[doc = "Checks if the value of the field is `SYSBSLIND_1`"]
    #[inline(always)]
    pub fn is_sysbslind_1(&self) -> bool {
        *self == SYSBSLIND_A::SYSBSLIND_1
    }
}
#[doc = "Field `SYSJTAGPIN` reader - Dedicated JTAG pins enable"]
pub type SYSJTAGPIN_R = crate::BitReader<SYSJTAGPIN_A>;
#[doc = "Dedicated JTAG pins enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSJTAGPIN_A {
    #[doc = "0: Shared JTAG pins (JTAG mode selectable using SBW sequence)"]
    SHARED = 0,
    #[doc = "1: Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    DEDICATED = 1,
}
impl From<SYSJTAGPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSJTAGPIN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSJTAGPIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSJTAGPIN_A {
        match self.bits {
            false => SYSJTAGPIN_A::SHARED,
            true => SYSJTAGPIN_A::DEDICATED,
        }
    }
    #[doc = "Checks if the value of the field is `SHARED`"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == SYSJTAGPIN_A::SHARED
    }
    #[doc = "Checks if the value of the field is `DEDICATED`"]
    #[inline(always)]
    pub fn is_dedicated(&self) -> bool {
        *self == SYSJTAGPIN_A::DEDICATED
    }
}
#[doc = "Field `SYSJTAGPIN` writer - Dedicated JTAG pins enable"]
pub type SYSJTAGPIN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCTL_SPEC, SYSJTAGPIN_A, O>;
impl<'a, const O: u8> SYSJTAGPIN_W<'a, O> {
    #[doc = "Shared JTAG pins (JTAG mode selectable using SBW sequence)"]
    #[inline(always)]
    pub fn shared(self) -> &'a mut W {
        self.variant(SYSJTAGPIN_A::SHARED)
    }
    #[doc = "Dedicated JTAG pins (explicit 4-wire JTAG mode selection)"]
    #[inline(always)]
    pub fn dedicated(self) -> &'a mut W {
        self.variant(SYSJTAGPIN_A::DEDICATED)
    }
}
impl R {
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - BSL entry indication"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM-based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W<0> {
        SYSRIVECT_W::new(self)
    }
    #[doc = "Bit 2 - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W<2> {
        SYSPMMPE_W::new(self)
    }
    #[doc = "Bit 5 - Dedicated JTAG pins enable"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W<5> {
        SYSJTAGPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](index.html) module"]
pub struct SYSCTL_SPEC;
impl crate::RegisterSpec for SYSCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysctl::R](R) reader structure"]
impl crate::Readable for SYSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctl::W](W) writer structure"]
impl crate::Writable for SYSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SYSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
