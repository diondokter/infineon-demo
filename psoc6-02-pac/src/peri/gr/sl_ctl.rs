#[doc = "Register `SL_CTL` reader"]
pub type R = crate::R<SlCtlSpec>;
#[doc = "Register `SL_CTL` writer"]
pub type W = crate::W<SlCtlSpec>;
#[doc = "Field `ENABLED_0` reader - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type Enabled0R = crate::BitReader;
#[doc = "Field `ENABLED_0` writer - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type Enabled0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_1` reader - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type Enabled1R = crate::BitReader;
#[doc = "Field `ENABLED_1` writer - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type Enabled1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_2` reader - N/A"]
pub type Enabled2R = crate::BitReader;
#[doc = "Field `ENABLED_2` writer - N/A"]
pub type Enabled2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_3` reader - N/A"]
pub type Enabled3R = crate::BitReader;
#[doc = "Field `ENABLED_3` writer - N/A"]
pub type Enabled3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_4` reader - N/A"]
pub type Enabled4R = crate::BitReader;
#[doc = "Field `ENABLED_4` writer - N/A"]
pub type Enabled4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_5` reader - N/A"]
pub type Enabled5R = crate::BitReader;
#[doc = "Field `ENABLED_5` writer - N/A"]
pub type Enabled5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_6` reader - N/A"]
pub type Enabled6R = crate::BitReader;
#[doc = "Field `ENABLED_6` writer - N/A"]
pub type Enabled6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_7` reader - N/A"]
pub type Enabled7R = crate::BitReader;
#[doc = "Field `ENABLED_7` writer - N/A"]
pub type Enabled7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_8` reader - N/A"]
pub type Enabled8R = crate::BitReader;
#[doc = "Field `ENABLED_8` writer - N/A"]
pub type Enabled8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_9` reader - N/A"]
pub type Enabled9R = crate::BitReader;
#[doc = "Field `ENABLED_9` writer - N/A"]
pub type Enabled9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_10` reader - N/A"]
pub type Enabled10R = crate::BitReader;
#[doc = "Field `ENABLED_10` writer - N/A"]
pub type Enabled10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_11` reader - N/A"]
pub type Enabled11R = crate::BitReader;
#[doc = "Field `ENABLED_11` writer - N/A"]
pub type Enabled11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_12` reader - N/A"]
pub type Enabled12R = crate::BitReader;
#[doc = "Field `ENABLED_12` writer - N/A"]
pub type Enabled12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_13` reader - N/A"]
pub type Enabled13R = crate::BitReader;
#[doc = "Field `ENABLED_13` writer - N/A"]
pub type Enabled13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_14` reader - N/A"]
pub type Enabled14R = crate::BitReader;
#[doc = "Field `ENABLED_14` writer - N/A"]
pub type Enabled14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_15` reader - N/A"]
pub type Enabled15R = crate::BitReader;
#[doc = "Field `ENABLED_15` writer - N/A"]
pub type Enabled15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_0` reader - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
pub type Disabled0R = crate::BitReader;
#[doc = "Field `DISABLED_0` writer - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
pub type Disabled0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_1` reader - N/A"]
pub type Disabled1R = crate::BitReader;
#[doc = "Field `DISABLED_1` writer - N/A"]
pub type Disabled1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_2` reader - N/A"]
pub type Disabled2R = crate::BitReader;
#[doc = "Field `DISABLED_2` writer - N/A"]
pub type Disabled2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_3` reader - N/A"]
pub type Disabled3R = crate::BitReader;
#[doc = "Field `DISABLED_3` writer - N/A"]
pub type Disabled3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_4` reader - N/A"]
pub type Disabled4R = crate::BitReader;
#[doc = "Field `DISABLED_4` writer - N/A"]
pub type Disabled4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_5` reader - N/A"]
pub type Disabled5R = crate::BitReader;
#[doc = "Field `DISABLED_5` writer - N/A"]
pub type Disabled5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_6` reader - N/A"]
pub type Disabled6R = crate::BitReader;
#[doc = "Field `DISABLED_6` writer - N/A"]
pub type Disabled6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_7` reader - N/A"]
pub type Disabled7R = crate::BitReader;
#[doc = "Field `DISABLED_7` writer - N/A"]
pub type Disabled7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_8` reader - N/A"]
pub type Disabled8R = crate::BitReader;
#[doc = "Field `DISABLED_8` writer - N/A"]
pub type Disabled8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_9` reader - N/A"]
pub type Disabled9R = crate::BitReader;
#[doc = "Field `DISABLED_9` writer - N/A"]
pub type Disabled9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_10` reader - N/A"]
pub type Disabled10R = crate::BitReader;
#[doc = "Field `DISABLED_10` writer - N/A"]
pub type Disabled10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_11` reader - N/A"]
pub type Disabled11R = crate::BitReader;
#[doc = "Field `DISABLED_11` writer - N/A"]
pub type Disabled11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_12` reader - N/A"]
pub type Disabled12R = crate::BitReader;
#[doc = "Field `DISABLED_12` writer - N/A"]
pub type Disabled12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_13` reader - N/A"]
pub type Disabled13R = crate::BitReader;
#[doc = "Field `DISABLED_13` writer - N/A"]
pub type Disabled13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_14` reader - N/A"]
pub type Disabled14R = crate::BitReader;
#[doc = "Field `DISABLED_14` writer - N/A"]
pub type Disabled14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLED_15` reader - N/A"]
pub type Disabled15R = crate::BitReader;
#[doc = "Field `DISABLED_15` writer - N/A"]
pub type Disabled15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_0(&self) -> Enabled0R {
        Enabled0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_1(&self) -> Enabled1R {
        Enabled1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn enabled_2(&self) -> Enabled2R {
        Enabled2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn enabled_3(&self) -> Enabled3R {
        Enabled3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn enabled_4(&self) -> Enabled4R {
        Enabled4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enabled_5(&self) -> Enabled5R {
        Enabled5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn enabled_6(&self) -> Enabled6R {
        Enabled6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn enabled_7(&self) -> Enabled7R {
        Enabled7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn enabled_8(&self) -> Enabled8R {
        Enabled8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn enabled_9(&self) -> Enabled9R {
        Enabled9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn enabled_10(&self) -> Enabled10R {
        Enabled10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn enabled_11(&self) -> Enabled11R {
        Enabled11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn enabled_12(&self) -> Enabled12R {
        Enabled12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn enabled_13(&self) -> Enabled13R {
        Enabled13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn enabled_14(&self) -> Enabled14R {
        Enabled14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn enabled_15(&self) -> Enabled15R {
        Enabled15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub fn disabled_0(&self) -> Disabled0R {
        Disabled0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn disabled_1(&self) -> Disabled1R {
        Disabled1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn disabled_2(&self) -> Disabled2R {
        Disabled2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn disabled_3(&self) -> Disabled3R {
        Disabled3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn disabled_4(&self) -> Disabled4R {
        Disabled4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn disabled_5(&self) -> Disabled5R {
        Disabled5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn disabled_6(&self) -> Disabled6R {
        Disabled6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn disabled_7(&self) -> Disabled7R {
        Disabled7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn disabled_8(&self) -> Disabled8R {
        Disabled8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn disabled_9(&self) -> Disabled9R {
        Disabled9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn disabled_10(&self) -> Disabled10R {
        Disabled10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn disabled_11(&self) -> Disabled11R {
        Disabled11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn disabled_12(&self) -> Disabled12R {
        Disabled12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn disabled_13(&self) -> Disabled13R {
        Disabled13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn disabled_14(&self) -> Disabled14R {
        Disabled14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn disabled_15(&self) -> Disabled15R {
        Disabled15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_0(&mut self) -> Enabled0W<SlCtlSpec> {
        Enabled0W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_1(&mut self) -> Enabled1W<SlCtlSpec> {
        Enabled1W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_2(&mut self) -> Enabled2W<SlCtlSpec> {
        Enabled2W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_3(&mut self) -> Enabled3W<SlCtlSpec> {
        Enabled3W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_4(&mut self) -> Enabled4W<SlCtlSpec> {
        Enabled4W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_5(&mut self) -> Enabled5W<SlCtlSpec> {
        Enabled5W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_6(&mut self) -> Enabled6W<SlCtlSpec> {
        Enabled6W::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_7(&mut self) -> Enabled7W<SlCtlSpec> {
        Enabled7W::new(self, 7)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_8(&mut self) -> Enabled8W<SlCtlSpec> {
        Enabled8W::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_9(&mut self) -> Enabled9W<SlCtlSpec> {
        Enabled9W::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_10(&mut self) -> Enabled10W<SlCtlSpec> {
        Enabled10W::new(self, 10)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_11(&mut self) -> Enabled11W<SlCtlSpec> {
        Enabled11W::new(self, 11)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_12(&mut self) -> Enabled12W<SlCtlSpec> {
        Enabled12W::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_13(&mut self) -> Enabled13W<SlCtlSpec> {
        Enabled13W::new(self, 13)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_14(&mut self) -> Enabled14W<SlCtlSpec> {
        Enabled14W::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_15(&mut self) -> Enabled15W<SlCtlSpec> {
        Enabled15W::new(self, 15)
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    #[must_use]
    pub fn disabled_0(&mut self) -> Disabled0W<SlCtlSpec> {
        Disabled0W::new(self, 16)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_1(&mut self) -> Disabled1W<SlCtlSpec> {
        Disabled1W::new(self, 17)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_2(&mut self) -> Disabled2W<SlCtlSpec> {
        Disabled2W::new(self, 18)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_3(&mut self) -> Disabled3W<SlCtlSpec> {
        Disabled3W::new(self, 19)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_4(&mut self) -> Disabled4W<SlCtlSpec> {
        Disabled4W::new(self, 20)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_5(&mut self) -> Disabled5W<SlCtlSpec> {
        Disabled5W::new(self, 21)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_6(&mut self) -> Disabled6W<SlCtlSpec> {
        Disabled6W::new(self, 22)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_7(&mut self) -> Disabled7W<SlCtlSpec> {
        Disabled7W::new(self, 23)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_8(&mut self) -> Disabled8W<SlCtlSpec> {
        Disabled8W::new(self, 24)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_9(&mut self) -> Disabled9W<SlCtlSpec> {
        Disabled9W::new(self, 25)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_10(&mut self) -> Disabled10W<SlCtlSpec> {
        Disabled10W::new(self, 26)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_11(&mut self) -> Disabled11W<SlCtlSpec> {
        Disabled11W::new(self, 27)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_12(&mut self) -> Disabled12W<SlCtlSpec> {
        Disabled12W::new(self, 28)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_13(&mut self) -> Disabled13W<SlCtlSpec> {
        Disabled13W::new(self, 29)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_14(&mut self) -> Disabled14W<SlCtlSpec> {
        Disabled14W::new(self, 30)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_15(&mut self) -> Disabled15W<SlCtlSpec> {
        Disabled15W::new(self, 31)
    }
}
#[doc = "Slave control\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlCtlSpec;
impl crate::RegisterSpec for SlCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_ctl::R`](R) reader structure"]
impl crate::Readable for SlCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sl_ctl::W`](W) writer structure"]
impl crate::Writable for SlCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL_CTL to value 0xffff"]
impl crate::Resettable for SlCtlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
