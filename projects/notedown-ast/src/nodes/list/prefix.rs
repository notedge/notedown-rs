use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ListPrefixSymbol {
    Unknown,
    /// ```note
    /// -
    /// ```
    Hyphen,
    /// ```note
    /// >
    /// ```
    Quote,
    /// ```note
    /// >+ Summary Open
    /// ```
    SummaryOpen,
    /// ```note
    /// >- Summary Open
    /// ```
    SummaryClosed,
    /// Single, serial number from the beginning
    /// ```note
    /// 1.
    /// 2.
    /// 3.
    /// ```
    Arabic,
    /// Complex, multi-layered, not serial number from the beginning
    /// ```note
    /// 4.4.
    /// 4.5.
    /// 4.6.
    /// ```
    ArabicNest {
        prefix_number: Vec<usize>,
        number: usize,
    },
    /// ```note
    /// I.
    /// II.
    /// ```
    RomanNumerals,
}

impl Default for ListPrefixSymbol {
    fn default() -> Self {
        Self::Hyphen
    }
}

impl ListPrefixSymbol {
    pub fn parse(input: &str) -> Self {
        match input {
            s if s.starts_with(">") => Self::Quote,
            _ => Self::Unknown,
        }
    }
}

impl ListPrefixSymbol {
    #[inline]
    pub fn is_quote(&self) -> bool {
        matches!(self, Self::Quote)
    }
    #[inline]
    pub fn is_ordered(&self) -> bool {
        matches!(self, Self::Arabic | Self::ArabicNest { .. })
    }
}
