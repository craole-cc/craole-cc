use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Ansible                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ansible {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Ansible(pub Variant,);

  impl Ansible {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
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

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiAnsible,)
      .colored("brand-ansible",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandAnsibleOutline,)
      .colored("brand-ansible",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Docker                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod docker {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Docker(pub Variant,);

  impl Docker {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct DockerExt(pub Extended,);

  impl DockerExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::SiSimple => si_simple(),
      }
    }
  }

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    SiSimple,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/docker.svg",)
      .with_link("https://www.docker.com/",)
      .with_tooltip("Containerization platform",)
      .with_label("Docker",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::FaDockerBrands,)
      .colored("brand-docker",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn si_simple() -> Icon { base().via_leptos(icon::SiDocker,).colored("brand-docker",) }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Git                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod git {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Git(pub Variant,);

  impl Git {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct GitExt(pub Extended,);

  impl GitExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    FaBrands,
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/git.svg",)
      .with_link("https://git-scm.com/",)
      .with_tooltip("Free and open source distributed version control system",)
      .with_label("Git",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::BiGit,).colored("brand-git",) }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::RiGitMergeDevelopmentLine,)
      .colored("brand-git",)
  }

  #[must_use]
  pub fn fa_brands() -> Icon { base().via_leptos(icon::FaGitBrands,).colored("brand-git",) }

  #[must_use]
  pub fn fa_square() -> Icon {
    base()
      .via_leptos(icon::FaSquareGitBrands,)
      .colored("brand-git",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitHub                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod github {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct GitHub(pub Variant,);

  impl GitHub {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => filled(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct GitHubExt(pub Extended,);

  impl GitHubExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    FaBrands,
    FaSquare,
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://github.com/craole-cc",)
      .with_tooltip("View my GitHub profile",)
      .with_label("GitHub",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/github-refined.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::RiGithubLogosFill,)
      .colored("brand-github",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::RiGithubLogosLine,)
      .colored("brand-github",)
  }

  #[must_use]
  pub fn fa_brands() -> Icon {
    base()
      .via_leptos(icon::FaGithubBrands,)
      .colored("brand-github",)
  }

  #[must_use]
  pub fn fa_square() -> Icon {
    base()
      .via_leptos(icon::FaSquareGithubBrands,)
      .colored("brand-github",)
  }

  #[must_use]
  pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitLab                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gitlab {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct GitLab(pub Variant,);

  impl GitLab {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct GitLabExt(pub Extended,);

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    AiFilled,
    AiOutlined,
    FaBrands,
    FaSquare,
  }

  impl GitLabExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::AiFilled => ai_filled(),
        | Extended::AiOutlined => ai_outlined(),
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/gitlab.svg",)
      .with_link("https://gitlab.com/craole",)
      .with_tooltip("View my GitLab profile",)
      .with_label("GitLab",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::RiGitlabLogosFill,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::RiGitlabLogosLine,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn ai_filled() -> Icon {
    base()
      .via_leptos(icon::AiGitlabFilled,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn ai_outlined() -> Icon {
    base()
      .via_leptos(icon::AiGitlabOutlined,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn fa_brands() -> Icon {
    base()
      .via_leptos(icon::FaGitlabBrands,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn fa_square() -> Icon {
    base()
      .via_leptos(icon::FaSquareGitlabBrands,)
      .colored("brand-gitlab",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Kubernetes                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod kubernetes {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Kubernetes(pub Variant,);

  impl Kubernetes {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
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

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiKubernetes,)
      .colored("brand-kubernetes",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::AiKubernetesOutlined,)
      .colored("brand-kubernetes",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Linux                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linux {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Linux(pub Variant,);

  impl Linux {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  pub struct LinuxExt(pub Extended,);

  impl LinuxExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::SiSimple => si_simple(),
      }
    }
  }

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    SiSimple,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/linux.svg",)
      .with_link("https://www.linux.org/",)
      .with_tooltip("Open source operating system kernel",)
      .with_label("Linux",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::FaLinuxBrands,)
      .colored("brand-linux",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn si_simple() -> Icon { base().via_leptos(icon::SiLinux,).colored("brand-linux",) }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nix                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nix {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Nix(pub Variant,);

  impl Nix {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
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

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiNixos,).colored("brand-nix",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Raspberry Pi                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod raspberry_pi {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct RaspberryPi(pub Variant,);

  impl RaspberryPi {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
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

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::FaRaspberryPiBrands,)
      .colored("brand-raspberry",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Terraform                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod terraform {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Terraform(pub Variant,);

  impl Terraform {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
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

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiTerraform,)
      .colored("brand-terraform",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandTerraformOutline,)
      .colored("brand-terraform",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Windows                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod windows {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Windows(pub Variant,);

  impl Windows {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct WindowsExt(pub Extended,);

  impl WindowsExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_brands(),
      }
    }
  }

  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    FaBrands,
    FaSquare,
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://www.microsoft.com/windows/",)
      .with_tooltip("Microsoft Windows operating system",)
      .with_label("Windows",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/windows.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::RiWindowsLogosFill,)
      .colored("brand-windows",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::RiWindowsLogosLine,)
      .colored("brand-windows",)
  }

  #[must_use]
  pub fn fa_brands() -> Icon {
    base()
      .via_leptos(icon::FaWindowsBrands,)
      .colored("brand-windows",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}
