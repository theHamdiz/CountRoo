
#[derive(Debug, Clone, Copy)]
pub enum AccentColor<'a> {
    Hex(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct LanguageColorMapping<'a> {
    pub(crate) extension: &'a str,
    pub(crate) color: AccentColor<'a>,
}

#[derive(Debug, Clone)]
pub struct LanguageBrandings<'a>{
    pub(crate) color_map: Vec<LanguageColorMapping<'a>>,
}

impl<'a> LanguageBrandings<'a>{
    fn new(color_map: Vec<LanguageColorMapping<'a>>) -> LanguageBrandings<'a> {
        LanguageBrandings {
            color_map,
        }
    }
}

impl<'a> Default for LanguageBrandings<'a>{
    fn default() -> Self{
        let color_map : Vec<LanguageColorMapping<'a>> = vec![
        LanguageColorMapping{ extension:".1sc", color: AccentColor::Hex("#E45649")}, // SourcePawn
        LanguageColorMapping{ extension:".abap", color: AccentColor::Hex("#E8274B")}, // ABAP
        LanguageColorMapping{ extension:".abc", color: AccentColor::Hex("#3A6E79")}, // ABC
        LanguageColorMapping{ extension:".ada", color: AccentColor::Hex("#02f88c")}, // Ada
        LanguageColorMapping{ extension:".adb", color: AccentColor::Hex("#02f88c")}, // Ada
        LanguageColorMapping{ extension:".ads", color: AccentColor::Hex("#02f88c")}, // Ada
        LanguageColorMapping{ extension:".ahk", color: AccentColor::Hex("#6594b9")}, // AutoHotkey
        LanguageColorMapping{ extension:".apib", color: AccentColor::Hex("#96C224")}, // API Blueprint (grass green)
        LanguageColorMapping{ extension:".applescript", color: AccentColor::Hex("#101F1F")}, // AppleScript
        LanguageColorMapping{ extension:".as", color: AccentColor::Hex("#B38959")},  // ActionScript
        LanguageColorMapping{ extension:".ascx", color: AccentColor::Hex("#5d87b3")}, // ASP.NET (light blue)
        LanguageColorMapping{ extension:".asm", color: AccentColor::Hex("#A8B89F")}, // Assembly
        LanguageColorMapping{ extension:".aspx", color: AccentColor::Hex("#5d87b3")}, // ASP.NET
        LanguageColorMapping{ extension:".awk", color: AccentColor::Hex("#d0d0d0")},  // Awk
        LanguageColorMapping{ extension:".bash", color: AccentColor::Hex("#89e051")}, // Bash
        LanguageColorMapping{ extension:".bb", color: AccentColor::Hex("#A571D3")},  // BlitzBasic (purple-ish)
        LanguageColorMapping{ extension:".bcl", color: AccentColor::Hex("#295789")}, // Caché ObjectScript (dark blue)
        LanguageColorMapping{ extension:".boo", color: AccentColor::Hex("#d4bec1")}, // Boo (light purple)
        LanguageColorMapping{ extension:".c", color: AccentColor::Hex("#555555")},   // C (neutral gray)
        LanguageColorMapping{ extension:".cbl", color: AccentColor::Hex("#A040A0")}, // COBOL (purple)
        LanguageColorMapping{ extension:".cc", color: AccentColor::Hex("#f34b7d")},  // C++ (pinkish-red)
        LanguageColorMapping{ extension:".cfg", color: AccentColor::Hex("#e0d0a0")}, // Config files (light yellow)
        LanguageColorMapping{ extension:".cfm", color: AccentColor::Hex("#878E99")}, // ColdFusion (gray-blue)
        LanguageColorMapping{ extension:".cfml", color: AccentColor::Hex("#878E99")}, // ColdFusion
        LanguageColorMapping{ extension:".cgi", color: AccentColor::Hex("#fc913a")}, // Perl/CGI (orange)
        LanguageColorMapping{ extension:".cl", color: AccentColor::Hex("#234d20")},  // Common Lisp (dark green)
        LanguageColorMapping{ extension:".clj", color: AccentColor::Hex("#db5855")}, // Clojure (reddish-orange)
        LanguageColorMapping{ extension:".cljs", color: AccentColor::Hex("#db5855")}, // ClojureScript
        LanguageColorMapping{ extension:".cls", color: AccentColor::Hex("#e0e0e0")}, // Visual Basic class files (light gray)
        LanguageColorMapping{ extension:".cmake", color: AccentColor::Hex("#DA3434")}, // CMake (red)
        LanguageColorMapping{ extension:".coffee", color: AccentColor::Hex("#244776")}, // CoffeeScript (dark blue)
        LanguageColorMapping{ extension:".cp", color: AccentColor::Hex("#f34b7d")}, // C++ (same as .cpp)
        LanguageColorMapping{ extension:".cpp", color: AccentColor::Hex("#f34b7d")}, // C++
        LanguageColorMapping{ extension:".cr", color: AccentColor::Hex("#d37295")}, // Crystal (pink-purple)
        LanguageColorMapping{ extension:".cs", color: AccentColor::Hex("#8a3996")},  // C# (purple)
        LanguageColorMapping{ extension:".csh", color: AccentColor::Hex("#89e051")}, // C shell (often associated with Bash)
        LanguageColorMapping{ extension:".cson", color: AccentColor::Hex("#244776")}, // CSON (same as CoffeeScript)
        LanguageColorMapping{ extension:".css", color: AccentColor::Hex("#563d7c")}, // CSS
        LanguageColorMapping{ extension:".csv", color: AccentColor::Hex("#23ab24")}, // CSV (green)
        LanguageColorMapping{ extension:".cxx", color: AccentColor::Hex("#f34b7d")}, // C++ (same as .cpp)
        LanguageColorMapping{ extension:".cmd", color: AccentColor::Hex("#FFFF80")}, // Example DOS-yellow color
        LanguageColorMapping{ extension:".bat", color: AccentColor::Hex("#FFFF80")}, // Example DOs-yellow color
        LanguageColorMapping{ extension:".d", color: AccentColor::Hex("#ba595e")},  // D (reddish-brown)
        LanguageColorMapping{ extension:".dart", color: AccentColor::Hex("#00B4AB")}, // Dart
        LanguageColorMapping{ extension:".def", color: AccentColor::Hex("#e0e0e0")}, // Generic definition files
        LanguageColorMapping{ extension:".diff", color: AccentColor::Hex("#888888")}, // Diff/patch files (gray)
        LanguageColorMapping{ extension:".dml", color: AccentColor::Hex("#005C99")}, // Data Manipulation Language (blue)
        LanguageColorMapping{ extension:".do", color: AccentColor::Hex("#8B0000")},  // Stata ADO file (dark red)
        LanguageColorMapping{ extension:".dtd", color: AccentColor::Hex("#e0d0a0")}, // Document Type Definition (light yellow)
        LanguageColorMapping{ extension:".e", color: AccentColor::Hex("#ccce35")},   // Eiffel (yellowish)

        LanguageColorMapping{ extension:".ebnf", color: AccentColor::Hex("#9A7B59")}, // EBNF (golden brown)
        LanguageColorMapping{ extension:".el", color: AccentColor::Hex("#027878")},  // Emacs Lisp (teal)
        LanguageColorMapping{ extension:".erb", color: AccentColor::Hex("#701516")}, // Embedded Ruby (same as Ruby)
        LanguageColorMapping{ extension:".erl", color: AccentColor::Hex("#B83998")}, // Erlang (magenta-ish)
        LanguageColorMapping{ extension:".es", color: AccentColor::Hex("#CC7832")},  // JavaScript (often used for older specifications)
        LanguageColorMapping{ extension:".escript", color: AccentColor::Hex("#CC7832")}, // Erlang Script (same as .es)
        LanguageColorMapping{ extension:".ex", color: AccentColor::Hex("#6e4a7e")},  // Elixir (purple)
        LanguageColorMapping{ extension:".exs", color: AccentColor::Hex("#6e4a7e")}, // Elixir Script
        LanguageColorMapping{ extension:".f", color: AccentColor::Hex("#572e30")},   // Fortran (reddish-brown)
        LanguageColorMapping{ extension:".f03", color: AccentColor::Hex("#572e30")}, // Fortran
        LanguageColorMapping{ extension:".f77", color: AccentColor::Hex("#572e30")}, // Fortran
        LanguageColorMapping{ extension:".f90", color: AccentColor::Hex("#572e30")}, // Fortran
        LanguageColorMapping{ extension:".f95", color: AccentColor::Hex("#572e30")}, // Fortran
        LanguageColorMapping{ extension:".fish", color: AccentColor::Hex("#89e051")}, // Fish shell (like Bash)
        LanguageColorMapping{ extension:".for", color: AccentColor::Hex("#572e30")}, // Fortran (same as .f)
        LanguageColorMapping{ extension:".fpp", color: AccentColor::Hex("#f34b7d")}, // Fortran preprocessor (same as C++)
        LanguageColorMapping{ extension:".fs", color: AccentColor::Hex("#584475")},  // F# (purple)
        LanguageColorMapping{ extension:".fsi", color: AccentColor::Hex("#584475")}, // F# signature
        LanguageColorMapping{ extension:".fsx", color: AccentColor::Hex("#584475")}, // F# script
        LanguageColorMapping{ extension:".fsscript", color: AccentColor::Hex("#584475")}, // F# script
        LanguageColorMapping{ extension:".g4", color: AccentColor::Hex("#FFAB28")}, // ANTLR Grammar (orange)
        LanguageColorMapping{ extension:".go", color: AccentColor::Hex("#00ADD8")},  // Go
        LanguageColorMapping{ extension:".gotmpl", color: AccentColor::Hex("#3777E6")}, // Go template (similar to TypeScript)
        LanguageColorMapping{ extension:".groovy", color: AccentColor::Hex("#e69f56")}, // Groovy (orange)
        LanguageColorMapping{ extension:".gs", color: AccentColor::Hex("#FFD740")}, // Google Apps Script (yellow)

        LanguageColorMapping{ extension:".h", color: AccentColor::Hex("#408080")},   // C/C++ Header (teal-ish)
        LanguageColorMapping{ extension:".handlebars", color: AccentColor::Hex("#f7931e")}, // Handlebars (orange)
        LanguageColorMapping{ extension:".hbs", color: AccentColor::Hex("#f7931e")},      // Handlebars
        LanguageColorMapping{ extension:".hlsl", color: AccentColor::Hex("#aace60")},     // HLSL (greenish)
        LanguageColorMapping{ extension:".hpp", color: AccentColor::Hex("#f34b7d")},  // C++ header (same as .cpp)
        LanguageColorMapping{ extension:".hs", color: AccentColor::Hex("#29b544")},   // Haskell (green)
        LanguageColorMapping{ extension:".hx", color: AccentColor::Hex("#ea8a00")},   // Haxe (orange)
        LanguageColorMapping{ extension:".hxx", color: AccentColor::Hex("#f34b7d")},  // C++ header (same as .cpp)
        LanguageColorMapping{ extension:".icl", color: AccentColor::Hex("#808000")},   // Clean (olive green)
        LanguageColorMapping{ extension:".imba", color: AccentColor::Hex("#16cec6")}, // Imba (light blue-green)
        LanguageColorMapping{ extension:".inc", color: AccentColor::Hex("#e0e0e0")},  // Generic include files
        LanguageColorMapping{ extension:".ini", color: AccentColor::Hex("#d1dbe0")},  // INI config files (light purple)
        LanguageColorMapping{ extension:".ino", color: AccentColor::Hex("#c867c3")},  // Arduino (purple-ish)
        LanguageColorMapping{ extension:".int", color: AccentColor::Hex("#4F4F4F")}, // Interface file (dark gray)
        LanguageColorMapping{ extension:".inx", color: AccentColor::Hex("#d1dbe0")},  // InDesign Markup Language (like INI)
        LanguageColorMapping{ extension:".ipynb", color: AccentColor::Hex("#DA5B0B")}, // Jupyter Notebook (orange)
        LanguageColorMapping{ extension:".j", color: AccentColor::Hex("#93A1A1")},   // J (gray)
        LanguageColorMapping{ extension:".jade", color: AccentColor::Hex("#58b346")}, // Jade (green) – replaced by Pug

        LanguageColorMapping{ extension:".java", color: AccentColor::Hex("#b07219")}, // Java
        LanguageColorMapping{ extension:".jl", color: AccentColor::Hex("#a270ba")},  // Julia (purple)
        LanguageColorMapping{ extension:".js", color: AccentColor::Hex("#f1e05a")},  // JavaScript
        LanguageColorMapping{ extension:".json", color: AccentColor::Hex("#292929")}, // JSON
        LanguageColorMapping{ extension:".jsonld", color: AccentColor::Hex("#292929")}, // JSON-LD (same as JSON)
        LanguageColorMapping{ extension:".jsp", color: AccentColor::Hex("#6D83B8")}, // Java Server Pages (blue-ish)
        LanguageColorMapping{ extension:".jsx", color: AccentColor::Hex("#61DAFB")},  // JSX
        LanguageColorMapping{ extension:".ksh", color: AccentColor::Hex("#408080")}, // KornShell (teal-ish)
        LanguageColorMapping{ extension:".kt", color: AccentColor::Hex("#A97BFF")}, // Kotlin (purple)
        LanguageColorMapping{ extension:".kts", color: AccentColor::Hex("#A97BFF")}, // Kotlin Script
        LanguageColorMapping{ extension:".launch", color: AccentColor::Hex("#1B5089")}, // VS Code launch config (dark blue)
        LanguageColorMapping{ extension:".less", color: AccentColor::Hex("#2a4d69")}, // LESS (dark blue)
        LanguageColorMapping{ extension:".lhs", color: AccentColor::Hex("#71ab5a")}, // Literate Haskell (green)
        LanguageColorMapping{ extension:".lisp", color: AccentColor::Hex("#027878")}, // Lisp (teal, related to Emacs)
        LanguageColorMapping{ extension:".log", color: AccentColor::Hex("#A0A0A0")}, // General log files (gray)
        LanguageColorMapping{ extension:".ls", color: AccentColor::Hex("#3c6d90")},  // LiveScript (blue)
        LanguageColorMapping{ extension:".lsp", color: AccentColor::Hex("#027878")}, // Lisp (same color association)
        LanguageColorMapping{ extension:".lua", color: AccentColor::Hex("#000080")}, // Lua

        LanguageColorMapping{ extension:".m", color: AccentColor::Hex("#1e4a7e")},    // Objective-C (purple-ish)
        LanguageColorMapping{ extension:".m4", color: AccentColor::Hex("#EE82EE")},   // M4 macro language (violet)
        LanguageColorMapping{ extension:".mak", color: AccentColor::Hex("#408080")},  // Makefile (teal-ish)
        LanguageColorMapping{ extension:".md", color: AccentColor::Hex("#BFBFBF")},   // Markdown (gray)
        LanguageColorMapping{ extension:".mk", color: AccentColor::Hex("#408080")},   // Makefile (same as .mak)
        LanguageColorMapping{ extension:".ml", color: AccentColor::Hex("#e3d25b")},   // OCaml (yellow)
        LanguageColorMapping{ extension:".mli", color: AccentColor::Hex("#e3d25b")},  // OCaml interface
        LanguageColorMapping{ extension:".mlir", color: AccentColor::Hex("#5EC8DB")}, // MLIR (cyan-ish)
        LanguageColorMapping{ extension:".mm", color: AccentColor::Hex("#1e4a7e")},   // Objective-C++
        LanguageColorMapping{ extension:".mo", color: AccentColor::Hex("#FF2077")},   // Modelica (pink)
        LanguageColorMapping{ extension:".mod", color: AccentColor::Hex("#902000")},  // Modula-2 (dark red)
        LanguageColorMapping{ extension:".ms", color: AccentColor::Hex("#800080")},   // MAXScript (purple)
        LanguageColorMapping{ extension:".mtml", color: AccentColor::Hex("#b7e1f4")}, // MTML (light blue)
        LanguageColorMapping{ extension:".mustache", color: AccentColor::Hex("#724b3b")}, // Mustache (brownish)
        LanguageColorMapping{ extension:".njk", color: AccentColor::Hex("#724b3b")}, // Nunjucks (same as Mustache)

        LanguageColorMapping{ extension:".ny", color: AccentColor::Hex("#C41A16")}, // Nyquist (dark red)
        LanguageColorMapping{ extension:".oc", color: AccentColor::Hex("#f8ac59")}, // Objective-C (orange)
        LanguageColorMapping{ extension:".odc", color: AccentColor::Hex("#408080")}, // Oxygene (teal-ish)
        LanguageColorMapping{ extension:".pas", color: AccentColor::Hex("#E3F171")}, // Pascal (light green)
        LanguageColorMapping{ extension:".patch", color: AccentColor::Hex("#888888")}, // Patch files (gray, like .diff)
        LanguageColorMapping{ extension:".php", color: AccentColor::Hex("#4F5D95")}, // PHP
        LanguageColorMapping{ extension:".php3", color: AccentColor::Hex("#4F5D95")}, // PHP
        LanguageColorMapping{ extension:".php4", color: AccentColor::Hex("#4F5D95")}, // PHP
        LanguageColorMapping{ extension:".php5", color: AccentColor::Hex("#4F5D95")}, // PHP
        LanguageColorMapping{ extension:".phtml", color: AccentColor::Hex("#4F5D95")}, // PHP
        LanguageColorMapping{ extension:".pl", color: AccentColor::Hex("#f0a83a")},  // Perl (orange)
        LanguageColorMapping{ extension:".pl6", color: AccentColor::Hex("#00896b")}, // Perl 6 (teal-green)
        LanguageColorMapping{ extension:".plx", color: AccentColor::Hex("#f0a83a")},  // Perl Module (same as .pl)
        LanguageColorMapping{ extension:".pm", color: AccentColor::Hex("#f0a83a")},  // Perl Module
        LanguageColorMapping{ extension:".po", color: AccentColor::Hex("#cc6600")},  // Gettext Translation (orange)
        LanguageColorMapping{ extension:".pot", color: AccentColor::Hex("#cc6600")}, // Gettext Translation template
        LanguageColorMapping{ extension:".pov", color: AccentColor::Hex("#4A5C95")}, // POV-Ray Scene Description Language (blue-gray)
        LanguageColorMapping{ extension:".pp", color: AccentColor::Hex("#302B6D")}, // Puppet (purple)
        LanguageColorMapping{ extension:".prg", color: AccentColor::Hex("#e3d25b")}, // Progress, OpenEdge ABL (yellow, similar to OCaml)
        LanguageColorMapping{ extension:".ps", color: AccentColor::Hex("#0092ca")}, // PostScript (blue)
        LanguageColorMapping{ extension:".ps1", color: AccentColor::Hex("#57A64A")}, // PowerShell (green)
        LanguageColorMapping{ extension:".psd1", color: AccentColor::Hex("#57A64A")}, // PowerShell data file
        LanguageColorMapping{ extension:".psm1", color: AccentColor::Hex("#57A64A")}, // PowerShell module

        LanguageColorMapping{ extension:".py", color: AccentColor::Hex("#3572A5")},  // Python
        LanguageColorMapping{ extension:".pyc", color: AccentColor::Hex("#3572A5")}, // Python compiled
        LanguageColorMapping{ extension:".pyd", color: AccentColor::Hex("#3572A5")}, // Python dynamic library
        LanguageColorMapping{ extension:".pyi", color: AccentColor::Hex("#3572A5")}, // Python type hints
        LanguageColorMapping{ extension:".pyo", color: AccentColor::Hex("#3572A5")}, // Python optimized code
        LanguageColorMapping{ extension:".pyw", color: AccentColor::Hex("#3572A5")}, // Python GUI
        LanguageColorMapping{ extension:".pyx", color: AccentColor::Hex("#3572A5")}, // Cython
        LanguageColorMapping{ extension:".qml", color: AccentColor::Hex("#44a57a")}, // QML (green)
        LanguageColorMapping{ extension:".r", color: AccentColor::Hex("#358a5b")},   // R (green)
        LanguageColorMapping{ extension:".rake", color: AccentColor::Hex("#d12127")}, // Rake (red)
        LanguageColorMapping{ extension:".rb", color: AccentColor::Hex("#701516")},  // Ruby
        LanguageColorMapping{ extension:".rhtml", color: AccentColor::Hex("#701516")}, // Ruby HTML (RHTML)
        LanguageColorMapping{ extension:".rjs", color: AccentColor::Hex("#701516")}, // Ruby JS
        LanguageColorMapping{ extension:".rs", color: AccentColor::Hex("#dea584")},  // Rust
        LanguageColorMapping{ extension:".rst", color: AccentColor::Hex("#7D4900")}, // reStructuredText (brownish orange)
        LanguageColorMapping{ extension:".rt", color: AccentColor::Hex("#306e9e")}, // React Template (blue)
        LanguageColorMapping{ extension:".ru", color: AccentColor::Hex("#701516")},  // Ruby (just in case)

        LanguageColorMapping{ extension:".s", color: AccentColor::Hex("#C97B4A")},  // Assembly (sometimes distinct from .asm)
        LanguageColorMapping{ extension:".sass", color: AccentColor::Hex("#CF649A")}, // Sass (pinkish)
        LanguageColorMapping{ extension:".scala", color: AccentColor::Hex("#c22d40")}, // Scala (red)
        LanguageColorMapping{ extension:".scm", color: AccentColor::Hex("#5e9d08")}, // Scheme (green)
        LanguageColorMapping{ extension:".scpt", color: AccentColor::Hex("#101F1F")}, // AppleScript
        LanguageColorMapping{ extension:".scss", color: AccentColor::Hex("#CF649A")}, // SCSS (same as Sass)
        LanguageColorMapping{ extension:".sh", color: AccentColor::Hex("#89e051")}, // Shell (like Bash)
        LanguageColorMapping{ extension:".shtml", color: AccentColor::Hex("#e45649")}, // Server Side Includes (SSI) (reddish)
        LanguageColorMapping{ extension:".sls", color: AccentColor::Hex("#DC3E1F")}, // SaltStack (red)
        LanguageColorMapping{ extension:".smarty", color: AccentColor::Hex("#f0c040")}, // Smarty (golden yellow)
        LanguageColorMapping{ extension:".sol", color: AccentColor::Hex("#365980")}, // Solidity (navy blue)
        LanguageColorMapping{ extension:".sql", color: AccentColor::Hex("#e38c00")},  // SQL
        LanguageColorMapping{ extension:".st", color: AccentColor::Hex("#3a7979")}, // Smalltalk (teal)
        LanguageColorMapping{ extension:".styl", color: AccentColor::Hex("#ff6347")}, // Stylus (tomato red)
        LanguageColorMapping{ extension:".sv", color: AccentColor::Hex("#3c6d90")}, // SystemVerilog (blue)
        LanguageColorMapping{ extension:".svh", color: AccentColor::Hex("#3c6d90")}, // SystemVerilog Header
        LanguageColorMapping{ extension:".swift", color: AccentColor::Hex("#F05138")}, // Swift

        LanguageColorMapping{ extension:".t", color: AccentColor::Hex("#E00404")}, // Tcl (red)
        LanguageColorMapping{ extension:".tcl", color: AccentColor::Hex("#E44A2D")}, // Tcl (orange)
        LanguageColorMapping{ extension:".tex", color: AccentColor::Hex("#3D6117")}, // TeX/LaTeX (greenish)
        LanguageColorMapping{ extension:".tf", color: AccentColor::Hex("#E15A1F")}, // Terraform (orange)
        LanguageColorMapping{ extension:".tfvars", color: AccentColor::Hex("#E15A1F")}, // Terraform variable file
        LanguageColorMapping{ extension:".thor", color: AccentColor::Hex("#701516")}, // Thor (Ruby association)
        LanguageColorMapping{ extension:".tmLanguage", color: AccentColor::Hex("#252525")}, // TextMate Language grammar (dark gray)
        LanguageColorMapping{ extension:".ts", color: AccentColor::Hex("#3777E6")},  // TypeScript
        LanguageColorMapping{ extension:".tsx", color: AccentColor::Hex("#3777E6")}, // TypeScript with JSX
        LanguageColorMapping{ extension:".twig", color: AccentColor::Hex("#c1d026")}, // Twig (yellowish green)
        LanguageColorMapping{ extension:".txt", color: AccentColor::Hex("#DCDCDC")}, // Plain text files (light gray)
        LanguageColorMapping{ extension:".vb", color: AccentColor::Hex("#8a3996")}, // Visual Basic (like C#)
        LanguageColorMapping{ extension:".vba", color: AccentColor::Hex("#867db1")}, // Visual Basic for Applications (light blue-purple)
        LanguageColorMapping{ extension:".vbs", color: AccentColor::Hex("#867db1")}, // VBScript
        LanguageColorMapping{ extension:".vhdl", color: AccentColor::Hex("#543978")}, // VHDL (purple)
        LanguageColorMapping{ extension:".vtl", color: AccentColor::Hex("#1e4a7e")}, // Velocity Template Language (purple-ish)
        LanguageColorMapping{ extension:".vue", color: AccentColor::Hex("#41B883")}, // Vue.js
        LanguageColorMapping{ extension:".webapp", color: AccentColor::Hex("#68C3A6")}, // ColdFusion Web Application (greenish-blue)

        LanguageColorMapping{ extension:".wxml", color: AccentColor::Hex("#388bff")}, // WeChat Mini Program WXML (blue)
        LanguageColorMapping{ extension:".wxss", color: AccentColor::Hex("#388bff")}, // WeChat Mini Program WXSS (blue)
        LanguageColorMapping{ extension:".xaml", color: AccentColor::Hex("#6A2DAD")}, // XAML (purple)
        LanguageColorMapping{ extension:".xml", color: AccentColor::Hex("#0066cc")},  // XML
        LanguageColorMapping{ extension:".xsd", color: AccentColor::Hex("#0066cc")}, // XML Schema Definition
        LanguageColorMapping{ extension:".xsl", color: AccentColor::Hex("#0066cc")}, // XML Stylesheet Language
        LanguageColorMapping{ extension:".xslt", color: AccentColor::Hex("#0066cc")}, // XSLT
        LanguageColorMapping{ extension:".yaml", color: AccentColor::Hex("#cb171e")}, // YAML
        LanguageColorMapping{ extension:".yml", color: AccentColor::Hex("#cb171e")}, // YAML
        LanguageColorMapping{ extension:".zsh", color: AccentColor::Hex("#89e051")}, // Zsh (like Bash)

        // Default Fallback
        LanguageColorMapping{ extension:"*", color: AccentColor::Hex("#cccccc")},];    // Light gray as a default

        LanguageBrandings::new(
            color_map
        )
    }
}
