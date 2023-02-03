/// The **target triple** to use when compiling the input file.
/// defaults to `TargetTriple::Native`
///
/// The target triple is used to specify the **target architecture** and **operating system** that the
/// executable will be compiled for. This is used to **generate** the **correct** **machine code** for the
/// target architecture and operating system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetTriple {
    /// The **native target triple** for the current machine.
    Native(TargetTripleData),

    /// A **custom target triple**.
    /// Useful for **cross-compiling**.
    Custom(TargetTripleData),
}

/// The **underlying data** for a [`TargetTriple`].
///
/// This is used to specify the **target architecture**, **operating system**, and **environment** that the
/// executable will be compiled for. This is used to **generate** the **correct** **machine code** for the
/// target architecture and operating system.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::settings::{TargetTriple, TargetTripleData, TargetArch, TargetOs, TargetEnv};
///
/// // The default target triple is `TargetTriple::Native`.
/// // The default target triple data is `TargetTripleData::Native`.
/// // The default target architecture is `TargetArch::Native`.
/// // The default target operating system is `TargetOs::Native`.
/// // The default target environment is `TargetEnv::Native`.
/// assert_eq!(TargetTriple::Native(TargetTripleData::new()), TargetTriple::default());
/// assert_eq!(TargetTripleData::Native, TargetTripleData::default());
/// assert_eq!(TargetArch::Native, TargetArch::default());
/// assert_eq!(TargetOs::Native, TargetOs::default());
/// assert_eq!(TargetEnv::Native, TargetEnv::default());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetTripleData {
    /// The **target architecture** to use when compiling the input file.
    pub arch: TargetArch,

    /// The **target operating system** to use when compiling the input file.
    pub os: TargetOs,

    /// The **target environment** to use when compiling the input file.
    pub env: TargetEnv,
}

impl TargetTripleData {
    /// Creates a new [`TargetTripleData`] with the **default** values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_cfg::settings::{TargetTriple, TargetTripleData, TargetArch, TargetOs, TargetEnv};
    ///
    /// // The default target triple is `TargetTriple::Native`.
    /// // The default target triple data is `TargetTripleData::Native`.
    /// // The default target architecture is `TargetArch::Native`.
    /// // The default target operating system is `TargetOs::Native`.
    /// // The default target environment is `TargetEnv::Native`.
    /// assert_eq!(TargetTriple::Native(TargetTripleData::new()), TargetTriple::default());
    /// assert_eq!(TargetTripleData::Native, TargetTripleData::default());
    /// assert_eq!(TargetArch::Native, TargetArch::default());
    /// assert_eq!(TargetOs::Native, TargetOs::default());
    /// assert_eq!(TargetEnv::Native, TargetEnv::default());
    /// ```
    pub fn new() -> TargetTripleData {
        todo!()
        // Self::Native
    }

    // /// Creates a new [`TargetTripleData`] with the **custom** values.
    // /// Useful for **cross-compiling**.
    // pub fn custom(arch: TargetArch, os: TargetOs, env: TargetEnv) -> Self {
    //     // Self::Custom(arch, os, env)
    // }
}

/// The **target architecture** to use when compiling the input file.
/// defaults to `TargetArch::Native`
///
/// The target architecture is used to specify the **target architecture** that the executable will be compiled
/// for. This is used to **generate** the **correct** **machine code** for the target architecture.
/// This is used to **generate** the **correct** **machine code** for the target architecture.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TargetArch {
    /// The **native** target architecture. This is the default target architecture.
    #[default]
    Native,

    /// The **x86_64** target architecture.
    #[cfg(target_arch = "x86_64")]
    X86_64,

    /// The **ARM** target architecture.
    #[cfg(target_arch = "arm")]
    Arm,

    /// The **ARM64** target architecture.
    /// This is also known as **aarch64**.
    #[cfg(target_arch = "aarch64")]
    Arm64,
}

/// The **target operating system** to use when compiling the input file.
/// defaults to `TargetOs::Native`
///
/// The target operating system is used to specify the **target operating system** that the executable will be
/// compiled for. This is used to **generate** the **correct** **machine code** for the target operating system.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TargetOs {
    /// The **native** target operating system. This is the default target operating system.
    #[default]
    Native,

    /// The **Linux** target operating system.
    #[cfg(target_os = "linux")]
    Linux,

    /// The **Windows** target operating system.
    #[cfg(target_os = "windows")]
    Windows,

    /// The **macOS** target operating system.
    #[cfg(target_os = "macos")]
    MacOs,
}

/// The **target environment** to use when compiling the input file.
/// defaults to `TargetEnv::Native`
///
/// The target environment is used to specify the **target environment** that the executable will be compiled for.
/// This is used to **generate** the **correct** **machine code** for the target environment.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TargetEnv {
    /// The **native** target environment. This is the default target environment.
    #[default]
    Native,

    /// The **GNU** target environment.
    #[cfg(target_env = "gnu")]
    Gnu,

    /// The **MSVC** target environment.
    #[cfg(target_env = "msvc")]
    Msvc,

    /// The **MacOS** target environment.
    #[cfg(target_env = "macos")]
    MacOs,
}
