searchState.loadedDescShard("patina", 0, "The Patina CLI is a simple command line application …\nMain entry point for the application. This launches the …\nRender and apply a patina\nThe available commands for the CLI\nOptions that apply globally to the CLI\nThe patina CLI renders files from templates and sets of …\nOptions that apply to patina subcommands\nRender a patina to stdout\nThe specified command to run\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGlobal options apply to all subcommands\nIncluded global options\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParse and return command line arguments\nThe file path to the patina toml file\nRun the CLI\nThe verbosity level of the CLI\nCommand line options\nCommand line options\nRenders a Patina from a Patina toml file path.\nA Patina describes a set of variables and templates that …\nA PatinaFile describes a template file and its target …\nA short description of the Patina\nA list of files referencing templates and their target …\nReturns the argument unchanged.\nReturns the argument unchanged.\nLoad a Patina from a TOML file\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nLoad the template file as a string\nThe name of the Patina\nThe path to the garget output file\nThe path to the template file\nA map of variables that can be used in the templates\nRenders all of the files in a Patina, each to a string in …\nContains the error value\nAn enum representing all possible errors that can occur in …\nAn error that occurs when a file cannot be read\nAn error that occurs when a file cannot be written\nContains the success value\nAn error that occurs when rendering a handlebars template\nA Result type that uses the <code>Error</code> enum\nAn error that occurs when parsing Toml data\nOptionally returns references to the inner fields if this …\nOptionally returns mutable references to the inner fields …\nOptionally returns references to the inner fields if this …\nOptionally returns mutable references to the inner fields …\nOptionally returns references to the inner fields if this …\nOptionally returns mutable references to the inner fields …\nOptionally returns references to the inner fields if this …\nOptionally returns mutable references to the inner fields …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the inner fields if this is a <code>Error::FileRead</code>, …\nReturns the inner fields if this is a <code>Error::FileWrite</code>, …\nReturns the inner fields if this is a <code>Error::RenderTemplate</code>…\nReturns the inner fields if this is a <code>Error::TomlParse</code>, …\nReturns true if this is a <code>Error::FileRead</code>, otherwise false\nReturns true if this is a <code>Error::FileWrite</code>, otherwise false\nReturns true if this is a <code>Error::RenderTemplate</code>, otherwise …\nReturns true if this is a <code>Error::TomlParse</code>, otherwise false")