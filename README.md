
# Date Time Parser for Multiple Languages

Welcome to the `date_time_parser_multi_language` package. ☺️  
This package can be used to parse date information out of strings.  
The title is misleading at the moment, because it doesn't support time parsing. For now!

Date information can be parsed out of natural language strings like `Do something tomorrow`, or `Remind me in three weeks`.  
Stuff like that, what you may need when you create a todo app or so.

## How to Use the Package

Before you can start, you have to determine some things.

1. **Which language do you want to parse.**  
   Check [Supported Languages](#supported-languages) to check if the language you need is implemented.  
   If the language is missing, feel free to extend the package.
   
2. **Which date format you want to use.**  
   In the US, it is common to use MM.DD.YYYY, in Europe it is DD.MM.YYYY.  
   This setting influences the parsing heavily.

Below, you will find a basic example.

```rust
include::examples/en.rs[]
```

In the [examples directory](./examples) you can find more examples.  
Alternatively, you can check out the `test-client`, where you can test the parsing too.

## Supported Languages

| Language | Code | State        |
|----------|------|--------------|
| English  | EN   | Experimental |