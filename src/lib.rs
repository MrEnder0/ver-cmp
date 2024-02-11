mod test;

pub fn is_ver_greater(ver1: &str, ver2: &str) -> Result<bool, &'static str> {
    let ver1: Vec<u32> = ver1.split('.').map(|s| s.parse().unwrap()).collect();
    let ver2: Vec<u32> = ver2.split('.').map(|s| s.parse().unwrap()).collect();

    // If the version is not in the format x.y.z, return None
    if ver1.len() != 3 || ver2.len() != 3 {
        return Err("Invalid version format");
    }

    for (v1, v2) in ver1.iter().zip(ver2.iter()) {
        match v1.cmp(v2) {
            std::cmp::Ordering::Greater => return Ok(true),
            std::cmp::Ordering::Less => return Ok(false),
            _ => continue,
        }
    }
    
    Err("Versions are equal")
}

pub fn greater_ver(ver1: &str, ver2: &str) -> Result<String, &'static str> {
    let greater = match is_ver_greater(ver1, ver2) {
        Ok(true) => ver1,
        Ok(false) => ver2,
        Err(_) => return Err("Unable to compare versions due to invalid version format"),
    };

    Ok(greater.to_string())
}
