/// file that convert a gfa file into an hash map
use crate::{
    gfa2::{
        GFA2,
        Segment,
        Fragment,
        Edge,
        Gap,
        GroupO,
        GroupU,
    },
    tag::*,
};

use fnv::FnvHashMap;
use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn hash_gfa2<T: OptFields>(gfa2: &GFA2<BString, T>) -> u64 {
    let mut hasher = DefaultHasher::new();

    for head in gfa2.headers.iter() {
       head.version.hash(&mut hasher); 
    }

    for seg in gfa2.segments.iter() {
        seg.id.hash(&mut hasher);
        seg.len.hash(&mut hasher);
        seg.sequence.hash(&mut hasher);
    }

    for frag in gfa2.fragments.iter() {
        frag.id.hash(&mut hasher);
        frag.ext_ref.hash(&mut hasher);
        frag.sbeg.hash(&mut hasher);
        frag.send.hash(&mut hasher);
        frag.fbeg.hash(&mut hasher);
        frag.fend.hash(&mut hasher);
        frag.alignment.hash(&mut hasher);
    }

    for edge in gfa2.edges.iter() {
        edge.id.hash(&mut hasher);
        edge.sid1.hash(&mut hasher);
        edge.sid2.hash(&mut hasher);
        edge.beg1.hash(&mut hasher);
        edge.end1.hash(&mut hasher);
        edge.beg2.hash(&mut hasher);
        edge.end2.hash(&mut hasher);
        edge.alignment.hash(&mut hasher);
    }

    for gap in gfa2.gaps.iter() {
        gap.id.hash(&mut hasher);
        gap.sid1.hash(&mut hasher);
        gap.sid2.hash(&mut hasher);
        gap.dist.hash(&mut hasher);
        gap.var.hash(&mut hasher);
    }

    for ogroup in gfa2.groups_o.iter() {
        ogroup.id.hash(&mut hasher);
        ogroup.var_field.hash(&mut hasher);
    }

    for ugroup in gfa2.groups_u.iter() {
        ugroup.id.hash(&mut hasher);
        ugroup.var_field.hash(&mut hasher);
    }

    hasher.finish()
}

/// This is a helper struct for handling serialization/deserialization
/// of NameMaps to text-based formats such as ASCII
#[derive(Serialize, Deserialize)]
struct NameMapString {
    pub(crate) name_map: FnvHashMap<String, usize>,
    pub(crate) inverse_map: Vec<String>,
    pub(crate) hash: u64,
}

impl NameMapString {
    fn from_name_map(map: &NameMap) -> Self {
        let name_map: FnvHashMap<String, usize> = map
            .name_map
            .iter()
            .map(|(k, v)| (k.to_str().unwrap().into(), *v))
            .collect();

        let inverse_map: Vec<String> = map
            .inverse_map
            .iter()
            .map(|k| k.to_str().unwrap().into())
            .collect();

        NameMapString {
            name_map,
            inverse_map,
            hash: map.hash,
        }
    }

    fn into_name_map(self) -> NameMap {
        let name_map: FnvHashMap<Vec<u8>, usize> = self
            .name_map
            .iter()
            .map(|(k, v)| {
                let k: Vec<u8> = Vec::from(k.as_bytes());
                (k, *v)
            })
            .collect();

        let inverse_map: Vec<BString> = self
            .inverse_map
            .iter()
            .map(|k| k.as_bytes().into())
            .collect();

        NameMap {
            name_map,
            inverse_map,
            hash: self.hash,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NameMap {
    pub(crate) name_map: FnvHashMap<Vec<u8>, usize>,
    pub(crate) inverse_map: Vec<BString>,
    /// The hash is calculated on the GFA<BString, _> value
    pub(crate) hash: u64,
}

impl NameMap {
    /// Save the NameMap to a JSON file.
    pub fn save_json<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> std::io::Result<()> {
        use std::{fs::File, io::BufWriter};
        let file = File::create(path.as_ref())?;
        let writer = BufWriter::new(file);
        let name_map = NameMapString::from_name_map(self);
        serde_json::to_writer(writer, &name_map)?;
        Ok(())
    }

    /// Load a NameMap from a JSON file.
    pub fn load_json<P: AsRef<std::path::Path>>(
        path: P,
    ) -> std::io::Result<Self> {
        use std::{fs::File, io::BufReader};
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let name_map: NameMapString = serde_json::from_reader(reader)?;
        Ok(name_map.into_name_map())
    }

    pub fn map_name<N: AsRef<[u8]>>(&self, name: N) -> Option<usize> {
        self.name_map.get(name.as_ref()).copied()
    }

    pub fn inverse_map_name(&self, id: usize) -> Option<&'_ BStr> {
        self.inverse_map.get(id).map(|bs| bs.as_ref())
    }

    fn map_ogroup_segments<T: OptFields>(
        &self,
        group: &GroupO<BString, T>,
    ) -> Option<GroupO<usize, T>> {
        let mut misses = 0;
        let new_segs: BString = group
            .iter()
            .filter_map(|(seg, o)| {
                let n = self.map_name(seg).map(|s| (s, o));
                if n.is_none() {
                    misses += 1;
                }
                n
            })
            .enumerate()
            .flat_map(|(i, (seg, o))| {
                let s = if i == 0 {
                    format!("{}{}", seg, o)
                } else {
                    format!(" {}{}", seg, o)
                };
                Vec::from(s.as_bytes())
            })
            .collect();

            if misses > 0 {
                return None;
            }
            let new_ogroup = GroupO::new(
                group.id.clone(),
                new_segs,
                group.tag.clone(),
            );

            Some(new_ogroup)
    }

    fn inverse_map_ogroup_segments<T: OptFields>(
        &self,
        group: &GroupO<usize, T>,
    ) -> Option<GroupO<BString, T>> {
        let mut misses = 0;
        let new_segs: BString = group
            .iter()
            .filter_map(|(seg, o)| {
                let n = self.inverse_map.get(seg).map(|s| (s, o));
                if n.is_none() {
                    misses += 1;
                }
                n
            })
            .enumerate()
            .flat_map(|(i, (seg, o))| {
                let s = if i == 0 {
                    format!("{}{}", seg, o)
                } else {
                    format!(" {}{}", seg, o)
                };
                Vec::from(s.as_bytes())
            })
            .collect();

            if misses > 0 {
                return None;
            }
            let new_ogroup = GroupO::new(
                group.id.clone(),
                new_segs,
                group.tag.clone(),
            );

            Some(new_ogroup)
    }

    fn map_ugroup_segments<T: OptFields>(
        &self,
        group: &GroupU<BString, T>,
    ) -> Option<GroupU<usize, T>> {
        let mut misses = 0;
        let new_segs: BString = group
            .iter()
            .filter_map(|seg| {
                let n = self.map_name(seg).map(|s| s);
                if n.is_none() {
                    misses += 1;
                }
                n
            })
            .enumerate()
            .flat_map(|(i, seg)| {
                let s = if i == 0 {
                    format!("{}", seg)
                } else {
                    format!(" {}", seg)
                };
                Vec::from(s.as_bytes())
            })
            .collect();

            if misses > 0 {
                return None;
            }
            let new_ugroup = GroupU::new(
                group.id.clone(),
                new_segs,
                group.tag.clone(),
            );

            Some(new_ugroup)
    }

    fn inverse_map_ugroup_segments<T: OptFields>(
        &self,
        group: &GroupU<usize, T>,
    ) -> Option<GroupU<BString, T>> {
        let mut misses = 0;
        let new_segs: BString = group
            .iter()
            .filter_map(|seg| {
                let n = self.inverse_map.get(seg).map(|s| s);
                if n.is_none() {
                    misses += 1;
                }
                n
            })
            .enumerate()
            .flat_map(|(i, seg)| {
                let s = if i == 0 {
                    format!("{}", seg)
                } else {
                    format!(" {}", seg)
                };
                Vec::from(s.as_bytes())
            })
            .collect();

            if misses > 0 {
                return None;
            }
            let new_ugroup = GroupU::new(
                group.id.clone(),
                new_segs,
                group.tag.clone(),
            );

            Some(new_ugroup)
    }


    pub fn gfa2_bstring_to_usize<T: OptFields>(
        &self,
        gfa2: &GFA2<BString, T>,
        check_hash: bool,
    ) -> Option<GFA2<usize, T>> {

        if check_hash {
            if hash_gfa2(gfa2) != self.hash {
                return None;
            }
        }

        let mut segments = Vec::with_capacity(gfa2.segments.len());
        let mut fragments = Vec::with_capacity(gfa2.fragments.len());
        let mut edges = Vec::with_capacity(gfa2.edges.len());
        let mut gaps = Vec::with_capacity(gfa2.gaps.len());
        let mut ogroups = Vec::with_capacity(gfa2.groups_o.len());
        let mut ugroups = Vec::with_capacity(gfa2.groups_u.len());

        for seg in gfa2.segments.iter() {
            let id = self.map_name(&seg.id)?;
            let mut new_seg: Segment<usize, T> = seg.nameless_clone();
            new_seg.id = id;
            segments.push(new_seg);
        }

        for frag in gfa2.fragments.iter() {
            let id = self.map_name(&frag.id)?;
            let mut new_frag: Fragment<usize, T> = frag.nameless_clone();
            new_frag.id = id;
            fragments.push(new_frag);
        }

        for edge in gfa2.edges.iter() {
            let id = self.map_name(&edge.id)?;
            let mut new_edge: Edge<usize, T> = edge.nameless_clone();
            new_edge.id = id;
            edges.push(new_edge);
        }

        for gap in gfa2.gaps.iter() {
            let id = self.map_name(&gap.id)?;
            let mut new_gap: Gap<usize, T> = gap.nameless_clone();
            new_gap.id = id;
            gaps.push(new_gap);
        }

        for ogroup in gfa2.groups_o.iter() {
            let new_ogroup = self.map_ogroup_segments(ogroup)?;
            ogroups.push(new_ogroup); 
        }

        for ugroup in gfa2.groups_u.iter() {
            let new_ugroup = self.map_ugroup_segments(ugroup)?;
            ugroups.push(new_ugroup); 
        }

        Some(GFA2 {
            headers: gfa2.headers.clone(),
            segments,
            fragments,
            edges,
            gaps,
            groups_o : ogroups,
            groups_u: ugroups,
        })
    }

    pub fn gfa2_usize_to_bstring<T: OptFields>(
        &self,
        gfa2: &GFA2<usize, T>,
    ) -> Option<GFA2<BString, T>> {

        let mut segments = Vec::with_capacity(gfa2.segments.len());
        let mut fragments = Vec::with_capacity(gfa2.fragments.len());
        let mut edges = Vec::with_capacity(gfa2.edges.len());
        let mut gaps = Vec::with_capacity(gfa2.gaps.len());
        let mut ogroups = Vec::with_capacity(gfa2.groups_o.len());
        let mut ugroups = Vec::with_capacity(gfa2.groups_u.len());

        for seg in gfa2.segments.iter() {
            let id = self.inverse_map_name(seg.id)?;
            let mut new_seg: Segment<BString, T> = seg.nameless_clone();
            new_seg.id = id.into();
            segments.push(new_seg);
        }

        for frag in gfa2.fragments.iter() {
            let id = self.inverse_map_name(frag.id)?;
            let mut new_frag: Fragment<BString, T> = frag.nameless_clone();
            new_frag.id = id.into();
            fragments.push(new_frag);
        }

        for edge in gfa2.edges.iter() {
            let id = self.inverse_map_name(edge.id)?;
            let mut new_edge: Edge<BString, T> = edge.nameless_clone();
            new_edge.id = id.into();
            edges.push(new_edge);
        }

        for gap in gfa2.gaps.iter() {
            let id = self.inverse_map_name(gap.id)?;
            let mut new_gap: Gap<BString, T> = gap.nameless_clone();
            new_gap.id = id.into();
            gaps.push(new_gap);
        }

        for ogroup in gfa2.groups_o.iter() {
            let new_ogroup = self.inverse_map_ogroup_segments(ogroup)?;
            ogroups.push(new_ogroup); 
        }

        for ugroup in gfa2.groups_u.iter() {
            let new_ugroup = self.inverse_map_ugroup_segments(ugroup)?;
            ugroups.push(new_ugroup); 
        }
        
        Some(GFA2 {
            headers: gfa2.headers.clone(),
            segments,
            fragments,
            edges,
            gaps,
            groups_o : ogroups,
            groups_u: ugroups,
        })
    }

    pub fn build_from_gfa2<T: OptFields>(gfa2: &GFA2<BString, T>) -> Self {
        let mut name_map = FnvHashMap::default();
        let mut inverse_map = Vec::with_capacity(gfa2.segments.len());

        let mut get_ix = |name: &BStr| {
            let name: BString = name.into();
            let vec_name = Vec::from(name.clone());
            if let Some(ix) = name_map.get(&vec_name) {
                *ix
            } else {
                let ix = name_map.len();
                name_map.insert(vec_name, ix);
                inverse_map.push(name);
                ix
            }
        };

        let hash = hash_gfa2(gfa2);

        for seg in gfa2.segments.iter() {
            get_ix(seg.id.as_ref());
        }

        for frag in gfa2.fragments.iter() {
            get_ix(frag.id.as_ref());
        }

        for edge in gfa2.edges.iter() {
            get_ix(edge.id.as_ref());
        }

        for gap in gfa2.gaps.iter() {
            get_ix(gap.id.as_ref());
        }

        NameMap {
            name_map,
            inverse_map,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_gfa2::GFA2Parser;

    fn irl() -> &'static str {
        "./test/gfa2_files/irl.gfa"
    }

    fn load_irl() -> GFA2<BString, OptionalFields> {
        let parser = GFA2Parser::new();
        let gfa2 : GFA2<BString, OptionalFields> =
            parser.parse_file(&"./test/gfa2_files/irl.gfa").unwrap();
        gfa2
    }

    #[test]
    fn irl_name_map_serde() {
        let gfa2 = load_irl();
        let name_map = NameMap::build_from_gfa2(&gfa2);

        let new_gfa = name_map.gfa2_bstring_to_usize(&gfa2, false).unwrap();

        let _ = std::fs::remove_file(irl());
        name_map.save_json(irl()).unwrap();
        let loaded_map = NameMap::load_json(irl()).unwrap();

        assert_eq!(name_map, loaded_map);

        let inverted_gfa = loaded_map.gfa2_usize_to_bstring(&new_gfa).unwrap();

        assert_eq!(gfa2, inverted_gfa);
    }
}