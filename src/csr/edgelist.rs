use crate::utils::prim_unsigned::PrimUnsigned;

/// Edge(vertex:T, direction: bool
/// direction: true for in_edge, false for out edge.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Edge<T>(T, bool);

#[derive(Debug, PartialEq, Eq)]
pub struct EdgeList<T> (pub Vec<Edge<T>>);

impl<T: PrimUnsigned> EdgeList<T> {
    pub fn from_bytes(input: &[u8]) -> Self {
        let mut vec = Vec::new();
        let mut it = input.iter().copied();

        let mut sum = T::_0;
        while let Some((vid_offset, direction)) = Self::read_vb_encode(&mut it) {
            sum = sum + vid_offset;
            vec.push(Edge(sum, direction));
        }
        Self(vec)
    }

    #[inline]
    fn read_vb_encode(it: &mut impl Iterator<Item=u8>) -> Option<(T, bool)> {
        let mut ans = T::_0;
        let mut bit_count = 0u8;
        while let Some(item) = it.next() {
            if item & 0x80 == 0 {
                ans = ans | (T::from_u8(item & 0x7F) << bit_count);
                bit_count += 7;
            } else {
                ans = ans | (T::from_u8(item & 0x3F) << bit_count);
                return Some((ans, item & 0x40 > 0));
            }
        }
        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let vec = &self.0;
        let mut sum = T::_0;
        for Edge(mut vid, direction) in vec {
            vid = vid - sum;
            sum = sum + vid;
            while vid > T::_0 {
                let mut data : u8 = vid.lowest_u8() & 0x7F;
                vid = vid >> 7u8;
                if vid == T::_0 {
                    if data & 0x40 > 0 {
                        result.push(data);
                        data = 0;
                    }
                    if *direction {
                        data |= 0x40;
                    }
                    data |= 0x80;
                    result.push(data);
                    break;
                }
                result.push(data);
            }
        }
        result
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_eq;
    
    #[test]
    fn test() {
        let edges = EdgeList::<u32>(vec![
            Edge(1, true),
            Edge(350, true),
            Edge(355, false),
            Edge(359, true),
            Edge(403, false),
            Edge(409, true),
            Edge(501, true),
            Edge(10000, true),
            Edge(900000000u32, true)
        ]);
        let encoded = edges.as_bytes();
        let edges2 = EdgeList::<u32>::from_bytes(encoded.as_slice());
        assert_eq!(edges, edges2)
    }
}