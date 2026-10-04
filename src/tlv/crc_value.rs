#[doc = "Register `CRC_VALUE` reader"]
pub type R = crate::R<CrcValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CRC value of the descriptors from 1A04h to 1AF7h (CRC-CCITT)\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcValueSpec;
impl crate::RegisterSpec for CrcValueSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc_value::R`](R) reader structure"]
impl crate::Readable for CrcValueSpec {}
#[doc = "`reset()` method sets CRC_VALUE to value 0"]
impl crate::Resettable for CrcValueSpec {}
