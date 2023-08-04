use super::*;

impl mnode::IdentOrUnderscore {
    /// If `self` is an identifier,
    /// its value is returned.
    /// Otherwise, `self` is an underscore,
    /// in which case the string `"_"` is returned.
    pub(crate) fn val(&self) -> &str {
        match self {
            Self::Ident(ident) => &ident.value,
            Self::Underscore(_) => "_",
        }
    }
}

impl mnode::OptIdent {
    /// If `self` is `Some(ident)`,
    /// then `ident` is returned.
    /// Otherwise, `"_"` is returned.
    pub(crate) fn val_or_underscore(&self) -> &str {
        match self {
            Self::Some(ident) => &ident.value,
            Self::None => "_",
        }
    }
}

impl mnode::OptParenthesizedCommaSeparatedIdentsOrUnderscores {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Some(parenthesized) => parenthesized.idents.len(),
        }
    }
}

impl mnode::CommaSeparatedIdentsOrUnderscores {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Snoc(rdc, _) => rdc.len() + 1,
        }
    }
}

impl mnode::ZeroOrMoreMatchCases {
    pub(crate) fn to_vec(&self) -> Vec<&mnode::MatchCase> {
        match self {
            mnode::ZeroOrMoreMatchCases::Nil => vec![],
            mnode::ZeroOrMoreMatchCases::Snoc(rdc, rac) => {
                let mut rdc = rdc.to_vec();
                rdc.push(rac);
                rdc
            }
        }
    }
}

impl mnode::OptSquareBracketedParamDefs {
    pub(crate) fn defs(&self) -> Option<&mnode::CommaSeparatedParamDefs> {
        match self {
            Self::Some(defs) => Some(&defs.params),
            Self::None => None,
        }
    }
}
