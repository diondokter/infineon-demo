#[doc = "Register `MCWDT_LOCK` reader"]
pub type R = crate::R<McwdtLockSpec>;
#[doc = "Register `MCWDT_LOCK` writer"]
pub type W = crate::W<McwdtLockSpec>;
#[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum McwdtLock {
    #[doc = "0: No effect"]
    NoChg = 0,
    #[doc = "1: Clears bit 0"]
    Clr0 = 1,
    #[doc = "2: Clears bit 1"]
    Clr1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    Set01 = 3,
}
impl From<McwdtLock> for u8 {
    #[inline(always)]
    fn from(variant: McwdtLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for McwdtLock {
    type Ux = u8;
}
impl crate::IsEnum for McwdtLock {}
#[doc = "Field `MCWDT_LOCK` reader - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type McwdtLockR = crate::FieldReader<McwdtLock>;
impl McwdtLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McwdtLock {
        match self.bits {
            0 => McwdtLock::NoChg,
            1 => McwdtLock::Clr0,
            2 => McwdtLock::Clr1,
            3 => McwdtLock::Set01,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == McwdtLock::NoChg
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == McwdtLock::Clr0
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == McwdtLock::Clr1
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == McwdtLock::Set01
    }
}
#[doc = "Field `MCWDT_LOCK` writer - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type McwdtLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, McwdtLock, crate::Safe>;
impl<'a, REG> McwdtLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut crate::W<REG> {
        self.variant(McwdtLock::NoChg)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut crate::W<REG> {
        self.variant(McwdtLock::Clr0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut crate::W<REG> {
        self.variant(McwdtLock::Clr1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut crate::W<REG> {
        self.variant(McwdtLock::Set01)
    }
}
impl R {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&self) -> McwdtLockR {
        McwdtLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_lock(&mut self) -> McwdtLockW<McwdtLockSpec> {
        McwdtLockW::new(self, 30)
    }
}
#[doc = "Multi-Counter Watchdog Counter Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtLockSpec;
impl crate::RegisterSpec for McwdtLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_lock::R`](R) reader structure"]
impl crate::Readable for McwdtLockSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_lock::W`](W) writer structure"]
impl crate::Writable for McwdtLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_LOCK to value 0"]
impl crate::Resettable for McwdtLockSpec {
    const RESET_VALUE: u32 = 0;
}
