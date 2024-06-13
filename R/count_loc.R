#' Count lines of code
#'
#' @param paths Paths to include.
#' @param excluded Paths to exclude.
#' @param include_hidden Count hidden files and directories? Default is `FALSE`.
#' @param languages Languages to consider. See the list here: <https:#github.com/XAMPPRocky/tokei?tab=readme-ov-file#supported-languages>.
#'
#' @return A data.frame with 4 columns: number of blank lines, comments, code,
#' and whether the count is accurate.
#' @export
#'
#' @examples
#' tmp <- tempfile(fileext = ".R")
#' cat("
#' library(loc)
#' # This is a function
#' foo <- function(x) {
#'   x + 1
#' }
#'
#' ", file = tmp)
#' count_loc(tmp)
count_loc <- function(paths = ".", excluded = NULL, include_hidden = FALSE, languages = "R") {
  if (!missing(paths) && length(paths) == 0) {
    stop("Must specify at least one path.")
  }
  if (!missing(languages) && length(languages) == 0) {
    stop("Must specify at least one language.")
  }
  check_languages(languages)
  if (is.null(excluded)) {
    excluded <- paste(sample(letters, 100, TRUE), collapse = "")
  }
  count <- internal_count_loc(
    paths = paths,
    excluded = excluded,
    include_hidden = include_hidden,
    languages = languages
  )
  out <- lapply(seq_along(count), function(x) {
    res <- as.data.frame(count[[x]])
    res[["language"]] <- languages[x]
    res
  })
  out <- Reduce(rbind, out)
  out2 <- cbind(out[, "language", drop = FALSE], out[-ncol(out)])
  class(out2) <- c("loc", class(out2))
  out2
}

#' @export
print.loc <- function(x, ...) {
  lang <- unique(x$language)
  for (i in lang) {
    cat("\nLanguage:", i, "\n")
    cat("===================\n")
    cat("Lines of code:", format(x[x$language == i, "code"], big.mark = ","), "\n")
    cat("Blank lines:  ", format(x[x$language == i, "blanks"], big.mark = ","), "\n")
    cat("Comments:     ", format(x[x$language == i, "comments"], big.mark = ","), "\n")
    cat("Accurate: ", ifelse(
      is.na(x[x$language == i, "inaccurate"]),
      "    NA",
      ifelse(x[x$language == i, "inaccurate"], "    No", "    Yes"))
    )
    cat("\n")
  }
}

check_languages <- function(x) {
  not_in_list <- which(!x %in% accepted_languages)
  if (length(not_in_list) > 0) {
    stop(
      paste(
        "The following language(s) are unknown:",
        toString(x[not_in_list])
      )
    )
  }
}

accepted_languages <- c(
  "Abap",
  "ActionScript",
  "Ada",
  "Agda",
  "Alex",
  "Alloy",
  "Asn1",
  "Asp",
  "AspNet",
  "Assembly",
  "AssemblyGAS",
  # "ATS",
  "Autoconf",
  "AutoHotKey",
  "Automake",
  # "AWK",
  "Bash",
  "Batch",
  # "Bazel",
  "Bean",
  # "Bitbake",
  "BrightScript",
  "C",
  "Cabal",
  "Cassius",
  "Ceylon",
  "CHeader",
  "Clojure",
  "ClojureC",
  "ClojureScript",
  "CMake",
  "Cobol",
  "CoffeeScript",
  "Cogent",
  "ColdFusion",
  "ColdFusionScript",
  "Coq",
  "Cpp",
  "CppHeader",
  "Crystal",
  "CSharp",
  "CShell",
  "Css",
  # "Cuda",
  # "Cython",
  "D",
  # "DAML",
  "Dart",
  "DeviceTree",
  "Dhall",
  "Dockerfile",
  "DotNetResource",
  "DreamMaker",
  "Dust",
  # "Ebuild",
  # "EdgeDB",
  "Edn",
  "Elisp",
  "Elixir",
  "Elm",
  "Elvish",
  "EmacsDevEnv",
  "Emojicode",
  "Erlang",
  # "Factor",
  "FEN",
  "Fish",
  "FlatBuffers",
  # "ForgeConfig",
  "Forth",
  "FortranLegacy",
  "FortranModern",
  "FreeMarker",
  "FSharp",
  "Fstar",
  "GDB",
  "GdScript",
  "Gherkin",
  "Gleam",
  "Glsl",
  "Go",
  "Graphql",
  "Groovy",
  "Gwion",
  "Hamlet",
  "Handlebars",
  "Happy",
  # "Hare",
  "Haskell",
  "Haxe",
  "Hcl",
  "Hex",
  "Hlsl",
  "HolyC",
  "Html",
  # "Hy",
  "Idris",
  "Ini",
  "IntelHex",
  "Isabelle",
  "Jai",
  # "Janet",
  "Java",
  "JavaScript",
  # "Jq",
  "Json",
  "Jsx",
  "Julia",
  "Julius",
  "KakouneScript",
  "Kotlin",
  "Lean",
  "Less",
  # "LinkerScript",
  "Liquid",
  "Lisp",
  "LLVM",
  "Logtalk",
  "Lua",
  "Lucius",
  "Madlang",
  # "Max",
  "Makefile",
  "Markdown",
  "Meson",
  "Mint",
  # "Mlatu",
  "ModuleDef",
  "MoonScript",
  "MsBuild",
  "Mustache",
  "Nim",
  "Nix",
  "NotQuitePerl",
  # "NuGetConfig",
  # "Nushell",
  "ObjectiveC",
  "ObjectiveCpp",
  "OCaml",
  "Odin",
  # "OpenSCAD",
  # "OpenQASM",
  "Org",
  "Oz",
  "Pascal",
  "Perl",
  "Perl6",
  "Pest",
  "Php",
  # "Poke",
  "Polly",
  "Pony",
  "PostCss",
  "PowerShell",
  "Processing",
  "Prolog",
  "Protobuf",
  "PSL",
  "PureScript",
  "Python",
  "Qcl",
  "Qml",
  "R",
  "Racket",
  "Rakefile",
  "Razor",
  "Renpy",
  "ReStructuredText",
  "RON",
  "RPMSpecfile",
  "Ruby",
  "RubyHtml",
  "Rust",
  "Sass",
  "Scala",
  "Scheme",
  "Scons",
  "Sh",
  # "ShaderLab",
  # "Slang",
  "Sml",
  "Solidity",
  "SpecmanE",
  "Spice",
  "Sql",
  "SRecode",
  "Stratego",
  "Svelte",
  "Svg",
  "Swift",
  "Swig",
  "SystemVerilog",
  "Tcl",
  "Tex",
  "Text",
  "Thrift",
  "Toml",
  "Tsx",
  "Twig",
  "TypeScript",
  # "UMPL",
  "UnrealDeveloperMarkdown",
  "UnrealPlugin",
  "UnrealProject",
  "UnrealScript",
  "UnrealShader",
  "UnrealShaderHeader",
  "UrWeb",
  "UrWebProject",
  "Vala",
  "VB6",
  "VBScript",
  "Velocity",
  "Verilog",
  "VerilogArgsFile",
  "Vhdl",
  "VimScript",
  "VisualBasic",
  "VisualStudioProject",
  "VisualStudioSolution",
  "Vue",
  "WebAssembly",
  "Wolfram",
  "Xaml",
  "XcodeConfig",
  "Xml",
  "XSL",
  "Xtend",
  "Yaml",
  # "ZenCode",
  "Zig",
  # "ZoKrates",
  "Zsh"
)
