use crate::mesh_query::{EvalMeshQuery, MeshQuery};
use crate::smesh::{Connectivity, FaceId, HalfedgeId, SMesh, VertexId};

pub struct HalfedgeAroundVertexIter<'a> {
    conn: &'a Connectivity,
    start: HalfedgeId,
    current: Option<HalfedgeId>,
}

impl<'a> HalfedgeAroundVertexIter<'a> {
    pub fn new(
        connectivity: &'a Connectivity,
        vertex_id: VertexId,
    ) -> HalfedgeAroundVertexIter<'a> {
        let start = connectivity.q(vertex_id).halfedge().id().unwrap();
        HalfedgeAroundVertexIter {
            conn: connectivity,
            start,
            current: Some(start),
        }
    }
}

impl<'a> Iterator for HalfedgeAroundVertexIter<'a> {
    type Item = HalfedgeId;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current else {
            return None;
        };
        let next = self.conn.q(current).ccw_rotated_neighbour().id().unwrap();
        self.current = if next == self.start { None } else { Some(next) };
        Some(current)
    }
}

pub struct VertexAroundVertexIter<'a> {
    conn: &'a Connectivity,
    start: HalfedgeId,
    current: Option<HalfedgeId>,
}

impl<'a> VertexAroundVertexIter<'a> {
    pub fn new(connectivity: &'a Connectivity, vertex_id: VertexId) -> VertexAroundVertexIter<'a> {
        let start = connectivity.q(vertex_id).halfedge().id().unwrap();
        VertexAroundVertexIter {
            conn: connectivity,
            start,
            current: Some(start),
        }
    }
}

impl<'a> Iterator for VertexAroundVertexIter<'a> {
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current else {
            return None;
        };
        let dst_vert = self.conn.q(current).dst_vert().id();
        let next = self.conn.q(current).ccw_rotated_neighbour().id().unwrap();
        self.current = if next == self.start { None } else { Some(next) };
        dst_vert.ok()
    }
}

pub struct VertexAroundFaceIter<'a> {
    conn: &'a Connectivity,
    start: HalfedgeId,
    current: Option<HalfedgeId>,
}

impl<'a> VertexAroundFaceIter<'a> {
    pub fn new(connectivity: &'a Connectivity, face_id: FaceId) -> VertexAroundFaceIter<'a> {
        let start = connectivity.q(face_id).halfedge().id().unwrap();
        VertexAroundFaceIter {
            conn: connectivity,
            start,
            current: Some(start),
        }
    }
}

impl<'a> Iterator for VertexAroundFaceIter<'a> {
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current else {
            return None;
        };
        let dst_vert = self.conn.q(current).dst_vert().id();
        let next = self.conn.q(current).next().id().unwrap();
        self.current = if next == self.start { None } else { Some(next) };
        dst_vert.ok()
    }
}

impl MeshQuery<'_, VertexId> {
    pub fn vertices(&self) -> VertexAroundVertexIter {
        VertexAroundVertexIter::new(&self.conn, self.id().unwrap())
    }

    pub fn halfedges(&self) -> HalfedgeAroundVertexIter {
        HalfedgeAroundVertexIter::new(&self.conn, self.id().unwrap())
    }
}

impl MeshQuery<'_, FaceId> {
    pub fn vertices(&self) -> VertexAroundFaceIter {
        VertexAroundFaceIter::new(&self.conn, self.id().unwrap())
    }
}

mod test {
    use super::*;
    use crate::smesh::SMesh;
    use glam::vec3;
    use itertools::Itertools;

    #[test]
    fn vertex_around_vertex() {
        let mesh = &mut SMesh::new();

        let v0 = mesh.add_vertex(vec3(-1.0, -1.0, 0.0));
        let v1 = mesh.add_vertex(vec3(1.0, -1.0, 0.0));
        let v2 = mesh.add_vertex(vec3(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(vec3(-1.0, 1.0, 0.0));
        let v4 = mesh.add_vertex(vec3(0.0, -2.0, 0.0));

        let _ = mesh.add_face(vec![v0, v1, v2, v3]);
        let _ = mesh.add_face(vec![v0, v4, v1]);

        let mut ids = vec![];
        for v_id in mesh.q(v0).vertices() {
            println!("{:?}", v_id);
            ids.push(v_id);
        }
        assert_eq!(ids, vec![v3, v4, v1]);
    }

    #[test]
    fn vertex_around_face() {
        let mesh = &mut SMesh::new();

        let v0 = mesh.add_vertex(vec3(-1.0, -1.0, 0.0));
        let v1 = mesh.add_vertex(vec3(1.0, -1.0, 0.0));
        let v2 = mesh.add_vertex(vec3(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(vec3(-1.0, 1.0, 0.0));
        let v4 = mesh.add_vertex(vec3(0.0, -2.0, 0.0));

        let f0 = mesh.add_face(vec![v0, v1, v2, v3]).unwrap();
        let f1 = mesh.add_face(vec![v0, v4, v1]).unwrap();

        let mut ids = mesh.q(f0).vertices().collect_vec();
        assert_eq!(ids, vec![v0, v1, v2, v3]);
        ids = mesh.q(f1).vertices().collect_vec();
        assert_eq!(ids, vec![v0, v4, v1,]);
    }
}