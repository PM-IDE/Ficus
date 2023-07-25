pub trait SuffixTreeSlice<TElement>
where
    TElement: PartialEq,
{
    fn equals(&self, first: usize, second: usize) -> bool;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<TElement>;
    fn sub_slice(&self, start: usize, end: usize) -> &[TElement];
}

pub struct SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub slice: &'a [TElement],
}

impl<'a, TElement> SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub fn new(slice: &'a [TElement]) -> Self {
        Self { slice }
    }
}

impl<'a, TElement> SuffixTreeSlice<TElement> for SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    fn equals(&self, first: usize, second: usize) -> bool {
        if first >= self.slice.len() || second >= self.slice.len() {
            return false;
        }

        self.slice[first] == self.slice[second]
    }

    fn len(&self) -> usize {
        self.slice.len() + 1
    }

    fn get(&self, index: usize) -> Option<TElement> {
        if index >= self.slice.len() {
            None
        } else {
            Some(*self.slice.get(index).unwrap())
        }
    }

    fn sub_slice(&self, start: usize, end: usize) -> &[TElement] {
        &self.slice[start..end]
    }
}

pub struct MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    words: Vec<&'a [TElement]>,
}

impl<'a, TElement> MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub fn new(words: Vec<&'a [TElement]>) -> Self {
        Self { words }
    }
}

impl<'a, TElement> SuffixTreeSlice<TElement> for MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    fn equals(&self, first: usize, second: usize) -> bool {
        self.get(first) == self.get(second)
    }

    fn get(&self, index: usize) -> Option<TElement> {
        let mut next_word_border = 0;
        for slice in &self.words {
            next_word_border += slice.len();
            if index < next_word_border {
                return Some(slice[index - (next_word_border - slice.len())]);
            }

            if index == next_word_border {
                return None;
            }

            next_word_border += 1;
        }

        return None;
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for slice in &self.words {
            len += slice.len();
        }

        len += self.words.len();
        len
    }

    fn sub_slice(&self, start: usize, end: usize) -> &[TElement] {
        panic!();
    }
}