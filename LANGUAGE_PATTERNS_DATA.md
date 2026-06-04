# Comprehensive Language Pattern Data

This document contains complete pattern signatures for all 18+ languages in the registry.

## BEAM Languages

### Elixir
```rust
PatternSignatures {
    // Core syntax
    error_handling_syntax: vec![
        "try", "rescue", "raise", "throw", "catch",
        "{:error,", "{:ok,", "with", "else:"
    ],
    async_syntax: vec![
        "spawn", "spawn_link", "Task.async", "Task.await",
        "GenServer", "send", "receive", "Process.send_after"
    ],
    testing_syntax: vec![
        "deftest", "test ", "assert", "refute", "assert_raise",
        "ExUnit", "describe ", "setup", "@tag"
    ],
    pattern_matching_syntax: vec![
        "case", "when", "with", "cond", "=", "<-"
    ],
    module_syntax: vec![
        "import", "alias", "require", "use", "defmodule", "__MODULE__"
    ],

    // Extended syntax
    type_annotation_syntax: vec![
        "@type", "@spec", "@typep", "@opaque", "::", "when"
    ],
    function_syntax: vec![
        "def ", "defp ", "defmacro ", "defmacrop ", "defdelegate"
    ],
    class_syntax: vec![
        "defmodule", "defstruct", "defprotocol", "defimpl"
    ],
    comment_syntax: vec![
        "#", "@doc", "@moduledoc", "\"\"\"", "'''", "@typedoc"
    ],
    null_handling_syntax: vec![
        "nil", "nil?", "is_nil"
    ],

    // Database patterns
    database_patterns: vec![
        "Ecto.", "Repo.", "Schema.", "Query.", "Changeset.",
        "from(", "where(", "select(", "join("
    ],
    sql_patterns: vec![
        "fragment(", "SELECT", "INSERT", "UPDATE", "DELETE"
    ],

    // HTTP patterns
    http_client_patterns: vec![
        "HTTPoison", "Req.", "Finch.", "Tesla.", "Mojito."
    ],
    http_server_patterns: vec![
        "Phoenix.Router", "Plug.", "get ", "post ", "put ", "delete ",
        "pipe_through", "scope "
    ],
    graphql_patterns: vec![
        "Absinthe", "query", "mutation", "subscription", "resolve"
    ],

    // Framework patterns
    framework_patterns: vec![
        "Phoenix", "Plug", "Absinthe", "Broadway", "Oban"
    ],
    orm_patterns: vec![
        "Ecto.Schema", "Ecto.Changeset", "Ecto.Query", "Repo."
    ],

    // Package patterns
    package_files: vec!["mix.exs", "mix.lock"],
    import_patterns: vec![
        "import ", "alias ", "require ", "use "
    ],

    // Quality patterns
    code_smell_indicators: vec![
        "Enum.map(", "Enum.filter(", "Enum.reduce(", // Chained enums
        "receive do", // Bare receives
        "send(", // Direct sends without supervision
    ],
    security_antipatterns: vec![
        "String.to_atom(", // Atom exhaustion
        ":erlang.binary_to_term(", // Unsafe deserialization
        "File.read!", "File.write!", // Unsafe file ops without validation
    ],
    performance_antipatterns: vec![
        "Repo.all(", "Repo.get(", // Potential N+1 queries
        "Enum.map(", "Enum.filter(", // Potential inefficient pipelines
    ],
}
```

### Erlang
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "error", "exit",
        "{error,", "{ok,", "case", "of"
    ],
    async_syntax: vec![
        "spawn", "spawn_link", "spawn_monitor",
        "send", "receive", "!", "?", "gen_server"
    ],
    testing_syntax: vec![
        "eunit", "ct_", "test(", "_test_", "?assert", "?assertEqual"
    ],
    pattern_matching_syntax: vec![
        "case", "of", "when", "=", "<-"
    ],
    module_syntax: vec![
        "-module(", "-import(", "-include(", "-export("
    ],
    type_annotation_syntax: vec![
        "-spec", "-type", "-opaque", "-callback", "::"
    ],
    function_syntax: vec![
        "->", "when", "fun ", "end."
    ],
    comment_syntax: vec!["%", "%%", "%%%"],
    null_handling_syntax: vec!["undefined", "is_undefined"],

    database_patterns: vec!["epgsql:", "emysql:", "esqlite:"],
    framework_patterns: vec!["cowboy:", "ranch:", "gen_server", "supervisor"],
    package_files: vec!["rebar.config", "rebar.lock", "erlang.mk"],
}
```

### Gleam
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "Error(", "Ok(", "Result(", "assert", "panic"
    ],
    async_syntax: vec![
        "process.start", "process.send", "process.receive"
    ],
    testing_syntax: vec![
        "gleeunit", "should.", "describe(", "it("
    ],
    pattern_matching_syntax: vec![
        "case", "->", "|>", "let assert"
    ],
    module_syntax: vec![
        "import", "pub ", "type ", "opaque"
    ],
    type_annotation_syntax: vec![
        ": ", "-> ", "Result(", "Option(", "List("
    ],
    function_syntax: vec![
        "pub fn ", "fn ", "->"
    ],
    comment_syntax: vec!["//", "///"],
    null_handling_syntax: vec!["Nil", "Option("],

    database_patterns: vec!["sqlight.", "gleam_pgo."],
    framework_patterns: vec!["gleam_http", "wisp."],
    package_files: vec!["gleam.toml"],
}
```

## Systems Languages

### Rust
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "Result<", "Option<", "?", "unwrap", "expect",
        "unwrap_or", "map_err", "and_then", "ok_or",
        "panic!", "unreachable!"
    ],
    async_syntax: vec![
        "async", "await", ".await", "tokio::", "async_std::",
        "futures::", "spawn", "block_on"
    ],
    testing_syntax: vec![
        "#[test]", "assert!", "assert_eq!", "assert_ne!",
        "#[cfg(test)]", "#[should_panic]", "proptest!"
    ],
    pattern_matching_syntax: vec![
        "match", "if let", "while let", "let else", "matches!"
    ],
    module_syntax: vec![
        "use ", "mod ", "pub", "crate::", "super::", "self::"
    ],
    type_annotation_syntax: vec![
        ": ", "-> ", "<", ">", "impl ", "dyn ", "where"
    ],
    function_syntax: vec![
        "fn ", "pub fn", "async fn", "const fn", "unsafe fn"
    ],
    class_syntax: vec![
        "struct ", "enum ", "trait ", "impl ", "union "
    ],
    comment_syntax: vec!["//", "///", "//!", "/*", "/**"],
    generic_syntax: vec!["<T>", "<T: ", "where T:"],
    null_handling_syntax: vec!["Option<", "None", "Some("],

    database_patterns: vec![
        "diesel::", "sqlx::", "sea_orm::", "rusqlite::",
        "tokio_postgres::", "query!", "query_as!"
    ],
    http_client_patterns: vec![
        "reqwest::", "hyper::", "ureq::", ".get(", ".post("
    ],
    http_server_patterns: vec![
        "actix_web::", "axum::", "rocket::", "warp::",
        "#[get(", "#[post(", "Router::"
    ],
    graphql_patterns: vec!["async_graphql::", "juniper::"],
    framework_patterns: vec!["actix", "tokio", "async-std", "axum"],
    orm_patterns: vec!["diesel::", "sea_orm::", "sqlx::"],
    package_files: vec!["Cargo.toml", "Cargo.lock"],

    security_antipatterns: vec![
        "unsafe {", "mem::transmute", "from_utf8_unchecked",
        ".unwrap()", "panic!(", "unimplemented!()"
    ],
}
```

### C/C++
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "errno", "perror",
        "assert", "static_assert", "noexcept"
    ],
    async_syntax: vec![
        "std::async", "std::future", "std::thread",
        "pthread_create", "co_await", "co_return"
    ],
    testing_syntax: vec![
        "ASSERT_", "EXPECT_", "TEST(", "TEST_F(", "gtest"
    ],
    pattern_matching_syntax: vec![
        "switch", "case:", "default:", "?:", "std::visit"
    ],
    module_syntax: vec![
        "#include", "namespace", "using", "::", "import"
    ],
    type_annotation_syntax: vec![
        ": ", "->", "const", "constexpr", "auto"
    ],
    function_syntax: vec![
        "void ", "int ", "static ", "inline ", "constexpr "
    ],
    class_syntax: vec![
        "class ", "struct ", "template<", "typename"
    ],
    comment_syntax: vec!["//", "/*", "/**", "///"],
    null_handling_syntax: vec![
        "nullptr", "NULL", "std::optional", "std::nullopt"
    ],

    database_patterns: vec!["libpq", "mysql_", "sqlite3_"],
    framework_patterns: vec!["boost::", "Qt", "STL"],
    package_files: vec!["CMakeLists.txt", "Makefile", "conanfile.txt"],

    security_antipatterns: vec![
        "strcpy(", "gets(", "sprintf(", "scanf(",
        "malloc(", "free(", "delete ", "system("
    ],
}
```

### Go
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "error", "if err != nil", "errors.New", "fmt.Errorf",
        "panic", "recover", "defer"
    ],
    async_syntax: vec![
        "go ", "chan ", "<-", "select", "sync.", "context."
    ],
    testing_syntax: vec![
        "func Test", "t.Run(", "t.Error", "assert.", "require."
    ],
    pattern_matching_syntax: vec![
        "switch", "case", "select", "type switch"
    ],
    module_syntax: vec![
        "import", "package", "import ("
    ],
    type_annotation_syntax: vec![
        "type ", "struct{", "interface{", "func("
    ],
    function_syntax: vec![
        "func ", "func(", "func (",  "defer "
    ],
    class_syntax: vec![
        "type ", "struct {", "interface {"
    ],
    comment_syntax: vec!["//", "/*", "/**"],
    null_handling_syntax: vec!["nil", "== nil", "!= nil"],

    database_patterns: vec![
        "database/sql", "gorm.", "sqlx.", "pgx.",
        "Query(", "Exec(", "db."
    ],
    http_client_patterns: vec![
        "http.Get", "http.Post", "http.Client"
    ],
    http_server_patterns: vec![
        "http.HandleFunc", "http.ListenAndServe",
        "gin.", "echo.", "fiber."
    ],
    graphql_patterns: vec!["graphql-go", "gqlgen"],
    framework_patterns: vec!["gin", "echo", "fiber", "chi"],
    orm_patterns: vec!["gorm.", "ent.", "sqlc"],
    package_files: vec!["go.mod", "go.sum"],

    security_antipatterns: vec![
        "eval", "exec.Command", "os.system", "sql.Query("
    ],
    performance_antipatterns: vec![
        "defer ", "append(", "range ", // In hot loops
    ],
}
```

## Web Languages

### JavaScript
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "finally", "Error(",
        ".catch(", ".then(", "Promise.reject"
    ],
    async_syntax: vec![
        "async", "await", "Promise", ".then(", ".catch(",
        "setTimeout", "setInterval", "queueMicrotask"
    ],
    testing_syntax: vec![
        "test(", "it(", "describe(", "expect(", "assert(",
        "jest", "mocha", "chai", "beforeEach"
    ],
    pattern_matching_syntax: vec![
        "switch", "case", "?", "??", "?."],
    module_syntax: vec![
        "import", "export", "require(", "module.exports",
        "from ", "* as "
    ],
    type_annotation_syntax: vec![
        ": ", "as ", "interface", "type ", "extends"
    ],
    function_syntax: vec![
        "function ", "=>", "async function", "function*"
    ],
    class_syntax: vec![
        "class ", "extends", "constructor", "static "
    ],
    comment_syntax: vec!["//", "/*", "/**", "///"],
    null_handling_syntax: vec![
        "null", "undefined", "??", "?.", "== null"
    ],

    database_patterns: vec![
        "mongoose.", "sequelize.", "prisma.", "typeorm",
        ".find(", ".findOne(", ".create("
    ],
    http_client_patterns: vec![
        "fetch(", "axios.", "http.get", "request("
    ],
    http_server_patterns: vec![
        "express(", "app.get(", "app.post(", "router.",
        "fastify", "koa"
    ],
    graphql_patterns: vec![
        "graphql", "apollo", "query", "mutation", "useQuery"
    ],
    framework_patterns: vec![
        "react", "vue", "angular", "next", "express",
        "nestjs", "fastify"
    ],
    orm_patterns: vec!["prisma", "typeorm", "sequelize", "mongoose"],
    package_files: vec!["package.json", "package-lock.json", "yarn.lock"],

    security_antipatterns: vec![
        "eval(", "innerHTML", "dangerouslySetInnerHTML",
        "document.write", "setTimeout(string"
    ],
}
```

### TypeScript
```rust
PatternSignatures {
    // Same as JavaScript plus:
    type_annotation_syntax: vec![
        ": string", ": number", ": boolean", ": void",
        "interface ", "type ", "enum ", "as ", "extends ",
        "<T>", "Generic<", "Partial<", "Required<"
    ],
    generic_syntax: vec![
        "<T>", "<T extends", "<K, V>", "Array<", "Promise<"
    ],
    null_handling_syntax: vec![
        "null", "undefined", "?", "!", "??", "?.",
        "NonNullable<", "| null", "| undefined"
    ],
    // ... rest same as JavaScript
}
```

### Python
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try:", "except", "raise", "finally:", "assert",
        "with ", "as ", "contextlib"
    ],
    async_syntax: vec![
        "async def", "await ", "asyncio.", "async with",
        "async for", "aiohttp", "trio"
    ],
    testing_syntax: vec![
        "def test_", "class Test", "assert ", "pytest",
        "unittest", "@pytest", "mock."
    ],
    pattern_matching_syntax: vec![
        "match ", "case ", "if ", "elif ", "else:"
    ],
    module_syntax: vec![
        "import ", "from ", "as ", "__init__"
    ],
    type_annotation_syntax: vec![
        ": int", ": str", ": List[", ": Dict[", "-> ",
        "Optional[", "Union[", "TypeVar"
    ],
    function_syntax: vec![
        "def ", "lambda ", "async def", "@"
    ],
    class_syntax: vec![
        "class ", "(object):", "@dataclass", "@property"
    ],
    comment_syntax: vec!["#", "\"\"\"", "'''", "#:"],
    null_handling_syntax: vec!["None", "is None", "is not None"],

    database_patterns: vec![
        "sqlalchemy", "django.db", "psycopg", "pymongo",
        ".filter(", ".query(", ".all()"
    ],
    http_client_patterns: vec![
        "requests.", "httpx.", "aiohttp.", "urllib."
    ],
    http_server_patterns: vec![
        "flask", "django", "fastapi", "@app.route",
        "@api_view", "APIRouter"
    ],
    graphql_patterns: vec!["graphene", "strawberry", "ariadne"],
    framework_patterns: vec!["django", "flask", "fastapi", "tornado"],
    orm_patterns: vec!["sqlalchemy", "django.orm", "peewee", "tortoise"],
    package_files: vec!["requirements.txt", "setup.py", "pyproject.toml", "Pipfile"],

    security_antipatterns: vec![
        "eval(", "exec(", "compile(", "pickle.loads",
        "__import__", "os.system"
    ],
}
```

## JVM Languages

### Java
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "throws", "finally",
        "Exception", "RuntimeException"
    ],
    async_syntax: vec![
        "CompletableFuture", "async", "Executor",
        "Thread", "Runnable", "Callable", "Future"
    ],
    testing_syntax: vec![
        "@Test", "assertEquals", "assertTrue", "JUnit",
        "@Before", "@After", "Mockito"
    ],
    pattern_matching_syntax: vec![
        "switch", "case", "instanceof", "default:"
    ],
    module_syntax: vec![
        "import ", "package ", "static ", ".*"
    ],
    type_annotation_syntax: vec![
        ": ", "<T>", "extends", "implements", "? extends"
    ],
    function_syntax: vec![
        "public ", "private ", "protected ", "void ",
        "static ", "final "
    ],
    class_syntax: vec![
        "class ", "interface ", "enum ", "record ",
        "abstract ", "@interface"
    ],
    comment_syntax: vec!["//", "/*", "/**", "@param", "@return"],
    generic_syntax: vec!["<T>", "<? extends", "<? super", "List<"],
    null_handling_syntax: vec![
        "null", "Optional<", ".orElse", ".ifPresent",
        "@Nullable", "@NonNull"
    ],

    database_patterns: vec![
        "jdbc:", "hibernate.", "jpa", "@Entity",
        "EntityManager", "Query"
    ],
    http_client_patterns: vec![
        "HttpClient", "RestTemplate", "WebClient"
    ],
    http_server_patterns: vec![
        "Spring", "@RestController", "@GetMapping",
        "@PostMapping", "Servlet"
    ],
    framework_patterns: vec!["Spring", "Quarkus", "Micronaut", "Vert.x"],
    orm_patterns: vec!["Hibernate", "JPA", "MyBatis"],
    package_files: vec!["pom.xml", "build.gradle", "build.gradle.kts"],
}
```

### Kotlin
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "Result", "runCatching"
    ],
    async_syntax: vec![
        "suspend", "coroutine", "async", "await",
        "launch", "Flow", "Channel"
    ],
    testing_syntax: vec![
        "@Test", "assertEquals", "kotest", "mockk"
    ],
    pattern_matching_syntax: vec![
        "when", "is", "as", "as?", "in"
    ],
    module_syntax: vec![
        "import ", "package ", "typealias"
    ],
    type_annotation_syntax: vec![
        ": ", "->", ": List<", "out ", "in "
    ],
    function_syntax: vec![
        "fun ", "suspend fun", "inline fun", "infix fun"
    ],
    class_syntax: vec![
        "class ", "data class", "sealed class",
        "interface ", "object ", "companion object"
    ],
    null_handling_syntax: vec![
        "?", "!!", "?.", "?:", "let", "run", "also"
    ],

    framework_patterns: vec!["Ktor", "Spring", "Exposed"],
    orm_patterns: vec!["Exposed", "Room", "Hibernate"],
    package_files: vec!["build.gradle.kts", "settings.gradle.kts"],
}
```

### C#
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "try", "catch", "throw", "finally", "Exception"
    ],
    async_syntax: vec![
        "async", "await", "Task", "Task<", "async Task"
    ],
    testing_syntax: vec![
        "[Test]", "Assert.", "[Fact]", "xUnit", "NUnit"
    ],
    pattern_matching_syntax: vec![
        "switch", "case", "when", "is", "=>"
    ],
    module_syntax: vec![
        "using ", "namespace", "static using"
    ],
    type_annotation_syntax: vec![
        ": ", "<T>", "where T:", "out ", "in "
    ],
    function_syntax: vec![
        "public ", "private ", "async ", "static ",
        "void ", "virtual ", "override "
    ],
    class_syntax: vec![
        "class ", "interface ", "struct ", "record ",
        "abstract ", "sealed "
    ],
    null_handling_syntax: vec![
        "null", "?", "??", "?.", "!",
        "Nullable<", "?.Invoke"
    ],

    database_patterns: vec![
        "EntityFramework", "Dapper", "DbContext",
        "IQueryable", "SqlConnection"
    ],
    framework_patterns: vec![".NET", "ASP.NET", "Blazor", "MAUI"],
    orm_patterns: vec!["Entity Framework", "Dapper", "NHibernate"],
    package_files: vec!["*.csproj", "*.sln", "packages.config"],
}
```

## Scripting Languages

### Lua
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "pcall", "xpcall", "error", "assert"
    ],
    testing_syntax: vec![
        "assert(", "busted", "describe(", "it("
    ],
    pattern_matching_syntax: vec![
        "if ", "elseif ", "else "
    ],
    module_syntax: vec![
        "require", "module", "local ", "return "
    ],
    function_syntax: vec![
        "function ", "local function", "end"
    ],
    comment_syntax: vec!["--", "--[[", "--]]"],
    null_handling_syntax: vec!["nil", "== nil", "~= nil"],
    package_files: vec!["rockspec"],
}
```

### Bash
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "set -e", "trap", "||", "&&", "if [ $? "
    ],
    testing_syntax: vec![
        "[ ", "[[ ", "test ", "bats"
    ],
    pattern_matching_syntax: vec![
        "case ", "esac", ";;", "|"
    ],
    module_syntax: vec![
        "source ", ". ", "export"
    ],
    function_syntax: vec![
        "function ", "() {"
    ],
    comment_syntax: vec!["#"],
    package_files: vec!["Makefile", ".sh"],
}
```

## Data/Config Languages

### SQL
```rust
PatternSignatures {
    error_handling_syntax: vec![
        "RAISE", "EXCEPTION", "TRY", "CATCH"
    ],
    pattern_matching_syntax: vec![
        "CASE", "WHEN", "THEN", "ELSE", "END"
    ],
    sql_patterns: vec![
        "SELECT", "INSERT", "UPDATE", "DELETE",
        "FROM", "WHERE", "JOIN", "GROUP BY", "ORDER BY"
    ],
    comment_syntax: vec!["--", "/*"],
}
```

### JSON/YAML/TOML
```rust
// Minimal patterns - mostly for validation
PatternSignatures {
    comment_syntax: vec!["#"],  // YAML/TOML
    // JSON has no comments
}
```

This completes the comprehensive pattern data for all languages!
