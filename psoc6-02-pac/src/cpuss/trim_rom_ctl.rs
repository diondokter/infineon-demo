#[doc = "Register `TRIM_ROM_CTL` reader"]
pub type R = crate::R<TrimRomCtlSpec>;
#[doc = "Register `TRIM_ROM_CTL` writer"]
pub type W = crate::W<TrimRomCtlSpec>;
#[doc = "Field `TRIM` reader - N/A"]
pub type TrimR = crate::FieldReader<u32>;
#[doc = "Field `TRIM` writer - N/A"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<TrimRomCtlSpec> {
        TrimW::new(self, 0)
    }
}
#[doc = "ROM trim control\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_rom_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_rom_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimRomCtlSpec;
impl crate::RegisterSpec for TrimRomCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim_rom_ctl::R`](R) reader structure"]
impl crate::Readable for TrimRomCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`trim_rom_ctl::W`](W) writer structure"]
impl crate::Writable for TrimRomCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM_ROM_CTL to value 0"]
impl crate::Resettable for TrimRomCtlSpec {
    const RESET_VALUE: u32 = 0;
}
