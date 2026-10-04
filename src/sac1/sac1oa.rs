#[doc = "Register `SAC1OA` reader"]
pub type R = crate::R<Sac1oaSpec>;
#[doc = "Register `SAC1OA` writer"]
pub type W = crate::W<Sac1oaSpec>;
#[doc = "SAC OA Positive input source selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: External source selected"]
    Psel0 = 0,
    #[doc = "1: 12-bit reference DAC source selected"]
    Psel1 = 1,
    #[doc = "2: Pair OA source selected"]
    Psel2 = 2,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - SAC OA Positive input source selection"]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Psel> {
        match self.bits {
            0 => Some(Psel::Psel0),
            1 => Some(Psel::Psel1),
            2 => Some(Psel::Psel2),
            _ => None,
        }
    }
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == Psel::Psel0
    }
    #[doc = "12-bit reference DAC source selected"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == Psel::Psel1
    }
    #[doc = "Pair OA source selected"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == Psel::Psel2
    }
}
#[doc = "Field `PSEL` writer - SAC OA Positive input source selection"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Psel>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel0)
    }
    #[doc = "12-bit reference DAC source selected"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel1)
    }
    #[doc = "Pair OA source selected"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel2)
    }
}
#[doc = "SAC Positive input MUX control.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmuxen {
    #[doc = "0: All positive input sources are disconnected to OA positive port"]
    Pmuxen0 = 0,
    #[doc = "1: All positive input sources are connected to OA positive port"]
    Pmuxen1 = 1,
}
impl From<Pmuxen> for bool {
    #[inline(always)]
    fn from(variant: Pmuxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUXEN` reader - SAC Positive input MUX control."]
pub type PmuxenR = crate::BitReader<Pmuxen>;
impl PmuxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmuxen {
        match self.bits {
            false => Pmuxen::Pmuxen0,
            true => Pmuxen::Pmuxen1,
        }
    }
    #[doc = "All positive input sources are disconnected to OA positive port"]
    #[inline(always)]
    pub fn is_pmuxen_0(&self) -> bool {
        *self == Pmuxen::Pmuxen0
    }
    #[doc = "All positive input sources are connected to OA positive port"]
    #[inline(always)]
    pub fn is_pmuxen_1(&self) -> bool {
        *self == Pmuxen::Pmuxen1
    }
}
#[doc = "Field `PMUXEN` writer - SAC Positive input MUX control."]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG, Pmuxen>;
impl<'a, REG> PmuxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All positive input sources are disconnected to OA positive port"]
    #[inline(always)]
    pub fn pmuxen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxen::Pmuxen0)
    }
    #[doc = "All positive input sources are connected to OA positive port"]
    #[inline(always)]
    pub fn pmuxen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxen::Pmuxen1)
    }
}
#[doc = "SAC OA Negative input source selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsel {
    #[doc = "0: External source selected"]
    Nsel0 = 0,
    #[doc = "1: PGA source selected"]
    Nsel1 = 1,
    #[doc = "2: Device Specific"]
    Nsel2 = 2,
}
impl From<Nsel> for u8 {
    #[inline(always)]
    fn from(variant: Nsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsel {
    type Ux = u8;
}
impl crate::IsEnum for Nsel {}
#[doc = "Field `NSEL` reader - SAC OA Negative input source selection"]
pub type NselR = crate::FieldReader<Nsel>;
impl NselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nsel> {
        match self.bits {
            0 => Some(Nsel::Nsel0),
            1 => Some(Nsel::Nsel1),
            2 => Some(Nsel::Nsel2),
            _ => None,
        }
    }
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn is_nsel_0(&self) -> bool {
        *self == Nsel::Nsel0
    }
    #[doc = "PGA source selected"]
    #[inline(always)]
    pub fn is_nsel_1(&self) -> bool {
        *self == Nsel::Nsel1
    }
    #[doc = "Device Specific"]
    #[inline(always)]
    pub fn is_nsel_2(&self) -> bool {
        *self == Nsel::Nsel2
    }
}
#[doc = "Field `NSEL` writer - SAC OA Negative input source selection"]
pub type NselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nsel>;
impl<'a, REG> NselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External source selected"]
    #[inline(always)]
    pub fn nsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Nsel0)
    }
    #[doc = "PGA source selected"]
    #[inline(always)]
    pub fn nsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Nsel1)
    }
    #[doc = "Device Specific"]
    #[inline(always)]
    pub fn nsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Nsel2)
    }
}
#[doc = "SAC Negative input MUX controL\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmuxen {
    #[doc = "0: All negative input sources are disconnected to OA negative port"]
    Nmuxen0 = 0,
    #[doc = "1: All negative input sources are connected to OA negative port"]
    Nmuxen1 = 1,
}
impl From<Nmuxen> for bool {
    #[inline(always)]
    fn from(variant: Nmuxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMUXEN` reader - SAC Negative input MUX controL"]
pub type NmuxenR = crate::BitReader<Nmuxen>;
impl NmuxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmuxen {
        match self.bits {
            false => Nmuxen::Nmuxen0,
            true => Nmuxen::Nmuxen1,
        }
    }
    #[doc = "All negative input sources are disconnected to OA negative port"]
    #[inline(always)]
    pub fn is_nmuxen_0(&self) -> bool {
        *self == Nmuxen::Nmuxen0
    }
    #[doc = "All negative input sources are connected to OA negative port"]
    #[inline(always)]
    pub fn is_nmuxen_1(&self) -> bool {
        *self == Nmuxen::Nmuxen1
    }
}
#[doc = "Field `NMUXEN` writer - SAC Negative input MUX controL"]
pub type NmuxenW<'a, REG> = crate::BitWriter<'a, REG, Nmuxen>;
impl<'a, REG> NmuxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All negative input sources are disconnected to OA negative port"]
    #[inline(always)]
    pub fn nmuxen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmuxen::Nmuxen0)
    }
    #[doc = "All negative input sources are connected to OA negative port"]
    #[inline(always)]
    pub fn nmuxen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmuxen::Nmuxen1)
    }
}
#[doc = "SAC OA Enable selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oaen {
    #[doc = "0: SAC OA is disabled, then the SAC OA output high impedance"]
    Oaen0 = 0,
    #[doc = "1: SAC OA is enabled, normal mode"]
    Oaen1 = 1,
}
impl From<Oaen> for bool {
    #[inline(always)]
    fn from(variant: Oaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAEN` reader - SAC OA Enable selection"]
pub type OaenR = crate::BitReader<Oaen>;
impl OaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oaen {
        match self.bits {
            false => Oaen::Oaen0,
            true => Oaen::Oaen1,
        }
    }
    #[doc = "SAC OA is disabled, then the SAC OA output high impedance"]
    #[inline(always)]
    pub fn is_oaen_0(&self) -> bool {
        *self == Oaen::Oaen0
    }
    #[doc = "SAC OA is enabled, normal mode"]
    #[inline(always)]
    pub fn is_oaen_1(&self) -> bool {
        *self == Oaen::Oaen1
    }
}
#[doc = "Field `OAEN` writer - SAC OA Enable selection"]
pub type OaenW<'a, REG> = crate::BitWriter<'a, REG, Oaen>;
impl<'a, REG> OaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAC OA is disabled, then the SAC OA output high impedance"]
    #[inline(always)]
    pub fn oaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oaen::Oaen0)
    }
    #[doc = "SAC OA is enabled, normal mode"]
    #[inline(always)]
    pub fn oaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oaen::Oaen1)
    }
}
#[doc = "SAC OA power mode selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oapm {
    #[doc = "0: High speed and high power"]
    Oapm0 = 0,
    #[doc = "1: Llow speed and low power"]
    Oapm1 = 1,
}
impl From<Oapm> for bool {
    #[inline(always)]
    fn from(variant: Oapm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAPM` reader - SAC OA power mode selection"]
pub type OapmR = crate::BitReader<Oapm>;
impl OapmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oapm {
        match self.bits {
            false => Oapm::Oapm0,
            true => Oapm::Oapm1,
        }
    }
    #[doc = "High speed and high power"]
    #[inline(always)]
    pub fn is_oapm_0(&self) -> bool {
        *self == Oapm::Oapm0
    }
    #[doc = "Llow speed and low power"]
    #[inline(always)]
    pub fn is_oapm_1(&self) -> bool {
        *self == Oapm::Oapm1
    }
}
#[doc = "Field `OAPM` writer - SAC OA power mode selection"]
pub type OapmW<'a, REG> = crate::BitWriter<'a, REG, Oapm>;
impl<'a, REG> OapmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed and high power"]
    #[inline(always)]
    pub fn oapm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oapm::Oapm0)
    }
    #[doc = "Llow speed and low power"]
    #[inline(always)]
    pub fn oapm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oapm::Oapm1)
    }
}
#[doc = "SAC Enable selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sacen {
    #[doc = "0: SAC all modules are disabled, then the SAC output high impedance"]
    Sacen0 = 0,
    #[doc = "1: SAC all modules are enabled, normal mode"]
    Sacen1 = 1,
}
impl From<Sacen> for bool {
    #[inline(always)]
    fn from(variant: Sacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACEN` reader - SAC Enable selection"]
pub type SacenR = crate::BitReader<Sacen>;
impl SacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sacen {
        match self.bits {
            false => Sacen::Sacen0,
            true => Sacen::Sacen1,
        }
    }
    #[doc = "SAC all modules are disabled, then the SAC output high impedance"]
    #[inline(always)]
    pub fn is_sacen_0(&self) -> bool {
        *self == Sacen::Sacen0
    }
    #[doc = "SAC all modules are enabled, normal mode"]
    #[inline(always)]
    pub fn is_sacen_1(&self) -> bool {
        *self == Sacen::Sacen1
    }
}
#[doc = "Field `SACEN` writer - SAC Enable selection"]
pub type SacenW<'a, REG> = crate::BitWriter<'a, REG, Sacen>;
impl<'a, REG> SacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAC all modules are disabled, then the SAC output high impedance"]
    #[inline(always)]
    pub fn sacen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sacen::Sacen0)
    }
    #[doc = "SAC all modules are enabled, normal mode"]
    #[inline(always)]
    pub fn sacen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sacen::Sacen1)
    }
}
impl R {
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&self) -> PmuxenR {
        PmuxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&self) -> NselR {
        NselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&self) -> NmuxenR {
        NmuxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&self) -> OaenR {
        OaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&self) -> OapmR {
        OapmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&self) -> SacenR {
        SacenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC OA Positive input source selection"]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, Sac1oaSpec> {
        PselW::new(self, 0)
    }
    #[doc = "Bit 3 - SAC Positive input MUX control."]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PmuxenW<'_, Sac1oaSpec> {
        PmuxenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - SAC OA Negative input source selection"]
    #[inline(always)]
    pub fn nsel(&mut self) -> NselW<'_, Sac1oaSpec> {
        NselW::new(self, 4)
    }
    #[doc = "Bit 7 - SAC Negative input MUX controL"]
    #[inline(always)]
    pub fn nmuxen(&mut self) -> NmuxenW<'_, Sac1oaSpec> {
        NmuxenW::new(self, 7)
    }
    #[doc = "Bit 8 - SAC OA Enable selection"]
    #[inline(always)]
    pub fn oaen(&mut self) -> OaenW<'_, Sac1oaSpec> {
        OaenW::new(self, 8)
    }
    #[doc = "Bit 9 - SAC OA power mode selection"]
    #[inline(always)]
    pub fn oapm(&mut self) -> OapmW<'_, Sac1oaSpec> {
        OapmW::new(self, 9)
    }
    #[doc = "Bit 10 - SAC Enable selection"]
    #[inline(always)]
    pub fn sacen(&mut self) -> SacenW<'_, Sac1oaSpec> {
        SacenW::new(self, 10)
    }
}
#[doc = "SAC OA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1oa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1oa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac1oaSpec;
impl crate::RegisterSpec for Sac1oaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac1oa::R`](R) reader structure"]
impl crate::Readable for Sac1oaSpec {}
#[doc = "`write(|w| ..)` method takes [`sac1oa::W`](W) writer structure"]
impl crate::Writable for Sac1oaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC1OA to value 0"]
impl crate::Resettable for Sac1oaSpec {}
