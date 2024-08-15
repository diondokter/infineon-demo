#[doc = "Register `WUP_CTRL_R` reader"]
pub type R = crate::R<WupCtrlRSpec>;
#[doc = "Register `WUP_CTRL_R` writer"]
pub type W = crate::W<WupCtrlRSpec>;
#[doc = "Field `WUP_CARD_INT` reader - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardIntR = crate::BitReader;
#[doc = "Field `WUP_CARD_INT` writer - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP_CARD_INSERT` reader - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardInsertR = crate::BitReader;
#[doc = "Field `WUP_CARD_INSERT` writer - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardInsertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP_CARD_REMOVAL` reader - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardRemovalR = crate::BitReader;
#[doc = "Field `WUP_CARD_REMOVAL` writer - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WupCardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_int(&self) -> WupCardIntR {
        WupCardIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_insert(&self) -> WupCardInsertR {
        WupCardInsertR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_removal(&self) -> WupCardRemovalR {
        WupCardRemovalR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_int(&mut self) -> WupCardIntW<WupCtrlRSpec> {
        WupCardIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_insert(&mut self) -> WupCardInsertW<WupCtrlRSpec> {
        WupCardInsertW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_removal(&mut self) -> WupCardRemovalW<WupCtrlRSpec> {
        WupCardRemovalW::new(self, 2)
    }
}
#[doc = "Wakeup Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wup_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wup_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WupCtrlRSpec;
impl crate::RegisterSpec for WupCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wup_ctrl_r::R`](R) reader structure"]
impl crate::Readable for WupCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`wup_ctrl_r::W`](W) writer structure"]
impl crate::Writable for WupCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WUP_CTRL_R to value 0"]
impl crate::Resettable for WupCtrlRSpec {
    const RESET_VALUE: u8 = 0;
}
