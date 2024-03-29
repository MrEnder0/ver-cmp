#[cfg(test)]
use super::*;

#[test]
fn test_is_ver_greater() {
    // Returns greater
    assert_eq!(
        compare_versions("4.2.3", "1.2.2").unwrap(),
        Ordering::Greater
    );
    // Returns less
    assert_eq!(compare_versions("0.5.3", "1.2.4").unwrap(), Ordering::Less);
    // Returns equal
    assert_eq!(compare_versions("0.1.0", "0.1.0").unwrap(), Ordering::Equal);
    // Returns error due to invalid version format for ver1
    assert_eq!(
        compare_versions("1.2", "1.2.3").unwrap_err(),
        "Invalid version format"
    );
    // Returns error due to invalid version format for ver2
    assert_eq!(
        compare_versions("0.2.3", "1.2").unwrap_err(),
        "Invalid version format"
    );
    // Compares versions with leading zeros, should return equal
    assert_eq!(
        compare_versions("01.2.3", "1.2.3").unwrap(),
        Ordering::Equal
    );
}

#[test]
fn test_greater_ver() {
    // Returns 7.3.5 as it is greater
    assert_eq!(greater_ver("7.3.5", "3.2.1").unwrap(), "7.3.5");
    // Returns 2.3.8 as it is greater
    assert_eq!(greater_ver("1.1.1", "2.3.8").unwrap(), "2.3.8");
    // Returns 0.0.1 as both are equal
    assert_eq!(greater_ver("0.0.1", "0.0.1").unwrap(), "0.0.1");
    // Returns error due to invalid version format for ver1
    assert_eq!(
        greater_ver("1.2", "3.2.1").unwrap_err(),
        "Unable to compare versions due to invalid version format"
    );
    // Returns error due to invalid version format for ver2
    assert_eq!(
        greater_ver("2.3.4", "4.2").unwrap_err(),
        "Unable to compare versions due to invalid version format"
    );
}

#[test]
fn test_is_version_between() {
    assert_eq!(is_ver_between("1.0.0", "0.9.0", "1.1.0"), Ok(true));
    assert_eq!(is_ver_between("1.0.0", "1.0.0", "1.1.0"), Ok(true));
    assert_eq!(is_ver_between("1.0.0", "0.9.0", "0.9.9"), Ok(false));
    assert_eq!(is_ver_between("1.0.0", "1.1.0", "2.0.0"), Ok(false));
    assert_eq!(
        is_ver_between("1.0.0", "1.0", "1.1.0"),
        Err("Invalid version format")
    );
    assert_eq!(
        is_ver_between("1.0.0", "1.0.0.0", "1.1.0"),
        Err("Invalid version format")
    );
}
