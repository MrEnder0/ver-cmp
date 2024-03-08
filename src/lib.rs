use std::cmp::Ordering;

mod test;

/// Compares two versions in the format x.y.z and returns the ordering
pub fn compare_versions(ver1: &str, ver2: &str) -> Result<Ordering, &'static str> {
    let ver1: Vec<u16> = ver1.split('.').map(|s| s.parse().unwrap()).collect();
    let ver2: Vec<u16> = ver2.split('.').map(|s| s.parse().unwrap()).collect();

    // Return an error if the versions is not in the format x.y.z
    if ver1.len() != 3 || ver2.len() != 3 {
        return Err("Invalid version format");
    }

    for (v1, v2) in ver1.iter().zip(ver2.iter()) {
        match v1.cmp(v2) {
            std::cmp::Ordering::Greater => return Ok(Ordering::Greater),
            std::cmp::Ordering::Less => return Ok(Ordering::Less),
            _ => continue,
        }
    }

    Ok(Ordering::Equal)
}

/// Returns the greater of two versions in the x.y.z format
pub fn greater_ver(ver1: &str, ver2: &str) -> Result<String, &'static str> {
    let greater = match compare_versions(ver1, ver2) {
        Ok(Ordering::Greater) => ver1,
        Ok(Ordering::Less) => ver2,
        Ok(Ordering::Equal) => ver1,
        Err(_) => return Err("Unable to compare versions due to invalid version format"),
    };

    Ok(greater.to_string())
}

/// Returns the lesser of two versions in the x.y.z format
pub fn lesser_ver(ver1: &str, ver2: &str) -> Result<String, &'static str> {
    let lesser = match compare_versions(ver1, ver2) {
        Ok(Ordering::Greater) => ver2,
        Ok(Ordering::Less) => ver1,
        Ok(Ordering::Equal) => ver1,
        Err(_) => return Err("Unable to compare versions due to invalid version format"),
    };

    Ok(lesser.to_string())
}

/// Returns true if the version is between the lower and upper bounds
pub fn is_ver_between(ver: &str, lower: &str, upper: &str) -> Result<bool, &'static str> {
    let lower = compare_versions(ver, lower)?;
    let upper = compare_versions(ver, upper)?;

    if (lower == Ordering::Greater || lower == Ordering::Equal)
        && (upper == Ordering::Less || upper == Ordering::Equal)
    {
        return Ok(true);
    }

    Ok(false)
}
