use anyhow::Result;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

pub struct Compiler {
    project_type: ProjectType,
}

#[derive(Debug, Clone)]
pub enum ProjectType {
    CMake,
    Make,
    QMake,
    Conan,
    Cargo,
    Go,
    Node,
    Python,
    Generic,
}

#[derive(Debug, Clone, Default)]
pub struct BuildConfig {
    pub install_root: Option<String>,
    pub qmake_args: Vec<String>,
    pub make_args: Vec<String>,
    pub use_qt_ifw_style: bool,
    pub bindist_target: Option<String>,
}

impl Compiler {
    pub fn new(project_type: ProjectType) -> Self {
        Self { project_type }
    }

    pub fn with_config(project_type: ProjectType, _config: &BuildConfig) -> Self {
        Self { project_type }
    }

    pub fn detect_from_dir(dir: &Path) -> Result<ProjectType> {
        // 优先检测 conanfile.py (Python 文件，需要最先检测)
        if dir.join("conanfile.py").exists() {
            return Ok(ProjectType::Conan);
        }
        if dir.join("conanfile.txt").exists() {
            return Ok(ProjectType::Conan);
        }
        
        // 检测 CMakeLists.txt
        if dir.join("CMakeLists.txt").exists() {
            return Ok(ProjectType::CMake);
        }
        
        // 检测 Makefile
        if dir.join("Makefile").exists() || dir.join("makefile").exists() {
            return Ok(ProjectType::Make);
        }
        
        // 检测 .pro 文件 (QMake 项目)
        if let Ok(entries) = dir.read_dir() {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pro") {
                    return Ok(ProjectType::QMake);
                }
            }
        }
        
        // 检测 Cargo.toml
        if dir.join("Cargo.toml").exists() {
            return Ok(ProjectType::Cargo);
        }
        
        // 检测 go.mod
        if dir.join("go.mod").exists() {
            return Ok(ProjectType::Go);
        }
        
        // 检测 package.json
        if dir.join("package.json").exists() {
            return Ok(ProjectType::Node);
        }
        
        // 检测 Python 项目
        if dir.join("setup.py").exists() || dir.join("pyproject.toml").exists() {
            return Ok(ProjectType::Python);
        }
        
        Ok(ProjectType::Generic)
    }

    // 从项目文件读取版本信息
    pub fn read_version_from_dir(dir: &Path) -> Option<String> {
        // 从 conanfile.py 读取 (Python 文件，需要最先检测)
        if let Ok(content) = std::fs::read_to_string(dir.join("conanfile.py")) {
            // 简单的 Python 解析：查找 version = "x.x.x" 或 version = 'x.x.x'
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("version") && line.contains('=') {
                    // 查找 = 后面的内容
                    if let Some(eq_pos) = line.find('=') {
                        let value_part = line[eq_pos + 1..].trim();
                        // 提取引号内的版本号
                        if let Some(first_quote) = value_part.chars().next() {
                            if first_quote == '"' || first_quote == '\'' {
                                if let Some(end_quote) = value_part[1..].find(first_quote) {
                                    return Some(value_part[1..end_quote + 1].to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // 从 CMakeLists.txt 读取
        if let Ok(content) = std::fs::read_to_string(dir.join("CMakeLists.txt")) {
            for line in content.lines() {
                if line.trim().starts_with("VERSION") || line.trim().starts_with("version") {
                    if let Some(ver) = line.split_whitespace().nth(1) {
                        return Some(ver.trim_matches('"').trim_matches('\'').to_string());
                    }
                }
                // 支持 project(xxx VERSION x.x.x) 格式
                if line.contains("project(") && line.contains("VERSION") {
                    let parts: Vec<&str> = line.split("VERSION").collect();
                    if parts.len() > 1 {
                        let ver = parts[1].trim().split_whitespace().next().unwrap_or("1.0.0");
                        return Some(ver.trim_matches('"').trim_matches('\'').to_string());
                    }
                }
            }
        }
        
        // 从 package.json 读取
        if let Ok(content) = std::fs::read_to_string(dir.join("package.json")) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(ver) = json["version"].as_str() {
                    return Some(ver.to_string());
                }
            }
        }
        
        // 从 Cargo.toml 读取
        if let Ok(content) = std::fs::read_to_string(dir.join("Cargo.toml")) {
            for line in content.lines() {
                if line.trim().starts_with("version") {
                    if let Some(ver) = line.split('=').nth(1) {
                        return Some(ver.trim().trim_matches('"').trim_matches('\'').to_string());
                    }
                }
            }
        }
        
        // 从 conanfile.txt 读取
        if let Ok(content) = std::fs::read_to_string(dir.join("conanfile.txt")) {
            for line in content.lines() {
                if line.trim().starts_with("version") {
                    if let Some(ver) = line.split('=').nth(1) {
                        return Some(ver.trim().to_string());
                    }
                }
            }
        }
        
        // 从 .pro 和 .pri 文件读取 (qmake 项目)
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = path.extension().and_then(|s| s.to_str());
                if ext == Some("pro") || ext == Some("pri") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        // 先收集所有变量定义
                        let mut variables = std::collections::HashMap::new();
                        for line in content.lines() {
                            let line = line.trim();
                            if line.contains('=') && !line.starts_with('#') {
                                let parts: Vec<&str> = line.splitn(2, '=').collect();
                                if parts.len() == 2 {
                                    let key = parts[0].trim();
                                    let value = parts[1].trim();
                                    variables.insert(key.to_string(), value.to_string());
                                }
                            }
                        }

                        // 优先使用 CALIBRATIONTOOL_DISPLAY_VERSION（最详细的版本号）
                        if let Some(ver) = variables.get("CALIBRATIONTOOL_DISPLAY_VERSION") {
                            let clean_ver = ver.trim_matches('"').trim_matches('\'');
                            // 跳过包含 ${ 的未解析变量
                            if !clean_ver.contains("${") && !clean_ver.starts_with("$$") {
                                return Some(clean_ver.to_string());
                            }
                        }

                        // 查找 VERSION，支持变量引用
                        if let Some(version_value) = variables.get("VERSION") {
                            let ver = if version_value.starts_with("$$") {
                                // 是变量引用，如 $$CALIBRATIONTOOL_VERSION
                                let var_name = version_value.trim_start_matches("$$");
                                variables.get(var_name).map(|v| v.as_str()).unwrap_or("1.0.0")
                            } else {
                                version_value.as_str()
                            };
                            let clean_ver = ver.trim_matches('"').trim_matches('\'');
                            // 跳过包含 ${ 的未解析变量
                            if !clean_ver.contains("${") {
                                return Some(clean_ver.to_string());
                            }
                        }

                        // 也支持 PROJECT_VERSION
                        if let Some(ver) = variables.get("PROJECT_VERSION") {
                            let clean_ver = ver.trim_matches('"').trim_matches('\'');
                            if !clean_ver.contains("${") && !clean_ver.starts_with("$$") {
                                return Some(clean_ver.to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn build(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildResult> {
        self.build_with_config(source_dir, output_dir, &BuildConfig::default())
    }

    pub fn build_with_config(&self, source_dir: &Path, output_dir: &Path, config: &BuildConfig) -> Result<BuildResult> {
        let start = Instant::now();

        std::fs::create_dir_all(output_dir)?;

        let result = match &self.project_type {
            ProjectType::CMake => self.build_cmake(source_dir, output_dir),
            ProjectType::Make => self.build_make(source_dir, output_dir),
            ProjectType::QMake => self.build_qmake_with_config(source_dir, output_dir, config),
            ProjectType::Conan => self.build_conan(source_dir, output_dir),
            ProjectType::Cargo => self.build_cargo(source_dir, output_dir),
            ProjectType::Go => self.build_go(source_dir, output_dir),
            ProjectType::Node => self.build_node(source_dir, output_dir),
            ProjectType::Python => self.build_python(source_dir, output_dir),
            ProjectType::Generic => Ok(BuildOutput {

                message: "No compilation needed for generic project".to_string()
            }),
        };

        let duration = start.elapsed();

        match result {
            Ok(output) => Ok(BuildResult {

                output,
                duration_ms: duration.as_millis(),
            }),
            Err(e) => Ok(BuildResult {
                
                output: BuildOutput {
                    
                    message: format!("Build failed: {}", e),
                },
                duration_ms: duration.as_millis(),
            }),
        }
    }

    #[cfg(unix)]
    fn build_cmake(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let status = Command::new("cmake")
            .args(["-S", source_dir.to_str().unwrap()])
            .args(["-B", build_dir.to_str().unwrap()])
            .args(["-DCMAKE_BUILD_TYPE=Release"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                
                message: "CMake configuration failed".to_string(),
            });
        }

        let status = Command::new("cmake")
            .args(["--build", build_dir.to_str().unwrap()])
            .args(["--config", "Release"])
            .args(["-j", &num_cpus::get().to_string()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "CMake build completed successfully".to_string()
            } else {
                "CMake build failed".to_string()
            },
        })
    }

    #[cfg(windows)]
    fn build_cmake(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let status = Command::new("cmake")
            .args(["-S", source_dir.to_str().unwrap()])
            .args(["-B", build_dir.to_str().unwrap()])
            .args(["-DCMAKE_BUILD_TYPE=Release"])
            .args(["-G", "Visual Studio 17 2022"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        if let Ok(status) = status {
            if !status.success() {
                return Ok(BuildOutput {
                    
                    message: "CMake configuration failed".to_string(),
                });
            }
        }

        let status = Command::new("cmake")
            .args(["--build", build_dir.to_str().unwrap()])
            .args(["--config", "Release"])
            .args(["-j", &num_cpus::get().to_string()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "CMake build completed successfully".to_string()
            } else {
                "CMake build failed".to_string()
            },
        })
    }

    #[cfg(unix)]
    fn build_make(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let status = Command::new("make")
            .args(["-j", &num_cpus::get().to_string()])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Make build completed successfully".to_string()
            } else {
                "Make build failed".to_string()
            },
        })
    }

    #[cfg(windows)]
    fn build_make(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let status = Command::new("nmake")
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        if status.is_err() {
            return Ok(BuildOutput {
                
                message: "Make not available on Windows. Use CMake or MSBuild instead.".to_string(),
            });
        }

        let status = status.unwrap();

        Ok(BuildOutput {
            
            message: if status.success() {
                "Make build completed successfully".to_string()
            } else {
                "Make build failed".to_string()
            },
        })
    }

    #[cfg(unix)]
    fn build_qmake(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        self.build_qmake_with_config(source_dir, output_dir, &BuildConfig::default())
    }

    #[cfg(unix)]
    fn build_qmake_with_config(&self, source_dir: &Path, output_dir: &Path, config: &BuildConfig) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let qmake_cmd = if cfg!(windows) { "qmake" } else { "qmake-qt5" };
        
        // 1. qmake 配置
        println!("[INFO] 运行 qmake 配置项目...");
        
        // 添加 INSTALL_ROOT 参数 - 使用 String 避免生命周期问题
        let install_root = config.install_root.clone()
            .unwrap_or_else(|| format!("{}/install", output_dir.display()));
        let install_root_arg = format!("INSTALL_ROOT={}", install_root);
        
        let mut qmake_args = vec![source_dir.to_str().unwrap()];
        qmake_args.extend(config.qmake_args.iter().map(|s| s.as_str()));
        qmake_args.push(&install_root_arg);
        
        let status = Command::new(qmake_cmd)
            .args(&qmake_args)
            .current_dir(&build_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                message: "QMake configuration failed".to_string(),
            });
        }

        // 2. make 编译
        println!("[INFO] 正在编译 (make -j{})...", num_cpus::get());
        let num_jobs = num_cpus::get().to_string();
        let mut make_args = vec!["-j", num_jobs.as_str()];
        make_args.extend(config.make_args.iter().map(|s| s.as_str()));
        
        let status = Command::new("make")
            .args(&make_args)
            .current_dir(&build_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                message: "Make compilation failed".to_string(),
            });
        }

        // 3. make install (如果使用 Qt IFW 风格)
        if config.use_qt_ifw_style {
            println!("[INFO] 正在安装到 {}...", install_root);
            let status = Command::new("make")
                .arg("install")
                .current_dir(&build_dir)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()?;

            if !status.success() {
                return Ok(BuildOutput {
                    message: "Make install failed".to_string(),
                });
            }

            // 4. make bindist_installer (如果指定了)
            if let Some(ref bindist_target) = config.bindist_target {
                println!("[INFO] 正在创建安装包 (make {})...", bindist_target);
                let status = Command::new("make")
                    .arg(bindist_target)
                    .current_dir(&build_dir)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()?;

                if !status.success() {
                    return Ok(BuildOutput {
                        message: format!("Make {} failed", bindist_target),
                    });
                }
            }
        }

        Ok(BuildOutput {
            message: "QMake build completed successfully".to_string(),
        })
    }

    #[cfg(windows)]
    fn build_qmake(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let status = Command::new("qmake")
            .arg(source_dir.to_str().unwrap())
            .current_dir(&build_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                
                message: "QMake configuration failed".to_string(),
            });
        }

        let status = Command::new("nmake")
            .current_dir(&build_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        if status.is_err() {
            return Ok(BuildOutput {
                
                message: "nmake not found. Please install Qt with nmake support.".to_string(),
            });
        }

        let status = status.unwrap();

        Ok(BuildOutput {
            
            message: if status.success() {
                "QMake build completed successfully".to_string()
            } else {
                "QMake build failed".to_string()
            },
        })
    }

    #[cfg(unix)]
    fn build_conan(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let status = Command::new("conan")
            .args(["install", "."])
            .args(["--build=missing"])
            .args(["-if", &build_dir.to_str().unwrap()])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                
                message: "Conan install failed".to_string(),
            });
        }

        let status = Command::new("conan")
            .args(["build", "."])
            .args(["-if", &build_dir.to_str().unwrap()])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Conan build completed successfully".to_string()
            } else {
                "Conan build failed".to_string()
            },
        })
    }

    #[cfg(windows)]
    fn build_conan(&self, source_dir: &Path, output_dir: &Path) -> Result<BuildOutput> {
        let build_dir = output_dir.join("build");
        std::fs::create_dir_all(&build_dir)?;

        let status = Command::new("conan")
            .args(["install", "."])
            .args(["--build=missing"])
            .args(["-if", &build_dir.to_str().unwrap()])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Ok(BuildOutput {
                
                message: "Conan install failed".to_string(),
            });
        }

        let status = Command::new("conan")
            .args(["build", "."])
            .args(["-if", &build_dir.to_str().unwrap()])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Conan build completed successfully".to_string()
            } else {
                "Conan build failed".to_string()
            },
        })
    }

    fn build_cargo(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Cargo build completed successfully".to_string()
            } else {
                "Cargo build failed".to_string()
            },
        })
    }

    fn build_go(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let status = Command::new("go")
            .args(["build", "-o", "app"])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Go build completed successfully".to_string()
            } else {
                "Go build failed".to_string()
            },
        })
    }

    fn build_node(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
        
        let status = Command::new(npm_cmd)
            .args(["run", "build"])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Node build completed successfully".to_string()
            } else {
                "Node build failed".to_string()
            },
        })
    }

    fn build_python(&self, source_dir: &Path, _output_dir: &Path) -> Result<BuildOutput> {
        let python_cmd = if cfg!(windows) { "python" } else { "python3" };
        
        let status = Command::new(python_cmd)
            .args(["-m", "build"])
            .current_dir(source_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        Ok(BuildOutput {
            
            message: if status.success() {
                "Python build completed successfully".to_string()
            } else {
                "Python build failed".to_string()
            },
        })
    }
}

#[derive(Debug)]
pub struct BuildResult {
    pub output: BuildOutput,
    pub duration_ms: u128,
}

#[derive(Debug)]
pub struct BuildOutput {
    pub message: String,
}