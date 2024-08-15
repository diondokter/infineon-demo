#[doc = "Register `DATA_CTL` reader"]
pub type R = crate::R<DataCtlSpec>;
#[doc = "Register `DATA_CTL` writer"]
pub type W = crate::W<DataCtlSpec>;
#[doc = "PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WordLen {
    #[doc = "0: 16-bit"]
    BitLen16 = 0,
    #[doc = "1: 18-bit"]
    BitLen18 = 1,
    #[doc = "2: 20-bit"]
    BitLen20 = 2,
    #[doc = "3: 24-bit"]
    BitLen24 = 3,
}
impl From<WordLen> for u8 {
    #[inline(always)]
    fn from(variant: WordLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WordLen {
    type Ux = u8;
}
impl crate::IsEnum for WordLen {}
#[doc = "Field `WORD_LEN` reader - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
pub type WordLenR = crate::FieldReader<WordLen>;
impl WordLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WordLen {
        match self.bits {
            0 => WordLen::BitLen16,
            1 => WordLen::BitLen18,
            2 => WordLen::BitLen20,
            3 => WordLen::BitLen24,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == WordLen::BitLen16
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == WordLen::BitLen18
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == WordLen::BitLen20
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == WordLen::BitLen24
    }
}
#[doc = "Field `WORD_LEN` writer - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
pub type WordLenW<'a, REG> = crate::FieldWriter<'a, REG, 2, WordLen, crate::Safe>;
impl<'a, REG> WordLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen24)
    }
}
#[doc = "Field `BIT_EXTENSION` reader - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BitExtensionR = crate::BitReader;
#[doc = "Field `BIT_EXTENSION` writer - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BitExtensionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub fn word_len(&self) -> WordLenR {
        WordLenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&self) -> BitExtensionR {
        BitExtensionR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    #[must_use]
    pub fn word_len(&mut self) -> WordLenW<DataCtlSpec> {
        WordLenW::new(self, 0)
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    #[must_use]
    pub fn bit_extension(&mut self) -> BitExtensionW<DataCtlSpec> {
        BitExtensionW::new(self, 8)
    }
}
#[doc = "Data control\n\nYou can [`read`](crate::Reg::read) this register and get [`data_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCtlSpec;
impl crate::RegisterSpec for DataCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_ctl::R`](R) reader structure"]
impl crate::Readable for DataCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`data_ctl::W`](W) writer structure"]
impl crate::Writable for DataCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_CTL to value 0"]
impl crate::Resettable for DataCtlSpec {
    const RESET_VALUE: u32 = 0;
}
