#[doc = "Register `CRC_DATA[%s]` reader"]
pub type R = crate::R<CrcDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Bytes 1A04h to 1AF7h, the range the CRC value covers\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcDataSpec;
impl crate::RegisterSpec for CrcDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crc_data::R`](R) reader structure"]
impl crate::Readable for CrcDataSpec {}
#[doc = "`reset()` method sets CRC_DATA[%s] to value 0"]
impl crate::Resettable for CrcDataSpec {}
