# leafc_build

This crate defines data structures relevant to the build process of leafc (e.g. the build plan).
This is used for **incremental compilation** (i.e. only recompiling the parts of the program that have changed).
Results of previous builds are cached in the `target` directory. This allows for faster builds.

Dependency Graph and Action Graph

The build process of leafc is split into two phases:

1. **Dependency Graph**: This phase is responsible for determining the dependencies of each source file.
   This is done by parsing the source files and determining which files they `use` or `import`.
   This phase is implemented in the `leafc_build::dep_graph` module.

2. **Action Graph**: This phase is responsible for determining the build steps that need to be performed to build the program.
   This is done by determining which files need to be compiled, and which files need to be linked.
   This phase is implemented in the `leafc_build::action_graph` module.

   Here we begin on a fined-grained level and work our way up to a more and more coarse-grained level to get the benefits of incremental compilation all the way up the **data** inputs** in the compilation pipeline.

E.g. `inputs` are the set of all source files that need to be compiled.

   types of actions that can be performed and cached/memoized:

    - `Compile`: This action compiles a single source file into a single object file.
      <!-- This action is memoized by the `CompileCache` struct.
      The `CompileCache` struct is a cache of `Compile` actions.
      The `CompileCache` struct is stored in the `target` directory.
      The `CompileCache` struct is a map from `CompileCacheKey` to `CompileCacheValue`.
      The `CompileCacheKey` is a struct that contains the source file that needs to be compiled.
      The `CompileCacheValue` is a struct that contains the object file that was produced by the `Compile` action. -->
    - `Lex`: This action lexes a single source file into a single token stream.
      This action is memoized by the `LexCache` struct.
      The `LexCache` struct is a cache of `Lex` actions.
      The `LexCache` struct is stored in the `target` directory.
      The `LexCache` struct is a map from `LexCacheKey` to `LexCacheValue`.
      The `LexCacheKey` is a struct that contains the source file that needs to be lexed.
      The `LexCacheValue` is a struct that contains the token stream that was produced by the `Lex` action.
    - `Parse`: This action parses a single source file into a single AST.
    - `TypeCheck`: This action type checks a single source file into a single type-checked AST.
    <!-- - `Resolve`: This action resolves a single source file into a single resolved AST. -->
    - `Codegen`:
    <!-- - This action codegens a single source file into a single object file. -->
      <!-- This action is memoized by the `CodegenCache` struct.
      The `CodegenCache` struct is a cache of `Codegen` actions.
      The `CodegenCache` struct is stored in the `target` directory.
      The `CodegenCache` struct is a map from `CodegenCacheKey` to `CodegenCacheValue`.
      The `CodegenCacheKey` is a struct that contains the source file that needs to be codegen.
      The `CodegenCacheValue` is a struct that contains the object file that was produced by the `Codegen` action. -->

      <!-- MAYBE on this approach: interesting idea, but will probably just use database. Each of these actions has a corresponding `Cache` struct that is used to memoize the action. -->

      Each of these actions is stored within the `Database` for caching purposes.
      Then, when a new build is performed, the `Database` is used to determine which actions need to be performed.
      This includes determining which files need to be compiled, which strings need to be lexed, etc., which tokens need to be parsed, etc., which ASTs need to be type checked, etc., which type-checked ASTs need to be resolved, etc., which resolved ASTs need to be codegen, etc. Hence, the `Database` is the main data structure for the build process of leafc, and is in many ways the heart of the data flow of the build process. It is what allows for **incremental compilation**. This empowers the ability to never recompile
      a program twice. You ever only do the work that is necessary to get the program to the state that it needs to be in since the last build.

Types of cached data

This includes the `BuildPlan` struct, which is the main data structure for the build plan.

## Build Plan

The build plan is a data structure that describes the build process of leafc.

The build plan is a tree of `BuildPlan` structs, where each `BuildPlan` struct describes a single build step.
Each `BuildPlan` struct has a `BuildPlanKind` field that describes the kind of build step it describes.
The `BuildPlanKind` field can be one of the following:

- `BuildPlanKind::Binary`: This describes the process for a binary target (e.g. an executable program produced via `leafc`)
- `BuildPlanKind::Library`: This describes the build process of a library target (e.g. a library produced via `leafc`)
- `BuildPlanKind::Test`: This describes the build process of a test target (e.g. a test program produced via `leafc`)

## Build Plan Kind

The `BuildPlanKind` enum describes the kind of build step that a `BuildPlan` struct describes.
The `BuildPlanKind` enum can be one of the following:

- `BuildPlanKind::Leafc`: This is the root of the build plan tree.
  It describes the build process of the leafc binary itself.

- `BuildPlanKind::LeafcLib`: This describes the build process of the leafc library.

- `BuildPlanKind::LeafcLibTest`: This describes the build process of the leafc library tests.
