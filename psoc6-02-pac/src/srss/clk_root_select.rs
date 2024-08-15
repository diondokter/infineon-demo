#[doc = "Register `CLK_ROOT_SELECT[%s]` reader"]
pub type R = crate::R<ClkRootSelectSpec>;
#[doc = "Register `CLK_ROOT_SELECT[%s]` writer"]
pub type W = crate::W<ClkRootSelectSpec>;
#[doc = "Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_PATH_SELECT\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootMux {
    #[doc = "0: Select PATH0 (can be configured for FLL)"]
    Path0 = 0,
    #[doc = "1: Select PATH1 (can be configured for PLL0, if available in the product)"]
    Path1 = 1,
    #[doc = "2: Select PATH2 (can be configured for PLL1, if available in the product)"]
    Path2 = 2,
    #[doc = "3: Select PATH3 (can be configured for PLL2, if available in the product)"]
    Path3 = 3,
    #[doc = "4: Select PATH4 (can be configured for PLL3, if available in the product)"]
    Path4 = 4,
    #[doc = "5: Select PATH5 (can be configured for PLL4, if available in the product)"]
    Path5 = 5,
    #[doc = "6: Select PATH6 (can be configured for PLL5, if available in the product)"]
    Path6 = 6,
    #[doc = "7: Select PATH7 (can be configured for PLL6, if available in the product)"]
    Path7 = 7,
    #[doc = "8: Select PATH8 (can be configured for PLL7, if available in the product)"]
    Path8 = 8,
    #[doc = "9: Select PATH9 (can be configured for PLL8, if available in the product)"]
    Path9 = 9,
    #[doc = "10: Select PATH10 (can be configured for PLL9, if available in the product)"]
    Path10 = 10,
    #[doc = "11: Select PATH11 (can be configured for PLL10, if available in the product)"]
    Path11 = 11,
    #[doc = "12: Select PATH12 (can be configured for PLL11, if available in the product)"]
    Path12 = 12,
    #[doc = "13: Select PATH13 (can be configured for PLL12, if available in the product)"]
    Path13 = 13,
    #[doc = "14: Select PATH14 (can be configured for PLL13, if available in the product)"]
    Path14 = 14,
    #[doc = "15: Select PATH15 (can be configured for PLL14, if available in the product)"]
    Path15 = 15,
}
impl From<RootMux> for u8 {
    #[inline(always)]
    fn from(variant: RootMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RootMux {
    type Ux = u8;
}
impl crate::IsEnum for RootMux {}
#[doc = "Field `ROOT_MUX` reader - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_PATH_SELECT\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type RootMuxR = crate::FieldReader<RootMux>;
impl RootMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RootMux {
        match self.bits {
            0 => RootMux::Path0,
            1 => RootMux::Path1,
            2 => RootMux::Path2,
            3 => RootMux::Path3,
            4 => RootMux::Path4,
            5 => RootMux::Path5,
            6 => RootMux::Path6,
            7 => RootMux::Path7,
            8 => RootMux::Path8,
            9 => RootMux::Path9,
            10 => RootMux::Path10,
            11 => RootMux::Path11,
            12 => RootMux::Path12,
            13 => RootMux::Path13,
            14 => RootMux::Path14,
            15 => RootMux::Path15,
            _ => unreachable!(),
        }
    }
    #[doc = "Select PATH0 (can be configured for FLL)"]
    #[inline(always)]
    pub fn is_path0(&self) -> bool {
        *self == RootMux::Path0
    }
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    #[inline(always)]
    pub fn is_path1(&self) -> bool {
        *self == RootMux::Path1
    }
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    #[inline(always)]
    pub fn is_path2(&self) -> bool {
        *self == RootMux::Path2
    }
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    #[inline(always)]
    pub fn is_path3(&self) -> bool {
        *self == RootMux::Path3
    }
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    #[inline(always)]
    pub fn is_path4(&self) -> bool {
        *self == RootMux::Path4
    }
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    #[inline(always)]
    pub fn is_path5(&self) -> bool {
        *self == RootMux::Path5
    }
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    #[inline(always)]
    pub fn is_path6(&self) -> bool {
        *self == RootMux::Path6
    }
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    #[inline(always)]
    pub fn is_path7(&self) -> bool {
        *self == RootMux::Path7
    }
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    #[inline(always)]
    pub fn is_path8(&self) -> bool {
        *self == RootMux::Path8
    }
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    #[inline(always)]
    pub fn is_path9(&self) -> bool {
        *self == RootMux::Path9
    }
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    #[inline(always)]
    pub fn is_path10(&self) -> bool {
        *self == RootMux::Path10
    }
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    #[inline(always)]
    pub fn is_path11(&self) -> bool {
        *self == RootMux::Path11
    }
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    #[inline(always)]
    pub fn is_path12(&self) -> bool {
        *self == RootMux::Path12
    }
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    #[inline(always)]
    pub fn is_path13(&self) -> bool {
        *self == RootMux::Path13
    }
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    #[inline(always)]
    pub fn is_path14(&self) -> bool {
        *self == RootMux::Path14
    }
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    #[inline(always)]
    pub fn is_path15(&self) -> bool {
        *self == RootMux::Path15
    }
}
#[doc = "Field `ROOT_MUX` writer - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_PATH_SELECT\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type RootMuxW<'a, REG> = crate::FieldWriter<'a, REG, 4, RootMux, crate::Safe>;
impl<'a, REG> RootMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PATH0 (can be configured for FLL)"]
    #[inline(always)]
    pub fn path0(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path0)
    }
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    #[inline(always)]
    pub fn path1(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path1)
    }
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    #[inline(always)]
    pub fn path2(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path2)
    }
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    #[inline(always)]
    pub fn path3(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path3)
    }
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    #[inline(always)]
    pub fn path4(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path4)
    }
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    #[inline(always)]
    pub fn path5(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path5)
    }
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    #[inline(always)]
    pub fn path6(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path6)
    }
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    #[inline(always)]
    pub fn path7(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path7)
    }
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    #[inline(always)]
    pub fn path8(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path8)
    }
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    #[inline(always)]
    pub fn path9(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path9)
    }
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    #[inline(always)]
    pub fn path10(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path10)
    }
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    #[inline(always)]
    pub fn path11(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path11)
    }
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    #[inline(always)]
    pub fn path12(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path12)
    }
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    #[inline(always)]
    pub fn path13(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path13)
    }
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    #[inline(always)]
    pub fn path14(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path14)
    }
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    #[inline(always)]
    pub fn path15(self) -> &'a mut crate::W<REG> {
        self.variant(RootMux::Path15)
    }
}
#[doc = "Selects predivider value for this clock root and DSI input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootDiv {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NoDiv = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DivBy2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DivBy4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DivBy8 = 3,
}
impl From<RootDiv> for u8 {
    #[inline(always)]
    fn from(variant: RootDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RootDiv {
    type Ux = u8;
}
impl crate::IsEnum for RootDiv {}
#[doc = "Field `ROOT_DIV` reader - Selects predivider value for this clock root and DSI input."]
pub type RootDivR = crate::FieldReader<RootDiv>;
impl RootDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RootDiv {
        match self.bits {
            0 => RootDiv::NoDiv,
            1 => RootDiv::DivBy2,
            2 => RootDiv::DivBy4,
            3 => RootDiv::DivBy8,
            _ => unreachable!(),
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == RootDiv::NoDiv
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == RootDiv::DivBy2
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == RootDiv::DivBy4
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == RootDiv::DivBy8
    }
}
#[doc = "Field `ROOT_DIV` writer - Selects predivider value for this clock root and DSI input."]
pub type RootDivW<'a, REG> = crate::FieldWriter<'a, REG, 2, RootDiv, crate::Safe>;
impl<'a, REG> RootDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(RootDiv::NoDiv)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(RootDiv::DivBy2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(RootDiv::DivBy4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(RootDiv::DivBy8)
    }
}
#[doc = "Field `ENABLE` reader - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_PATH_SELECT\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn root_mux(&self) -> RootMuxR {
        RootMuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub fn root_div(&self) -> RootDivR {
        RootDivR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_PATH_SELECT\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn root_mux(&mut self) -> RootMuxW<ClkRootSelectSpec> {
        RootMuxW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    #[must_use]
    pub fn root_div(&mut self) -> RootDivW<ClkRootSelectSpec> {
        RootDivW::new(self, 4)
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkRootSelectSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Clock Root Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_root_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_root_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRootSelectSpec;
impl crate::RegisterSpec for ClkRootSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_root_select::R`](R) reader structure"]
impl crate::Readable for ClkRootSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_root_select::W`](W) writer structure"]
impl crate::Writable for ClkRootSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_ROOT_SELECT[%s]
to value 0"]
impl crate::Resettable for ClkRootSelectSpec {
    const RESET_VALUE: u32 = 0;
}
