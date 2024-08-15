use super::*;

#[test]
fn load_test_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(7, mesh.vertices.len()/3);
    assert_eq!(3, mesh.faces.len()/3); 
    Ok(())
}

#[test]
fn load_shuttle_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&path)?;
    assert_eq!(310, mesh.vertices.len()/3);
    // 170 tris and 223 quads
    assert_eq!(616, mesh.faces.len()/3); 
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

#[test]
fn select_adjacent() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let selection = mesh.select_adjacent_by_vertex_index(142);
    Ok(())
}

/// Need to do asserts to check validity!!!
#[test]
fn select_all() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let _ = mesh.select_all();
    Ok(())
}

#[test]
fn vertex_coordinates() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let _ = mesh.vertex_coordinates(142);
    Ok(())
}

#[test]
fn delete_vertex() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    mesh.delete_vertex(142, false);
    Ok(())
}

#[test]
fn delete_vertex_with_faces() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    mesh.delete_vertex(142, true);
    Ok(())
}

#[test]
fn add_vertex() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    mesh.add_vertex(Vector3::new(&[0., 0., 0.]));
    Ok(())
}

#[test]
fn add_face() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    mesh.add_vertex(Vector3::new(&[0., 0., 0.]));
    mesh.add_vertex(Vector3::new(&[0., 1., 0.]));
    mesh.add_vertex(Vector3::new(&[0., 0., 1.]));
    mesh.add_face([310, 311, 312]);
    Ok(())
}

#[test]
fn flip_face() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mut mesh = Mesh::read(&input_path)?;
    mesh.flip_face(37);
    Ok(())
}

#[test]
fn consistent_orientation() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let _ = mesh.consistent_orientation();
    Ok(())
}

#[test]
fn faces_with_minimum_angle() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::read(&input_path)?;
    let _ = mesh.faces_with_minimum_angle(10.);
    Ok(())
}


