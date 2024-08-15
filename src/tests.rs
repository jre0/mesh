use super::*;

#[test]
fn load_test_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::import(&path)?;
    assert_eq!(7, mesh.vertices.len()/3);
    assert_eq!(3, mesh.faces.len()/3); 
    Ok(())
}

#[test]
fn load_shuttle_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::import(&path)?;
    assert_eq!(310, mesh.vertices.len()/3);
    // 170 tris and 223 quads
    assert_eq!(616, mesh.faces.len()/3); 
    Ok(())
}

#[test]
fn write_test_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::import(&input_path)?;
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/test.obj";
    mesh.export(&output_path)?;
    Ok(())
}

#[test]
fn write_shuttle_mesh() -> Result<(), Error> {
    let input_path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::import(&input_path)?;
    let output_path = TEST_OUTPUT_PATH.to_owned() + "/shuttle.obj";
    mesh.export(&output_path)?;
    Ok(())
}

