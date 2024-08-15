#[doc = "Register `ARGUMENT_R` reader"]
pub type R = crate::R<ArgumentRSpec>;
#[doc = "Register `ARGUMENT_R` writer"]
pub type W = crate::W<ArgumentRSpec>;
#[doc = "Field `ARGUMENT` reader - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ArgumentR = crate::FieldReader<u32>;
#[doc = "Field `ARGUMENT` writer - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ArgumentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argument(&self) -> ArgumentR {
        ArgumentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    #[must_use]
    pub fn argument(&mut self) -> ArgumentW<ArgumentRSpec> {
        ArgumentW::new(self, 0)
    }
}
#[doc = "Argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`argument_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArgumentRSpec;
impl crate::RegisterSpec for ArgumentRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument_r::R`](R) reader structure"]
impl crate::Readable for ArgumentRSpec {}
#[doc = "`write(|w| ..)` method takes [`argument_r::W`](W) writer structure"]
impl crate::Writable for ArgumentRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGUMENT_R to value 0"]
impl crate::Resettable for ArgumentRSpec {
    const RESET_VALUE: u32 = 0;
}
