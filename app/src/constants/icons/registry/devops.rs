use crate::prelude::{icons::*, *};

//╔═══════════════════════════════════════════════════════════╗
//║ Ansible                                                   ║
//╚═══════════════════════════════════════════════════════════╝

pub const CLR_ANSIBLE: &str = "#EE0000";

pub fn ansible() -> Icon {
  Icon::new_local("icons/logos/ansible.svg").with_label("Ansible")
}

pub mod ansible_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/ansible.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::SiAnsible)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::TbBrandAnsibleOutline)
  }

  pub fn with_color(source: Source) -> Icon {
    ansible()
      .with_source(source)
      .and_class(fill_class(CLR_ANSIBLE_DARK, CLR_ANSIBLE_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Git                                                       ║
//╚═══════════════════════════════════════════════════════════╝

pub fn git() -> Icon {
  Icon::new_local("icons/logos/git.svg")
    .with_link("https://git-scm.com/")
    .with_link("A free and open source distributed version control system")
    .with_label("Git")
}

pub mod git_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/git.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::BiGit)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::RiGitMergeDevelopmentLine)
  }

  pub fn fa_brands() -> Source {
    Source::Leptos(icon::FaGitBrands)
  }

  pub fn fa_square() -> Source {
    Source::Leptos(icon::FaSquareGitBrands)
  }

  pub fn with_color(source: Source) -> Icon {
    git()
      .with_source(source)
      .and_class(fill_class(CLR_GIT_LIGHT, CLR_GIT_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ GitHub                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub fn github() -> Icon {
  Icon::new_local("icons/logos/github.svg")
    .with_link("https://github.com/craole-cc")
    .with_tooltip("View my GitHub profile")
    .with_label("GitHub")
}

pub mod github_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/github.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::RiGithubLogosFill)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::RiGithubLogosLine)
  }

  pub fn fa_brands() -> Source {
    Source::Leptos(icon::FaGithubBrands)
  }

  pub fn fa_square() -> Source {
    Source::Leptos(icon::FaSquareGithubBrands)
  }

  pub fn with_color(source: Source) -> Icon {
    github()
      .with_source(source)
      .and_class(fill_class(CLR_GITHUB_LIGHT, CLR_GITHUB_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gitlab                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub fn gitlab() -> Icon {
  Icon::new_local("icons/logos/gitlab.svg")
    .with_link("https://gitlab.com/craole")
    .with_tooltip("View my Gitlab profile")
    .with_label("Gitlab")
}

pub mod gitlab_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/gitlab.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::RiGitlabLogosFill)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::RiGitlabLogosLine)
  }

  pub fn ai_filled() -> Source {
    Source::Leptos(icon::AiGitlabFilled)
  }

  pub fn ai_outlined() -> Source {
    Source::Leptos(icon::AiGitlabOutlined)
  }

  pub fn fa_brands() -> Source {
    Source::Leptos(icon::FaGitlabBrands)
  }

  pub fn fa_square() -> Source {
    Source::Leptos(icon::FaSquareGitlabBrands)
  }

  pub fn with_color(source: Source) -> Icon {
    gitlab()
      .with_source(source)
      .and_class(fill_class(CLR_GITLAB_LIGHT, CLR_GITLAB_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nix                                                       ║
//╚═══════════════════════════════════════════════════════════╝

pub fn nix() -> Icon {
  Icon::new_local("icons/logos/nix.svg")
    .with_link("https://nix.dev/")
    .with_tooltip("Reproducible package manager and build system")
    .with_label("Nix")
}

pub mod nix_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/nix.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::SiNixos)
  }

  pub fn outlined() -> Source {
    filled()
  }

  pub fn with_color(source: Source) -> Icon {
    nix()
      .with_source(source)
      .and_class(fill_class(CLR_NIX_LIGHT, CLR_NIX_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Raspberry Pi                                              ║
//╚═══════════════════════════════════════════════════════════╝

pub fn raspberry_pi() -> Icon {
  Icon::new_local("icons/logos/raspberry.svg")
    .with_link("https://www.raspberrypi.org/")
    .with_tooltip("Single-board computer")
    .with_label("Raspberry Pi")
}

pub mod raspberry_pi_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/raspberry.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::FaRaspberryPiBrands)
  }

  pub fn outline() -> Source {
    filled()
  }

  pub fn with_color(source: Source) -> Icon {
    raspberry_pi()
      .with_source(source)
      .and_class(fill_class(CLR_RASPBERRY_PI_LIGHT, CLR_RASPBERRY_PI_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Windows                                                   ║
//╚═══════════════════════════════════════════════════════════╝

pub const CLR_WINDOWS: &str = "#0078D6";

pub fn windows() -> Icon {
  Icon::new_local("icons/logos/windows.svg")
    .with_link("https://www.microsoft.com/windows/")
    .with_tooltip("Microsoft Windows operating system")
    .with_label("Windows")
}

pub mod windows_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/windows.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::RiWindowsLogosFill)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::RiWindowsLogosLine)
  }

  pub fn fa_brands() -> Source {
    Source::Leptos(icon::FaWindowsBrands)
  }

  pub fn fa_square() -> Source {
    fa_brands()
  }

  pub fn with_color(source: Source) -> Icon {
    windows()
      .with_source(source)
      .and_class(fill_class(CLR_WINDOWS_LIGHT, CLR_WINDOWS_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Docker                                                    ║
//╚═══════════════════════════════════════════════════════════╝

pub const CLR_DOCKER: &str = "#2496ED";

pub fn docker() -> Icon {
  Icon::new_local("icons/logos/docker.svg")
    .with_link("https://www.docker.com/")
    .with_tooltip("Containerization platform")
    .with_label("Docker")
}

pub mod docker_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/docker.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::FaDockerBrands)
  }

  pub fn si_simple() -> Source {
    Source::Leptos(icon::SiDocker)
  }

  pub fn with_color(source: Source) -> Icon {
    docker()
      .with_source(source)
      .and_class(fill_class(CLR_DOCKER_LIGHT, CLR_DOCKER_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Linux                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub fn linux() -> Icon {
  Icon::new_local("icons/logos/linux.svg")
    .with_link("https://www.linux.org/")
    .with_tooltip("Open source operating system kernel")
    .with_label("Linux")
}

pub mod linux_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/linux.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::FaLinuxBrands)
  }

  pub fn si_simple() -> Source {
    Source::Leptos(icon::SiLinux)
  }

  pub fn with_color(source: Source) -> Icon {
    linux()
      .with_source(source)
      .and_class(fill_class(CLR_LINUX_LIGHT, CLR_LINUX_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Kubernetes                                                ║
//╚═══════════════════════════════════════════════════════════╝

pub fn kubernetes() -> Icon {
  Icon::new_local("icons/logos/kubernetes.svg")
    .with_link("https://kubernetes.io/")
    .with_tooltip("Container orchestration platform")
    .with_label("Kubernetes")
}

pub mod kubernetes_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/kubernetes.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::SiKubernetes)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::AiKubernetesOutlined)
  }

  pub fn with_color(source: Source) -> Icon {
    kubernetes()
      .with_source(source)
      .and_class(fill_class(CLR_KUBERNETES_LIGHT, CLR_KUBERNETES_DARK))
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Terraform                                                 ║
//╚═══════════════════════════════════════════════════════════╝

pub fn terraform() -> Icon {
  Icon::new_local("icons/logos/terraform.svg")
    .with_link("https://www.terraform.io/")
    .with_tooltip("Infrastructure as Code tool")
    .with_label("Terraform")
}

pub mod terraform_variants {
  use super::*;

  pub fn local() -> Source {
    Source::Local("icons/logos/terraform.svg")
  }

  pub fn filled() -> Source {
    Source::Leptos(icon::SiTerraform)
  }

  pub fn outlined() -> Source {
    Source::Leptos(icon::TbBrandTerraformOutline)
  }

  pub fn with_color(source: Source) -> Icon {
    terraform()
      .with_source(source)
      .and_class(fill_class(CLR_TERRAFORM_LIGHT, CLR_TERRAFORM_DARK))
  }
}
