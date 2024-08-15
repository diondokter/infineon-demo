#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `ID` reader - the ID of LCD controller peripheral is 0xF0F0"]
pub type IdR = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - the version number is 0x0001"]
pub type RevisionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the version number is 0x0001"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ID &amp; Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x0001_f0f0"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x0001_f0f0;
}
