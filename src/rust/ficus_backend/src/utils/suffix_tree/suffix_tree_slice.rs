pub struct SuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub slice: &'a [TElement],
}

impl<'a, TElement> SuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    pub fn equals(&self, first: usize, second: usize) -> bool {
        if first >= self.slice.len() || second >= self.slice.len() {
            return false;
        }

        self.slice[first] == self.slice[second]
    }

    pub fn len(&self) -> usize {
        self.slice.len() + 1
    }

    pub fn get(&self, index: usize) -> Option<TElement> {
        if index >= self.slice.len() {
            None
        } else {
            Some(*self.slice.get(index).unwrap())
        }
    }
}
