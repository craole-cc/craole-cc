use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Ansible                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ansible {
  use super::*;

  /// Icon selector for [Ansible](https://docs.ansible.com/).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `SiAnsible` with `--brand-ansible` |
  /// | `Outlined` | [`outlined`] — `TbBrandAnsibleOutline` with `--brand-ansible` |
  ///
  /// # Example
  /// ```rust
  /// let icon = ansible::Ansible(Variant::Filled,).get(); 
  /// ```
  pub struct Ansible(pub Variant,);

  impl Ansible {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/ansible.svg",)
      .with_link("https://docs.ansible.com/",)
      .with_tooltip("Automation platform for IT operations",)
      .with_label("Ansible",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiAnsible`](icon::SiAnsible) with `--brand-ansible` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiAnsible,),)
      .colored("brand-ansible",)
  }

  /// Outlined [`TbBrandAnsibleOutline`](icon::TbBrandAnsibleOutline) with `--brand-ansible` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandAnsibleOutline,),)
      .colored("brand-ansible",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Docker                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod docker {
  use super::*;

  /// Icon selector for [Docker](https://www.docker.com/).
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`]. For the Simple Icons style use [`DockerExt`] with
  /// [`Extended::SiSimple`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `FaDockerBrands` with `--brand-docker` |
  /// | `Outlined` | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = docker::Docker(Variant::Filled,).get(); 
  /// ```
  pub struct Docker(pub Variant,);

  impl Docker {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  /// Extended variant selector for non-standard Docker icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `SiSimple` | [`si_simple`] — `SiDocker` with `--brand-docker` |
  ///
  /// # Example
  /// ```rust
  /// let icon = docker::DockerExt(docker::Extended::SiSimple,).get(); 
  /// ```
  pub struct DockerExt(pub Extended,);

  impl DockerExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::SiSimple => si_simple(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Simple Icons alternative filled style.
    SiSimple,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/docker.svg",)
      .with_link("https://www.docker.com/",)
      .with_tooltip("Containerization platform",)
      .with_label("Docker",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`FaDockerBrands`](icon::FaDockerBrands) with `--brand-docker` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaDockerBrands,),)
      .colored("brand-docker",)
  }

  /// Falls back to [`filled`] — no distinct outlined style exists.
  pub fn outlined() -> Icon { filled() }

  /// Simple Icons [`SiDocker`](icon::SiDocker) alternative with `--brand-docker` colour.
  pub fn si_simple() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiDocker,),)
      .colored("brand-docker",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Git                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod git {
  use super::*;

  /// Icon selector for [Git](https://git-scm.com/).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `BiGit` with `--brand-git` |
  /// | `Outlined` | [`outlined`] — `RiGitMergeDevelopmentLine` with `--brand-git` |
  ///
  /// # Example
  /// ```rust
  /// let icon = git::Git(Variant::Outlined,).get(); 
  /// ```
  pub struct Git(pub Variant,);

  impl Git {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard Git icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaGitBrands` with `--brand-git` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareGitBrands` with `--brand-git` |
  ///
  /// # Example
  /// ```rust
  /// let icon = git::GitExt(git::Extended::FaSquare,).get(); 
  /// ```
  pub struct GitExt(pub Extended,);

  impl GitExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/git.svg",)
      .with_link("https://git-scm.com/",)
      .with_tooltip("Free and open source distributed version control system",)
      .with_label("Git",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`BiGit`](icon::BiGit) with `--brand-git` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::BiGit,),)
      .colored("brand-git",)
  }
  /// Outlined [`RiGitMergeDevelopmentLine`](icon::RiGitMergeDevelopmentLine) with `--brand-git`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiGitMergeDevelopmentLine,),)
      .colored("brand-git",)
  }
  /// Font Awesome [`FaGitBrands`](icon::FaGitBrands) with `--brand-git` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaGitBrands,),)
      .colored("brand-git",)
  }
  /// Font Awesome square [`FaSquareGitBrands`](icon::FaSquareGitBrands) with `--brand-git` colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareGitBrands,),)
      .colored("brand-git",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitHub                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod github {
  use super::*;

  /// Icon selector for [GitHub](https://github.com/craole-cc).
  ///
  /// > **Note:** `Default` resolves to [`filled`] — the coloured Leptos icon
  /// > is preferred over the monochrome local asset for GitHub.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`filled`] — `RiGithubLogosFill` with `--brand-github` |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiGithubLogosFill` with `--brand-github` |
  /// | `Outlined` | [`outlined`] — `RiGithubLogosLine` with `--brand-github` |
  ///
  /// # Example
  /// ```rust
  /// let icon = github::GitHub(Variant::Default,).get(); // resolves to filled
  /// ```
  pub struct GitHub(pub Variant,);

  impl GitHub {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Default` resolves to [`filled`] for this brand.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => filled(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard GitHub icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaGithubBrands` with `--brand-github` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareGithubBrands` with `--brand-github` |
  ///
  /// # Example
  /// ```rust
  /// let icon = github::GitHubExt(github::Extended::FaSquare,).get(); 
  /// ```
  pub struct GitHubExt(pub Extended,);

  impl GitHubExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/github.svg",)
      .with_link("https://github.com/craole-cc",)
      .with_tooltip("View my GitHub profile",)
      .with_label("GitHub",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`filled`].
  pub fn default() -> Icon { filled() }
  /// Filled [`RiGithubLogosFill`](icon::RiGithubLogosFill) with `--brand-github` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiGithubLogosFill,),)
      .colored("brand-github",)
  }
  /// Outlined [`RiGithubLogosLine`](icon::RiGithubLogosLine) with `--brand-github` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiGithubLogosLine,),)
      .colored("brand-github",)
  }
  /// Font Awesome [`FaGithubBrands`](icon::FaGithubBrands) with `--brand-github` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaGithubBrands,),)
      .colored("brand-github",)
  }
  /// Font Awesome square [`FaSquareGithubBrands`](icon::FaSquareGithubBrands) with `--brand-github`
  /// colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareGithubBrands,),)
      .colored("brand-github",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitLab                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gitlab {
  use super::*;

  /// Icon selector for [GitLab](https://gitlab.com/craole).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiGitlabLogosFill` with `--brand-gitlab` |
  /// | `Outlined` | [`outlined`] — `RiGitlabLogosLine` with `--brand-gitlab` |
  ///
  /// # Example
  /// ```rust
  /// let icon = gitlab::GitLab(Variant::Filled,).get(); 
  /// ```
  pub struct GitLab(pub Variant,);

  impl GitLab {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard GitLab icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `AiFilled` | [`ai_filled`] — `AiGitlabFilled` with `--brand-gitlab` |
  /// | `AiOutlined` | [`ai_outlined`] — `AiGitlabOutlined` with `--brand-gitlab` |
  /// | `FaBrands` | [`fa_brands`] — `FaGitlabBrands` with `--brand-gitlab` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareGitlabBrands` with `--brand-gitlab` |
  ///
  /// # Example
  /// ```rust
  /// let icon = gitlab::GitLabExt(gitlab::Extended::AiOutlined,).get(); 
  /// ```
  pub struct GitLabExt(pub Extended,);

  impl GitLabExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::AiFilled => ai_filled(),
        | Extended::AiOutlined => ai_outlined(),
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Ant Design filled icon.
    AiFilled,
    /// Ant Design outlined icon.
    AiOutlined,
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/gitlab.svg",)
      .with_link("https://gitlab.com/craole",)
      .with_tooltip("View my GitLab profile",)
      .with_label("GitLab",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiGitlabLogosFill`](icon::RiGitlabLogosFill) with `--brand-gitlab` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiGitlabLogosFill,),)
      .colored("brand-gitlab",)
  }
  /// Outlined [`RiGitlabLogosLine`](icon::RiGitlabLogosLine) with `--brand-gitlab` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiGitlabLogosLine,),)
      .colored("brand-gitlab",)
  }
  /// Ant Design filled [`AiGitlabFilled`](icon::AiGitlabFilled) with `--brand-gitlab` colour.
  pub fn ai_filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::AiGitlabFilled,),)
      .colored("brand-gitlab",)
  }
  /// Ant Design outlined [`AiGitlabOutlined`](icon::AiGitlabOutlined) with `--brand-gitlab` colour.
  pub fn ai_outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::AiGitlabOutlined,),)
      .colored("brand-gitlab",)
  }
  /// Font Awesome [`FaGitlabBrands`](icon::FaGitlabBrands) with `--brand-gitlab` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaGitlabBrands,),)
      .colored("brand-gitlab",)
  }
  /// Font Awesome square [`FaSquareGitlabBrands`](icon::FaSquareGitlabBrands) with `--brand-gitlab`
  /// colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareGitlabBrands,),)
      .colored("brand-gitlab",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Kubernetes                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod kubernetes {
  use super::*;

  /// Icon selector for [Kubernetes](https://kubernetes.io/).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `SiKubernetes` with `--brand-kubernetes` |
  /// | `Outlined` | [`outlined`] — `AiKubernetesOutlined` with `--brand-kubernetes` |
  ///
  /// # Example
  /// ```rust
  /// let icon = kubernetes::Kubernetes(Variant::Filled,).get(); 
  /// ```
  pub struct Kubernetes(pub Variant,);

  impl Kubernetes {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/kubernetes.svg",)
      .with_link("https://kubernetes.io/",)
      .with_tooltip("Container orchestration platform",)
      .with_label("Kubernetes",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`SiKubernetes`](icon::SiKubernetes) with `--brand-kubernetes` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiKubernetes,),)
      .colored("brand-kubernetes",)
  }
  /// Outlined [`AiKubernetesOutlined`](icon::AiKubernetesOutlined) with `--brand-kubernetes`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::AiKubernetesOutlined,),)
      .colored("brand-kubernetes",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Linux                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linux {
  use super::*;

  /// Icon selector for [Linux](https://www.linux.org/).
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`]. For the Simple Icons style use [`LinuxExt`] with
  /// [`Extended::SiSimple`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `FaLinuxBrands` with `--brand-linux` |
  /// | `Outlined` | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = linux::Linux(Variant::Filled,).get(); 
  /// ```
  pub struct Linux(pub Variant,);

  impl Linux {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  /// Extended variant selector for non-standard Linux icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `SiSimple` | [`si_simple`] — `SiLinux` with `--brand-linux` |
  ///
  /// # Example
  /// ```rust
  /// let icon = linux::LinuxExt(linux::Extended::SiSimple,).get(); 
  /// ```
  pub struct LinuxExt(pub Extended,);

  impl LinuxExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::SiSimple => si_simple(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Simple Icons alternative style.
    SiSimple,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/linux.svg",)
      .with_link("https://www.linux.org/",)
      .with_tooltip("Open source operating system kernel",)
      .with_label("Linux",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`FaLinuxBrands`](icon::FaLinuxBrands) with `--brand-linux` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaLinuxBrands,),)
      .colored("brand-linux",)
  }
  /// Simple Icons [`SiLinux`](icon::SiLinux) alternative with `--brand-linux` colour.
  pub fn si_simple() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiLinux,),)
      .colored("brand-linux",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nix                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nix {
  use super::*;

  /// Icon selector for [Nix](https://nix.dev/).
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `SiNixos` with `--brand-nix` |
  /// | `Outlined` | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = nix::Nix(Variant::Filled,).get(); 
  /// ```
  pub struct Nix(pub Variant,);

  impl Nix {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/nix.svg",)
      .with_link("https://nix.dev/",)
      .with_tooltip("Reproducible package manager and build system",)
      .with_label("Nix",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`SiNixos`](icon::SiNixos) with `--brand-nix` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiNixos,),)
      .colored("brand-nix",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Raspberry Pi                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod raspberry_pi {
  use super::*;

  /// Icon selector for [Raspberry Pi](https://www.raspberrypi.org/).
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `FaRaspberryPiBrands` with `--brand-raspberry` |
  /// | `Outlined` | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = raspberry_pi::RaspberryPi(Variant::Filled,).get(); 
  /// ```
  pub struct RaspberryPi(pub Variant,);

  impl RaspberryPi {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/raspberry.svg",)
      .with_link("https://www.raspberrypi.org/",)
      .with_tooltip("Single-board computer",)
      .with_label("Raspberry Pi",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`FaRaspberryPiBrands`](icon::FaRaspberryPiBrands) with `--brand-raspberry` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaRaspberryPiBrands,),)
      .colored("brand-raspberry",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Terraform                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod terraform {
  use super::*;

  /// Icon selector for [Terraform](https://www.terraform.io/).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `SiTerraform` with `--brand-terraform` |
  /// | `Outlined` | [`outlined`] — `TbBrandTerraformOutline` with `--brand-terraform` |
  ///
  /// # Example
  /// ```rust
  /// let icon = terraform::Terraform(Variant::Outlined,).get(); 
  /// ```
  pub struct Terraform(pub Variant,);

  impl Terraform {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/terraform.svg",)
      .with_link("https://www.terraform.io/",)
      .with_tooltip("Infrastructure as Code tool",)
      .with_label("Terraform",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`SiTerraform`](icon::SiTerraform) with `--brand-terraform` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiTerraform,),)
      .colored("brand-terraform",)
  }
  /// Outlined [`TbBrandTerraformOutline`](icon::TbBrandTerraformOutline) with `--brand-terraform`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandTerraformOutline,),)
      .colored("brand-terraform",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Windows                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod windows {
  use super::*;

  /// Icon selector for [Windows](https://www.microsoft.com/windows/).
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiWindowsLogosFill` with `--brand-windows` |
  /// | `Outlined` | [`outlined`] — `RiWindowsLogosLine` with `--brand-windows` |
  ///
  /// # Example
  /// ```rust
  /// let icon = windows::Windows(Variant::Outlined,).get(); 
  /// ```
  pub struct Windows(pub Variant,);

  impl Windows {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard Windows icon styles.
  ///
  /// > **Note:** `FaSquare` aliases `FaBrands` — both use
  /// > [`FaWindowsBrands`](icon::FaWindowsBrands) as no square variant exists.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaWindowsBrands` with `--brand-windows` |
  /// | `FaSquare` | [`fa_brands`] — same asset, no square variant exists |
  ///
  /// # Example
  /// ```rust
  /// let icon = windows::WindowsExt(windows::Extended::FaBrands,).get(); 
  /// ```
  pub struct WindowsExt(pub Extended,);

  impl WindowsExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_brands(), // no distinct square variant
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon — aliases [`FaBrands`](Extended::FaBrands).
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/windows.svg",)
      .with_link("https://www.microsoft.com/windows/",)
      .with_tooltip("Microsoft Windows operating system",)
      .with_label("Windows",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiWindowsLogosFill`](icon::RiWindowsLogosFill) with `--brand-windows` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiWindowsLogosFill,),)
      .colored("brand-windows",)
  }
  /// Outlined [`RiWindowsLogosLine`](icon::RiWindowsLogosLine) with `--brand-windows` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiWindowsLogosLine,),)
      .colored("brand-windows",)
  }
  /// Font Awesome [`FaWindowsBrands`](icon::FaWindowsBrands) with `--brand-windows` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaWindowsBrands,),)
      .colored("brand-windows",)
  }
}
