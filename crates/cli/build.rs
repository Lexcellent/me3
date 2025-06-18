fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_manifest_file("resources/manifest.xml");

        // 添加图标格式兼容性处理
        res.set_icon_with_id("resources/me3.ico", "MAINICON");

        // 使用更安全的编译方式
        if let Err(e) = res.compile() {
            eprintln!("⚠️ 资源编译失败: {}", e);
            eprintln!("💡 尝试将 ICO 文件转换为传统格式 (Windows 3.1)");
            // 非致命错误，不中断构建
        }
    }
}
