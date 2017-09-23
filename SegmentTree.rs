fn main() {
    let mut vec: Vec<i64> = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    let mut st = SegmentTree::new(&vec);

    assert_eq!(st.mconcat(0, 100), vec[0..100].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(12, 45), vec[12..45].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(89, 90), vec[89..90].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(30, 100), vec[30..100].iter().fold(0, |acc, x| acc+x));

    st.update(10, 100);
    vec[10] = 100;
    st.update(24, -100);
    vec[24] = -100;
    st.update(89, 0);
    vec[89] = 0;

    assert_eq!(st.mconcat(0, 100), vec[0..100].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(12, 45), vec[12..45].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(89, 90), vec[89..90].iter().fold(0, |acc, x| acc+x));
    assert_eq!(st.mconcat(30, 100), vec[30..100].iter().fold(0, |acc, x| acc+x));
}

trait Monoid {
    fn mappend(&self, b: &Self) -> Self;
    fn mempty() -> Self;
}

impl Monoid for i64 {
    fn mappend(&self, b: &i64) -> i64 {
        self + b
    }
    fn mempty() -> i64 {
        0
    }
}

struct SegmentTree<M: Monoid + Clone> {
    value: M,
    start: usize,
    end: usize,
    children: Option<Box<SegmentTreeChildren<M>>>
}

struct SegmentTreeChildren<M: Monoid + Clone> {
    head: SegmentTree<M>,
    tail: SegmentTree<M>
}

impl<M> SegmentTree<M> where M: Monoid + Clone {
    fn new(vec: &Vec<M>) -> SegmentTree<M> {
        SegmentTree::_build(vec, 0, vec.len())
    }

    fn mconcat(&self, i: usize, j: usize) -> M {
        if j <= self.start || self.end <=i {
            M::mempty()
        } else if i <= self.start && self.end <= j {
            self.value.clone()
        } else {
            match &self.children {
                &Some(ref children) => children.head.mconcat(i, j).mappend(&children.tail.mconcat(i, j)),
                &None => M::mempty()
            }
        }
    }

    fn update(&mut self, i: usize, m: M) {
        if let &mut Some(ref mut children) = &mut self.children {
            if i < children.head.end {
                children.head.update(i, m);
            } else {
                children.tail.update(i, m);
            }
            self.value = children.head.value.mappend(&children.tail.value);
        } else {
            self.value = m;
        }
    }

    fn _build(vec: &Vec<M>, start: usize, end: usize) -> SegmentTree<M> {
        if end <= start {
            SegmentTree{
                value: M::mempty(),
                start: start,
                end: end,
                children: None
            }
        } else if start+1 == end {
            SegmentTree {
                value: vec[start].clone(),
                start: start,
                end: end,
                children: None
            }
        } else {
            let mid = (start + end) / 2;
            let head = SegmentTree::_build(vec, start, mid);
            let tail = SegmentTree::_build(vec, mid, end);
            SegmentTree {
                value: head.value.mappend(&tail.value),
                start: start,
                end: end,
                children: Some(Box::new(
                    SegmentTreeChildren{
                        head: head,
                        tail: tail
                    }
                ))
            }
        }
    }
}
