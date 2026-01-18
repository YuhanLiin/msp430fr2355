#[doc = "Register `CSCTL8` reader"]
pub type R = crate::R<Csctl8Spec>;
#[doc = "Register `CSCTL8` writer"]
pub type W = crate::W<Csctl8Spec>;
#[doc = "ACLK clock request enable. Setting this enables conditional module requests for ACLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aclkreqen {
    #[doc = "0: ACLK conditional requests are disabled."]
    Aclkreqen0 = 0,
    #[doc = "1: ACLK conditional requests are enabled."]
    Aclkreqen1 = 1,
}
impl From<Aclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Aclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLKREQEN` reader - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
pub type AclkreqenR = crate::BitReader<Aclkreqen>;
impl AclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aclkreqen {
        match self.bits {
            false => Aclkreqen::Aclkreqen0,
            true => Aclkreqen::Aclkreqen1,
        }
    }
    #[doc = "ACLK conditional requests are disabled."]
    #[inline(always)]
    pub fn is_aclkreqen_0(&self) -> bool {
        *self == Aclkreqen::Aclkreqen0
    }
    #[doc = "ACLK conditional requests are enabled."]
    #[inline(always)]
    pub fn is_aclkreqen_1(&self) -> bool {
        *self == Aclkreqen::Aclkreqen1
    }
}
#[doc = "Field `ACLKREQEN` writer - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
pub type AclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Aclkreqen>;
impl<'a, REG> AclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACLK conditional requests are disabled."]
    #[inline(always)]
    pub fn aclkreqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aclkreqen::Aclkreqen0)
    }
    #[doc = "ACLK conditional requests are enabled."]
    #[inline(always)]
    pub fn aclkreqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aclkreqen::Aclkreqen1)
    }
}
#[doc = "MCLK clock request enable. Setting this enables conditional module requests for MCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkreqen {
    #[doc = "0: MCLK conditional requests are disabled."]
    Mclkreqen0 = 0,
    #[doc = "1: MCLK conditional requests are enabled."]
    Mclkreqen1 = 1,
}
impl From<Mclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Mclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKREQEN` reader - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
pub type MclkreqenR = crate::BitReader<Mclkreqen>;
impl MclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkreqen {
        match self.bits {
            false => Mclkreqen::Mclkreqen0,
            true => Mclkreqen::Mclkreqen1,
        }
    }
    #[doc = "MCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn is_mclkreqen_0(&self) -> bool {
        *self == Mclkreqen::Mclkreqen0
    }
    #[doc = "MCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn is_mclkreqen_1(&self) -> bool {
        *self == Mclkreqen::Mclkreqen1
    }
}
#[doc = "Field `MCLKREQEN` writer - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
pub type MclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Mclkreqen>;
impl<'a, REG> MclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn mclkreqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkreqen::Mclkreqen0)
    }
    #[doc = "MCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn mclkreqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkreqen::Mclkreqen1)
    }
}
#[doc = "SMCLK clock request enable. Setting this enables conditional module requests for SMCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smclkreqen {
    #[doc = "0: SMCLK conditional requests are disabled."]
    Smclkreqen0 = 0,
    #[doc = "1: SMCLK conditional requests are enabled."]
    Smclkreqen1 = 1,
}
impl From<Smclkreqen> for bool {
    #[inline(always)]
    fn from(variant: Smclkreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLKREQEN` reader - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
pub type SmclkreqenR = crate::BitReader<Smclkreqen>;
impl SmclkreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smclkreqen {
        match self.bits {
            false => Smclkreqen::Smclkreqen0,
            true => Smclkreqen::Smclkreqen1,
        }
    }
    #[doc = "SMCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn is_smclkreqen_0(&self) -> bool {
        *self == Smclkreqen::Smclkreqen0
    }
    #[doc = "SMCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn is_smclkreqen_1(&self) -> bool {
        *self == Smclkreqen::Smclkreqen1
    }
}
#[doc = "Field `SMCLKREQEN` writer - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
pub type SmclkreqenW<'a, REG> = crate::BitWriter<'a, REG, Smclkreqen>;
impl<'a, REG> SmclkreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK conditional requests are disabled."]
    #[inline(always)]
    pub fn smclkreqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkreqen::Smclkreqen0)
    }
    #[doc = "SMCLK conditional requests are enabled."]
    #[inline(always)]
    pub fn smclkreqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Smclkreqen::Smclkreqen1)
    }
}
#[doc = "MODOSC clock request enable. Setting this enables conditional module requests for MODOSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modoscreqen {
    #[doc = "0: MODOSC conditional requests are disabled."]
    Modoscreqen0 = 0,
    #[doc = "1: MODOSC conditional requests are enabled."]
    Modoscreqen1 = 1,
}
impl From<Modoscreqen> for bool {
    #[inline(always)]
    fn from(variant: Modoscreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODOSCREQEN` reader - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
pub type ModoscreqenR = crate::BitReader<Modoscreqen>;
impl ModoscreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modoscreqen {
        match self.bits {
            false => Modoscreqen::Modoscreqen0,
            true => Modoscreqen::Modoscreqen1,
        }
    }
    #[doc = "MODOSC conditional requests are disabled."]
    #[inline(always)]
    pub fn is_modoscreqen_0(&self) -> bool {
        *self == Modoscreqen::Modoscreqen0
    }
    #[doc = "MODOSC conditional requests are enabled."]
    #[inline(always)]
    pub fn is_modoscreqen_1(&self) -> bool {
        *self == Modoscreqen::Modoscreqen1
    }
}
#[doc = "Field `MODOSCREQEN` writer - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
pub type ModoscreqenW<'a, REG> = crate::BitWriter<'a, REG, Modoscreqen>;
impl<'a, REG> ModoscreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODOSC conditional requests are disabled."]
    #[inline(always)]
    pub fn modoscreqen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Modoscreqen::Modoscreqen0)
    }
    #[doc = "MODOSC conditional requests are enabled."]
    #[inline(always)]
    pub fn modoscreqen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Modoscreqen::Modoscreqen1)
    }
}
impl R {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> AclkreqenR {
        AclkreqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MclkreqenR {
        MclkreqenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SmclkreqenR {
        SmclkreqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&self) -> ModoscreqenR {
        ModoscreqenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK clock request enable. Setting this enables conditional module requests for ACLK"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> AclkreqenW<'_, Csctl8Spec> {
        AclkreqenW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK clock request enable. Setting this enables conditional module requests for MCLK"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MclkreqenW<'_, Csctl8Spec> {
        MclkreqenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMCLK clock request enable. Setting this enables conditional module requests for SMCLK"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SmclkreqenW<'_, Csctl8Spec> {
        SmclkreqenW::new(self, 2)
    }
    #[doc = "Bit 3 - MODOSC clock request enable. Setting this enables conditional module requests for MODOSC."]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> ModoscreqenW<'_, Csctl8Spec> {
        ModoscreqenW::new(self, 3)
    }
}
#[doc = "Clock System Control Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl8Spec;
impl crate::RegisterSpec for Csctl8Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl8::R`](R) reader structure"]
impl crate::Readable for Csctl8Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl8::W`](W) writer structure"]
impl crate::Writable for Csctl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL8 to value 0"]
impl crate::Resettable for Csctl8Spec {}
