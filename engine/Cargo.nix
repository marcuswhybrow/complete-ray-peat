# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "engine/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
}:
let
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
in let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    engine = rustPackages.unknown.engine."0.1.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".anstream."0.3.0" = overridableMkRustCrate (profileName: rec {
    name = "anstream";
    version = "0.3.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9e579a7752471abc2a8268df8b20005e3eadd975f585398f17efcfd8d4927371"; };
    features = builtins.concatLists [
      [ "auto" ]
      [ "default" ]
      [ "wincon" ]
    ];
    dependencies = {
      anstyle = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.0" { inherit profileName; };
      anstyle_parse = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-parse."0.2.0" { inherit profileName; };
      anstyle_query = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-query."1.0.0" { inherit profileName; };
      ${ if hostPlatform.isWindows then "anstyle_wincon" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-wincon."1.0.0" { inherit profileName; };
      colorchoice = rustPackages."registry+https://github.com/rust-lang/crates.io-index".colorchoice."1.0.0" { inherit profileName; };
      is_terminal = rustPackages."registry+https://github.com/rust-lang/crates.io-index".is-terminal."0.4.7" { inherit profileName; };
      utf8parse = rustPackages."registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "anstyle";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "41ed9a86bf92ae6580e0a31281f65a1b1d867c0cc68d5346e2ae128dddfa6a7d"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-parse."0.2.0" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-parse";
    version = "0.2.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e765fd216e48e067936442276d1d57399e37bce53c264d6fefbe298080cb57ee"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "utf8" ]
    ];
    dependencies = {
      utf8parse = rustPackages."registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-query."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-query";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5ca11d4be1bab0c8bc8734a9aa7bf4ee8316d462a08c6ac5052f888fef5b494b"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-wincon."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-wincon";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4bcd8291a340dd8ac70e18878bc4501dd7b4ff970cfa21c207d36ece51ea88fd"; };
    dependencies = {
      anstyle = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.0" { inherit profileName; };
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" = overridableMkRustCrate (profileName: rec {
    name = "bitflags";
    version = "1.3.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cc."1.0.79" = overridableMkRustCrate (profileName: rec {
    name = "cc";
    version = "1.0.79";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "50d30906286121d95be3d479533b458f87493b30a4b5f79a607db8f5d11aa91f"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap."4.2.4" = overridableMkRustCrate (profileName: rec {
    name = "clap";
    version = "4.2.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "956ac1f6381d8d82ab4684768f89c0ea3afe66925ceadb4eeb3fc452ffc55d62"; };
    features = builtins.concatLists [
      [ "color" ]
      [ "default" ]
      [ "derive" ]
      [ "error-context" ]
      [ "help" ]
      [ "std" ]
      [ "suggestions" ]
      [ "usage" ]
    ];
    dependencies = {
      clap_builder = rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_builder."4.2.4" { inherit profileName; };
      clap_derive = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_derive."4.2.0" { profileName = "__noProfile"; };
      once_cell = rustPackages."registry+https://github.com/rust-lang/crates.io-index".once_cell."1.17.1" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_builder."4.2.4" = overridableMkRustCrate (profileName: rec {
    name = "clap_builder";
    version = "4.2.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "84080e799e54cff944f4b4a4b0e71630b0e0443b25b985175c7dddc1a859b749"; };
    features = builtins.concatLists [
      [ "color" ]
      [ "error-context" ]
      [ "help" ]
      [ "std" ]
      [ "suggestions" ]
      [ "usage" ]
    ];
    dependencies = {
      anstream = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstream."0.3.0" { inherit profileName; };
      anstyle = rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.0" { inherit profileName; };
      bitflags = rustPackages."registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" { inherit profileName; };
      clap_lex = rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_lex."0.4.1" { inherit profileName; };
      strsim = rustPackages."registry+https://github.com/rust-lang/crates.io-index".strsim."0.10.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_derive."4.2.0" = overridableMkRustCrate (profileName: rec {
    name = "clap_derive";
    version = "4.2.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3f9644cd56d6b87dbe899ef8b053e331c0637664e9e21a33dfcdc36093f5c5c4"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      heck = rustPackages."registry+https://github.com/rust-lang/crates.io-index".heck."0.4.1" { inherit profileName; };
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.56" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.26" { inherit profileName; };
      syn = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.15" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_lex."0.4.1" = overridableMkRustCrate (profileName: rec {
    name = "clap_lex";
    version = "0.4.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8a2dd5a6fe8c6e3502f568a6353e5273bbb15193ad9a89e457b9970798efbea1"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".colorchoice."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "colorchoice";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "acbf1af155f9b9ef647e42cdc158db4b64a1b61f743629225fde6f3e0be2a7c7"; };
  });
  
  "unknown".engine."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "engine";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      clap = rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap."4.2.4" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".errno."0.3.1" = overridableMkRustCrate (profileName: rec {
    name = "errno";
    version = "0.3.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4bcfec3a70f97c962c307b2d2c56e358cf1d00b558d74262b5f929ee8cc7e73a"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "dragonfly" then "errno_dragonfly" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".errno-dragonfly."0.1.2" { inherit profileName; };
      ${ if hostPlatform.isUnix || hostPlatform.parsed.kernel.name == "hermit" || hostPlatform.parsed.kernel.name == "wasi" then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.141" { inherit profileName; };
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".errno-dragonfly."0.1.2" = overridableMkRustCrate (profileName: rec {
    name = "errno-dragonfly";
    version = "0.1.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "aa68f1b12764fab894d2755d2518754e71b4fd80ecfb822714a1206c2aab39bf"; };
    dependencies = {
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.141" { inherit profileName; };
    };
    buildDependencies = {
      cc = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.79" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".heck."0.4.1" = overridableMkRustCrate (profileName: rec {
    name = "heck";
    version = "0.4.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "95505c38b4572b2d910cecb0281560f54b440a19336cbbcb27bf6ce6adc6f5a8"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.3.1" = overridableMkRustCrate (profileName: rec {
    name = "hermit-abi";
    version = "0.3.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "fed44880c466736ef9a5c5b5facefb5ed0785676d0c02d612db14e54f0d84286"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".io-lifetimes."1.0.10" = overridableMkRustCrate (profileName: rec {
    name = "io-lifetimes";
    version = "1.0.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9c66c74d2ae7e79a5a8f7ac924adbe38ee42a859c6539ad869eb51f0b52dc220"; };
    features = builtins.concatLists [
      [ "close" ]
      [ "default" ]
      [ "hermit-abi" ]
      [ "libc" ]
      [ "windows-sys" ]
    ];
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "hermit" then "hermit_abi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.3.1" { inherit profileName; };
      ${ if !hostPlatform.isWindows then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.141" { inherit profileName; };
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".is-terminal."0.4.7" = overridableMkRustCrate (profileName: rec {
    name = "is-terminal";
    version = "0.4.7";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "adcf93614601c8129ddf72e2d5633df827ba6551541c6d8c59520a371475be1f"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "hermit" then "hermit_abi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.3.1" { inherit profileName; };
      io_lifetimes = rustPackages."registry+https://github.com/rust-lang/crates.io-index".io-lifetimes."1.0.10" { inherit profileName; };
      ${ if !(hostPlatform.isWindows || hostPlatform.parsed.kernel.name == "hermit" || hostPlatform.parsed.kernel.name == "unknown") then "rustix" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".rustix."0.37.13" { inherit profileName; };
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.141" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.141";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3304a64d199bb964be99741b7a14d26972741915b3649639149b2479bb46f4b5"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "extra_traits" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".linux-raw-sys."0.3.3" = overridableMkRustCrate (profileName: rec {
    name = "linux-raw-sys";
    version = "0.3.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9b085a4f2cde5781fc4b1717f2e86c62f5cda49de7ba99a7c2eae02b61c9064c"; };
    features = builtins.concatLists [
      [ "errno" ]
      [ "general" ]
      [ "ioctl" ]
      [ "no_std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".once_cell."1.17.1" = overridableMkRustCrate (profileName: rec {
    name = "once_cell";
    version = "1.17.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b7e5500299e16ebb147ae15a00a942af264cf3688f47923b8fc2cd5858f23ad3"; };
    features = builtins.concatLists [
      [ "alloc" ]
      [ "default" ]
      [ "race" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.56" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro2";
    version = "1.0.56";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2b63bdb0cd06f1f4dedf69b254734f9b45af66e4a031e42a7480257d9898b435"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      unicode_ident = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.8" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.26" = overridableMkRustCrate (profileName: rec {
    name = "quote";
    version = "1.0.26";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4424af4bf778aae2051a77b60283332f386554255d722233d09fbfc7e30da2fc"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.56" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".rustix."0.37.13" = overridableMkRustCrate (profileName: rec {
    name = "rustix";
    version = "0.37.13";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "f79bef90eb6d984c72722595b5b1348ab39275a5e5123faca6863bf07d75a4e0"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "io-lifetimes" ]
      [ "libc" ]
      [ "std" ]
      [ "termios" ]
      [ "use-libc-auxv" ]
    ];
    dependencies = {
      bitflags = rustPackages."registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" { inherit profileName; };
      ${ if !(hostPlatform.parsed.kernel.name == "linux" && (hostPlatform.parsed.cpu.name == "i686" || hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.significantByte == "littleEndian" && (hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l" || hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.name == "powerpc64" || hostPlatform.parsed.cpu.name == "riscv64" || hostPlatform.parsed.cpu.name == "mips" || hostPlatform.parsed.cpu.name == "mips64"))) then "libc_errno" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".errno."0.3.1" { inherit profileName; };
      io_lifetimes = rustPackages."registry+https://github.com/rust-lang/crates.io-index".io-lifetimes."1.0.10" { inherit profileName; };
      ${ if hostPlatform.parsed.kernel.name == "linux" && (hostPlatform.parsed.cpu.name == "i686" || hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.significantByte == "littleEndian" && (hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l" || hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.name == "powerpc64" || hostPlatform.parsed.cpu.name == "riscv64" || hostPlatform.parsed.cpu.name == "mips" || hostPlatform.parsed.cpu.name == "mips64")) || !(hostPlatform.parsed.kernel.name == "linux" && (hostPlatform.parsed.cpu.name == "i686" || hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.significantByte == "littleEndian" && (hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l" || hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.name == "powerpc64" || hostPlatform.parsed.cpu.name == "riscv64" || hostPlatform.parsed.cpu.name == "mips" || hostPlatform.parsed.cpu.name == "mips64"))) then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.141" { inherit profileName; };
      ${ if hostPlatform.parsed.kernel.name == "linux" && (hostPlatform.parsed.cpu.name == "i686" || hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.significantByte == "littleEndian" && (hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l" || hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.name == "powerpc64" || hostPlatform.parsed.cpu.name == "riscv64" || hostPlatform.parsed.cpu.name == "mips" || hostPlatform.parsed.cpu.name == "mips64")) || (hostPlatform.parsed.kernel.name == "android" || hostPlatform.parsed.kernel.name == "linux") && !(hostPlatform.parsed.kernel.name == "linux" && (hostPlatform.parsed.cpu.name == "i686" || hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.significantByte == "littleEndian" && (hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l" || hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.cpu.bits == 64 || hostPlatform.parsed.cpu.name == "powerpc64" || hostPlatform.parsed.cpu.name == "riscv64" || hostPlatform.parsed.cpu.name == "mips" || hostPlatform.parsed.cpu.name == "mips64"))) then "linux_raw_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".linux-raw-sys."0.3.3" { inherit profileName; };
      ${ if hostPlatform.isWindows then "windows_sys" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".strsim."0.10.0" = overridableMkRustCrate (profileName: rec {
    name = "strsim";
    version = "0.10.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "73473c0e59e6d5812c5dfe2a064a6444949f089e20eec9a2e5506596494e4623"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syn."2.0.15" = overridableMkRustCrate (profileName: rec {
    name = "syn";
    version = "2.0.15";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a34fcf3e8b60f57e6a14301a2e916d323af98b0ea63c599441eec8558660c822"; };
    features = builtins.concatLists [
      [ "clone-impls" ]
      [ "default" ]
      [ "derive" ]
      [ "full" ]
      [ "parsing" ]
      [ "printing" ]
      [ "proc-macro" ]
      [ "quote" ]
    ];
    dependencies = {
      proc_macro2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.56" { inherit profileName; };
      quote = rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.26" { inherit profileName; };
      unicode_ident = rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.8" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.8" = overridableMkRustCrate (profileName: rec {
    name = "unicode-ident";
    version = "1.0.8";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e5464a87b239f13a63a501f2701565754bae92d243d4bb7eb12f6d57d2269bf4"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" = overridableMkRustCrate (profileName: rec {
    name = "utf8parse";
    version = "0.2.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "711b9620af191e0cdc7468a8d14e709c3dcdb115b36f838e601583af800a370a"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows-sys";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "677d2418bec65e3338edb076e806bc1ec15693c5d0104683f2efe857f61056a9"; };
    features = builtins.concatLists [
      [ "Win32" ]
      [ "Win32_Foundation" ]
      [ "Win32_NetworkManagement" ]
      [ "Win32_NetworkManagement_IpHelper" ]
      [ "Win32_Networking" ]
      [ "Win32_Networking_WinSock" ]
      [ "Win32_Security" ]
      [ "Win32_Storage" ]
      [ "Win32_Storage_FileSystem" ]
      [ "Win32_System" ]
      [ "Win32_System_Console" ]
      [ "Win32_System_Diagnostics" ]
      [ "Win32_System_Diagnostics_Debug" ]
      [ "Win32_System_IO" ]
      [ "Win32_System_Threading" ]
      [ "default" ]
    ];
    dependencies = {
      windows_targets = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows-targets";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7b1eb6f0cd7c80c79759c929114ef071b87354ce476d9d94271031c0497adfd5"; };
    dependencies = {
      ${ if false then "windows_aarch64_gnullvm" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.48.0" { inherit profileName; };
      ${ if hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.abi.name == "msvc" then "windows_aarch64_msvc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.48.0" { inherit profileName; };
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "gnu" then "windows_i686_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.48.0" { inherit profileName; };
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "msvc" then "windows_i686_msvc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.48.0" { inherit profileName; };
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "gnu" then "windows_x86_64_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.48.0" { inherit profileName; };
      ${ if false then "windows_x86_64_gnullvm" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.48.0" { inherit profileName; };
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "msvc" then "windows_x86_64_msvc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.48.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_gnullvm";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "91ae572e1b79dba883e0d315474df7305d12f569b400fcf90581b06062f7e1bc"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_msvc";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b2ef27e0d7bdfcfc7b868b317c1d32c641a6fe4629c171b8928c7b08d98d7cf3"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_gnu";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "622a1962a7db830d6fd0a69683c80a18fda201879f0f447f065a3b7467daa241"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_msvc";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4542c6e364ce21bf45d69fdd2a8e455fa38d316158cfd43b3ac1c5b1b19f8e00"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnu";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ca2b8a661f7628cbd23440e50b05d705db3686f894fc9580820623656af974b1"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnullvm";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7896dbc1f41e08872e9d5e8f8baa8fdd2677f29468c4e156210174edc7f7b953"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.48.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_msvc";
    version = "0.48.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1a515f5799fe4961cb532f983ce2b23082366b898e52ffbce459c86f67c8378a"; };
  });
  
}
