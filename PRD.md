The purpose of this project is provide an agent, a command line access to store, retrieve, search, sort, backlog items. You shouldn't have a title, a description, a date created, a date modified. This should be a local database, such as SQLite, something easy. 

Output should be JSON. The command line interface should support a help, which will be displayed by default if no argument is provided.
The help should be geared for an agent. This is an agent tool. Everything inputs and outputs should all be optimized for agent use.

It should allow piping the data as an input. It should also allow optional output to a file, such as a `.jsonl`. 
Ideally, it would support vector searching, perhaps using the QMD library.
If there's a built-in SQLite vector indexer, use that. 
