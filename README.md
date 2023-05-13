A small parser for the arguments of a CLI. 

I don't really like clap so I tryed to make my own while using as few dependencies as possible (only the optional `colored` dependency).

Copyright (c) 2023 Lovis Rentsch. All Rights Reserved.

For fun by me in rust under the EUPL.

```
command <argument>
Arguments: Help, Demo, Test, path (-p=<path>)
When using this lib remember to adjust
  1. The Enum
  2. The match that checkes the provided Strings
  3. The part in main.rs (fill the match with your functions)
  4. The rules for the arguments
  5. Maybe the evaluation fn
```
