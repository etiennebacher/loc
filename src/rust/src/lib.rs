use extendr_api::prelude::*;
use tokei::{Config, LanguageType, Languages};

#[extendr]
fn internal_count_loc(
    paths: Strings,
    excluded: &str,
    include_hidden: bool,
    languages: Strings,
) -> List {
    let config = Config {
        hidden: Some(include_hidden),
        ..Default::default()
    };
    let paths = paths.iter().map(|si| si.as_str()).collect::<Vec<_>>();
    let excluded = &[excluded];

    let requested_languages = languages
        .into_iter()
        .map(|xi| string_to_language(xi.as_str()))
        .collect::<Vec<LanguageType>>();

    let mut languages = Languages::new();
    languages.get_statistics(paths.as_slice(), excluded, &config);

    let out = requested_languages
        .iter()
        .map(|xi| {
            let res = &languages[xi];
            list!(
                blanks = r!(res.blanks as i32),
                comments = r!(res.comments as i32),
                code = r!(res.code as i32),
                inaccurate = r!(res.inaccurate),
            )
        })
        .collect::<List>();

    out
}

fn string_to_language(string: &str) -> LanguageType {
    match string {
        "Abap" => LanguageType::Abap,
        "ActionScript" => LanguageType::ActionScript,
        "Ada" => LanguageType::Ada,
        "Agda" => LanguageType::Agda,
        "Alex" => LanguageType::Alex,
        "Alloy" => LanguageType::Alloy,
        "Asn1" => LanguageType::Asn1,
        "Asp" => LanguageType::Asp,
        "AspNet" => LanguageType::AspNet,
        "Assembly" => LanguageType::Assembly,
        "AssemblyGAS" => LanguageType::AssemblyGAS,
        // "ATS" => LanguageType::ATS,
        "Autoconf" => LanguageType::Autoconf,
        "AutoHotKey" => LanguageType::AutoHotKey,
        "Automake" => LanguageType::Automake,
        // "AWK" => LanguageType::AWK,
        "Bash" => LanguageType::Bash,
        "Batch" => LanguageType::Batch,
        // "Bazel" => LanguageType::Bazel,
        "Bean" => LanguageType::Bean,
        // "Bitbake" => LanguageType::Bitbake,
        "BrightScript" => LanguageType::BrightScript,
        "C" => LanguageType::C,
        "Cabal" => LanguageType::Cabal,
        "Cassius" => LanguageType::Cassius,
        "Ceylon" => LanguageType::Ceylon,
        "CHeader" => LanguageType::CHeader,
        "Clojure" => LanguageType::Clojure,
        "ClojureC" => LanguageType::ClojureC,
        "ClojureScript" => LanguageType::ClojureScript,
        "CMake" => LanguageType::CMake,
        "Cobol" => LanguageType::Cobol,
        "CoffeeScript" => LanguageType::CoffeeScript,
        "Cogent" => LanguageType::Cogent,
        "ColdFusion" => LanguageType::ColdFusion,
        "ColdFusionScript" => LanguageType::ColdFusionScript,
        "Coq" => LanguageType::Coq,
        "Cpp" => LanguageType::Cpp,
        "CppHeader" => LanguageType::CppHeader,
        "Crystal" => LanguageType::Crystal,
        "CSharp" => LanguageType::CSharp,
        "CShell" => LanguageType::CShell,
        "Css" => LanguageType::Css,
        // "Cuda" => LanguageType::Cuda,
        // "Cython" => LanguageType::Cython,
        "D" => LanguageType::D,
        // "DAML" => LanguageType::DAML,
        "Dart" => LanguageType::Dart,
        "DeviceTree" => LanguageType::DeviceTree,
        "Dhall" => LanguageType::Dhall,
        "Dockerfile" => LanguageType::Dockerfile,
        "DotNetResource" => LanguageType::DotNetResource,
        "DreamMaker" => LanguageType::DreamMaker,
        "Dust" => LanguageType::Dust,
        // "Ebuild" => LanguageType::Ebuild,
        // "EdgeDB" => LanguageType::EdgeDB,
        "Edn" => LanguageType::Edn,
        "Elisp" => LanguageType::Elisp,
        "Elixir" => LanguageType::Elixir,
        "Elm" => LanguageType::Elm,
        "Elvish" => LanguageType::Elvish,
        "EmacsDevEnv" => LanguageType::EmacsDevEnv,
        "Emojicode" => LanguageType::Emojicode,
        "Erlang" => LanguageType::Erlang,
        // "Factor" => LanguageType::Factor,
        "FEN" => LanguageType::FEN,
        "Fish" => LanguageType::Fish,
        "FlatBuffers" => LanguageType::FlatBuffers,
        // "ForgeConfig" => LanguageType::ForgeConfig,
        "Forth" => LanguageType::Forth,
        "FortranLegacy" => LanguageType::FortranLegacy,
        "FortranModern" => LanguageType::FortranModern,
        "FreeMarker" => LanguageType::FreeMarker,
        "FSharp" => LanguageType::FSharp,
        "Fstar" => LanguageType::Fstar,
        "GDB" => LanguageType::GDB,
        "GdScript" => LanguageType::GdScript,
        "Gherkin" => LanguageType::Gherkin,
        "Gleam" => LanguageType::Gleam,
        "Glsl" => LanguageType::Glsl,
        "Go" => LanguageType::Go,
        "Graphql" => LanguageType::Graphql,
        "Groovy" => LanguageType::Groovy,
        "Gwion" => LanguageType::Gwion,
        "Hamlet" => LanguageType::Hamlet,
        "Handlebars" => LanguageType::Handlebars,
        "Happy" => LanguageType::Happy,
        // "Hare" => LanguageType::Hare,
        "Haskell" => LanguageType::Haskell,
        "Haxe" => LanguageType::Haxe,
        "Hcl" => LanguageType::Hcl,
        "Hex" => LanguageType::Hex,
        "Hlsl" => LanguageType::Hlsl,
        "HolyC" => LanguageType::HolyC,
        "Html" => LanguageType::Html,
        // "Hy" => LanguageType::Hy,
        "Idris" => LanguageType::Idris,
        "Ini" => LanguageType::Ini,
        "IntelHex" => LanguageType::IntelHex,
        "Isabelle" => LanguageType::Isabelle,
        "Jai" => LanguageType::Jai,
        // "Janet" => LanguageType::Janet,
        "Java" => LanguageType::Java,
        "JavaScript" => LanguageType::JavaScript,
        // "Jq" => LanguageType::Jq,
        "Json" => LanguageType::Json,
        "Jsx" => LanguageType::Jsx,
        "Julia" => LanguageType::Julia,
        "Julius" => LanguageType::Julius,
        "KakouneScript" => LanguageType::KakouneScript,
        "Kotlin" => LanguageType::Kotlin,
        "Lean" => LanguageType::Lean,
        "Less" => LanguageType::Less,
        // "LinkerScript" => LanguageType::LinkerScript,
        "Liquid" => LanguageType::Liquid,
        "Lisp" => LanguageType::Lisp,
        "LLVM" => LanguageType::LLVM,
        "Logtalk" => LanguageType::Logtalk,
        "Lua" => LanguageType::Lua,
        "Lucius" => LanguageType::Lucius,
        "Madlang" => LanguageType::Madlang,
        // "Max" => LanguageType::Max,
        "Makefile" => LanguageType::Makefile,
        "Markdown" => LanguageType::Markdown,
        "Meson" => LanguageType::Meson,
        "Mint" => LanguageType::Mint,
        // "Mlatu" => LanguageType::Mlatu,
        "ModuleDef" => LanguageType::ModuleDef,
        "MoonScript" => LanguageType::MoonScript,
        "MsBuild" => LanguageType::MsBuild,
        "Mustache" => LanguageType::Mustache,
        "Nim" => LanguageType::Nim,
        "Nix" => LanguageType::Nix,
        "NotQuitePerl" => LanguageType::NotQuitePerl,
        // "NuGetConfig" => LanguageType::NuGetConfig,
        // "Nushell" => LanguageType::Nushell,
        "ObjectiveC" => LanguageType::ObjectiveC,
        "ObjectiveCpp" => LanguageType::ObjectiveCpp,
        "OCaml" => LanguageType::OCaml,
        "Odin" => LanguageType::Odin,
        // "OpenSCAD" => LanguageType::OpenSCAD,
        // "OpenQASM" => LanguageType::OpenQASM,
        "Org" => LanguageType::Org,
        "Oz" => LanguageType::Oz,
        "Pascal" => LanguageType::Pascal,
        "Perl" => LanguageType::Perl,
        "Perl6" => LanguageType::Perl6,
        "Pest" => LanguageType::Pest,
        "Php" => LanguageType::Php,
        // "Poke" => LanguageType::Poke,
        "Polly" => LanguageType::Polly,
        "Pony" => LanguageType::Pony,
        "PostCss" => LanguageType::PostCss,
        "PowerShell" => LanguageType::PowerShell,
        "Processing" => LanguageType::Processing,
        "Prolog" => LanguageType::Prolog,
        "Protobuf" => LanguageType::Protobuf,
        "PSL" => LanguageType::PSL,
        "PureScript" => LanguageType::PureScript,
        "Python" => LanguageType::Python,
        "Qcl" => LanguageType::Qcl,
        "Qml" => LanguageType::Qml,
        "R" => LanguageType::R,
        "Racket" => LanguageType::Racket,
        "Rakefile" => LanguageType::Rakefile,
        "Razor" => LanguageType::Razor,
        "Renpy" => LanguageType::Renpy,
        "ReStructuredText" => LanguageType::ReStructuredText,
        "RON" => LanguageType::RON,
        "RPMSpecfile" => LanguageType::RPMSpecfile,
        "Ruby" => LanguageType::Ruby,
        "RubyHtml" => LanguageType::RubyHtml,
        "Rust" => LanguageType::Rust,
        "Sass" => LanguageType::Sass,
        "Scala" => LanguageType::Scala,
        "Scheme" => LanguageType::Scheme,
        "Scons" => LanguageType::Scons,
        "Sh" => LanguageType::Sh,
        // "ShaderLab" => LanguageType::ShaderLab,
        // "Slang" => LanguageType::Slang,
        "Sml" => LanguageType::Sml,
        "Solidity" => LanguageType::Solidity,
        "SpecmanE" => LanguageType::SpecmanE,
        "Spice" => LanguageType::Spice,
        "Sql" => LanguageType::Sql,
        "SRecode" => LanguageType::SRecode,
        "Stratego" => LanguageType::Stratego,
        "Svelte" => LanguageType::Svelte,
        "Svg" => LanguageType::Svg,
        "Swift" => LanguageType::Swift,
        "Swig" => LanguageType::Swig,
        "SystemVerilog" => LanguageType::SystemVerilog,
        "Tcl" => LanguageType::Tcl,
        "Tex" => LanguageType::Tex,
        "Text" => LanguageType::Text,
        "Thrift" => LanguageType::Thrift,
        "Toml" => LanguageType::Toml,
        "Tsx" => LanguageType::Tsx,
        "Twig" => LanguageType::Twig,
        "TypeScript" => LanguageType::TypeScript,
        // "UMPL" => LanguageType::UMPL,
        "UnrealDeveloperMarkdown" => LanguageType::UnrealDeveloperMarkdown,
        "UnrealPlugin" => LanguageType::UnrealPlugin,
        "UnrealProject" => LanguageType::UnrealProject,
        "UnrealScript" => LanguageType::UnrealScript,
        "UnrealShader" => LanguageType::UnrealShader,
        "UnrealShaderHeader" => LanguageType::UnrealShaderHeader,
        "UrWeb" => LanguageType::UrWeb,
        "UrWebProject" => LanguageType::UrWebProject,
        "Vala" => LanguageType::Vala,
        "VB6" => LanguageType::VB6,
        "VBScript" => LanguageType::VBScript,
        "Velocity" => LanguageType::Velocity,
        "Verilog" => LanguageType::Verilog,
        "VerilogArgsFile" => LanguageType::VerilogArgsFile,
        "Vhdl" => LanguageType::Vhdl,
        "VimScript" => LanguageType::VimScript,
        "VisualBasic" => LanguageType::VisualBasic,
        "VisualStudioProject" => LanguageType::VisualStudioProject,
        "VisualStudioSolution" => LanguageType::VisualStudioSolution,
        "Vue" => LanguageType::Vue,
        "WebAssembly" => LanguageType::WebAssembly,
        "Wolfram" => LanguageType::Wolfram,
        "Xaml" => LanguageType::Xaml,
        "XcodeConfig" => LanguageType::XcodeConfig,
        "Xml" => LanguageType::Xml,
        "XSL" => LanguageType::XSL,
        "Xtend" => LanguageType::Xtend,
        "Yaml" => LanguageType::Yaml,
        // "ZenCode" => LanguageType::ZenCode,
        "Zig" => LanguageType::Zig,
        // "ZoKrates" => LanguageType::ZoKrates,
        "Zsh" => LanguageType::Zsh,
        _ => unimplemented!(),
    }
}

extendr_module! {
    mod loc;
    fn internal_count_loc;
}
