use super::*;

/// a. Read/write basic .obj files;
#[test]
fn load_test_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(7, mesh.vertices.len());
    assert_eq!(3, mesh.faces.len());
    Ok(())
}

/// a. Read/write basic .obj files;
#[test]
fn load_shuttle_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(310, mesh.vertices.len());
    // 170 tris and 223 quads in the shuttle
    assert_eq!(616, mesh.faces.len());
    Ok(())
}

/// a. Read/write basic .obj files;
#[test]
fn write_test_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::read(&input_path)?;
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/test.obj";
    mesh.write(&output_path)?;
    Ok(())
}

/// a. Read/write basic .obj files;
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
    vertices
        .first()
        .ok_or("no vertices")?
        .adjacent_vertices()
        .write(&output_path)?;
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
    faces
        .first()
        .ok_or("no faces")?
        .adjacent_faces()
        .write(&output_path)?;
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
    let vertex1 = { *mesh.vertex_list().first().ok_or("no vertices")? }.clone();
    mesh.remove_vertex(&vertex1, true);
    mesh.write(&output_path)?;
    assert_eq!(mesh.vertices.len(), count - 1);
    Ok(())
}

/// F. Construct a new face from vertices, and a new vertex from coordinates.
/// (new vertices and face added to model)
#[test]
fn add_face() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_with_new_face.obj";
    let mut mesh = Mesh::read(&input_path)?;
    let count = mesh.faces.len();
    let a = Vertex::new(Vector3::new([10., 0., 0.]));
    let b = Vertex::new(Vector3::new([0., 10., 0.]));
    let c = Vertex::new(Vector3::new([10., 0., 10.]));
    let face = Face::new([&a, &b, &c]);
    mesh.insert_face(&face);
    mesh.write(&output_path)?;
    assert_eq!(mesh.faces.len(), count + 1);
    Ok(())
}

/// G. Flip the sense of a face.
#[test]
fn flip_face() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_with_flipped_face.obj";
    let mut mesh = Mesh::read(&input_path)?;
    let face = { *mesh.face_list().first().ok_or("no faces")? }.clone();
    mesh.flip_face(&face);
    mesh.write(&output_path)?;
    Ok(())
}

/// 2. Write a function that returns whether all faces are consistently oriented.
#[test]
fn consistently_oriented() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    assert_eq!(mesh.consistent_orientation(), true);
    Ok(())
}

/// 2. Write a function that returns whether all faces are consistently oriented.
/// (Not consistently oriented because of flipped face )
#[test]
fn not_consistently_oriented() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    let face = { *mesh.face_list().first().ok_or("no faces")? }.clone();
    mesh.flip_face(&face);
    assert_eq!(mesh.consistent_orientation(), false);
    Ok(())
}

/// 3. Write a function that returns the number of loops bounding a surface mesh.
#[test]
fn surface_bounding_loop_count() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle_with_holes.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_with_holes_edges_from_loop_count.obj";
    let mesh = Mesh::read(&input_path)?;
    let (count, mesh) = mesh.surface_bounding_loop_count()?;
    mesh.write(&output_path)?;
    assert_eq!(count, 4);
    Ok(())
}

/// 4. Write a function that returns all faces with minimum angle below a specified angle in degrees.
/// I might have miss understood this task. However, this one is fun to play around with.
/// Try running the test multiple times with different angles and viewing the mesh in blender.
/// The faces HashSet gives different selections on each run (not good for consistent testing).
#[test]
fn grow_selection_with_max_angle() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle_selection_below_angle.obj";
    let mesh = Mesh::read(&input_path)?;
    let face = *mesh.face_list().first().ok_or("no faces")?;
    face.grow_selection_with_max_angle(15.)?
        .write(&output_path)?;
    Ok(())
}
