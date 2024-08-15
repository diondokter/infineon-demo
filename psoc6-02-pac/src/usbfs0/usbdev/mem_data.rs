#[doc = "Register `MEM_DATA[%s]` reader"]
pub type R = crate::R<MemDataSpec>;
#[doc = "Register `MEM_DATA[%s]` writer"]
pub type W = crate::W<MemDataSpec>;
#[doc = "Field `DR` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DrR = crate::FieldReader;
#[doc = "Field `DR` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<MemDataSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDataSpec;
impl crate::RegisterSpec for MemDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_data::R`](R) reader structure"]
impl crate::Readable for MemDataSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_data::W`](W) writer structure"]
impl crate::Writable for MemDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DATA[%s]
to value 0"]
impl crate::Resettable for MemDataSpec {
    const RESET_VALUE: u32 = 0;
}
