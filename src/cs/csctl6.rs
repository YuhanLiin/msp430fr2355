#[doc = "Register `CSCTL6` reader"]
pub type R = crate::R<Csctl6Spec>;
#[doc = "Register `CSCTL6` writer"]
pub type W = crate::W<Csctl6Spec>;
#[doc = "XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xt1autooff {
    #[doc = "0: XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation."]
    Xt1autooff0 = 0,
    #[doc = "1: XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation."]
    Xt1autooff1 = 1,
}
impl From<Xt1autooff> for bool {
    #[inline(always)]
    fn from(variant: Xt1autooff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT1AUTOOFF` reader - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
pub type Xt1autooffR = crate::BitReader<Xt1autooff>;
impl Xt1autooffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1autooff {
        match self.bits {
            false => Xt1autooff::Xt1autooff0,
            true => Xt1autooff::Xt1autooff1,
        }
    }
    #[doc = "XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation."]
    #[inline(always)]
    pub fn is_xt1autooff_0(&self) -> bool {
        *self == Xt1autooff::Xt1autooff0
    }
    #[doc = "XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation."]
    #[inline(always)]
    pub fn is_xt1autooff_1(&self) -> bool {
        *self == Xt1autooff::Xt1autooff1
    }
}
#[doc = "Field `XT1AUTOOFF` writer - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
pub type Xt1autooffW<'a, REG> = crate::BitWriter<'a, REG, Xt1autooff>;
impl<'a, REG> Xt1autooffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XT1 is on if XT1 is selected by the port selection and XT1 is not in bypass mode of operation."]
    #[inline(always)]
    pub fn xt1autooff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1autooff::Xt1autooff0)
    }
    #[doc = "XT1 is off if it is not used as a source for ACLK, MCLK, or SMCLK or is not used as a reference source required for FLL operation."]
    #[inline(always)]
    pub fn xt1autooff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1autooff::Xt1autooff1)
    }
}
#[doc = "Automatic Gain Control (AGC) disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xt1agcoff {
    #[doc = "0: AGC on"]
    Xt1agcoff0 = 0,
    #[doc = "1: AGC off"]
    Xt1agcoff1 = 1,
}
impl From<Xt1agcoff> for bool {
    #[inline(always)]
    fn from(variant: Xt1agcoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT1AGCOFF` reader - Automatic Gain Control (AGC) disable."]
pub type Xt1agcoffR = crate::BitReader<Xt1agcoff>;
impl Xt1agcoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1agcoff {
        match self.bits {
            false => Xt1agcoff::Xt1agcoff0,
            true => Xt1agcoff::Xt1agcoff1,
        }
    }
    #[doc = "AGC on"]
    #[inline(always)]
    pub fn is_xt1agcoff_0(&self) -> bool {
        *self == Xt1agcoff::Xt1agcoff0
    }
    #[doc = "AGC off"]
    #[inline(always)]
    pub fn is_xt1agcoff_1(&self) -> bool {
        *self == Xt1agcoff::Xt1agcoff1
    }
}
#[doc = "Field `XT1AGCOFF` writer - Automatic Gain Control (AGC) disable."]
pub type Xt1agcoffW<'a, REG> = crate::BitWriter<'a, REG, Xt1agcoff>;
impl<'a, REG> Xt1agcoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGC on"]
    #[inline(always)]
    pub fn xt1agcoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1agcoff::Xt1agcoff0)
    }
    #[doc = "AGC off"]
    #[inline(always)]
    pub fn xt1agcoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1agcoff::Xt1agcoff1)
    }
}
#[doc = "The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xt1hffreq {
    #[doc = "0: 1 to 4 MHz"]
    Xt1hffreq0 = 0,
    #[doc = "1: 4 MHz to 6 MHz"]
    Xt1hffreq1 = 1,
    #[doc = "2: 6 MHz to 16 MHz"]
    Xt1hffreq2 = 2,
    #[doc = "3: 16 MHz to 24 MHz"]
    Xt1hffreq3 = 3,
}
impl From<Xt1hffreq> for u8 {
    #[inline(always)]
    fn from(variant: Xt1hffreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xt1hffreq {
    type Ux = u8;
}
impl crate::IsEnum for Xt1hffreq {}
#[doc = "Field `XT1HFFREQ` reader - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
pub type Xt1hffreqR = crate::FieldReader<Xt1hffreq>;
impl Xt1hffreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1hffreq {
        match self.bits {
            0 => Xt1hffreq::Xt1hffreq0,
            1 => Xt1hffreq::Xt1hffreq1,
            2 => Xt1hffreq::Xt1hffreq2,
            3 => Xt1hffreq::Xt1hffreq3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 to 4 MHz"]
    #[inline(always)]
    pub fn is_xt1hffreq_0(&self) -> bool {
        *self == Xt1hffreq::Xt1hffreq0
    }
    #[doc = "4 MHz to 6 MHz"]
    #[inline(always)]
    pub fn is_xt1hffreq_1(&self) -> bool {
        *self == Xt1hffreq::Xt1hffreq1
    }
    #[doc = "6 MHz to 16 MHz"]
    #[inline(always)]
    pub fn is_xt1hffreq_2(&self) -> bool {
        *self == Xt1hffreq::Xt1hffreq2
    }
    #[doc = "16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn is_xt1hffreq_3(&self) -> bool {
        *self == Xt1hffreq::Xt1hffreq3
    }
}
#[doc = "Field `XT1HFFREQ` writer - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
pub type Xt1hffreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xt1hffreq, crate::Safe>;
impl<'a, REG> Xt1hffreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 to 4 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1hffreq::Xt1hffreq0)
    }
    #[doc = "4 MHz to 6 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1hffreq::Xt1hffreq1)
    }
    #[doc = "6 MHz to 16 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1hffreq::Xt1hffreq2)
    }
    #[doc = "16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn xt1hffreq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1hffreq::Xt1hffreq3)
    }
}
#[doc = "XT1 bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xt1bypass {
    #[doc = "0: XT1 source internally"]
    Xt1bypass0 = 0,
    #[doc = "1: XT1 sources externally from pin"]
    Xt1bypass1 = 1,
}
impl From<Xt1bypass> for bool {
    #[inline(always)]
    fn from(variant: Xt1bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT1BYPASS` reader - XT1 bypass select"]
pub type Xt1bypassR = crate::BitReader<Xt1bypass>;
impl Xt1bypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1bypass {
        match self.bits {
            false => Xt1bypass::Xt1bypass0,
            true => Xt1bypass::Xt1bypass1,
        }
    }
    #[doc = "XT1 source internally"]
    #[inline(always)]
    pub fn is_xt1bypass_0(&self) -> bool {
        *self == Xt1bypass::Xt1bypass0
    }
    #[doc = "XT1 sources externally from pin"]
    #[inline(always)]
    pub fn is_xt1bypass_1(&self) -> bool {
        *self == Xt1bypass::Xt1bypass1
    }
}
#[doc = "Field `XT1BYPASS` writer - XT1 bypass select"]
pub type Xt1bypassW<'a, REG> = crate::BitWriter<'a, REG, Xt1bypass>;
impl<'a, REG> Xt1bypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XT1 source internally"]
    #[inline(always)]
    pub fn xt1bypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1bypass::Xt1bypass0)
    }
    #[doc = "XT1 sources externally from pin"]
    #[inline(always)]
    pub fn xt1bypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1bypass::Xt1bypass1)
    }
}
#[doc = "XT1 mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xts {
    #[doc = "0: Low-frequency mode."]
    Xts0 = 0,
    #[doc = "1: High-frequency mode."]
    Xts1 = 1,
}
impl From<Xts> for bool {
    #[inline(always)]
    fn from(variant: Xts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTS` reader - XT1 mode select"]
pub type XtsR = crate::BitReader<Xts>;
impl XtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xts {
        match self.bits {
            false => Xts::Xts0,
            true => Xts::Xts1,
        }
    }
    #[doc = "Low-frequency mode."]
    #[inline(always)]
    pub fn is_xts_0(&self) -> bool {
        *self == Xts::Xts0
    }
    #[doc = "High-frequency mode."]
    #[inline(always)]
    pub fn is_xts_1(&self) -> bool {
        *self == Xts::Xts1
    }
}
#[doc = "Field `XTS` writer - XT1 mode select"]
pub type XtsW<'a, REG> = crate::BitWriter<'a, REG, Xts>;
impl<'a, REG> XtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-frequency mode."]
    #[inline(always)]
    pub fn xts_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xts::Xts0)
    }
    #[doc = "High-frequency mode."]
    #[inline(always)]
    pub fn xts_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xts::Xts1)
    }
}
#[doc = "The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xt1drive {
    #[doc = "0: Lowest drive strength and current consumption"]
    Xt1drive0 = 0,
    #[doc = "1: Lower drive strength and current consumption"]
    Xt1drive1 = 1,
    #[doc = "2: Higher drive strength and current consumption"]
    Xt1drive2 = 2,
    #[doc = "3: Highest drive strength and current consumption"]
    Xt1drive3 = 3,
}
impl From<Xt1drive> for u8 {
    #[inline(always)]
    fn from(variant: Xt1drive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xt1drive {
    type Ux = u8;
}
impl crate::IsEnum for Xt1drive {}
#[doc = "Field `XT1DRIVE` reader - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
pub type Xt1driveR = crate::FieldReader<Xt1drive>;
impl Xt1driveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1drive {
        match self.bits {
            0 => Xt1drive::Xt1drive0,
            1 => Xt1drive::Xt1drive1,
            2 => Xt1drive::Xt1drive2,
            3 => Xt1drive::Xt1drive3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive strength and current consumption"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == Xt1drive::Xt1drive0
    }
    #[doc = "Lower drive strength and current consumption"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == Xt1drive::Xt1drive1
    }
    #[doc = "Higher drive strength and current consumption"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == Xt1drive::Xt1drive2
    }
    #[doc = "Highest drive strength and current consumption"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == Xt1drive::Xt1drive3
    }
}
#[doc = "Field `XT1DRIVE` writer - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
pub type Xt1driveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xt1drive, crate::Safe>;
impl<'a, REG> Xt1driveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive0)
    }
    #[doc = "Lower drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive1)
    }
    #[doc = "Higher drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive2)
    }
    #[doc = "Highest drive strength and current consumption"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1drive::Xt1drive3)
    }
}
#[doc = "ACLK source divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /16"]
    _16 = 1,
    #[doc = "2: /32"]
    _32 = 2,
    #[doc = "3: /64"]
    _64 = 3,
    #[doc = "4: /128"]
    _128 = 4,
    #[doc = "5: /256"]
    _256 = 5,
    #[doc = "6: /384"]
    _384 = 6,
    #[doc = "7: /512"]
    _512 = 7,
    #[doc = "8: /768(Only available in 24MHz clock system, 24 MHz preference)"]
    _768 = 8,
    #[doc = "9: /1024(Only available in 24MHz clock system, 24 MHz preference)"]
    _1024 = 9,
    #[doc = "10: /108(Only available in 24MHz clock system, 24 MHz preference)"]
    _108 = 10,
    #[doc = "11: 338(Only available in 24MHz clock system, 24 MHz preference)"]
    _338 = 11,
    #[doc = "12: 414(Only available in 24MHz clock system, 24 MHz preference)"]
    _414 = 12,
    #[doc = "13: 640(Only available in 24MHz clock system, 24 MHz preference)"]
    _640 = 13,
    #[doc = "14: Reserved"]
    Diva14 = 14,
    #[doc = "15: Reserved"]
    Diva15 = 15,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - ACLK source divider."]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diva {
        match self.bits {
            0 => Diva::_1,
            1 => Diva::_16,
            2 => Diva::_32,
            3 => Diva::_64,
            4 => Diva::_128,
            5 => Diva::_256,
            6 => Diva::_384,
            7 => Diva::_512,
            8 => Diva::_768,
            9 => Diva::_1024,
            10 => Diva::_108,
            11 => Diva::_338,
            12 => Diva::_414,
            13 => Diva::_640,
            14 => Diva::Diva14,
            15 => Diva::Diva15,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Diva::_1
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Diva::_16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Diva::_32
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Diva::_64
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Diva::_128
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Diva::_256
    }
    #[doc = "/384"]
    #[inline(always)]
    pub fn is_384(&self) -> bool {
        *self == Diva::_384
    }
    #[doc = "/512"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Diva::_512
    }
    #[doc = "/768(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == Diva::_768
    }
    #[doc = "/1024(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Diva::_1024
    }
    #[doc = "/108(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_108(&self) -> bool {
        *self == Diva::_108
    }
    #[doc = "338(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_338(&self) -> bool {
        *self == Diva::_338
    }
    #[doc = "414(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_414(&self) -> bool {
        *self == Diva::_414
    }
    #[doc = "640(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == Diva::_640
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_diva_14(&self) -> bool {
        *self == Diva::Diva14
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_diva_15(&self) -> bool {
        *self == Diva::Diva15
    }
}
#[doc = "Field `DIVA` writer - ACLK source divider."]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 4, Diva, crate::Safe>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_1)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_128)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_256)
    }
    #[doc = "/384"]
    #[inline(always)]
    pub fn _384(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_384)
    }
    #[doc = "/512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_512)
    }
    #[doc = "/768(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_768)
    }
    #[doc = "/1024(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_1024)
    }
    #[doc = "/108(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _108(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_108)
    }
    #[doc = "338(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _338(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_338)
    }
    #[doc = "414(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _414(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_414)
    }
    #[doc = "640(Only available in 24MHz clock system, 24 MHz preference)"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::_640)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn diva_14(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn diva_15(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva15)
    }
}
#[doc = "The XT1 oscillator fault detection off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xt1faultoff {
    #[doc = "0: Enabling XT1 fault to switch ACLK to REFO"]
    Xt1faultoff0 = 0,
    #[doc = "1: Disabling XT1 fault to switch ACLK to REFO"]
    Xt1faultoff1 = 1,
}
impl From<Xt1faultoff> for bool {
    #[inline(always)]
    fn from(variant: Xt1faultoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XT1FAULTOFF` reader - The XT1 oscillator fault detection off"]
pub type Xt1faultoffR = crate::BitReader<Xt1faultoff>;
impl Xt1faultoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xt1faultoff {
        match self.bits {
            false => Xt1faultoff::Xt1faultoff0,
            true => Xt1faultoff::Xt1faultoff1,
        }
    }
    #[doc = "Enabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn is_xt1faultoff_0(&self) -> bool {
        *self == Xt1faultoff::Xt1faultoff0
    }
    #[doc = "Disabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn is_xt1faultoff_1(&self) -> bool {
        *self == Xt1faultoff::Xt1faultoff1
    }
}
#[doc = "Field `XT1FAULTOFF` writer - The XT1 oscillator fault detection off"]
pub type Xt1faultoffW<'a, REG> = crate::BitWriter<'a, REG, Xt1faultoff>;
impl<'a, REG> Xt1faultoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn xt1faultoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1faultoff::Xt1faultoff0)
    }
    #[doc = "Disabling XT1 fault to switch ACLK to REFO"]
    #[inline(always)]
    pub fn xt1faultoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Xt1faultoff::Xt1faultoff1)
    }
}
impl R {
    #[doc = "Bit 0 - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
    #[inline(always)]
    pub fn xt1autooff(&self) -> Xt1autooffR {
        Xt1autooffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic Gain Control (AGC) disable."]
    #[inline(always)]
    pub fn xt1agcoff(&self) -> Xt1agcoffR {
        Xt1agcoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
    #[inline(always)]
    pub fn xt1hffreq(&self) -> Xt1hffreqR {
        Xt1hffreqR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - XT1 bypass select"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> Xt1bypassR {
        Xt1bypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XT1 mode select"]
    #[inline(always)]
    pub fn xts(&self) -> XtsR {
        XtsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
    #[inline(always)]
    pub fn xt1drive(&self) -> Xt1driveR {
        Xt1driveR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - ACLK source divider."]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - The XT1 oscillator fault detection off"]
    #[inline(always)]
    pub fn xt1faultoff(&self) -> Xt1faultoffR {
        Xt1faultoffR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XT1 automatic off enable. This bit allows XT1 turned turns off when it is not used"]
    #[inline(always)]
    pub fn xt1autooff(&mut self) -> Xt1autooffW<'_, Csctl6Spec> {
        Xt1autooffW::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic Gain Control (AGC) disable."]
    #[inline(always)]
    pub fn xt1agcoff(&mut self) -> Xt1agcoffW<'_, Csctl6Spec> {
        Xt1agcoffW::new(self, 1)
    }
    #[doc = "Bits 2:3 - The XT1 High-frequency selection. These bits must be set to appropriate frequency for crystal or bypass modes of operation."]
    #[inline(always)]
    pub fn xt1hffreq(&mut self) -> Xt1hffreqW<'_, Csctl6Spec> {
        Xt1hffreqW::new(self, 2)
    }
    #[doc = "Bit 4 - XT1 bypass select"]
    #[inline(always)]
    pub fn xt1bypass(&mut self) -> Xt1bypassW<'_, Csctl6Spec> {
        Xt1bypassW::new(self, 4)
    }
    #[doc = "Bit 5 - XT1 mode select"]
    #[inline(always)]
    pub fn xts(&mut self) -> XtsW<'_, Csctl6Spec> {
        XtsW::new(self, 5)
    }
    #[doc = "Bits 6:7 - The XT1 oscillator current can be adjusted to its drive needs. Initially, it starts with the highest supply current for reliable and quick startup. If needed, user software can reduce the drive strength. The configuration of these bits is retained during LPM3.5 until LOCKLPM5 is cleared, but not the register bits itself; therefore, reconfiguration after wake-up from LPM3.5 before clearing LOCKLPM5 is required."]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> Xt1driveW<'_, Csctl6Spec> {
        Xt1driveW::new(self, 6)
    }
    #[doc = "Bits 8:11 - ACLK source divider."]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<'_, Csctl6Spec> {
        DivaW::new(self, 8)
    }
    #[doc = "Bit 13 - The XT1 oscillator fault detection off"]
    #[inline(always)]
    pub fn xt1faultoff(&mut self) -> Xt1faultoffW<'_, Csctl6Spec> {
        Xt1faultoffW::new(self, 13)
    }
}
#[doc = "Clock System Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl6Spec;
impl crate::RegisterSpec for Csctl6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl6::R`](R) reader structure"]
impl crate::Readable for Csctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl6::W`](W) writer structure"]
impl crate::Writable for Csctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL6 to value 0"]
impl crate::Resettable for Csctl6Spec {}
