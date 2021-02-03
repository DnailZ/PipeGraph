
use std::array::FixedSizeArray;

use desse::{Desse, DesseSized};
use crate::utils::prim_unsigned::PrimUnsigned;
use vec_file::VecFile;
use ndarray::Array2;
use super::AdjGraph;

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct Edge{
    src: u64,
    dest: u64,
}

struct CsrBuilder {
    buffer : Array2<VecFile<Edge>>,
    graph: AdjGraph,
}