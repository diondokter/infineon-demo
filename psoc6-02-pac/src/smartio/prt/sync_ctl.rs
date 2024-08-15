#[doc = "Register `SYNC_CTL` reader"]
pub type R = crate::R<SyncCtlSpec>;
#[doc = "Register `SYNC_CTL` writer"]
pub type W = crate::W<SyncCtlSpec>;
#[doc = "Field `IO_SYNC_EN` reader - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IoSyncEnR = crate::FieldReader;
#[doc = "Field `IO_SYNC_EN` writer - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IoSyncEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHIP_SYNC_EN` reader - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type ChipSyncEnR = crate::FieldReader;
#[doc = "Field `CHIP_SYNC_EN` writer - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type ChipSyncEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&self) -> IoSyncEnR {
        IoSyncEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&self) -> ChipSyncEnR {
        ChipSyncEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn io_sync_en(&mut self) -> IoSyncEnW<SyncCtlSpec> {
        IoSyncEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn chip_sync_en(&mut self) -> ChipSyncEnW<SyncCtlSpec> {
        ChipSyncEnW::new(self, 8)
    }
}
#[doc = "Synchronization control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncCtlSpec;
impl crate::RegisterSpec for SyncCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ctl::R`](R) reader structure"]
impl crate::Readable for SyncCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_ctl::W`](W) writer structure"]
impl crate::Writable for SyncCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_CTL to value 0"]
impl crate::Resettable for SyncCtlSpec {
    const RESET_VALUE: u32 = 0;
}
