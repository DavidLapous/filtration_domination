use crate::simplicial_complex::{is_sorted, Dimension, SimplicialComplex, Vertex};
use crate::CriticalGrade;
use sorted_iter::assume::AssumeSortedByItemExt;
use sorted_iter::SortedIterator;

#[derive(Debug)]
pub struct Filtration<G, S> {
    /// Critical grade for each cell in each dimension.
    /// First, we index the dimensions, and then the cells.
    grades: Vec<Vec<G>>,

    /// The underlying simplicial complex being filtered.
    complex: S,
}

impl<G: CriticalGrade, S> Filtration<G, S>
where
    S: for<'a> SimplicialComplex<'a>,
{
    pub fn new(complex: S) -> Self {
        let mut grades = Vec::with_capacity(complex.max_dimension() + 1);
        for dim in 0..complex.max_dimension() + 1 {
            grades.push(vec![G::min_value(); complex.n_cells(dim)]);
        }
        Filtration { grades, complex }
    }

    pub fn new_empty(max_vertices: Vertex, max_dim: Dimension) -> Self {
        let s = S::new(max_vertices, max_dim);
        Self::new(s)
    }

    pub fn add(&mut self, g: G, s: &[Vertex]) -> Option<(Dimension, usize)> {
        assert!(is_sorted(s), "To add a simplex it must be sorted first.");

        let dim = s.len() - 1;
        self.add_iter(g, dim, s.iter().copied().assume_sorted_by_item())
    }

    pub fn add_iter<I: SortedIterator<Item = usize>>(
        &mut self,
        g: G,
        dim: Dimension,
        iter: I,
    ) -> Option<(Dimension, usize)> {
        let added_simplex = self.complex.add_iter(dim, iter);
        if let Some((dimension, idx)) = added_simplex {
            assert_eq!(idx, self.grades[dimension].len(),
                       "Programming error: the index of an added simplex is the total number of simplices in that dimension.");
            if dimension > 0 {
                for boundary_idx in self.complex.boundary_iterator(dimension, idx) {
                    // TODO: revisit this.
                    assert!(self.grades[dimension - 1][boundary_idx].lte(&g), "The grade of a simplex is greater than or equal to the grade of its facets: {:?} is not lte than {:?}.", self.grades[dimension - 1][boundary_idx], g);
                }
            }
            self.grades[dimension].push(g);
        }
        added_simplex
    }

    pub fn value_of(&self, dim: Dimension, idx: usize) -> &G {
        &self.grades[dim][idx]
    }

    pub fn simplicial_complex(&self) -> &S {
        &self.complex
    }
}