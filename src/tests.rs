use super::*;

#[test]
fn load_test_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/test.obj";
    let mesh = Mesh::import(&path)?;
    assert_eq!(7, mesh.vertices.len());
    assert_eq!(3, mesh.faces.len()); 
    Ok(())
}

#[test]
fn load_shuttle_mesh() -> Result<(), Error> {
    let path = TEST_DATA_PATH.to_owned() + "/shuttle.obj";
    let mesh = Mesh::import(&path)?;
    assert_eq!(310, mesh.vertices.len());
    // 170 tris and 223 quads
    assert_eq!(616, mesh.faces.len()); 
    Ok(())
}

// #[test]
// fn write_test_mesh() -> Result<(), Error> {
//     let path = TEST_DATA_PATH.to_owned() + "/test.obj";
//     let mesh = Mesh::import(&path)?;
//     assert_eq!(7, mesh.vertices.len());
//     assert_eq!(3, mesh.faces.len()); 
//     Ok(())
// }