use crate::prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Ansible                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ansible {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/ansible.svg",)
      .with_tooltip("",)
      .with_label("Ansible",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::SiAnsible,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::TbBrandAnsibleOutline,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_ANSIBLE_DARK, CLR_ANSIBLE_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Docker                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod docker {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/docker.svg",)
      .with_link("https://www.docker.com/",)
      .with_tooltip("Containerization platform",)
      .with_label("Docker",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::FaDockerBrands,),) }
  pub fn si_simple() -> Icon { base().with_source(Source::Leptos(icon::SiDocker,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_DOCKER_LIGHT, CLR_DOCKER_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Git                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod git {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/git.svg",)
      .with_link("https://git-scm.com/",)
      .with_tooltip("Free and open source distributed version control system",)
      .with_label("Git",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::BiGit,),) }
  pub fn outlined() -> Icon {
    base().with_source(Source::Leptos(icon::RiGitMergeDevelopmentLine,),)
  }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaGitBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareGitBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_GIT_LIGHT, CLR_GIT_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitHub                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod github {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/github.svg",)
      .with_link("https://github.com/craole-cc",)
      .with_tooltip("View my GitHub profile",)
      .with_label("GitHub",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiGithubLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiGithubLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaGithubBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareGithubBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_GITHUB_LIGHT, CLR_GITHUB_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gitlab                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gitlab {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/gitlab.svg",)
      .with_link("https://gitlab.com/craole",)
      .with_tooltip("View my Gitlab profile",)
      .with_label("Gitlab",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiGitlabLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiGitlabLogosLine,),) }
  pub fn ai_filled() -> Icon { base().with_source(Source::Leptos(icon::AiGitlabFilled,),) }
  pub fn ai_outlined() -> Icon { base().with_source(Source::Leptos(icon::AiGitlabOutlined,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaGitlabBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareGitlabBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_GITLAB_LIGHT, CLR_GITLAB_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Kubernetes                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod kubernetes {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/kubernetes.svg",)
      .with_link("https://kubernetes.io/",)
      .with_tooltip("Container orchestration platform",)
      .with_label("Kubernetes",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::SiKubernetes,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::AiKubernetesOutlined,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_KUBERNETES_LIGHT, CLR_KUBERNETES_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Linux                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linux {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/linux.svg",)
      .with_link("https://www.linux.org/",)
      .with_tooltip("Open source operating system kernel",)
      .with_label("Linux",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::FaLinuxBrands,),) }
  pub fn si_simple() -> Icon { base().with_source(Source::Leptos(icon::SiLinux,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_LINUX_LIGHT, CLR_LINUX_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nix                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nix {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/nix.svg",)
      .with_link("https://nix.dev/",)
      .with_tooltip("Reproducible package manager and build system",)
      .with_label("Nix",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::SiNixos,),) }
  pub fn outlined() -> Icon { filled() }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_NIX_LIGHT, CLR_NIX_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Raspberry Pi                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod raspberry_pi {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/raspberry.svg",)
      .with_link("https://www.raspberrypi.org/",)
      .with_tooltip("Single-board computer",)
      .with_label("Raspberry Pi",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::FaRaspberryPiBrands,),) }
  pub fn outlined() -> Icon { filled() }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_RASPBERRY_PI_LIGHT, CLR_RASPBERRY_PI_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Terraform                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod terraform {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/terraform.svg",)
      .with_link("https://www.terraform.io/",)
      .with_tooltip("Infrastructure as Code tool",)
      .with_label("Terraform",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::SiTerraform,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::TbBrandTerraformOutline,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_TERRAFORM_LIGHT, CLR_TERRAFORM_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Windows                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod windows {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/windows.svg",)
      .with_link("https://www.microsoft.com/windows/",)
      .with_tooltip("Microsoft Windows operating system",)
      .with_label("Windows",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiWindowsLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiWindowsLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaWindowsBrands,),) }
  pub fn fa_square() -> Icon { fa_brands() }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_WINDOWS_LIGHT, CLR_WINDOWS_DARK,),)
  }
}
