//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // tests7: 设置 TEST_FOO 环境变量为当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8: 启用名为 "pass" 的 feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

