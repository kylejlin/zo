use crate::syntax_tree::ipist::*;

impl ZeroOrMoreExprs {
    pub fn to_vec_of_cloned(&self) -> Vec<Expr> {
        match self {
            Self::Nil => vec![],
            Self::Snoc(left, right) => {
                let mut left = left.to_vec_of_cloned();
                left.push(right.clone());
                left
            }
        }
    }
}

impl ZeroOrMoreExprs {
    pub fn to_vec(&self) -> Vec<&Expr> {
        match self {
            Self::Nil => vec![],
            Self::Snoc(left, right) => {
                let mut left = left.to_vec();
                left.push(right);
                left
            }
        }
    }
}

impl std::ops::Index<usize> for ZeroOrMoreExprs {
    type Output = Expr;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(out) = self.get(index) {
            return out;
        }

        let len = self.len();
        panic!("index out of bounds: the len is {len} but the index is {index}");
    }
}

impl ZeroOrMoreExprs {
    pub fn get(&self, index: usize) -> Option<&Expr> {
        let index_from_back = self.len().checked_sub(index + 1)?;
        self.get_from_back(index_from_back)
    }

    pub fn get_from_back(&self, index: usize) -> Option<&Expr> {
        match self {
            ZeroOrMoreExprs::Nil => None,
            ZeroOrMoreExprs::Snoc(left, right) => {
                if index == 0 {
                    Some(&right)
                } else {
                    left.get_from_back(index - 1)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ZeroOrMoreExprs::Nil => 0,
            ZeroOrMoreExprs::Snoc(left, _) => 1 + left.len(),
        }
    }
}

impl ZeroOrMoreVconDefs {
    pub fn to_vec(&self) -> Vec<&VconDef> {
        match self {
            Self::Nil => vec![],
            Self::Snoc(left, right) => {
                let mut left = left.to_vec();
                left.push(right);
                left
            }
        }
    }
}

impl std::ops::Index<usize> for ZeroOrMoreVconDefs {
    type Output = VconDef;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(out) = self.get(index) {
            return out;
        }

        let len = self.len();
        panic!("index out of bounds: the len is {len} but the index is {index}");
    }
}

impl ZeroOrMoreVconDefs {
    pub fn get(&self, index: usize) -> Option<&VconDef> {
        let index_from_back = self.len().checked_sub(index + 1)?;
        self.get_from_back(index_from_back)
    }

    pub fn get_from_back(&self, index: usize) -> Option<&VconDef> {
        match self {
            ZeroOrMoreVconDefs::Nil => None,
            ZeroOrMoreVconDefs::Snoc(left, right) => {
                if index == 0 {
                    Some(&right)
                } else {
                    left.get_from_back(index - 1)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ZeroOrMoreVconDefs::Nil => 0,
            ZeroOrMoreVconDefs::Snoc(left, _) => 1 + left.len(),
        }
    }
}

impl std::ops::Index<usize> for ZeroOrMoreMatchCases {
    type Output = MatchCase;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(out) = self.get(index) {
            return out;
        }

        let len = self.len();
        panic!("index out of bounds: the len is {len} but the index is {index}");
    }
}

impl ZeroOrMoreMatchCases {
    pub fn get(&self, index: usize) -> Option<&MatchCase> {
        let index_from_back = self.len().checked_sub(index + 1)?;
        self.get_from_back(index_from_back)
    }

    pub fn get_from_back(&self, index: usize) -> Option<&MatchCase> {
        match self {
            ZeroOrMoreMatchCases::Nil => None,
            ZeroOrMoreMatchCases::Snoc(left, right) => {
                if index == 0 {
                    Some(&right)
                } else {
                    left.get_from_back(index - 1)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ZeroOrMoreMatchCases::Nil => 0,
            ZeroOrMoreMatchCases::Snoc(left, _) => 1 + left.len(),
        }
    }
}
