#[doc = "Register `CLK_ILO_CONFIG` reader"]
pub type R = crate::R<ClkIloConfigSpec>;
#[doc = "Register `CLK_ILO_CONFIG` writer"]
pub type W = crate::W<ClkIloConfigSpec>;
#[doc = "Field `ILO_BACKUP` reader - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type IloBackupR = crate::BitReader;
#[doc = "Field `ILO_BACKUP` writer - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type IloBackupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&self) -> IloBackupR {
        IloBackupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    #[must_use]
    pub fn ilo_backup(&mut self) -> IloBackupW<ClkIloConfigSpec> {
        IloBackupW::new(self, 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkIloConfigSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "ILO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ilo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ilo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkIloConfigSpec;
impl crate::RegisterSpec for ClkIloConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ilo_config::R`](R) reader structure"]
impl crate::Readable for ClkIloConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ilo_config::W`](W) writer structure"]
impl crate::Writable for ClkIloConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_ILO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for ClkIloConfigSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
