---
title: "NixOS: Declarative Infrastructure as a Way of Life"
slug: "nixos-declarative-infrastructure"
kind: "blog"
published: true
published_at: "2026-07-26"
tags: ["nixos", "nix", "infrastructure", "devops", "linux"]
excerpt: "Why declarative infrastructure changes the way we build, reproduce, and think about systems."
---

# NixOS: Declarative Infrastructure as a Way of Life

The promise is simple: describe your system, and it will be exactly that system. Not "probably that system." Not "that system, plus whatever drifted in since the last time someone SSH'd in at 2am to fix something." *Exactly* that system: reproducible down to the last dependency, every time, on every machine.

That promise holds cleanly when nixpkgs already has what you need, which, for most of what most people run, it does. When it doesn't, flake inputs pick up the slack: another repository, another set of packages, another version pinned exactly where you need it, and the system stays described, still reproducible. On the rare occasion even that isn't enough, there's room to go further: override a derivation, write your own, patch a source. The promise doesn't break at nixpkgs' edge; it just asks a little more the further out you go.

Years of managing machines have trained most of us to expect the opposite. We're used to configuration as a verb we perform *on* a machine: a sequence of commands, half-remembered, half-documented, leaving behind a system nobody can fully explain a year later. Ask most sysadmins how their server got into its current state and you'll get a shrug, maybe a wiki page three renames out of date.

NixOS refuses that story. Instead of a history of actions, you write a description of an outcome. The configuration isn't a log of what was done; it's a specification of what should exist. The build process doesn't care what came before: it reads the spec and produces a system that matches it, bit for bit, regardless of what was there yesterday.

## Configuration Becomes Code

Once your system is a function of a text file, that text file inherits everything version control gives you: history, review, branching, rollback. A bad configuration isn't a crisis; it's a `git revert`. A new machine isn't a manual checklist; it's the same flake, evaluated somewhere else.

A workstation, a laptop, a small cloud instance humming in the background: these stop being separate, drifting islands of `apt install` history and become expressions of a shared configuration, differentiated only where they need to be. Shared modules capture what's common; host-specific overrides capture what isn't.

None of this is entirely new territory. Docker, Kubernetes, and Ansible are all reaching for pieces of the same goal. Docker gives reproducible containers; Kubernetes gives declared orchestration across them; Ansible gives declared configuration pushed out to existing machines. Each is excellent at its layer, and each still leaves the layers around it to something else. Nix and NixOS fold more of that stack into one coherent toolkit: the package manager, the build system, the system configuration, and the deployment story all speak the same language and share the same guarantees.

## The Discipline Is the Point

None of this comes without effort, and it shouldn't be sold as though it does. To be clear, this isn't about cost; it's free and open-source software, same as most of what it's being compared to here. The effort is in the thinking: the declarative model demands more upfront thought than the imperative one. You can't `curl | sh` your way to a working state and patch the rough edges later; the system will build precisely what you asked for, and nothing more forgiving.

The upfront thinking isn't all on you from the first minute, though. Install NixOS through the graphical installer, same as any other Linux distribution, and it handles the common defaults: partitioning, a working desktop, the baseline config. From there you're adding to a working system, not starting from nothing. The friction pays off regardless: it pushes complexity to the surface early, where it's cheap to fix, instead of letting it accumulate until a machine becomes too fragile to touch. Writing infrastructure as code doesn't just make it reproducible, it makes it *legible*.

## It's Nix, Not Just NixOS

It's easy to let NixOS take all the credit when the deeper idea belongs to Nix itself. NixOS is what happens when you apply Nix's model to an entire operating system; the package manager underneath doesn't need the operating system to make good on its promise.

Plenty of infrastructure doesn't get the luxury of running NixOS: a managed server, a monitoring agent, a host analytics stack that only ships support for a handful of blessed distributions. Ubuntu, in other words, because that's what the tooling demands. Install Nix on top of that Ubuntu box, and the same store, the same packages, the same reproducibility guarantees come along regardless of what's underneath. `nix profile` gives that portability directly, an imperative, day-to-day way to manage packages that behaves identically wherever Nix is installed. Layer flakes on top, and the declarative half of the promise arrives too: pinned inputs, reproducible builds, environments defined once and reproduced anywhere.

The one real exception is Windows. Nix has no native install there; WSL is still the bridge. A few experimental efforts have recently gotten further than most, building packages natively on Windows without WSL or Cygwin in sight, though none of it is production-ready yet. A sign the gap isn't permanent, just unclosed for now.

## Every Project, Its Own Flake

The part I keep coming back to: I don't need Rust, Python, or any other language toolchain installed natively on the system at all. Each project scopes its own environment through a flake, pinned to the exact versions it needs. I never have to wonder what version of anything is sitting on my system, because the system doesn't need to know; the repo carries its own answer.

The same pattern covers more than code. Documentation repos that need Typst get a flake. Art and music production setups, plugins, DAWs, fonts, the whole pile, can be scoped the same way. Every project becomes self-contained and reproducible on its own terms, with nothing bleeding across into the next.

## The Dotfiles, Before and After

I've been chasing the infamous "dotfiles" for years, long before any of it had a name this respectable. Symlink scripts, shell functions, half-finished bootstrap.sh files scattered across repos I'd rather not link now. I still tinker, that part never stops, but Nix changes what the tinkering is *for*: it's no longer about holding the system together, it's about shaping it on purpose.

It can be genuinely simple, a flake, a profile, a few packages pinned and done, or it can go as deep as you're willing to take it. Overlays, custom libraries, registries of inputs feeding modules that assemble themselves: Nix doesn't ask you to go there, but it doesn't stop you either. For those of us with the itch, that headroom is half the appeal.

## Beyond the Machine

The same logic that governs a single host scales outward without changing shape: a registry of inputs, a library of composable functions, modules that assemble themselves whether they're targeting a workstation or a small ARM instance in someone else's data center. The abstractions that make one machine legible are the same ones that make a fleet of them legible.

I'm not here to argue it's the best tool for everyone, or the only right way to run a system. It's just the one I keep reaching for, and the one that's kept giving back more the longer I stick with it. What keeps me around, honestly, is how much of it I still haven't touched: corners of the ecosystem, patterns other people have built, whole ways of composing things I haven't gotten to yet.

I'll be following this up with a proper step-by-step guide: from the initial build, through enabling flakes, setting up home-manager, and the other pieces that tend to trip people up early on. Consider this the pitch; the walkthrough is coming.
