use super::*;

#[test]
fn load_test_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(7, mesh.vertices.len());
    assert_eq!(3, mesh.faces.len()); 
    Ok(())
}

#[test]
fn load_shuttle_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(310, mesh.vertices.len());
    // 170 tris and 223 quads in the shuttle
    assert_eq!(616, mesh.faces.len()); 
    Ok(())
}

#[test]
fn write_test_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::read(&input_path)?;
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/test.obj";
    mesh.write(&output_path)?;
    Ok(())
}

#[test]
fn write_shuttle_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle.obj";
    mesh.write(&output_path)?;
    Ok(())
}


/// B. Given a vertex/face, return the adjacent faces/vertices.
/// (vertex)
#[test]
fn select_adjacent_vertices_by_vertex() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_adjacent_verts.obj";
    let mesh = Mesh::read(&input_path)?;
    let vertices = mesh.vertex_list();
    vertices.first().ok_or("no vertices")?.adjacent_vertices().write(&output_path)?;
    Ok(())
}


/// B. Given a vertex/face, return the adjacent faces/vertices.
/// (face)
#[test]
fn select_adjacent_face_by_face() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_adjacent_faces.obj";
    let mesh = Mesh::read(&input_path)?;
    let faces = mesh.face_list();
    faces.first().ok_or("no faces")?.adjacent_faces().write(&output_path)?;
    Ok(())
}

/// C. Return all the vertices or faces. (both)
#[test]
fn list_all_vertices_and_faces_and_write_new_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_from_all_faces.obj";
    let mesh = Mesh::read(&input_path)?;
    // don't need verts to write because write will get verts from faces
    let _ = mesh.vertex_list();
    let faces = mesh.face_list();
    let mut mesh = Mesh::default();
    mesh.faces.extend(faces.iter().map(|x| (*x).clone()));
    mesh.write(&output_path)?;
    Ok(())
}

/// D. Return the coordinates of a given vertex.
#[test]
fn coordinates_of_vertex() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let vertices = mesh.vertex_list();
    let point = vertices.first().ok_or("no vertices")?.point();
    assert!(!point.x.is_nan());
    assert!(!point.y.is_nan());
    assert!(!point.z.is_nan());
    Ok(())
}

/// E. Delete a vertex or face, with optional flag to delete all connected faces (if a vertex).
/// (Vertex)
#[test]
fn delete_vertex() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_deleted_vertex.obj";
    let mut mesh = Mesh::read(&input_path)?;
    let count = mesh.vertices.len();
    let vertex1 = {
        *mesh.vertex_list().first().ok_or("no vertices")?
    }.clone();
    mesh.remove_vertex(&vertex1, true);
    mesh.write(&output_path)?;
    assert_eq!(mesh.vertices.len(), count - 1);
    Ok(())
}









// #[test]
// fn select_adjacent_face_by_face() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_adjacent_faces.obj";
//     let mesh = Mesh::read(&input_path)?;
//     let faces = mesh.face_list();
//     faces.first().ok_or("no faces")?.adjacent_faces().write(&output_path)?;
//     Ok(())
// }



// /// Need to do asserts to check validity!!!
// #[test]
// fn select_all() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mesh = Mesh::read(&input_path)?;
//     let _ = mesh.select_all();
//     Ok(())
// }

// #[test]
// fn vertex_coordinates() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mesh = Mesh::read(&input_path)?;
//     let _ = mesh.vertex_coordinates(142);
//     Ok(())
// }

// #[test]
// fn delete_vertex() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mut mesh = Mesh::read(&input_path)?;
//     mesh.delete_vertex(142, false);
//     Ok(())
// }

// #[test]
// fn delete_vertex_with_faces() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mut mesh = Mesh::read(&input_path)?;
//     mesh.delete_vertex(142, true);
//     Ok(())
// }

// #[test]
// fn add_vertex() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mut mesh = Mesh::read(&input_path)?;
//     mesh.add_vertex(Vector3::new(&[0., 0., 0.]));
//     Ok(())
// }

// #[test]
// fn add_face() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mut mesh = Mesh::read(&input_path)?;
//     mesh.add_vertex(Vector3::new(&[0., 0., 0.]));
//     mesh.add_vertex(Vector3::new(&[0., 1., 0.]));
//     mesh.add_vertex(Vector3::new(&[0., 0., 1.]));
//     mesh.add_face([310, 311, 312]);
//     Ok(())
// }

// #[test]
// fn flip_face() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mut mesh = Mesh::read(&input_path)?;
//     mesh.flip_face(37);
//     Ok(())
// }

// #[test]
// fn consistent_orientation() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mesh = Mesh::read(&input_path)?;
//     let _ = mesh.consistent_orientation();
//     Ok(())
// }

// #[test]
// fn faces_with_minimum_angle() -> Result<(), Error> {
//     let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
//     let mesh = Mesh::read(&input_path)?;
//     let _ = mesh.faces_with_minimum_angle(10.);
//     Ok(())
// }


