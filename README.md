# Document Generator (name to be refined)

The document generator, name TBD, aims to solve the problem of generating large numbers of documents according to a supplied template.
Often when creating and testing programs, the desire exists to create database of fake data, or generate a large number of sample queries, documents, commands, or generic text that conforms to a certain structure (JSON, XML, plaintext).

This project represents a command line interface (CLI) to do just that.

This CLI aims to be simple, yet extensible.
The work required on your part as a user involves the creation of a template for your document.

Templates can be of any structure: JSON, XML, plaintext, etc.
In your template, where you desire data to be generated, you should put a placeholder of the form: `${type}` or `${type:argument1,argument2,...}` in the case of optional arguments.
Supported placeholder types can be seen in the section below.

## Inspiration

This project was inspired by personal experience working with a series of microservices interfacing with an Elasticsearch database.
Particularly in a development environment in which such a database is containerised and populating it with data was either a relatively manual process, or any such automation was hardcoded into bash scripts that could be rerun.

Problems arose however as the document schema and search mappings changed to accomodate new data or changes to existing indexing, such that all existing data that was hardcoded into such scripts became outdated, and any change to this script became a long and tedious process.

## Usage

An inline template can be provided with the flag `-t` or `--template` and should be wrapped in single quotes to avoid the terminal populating `${...}` with environment variables.
```
e.g. ./docgen -t 'Name: ${first_name}, Address: ${address}, Contact number: ${phone}'
```

Additionally a file containing a template can be provided with the flag `-f` or `--template-file` and the path to the file.
```
e.g. ./docgen -f example_template.tpl
```

By default the tool will only generate a single document and print it to the screen.
This is helpful for the creation of your template.
When you are happy, you can specify the number of documents to generate with the `-n` flag.
```
e.g. ./docgen -f example_template.tpl -n 8000
```

## Supported placeholders
The placeholder types supported currently include:

Complex types:
- first_name
- last_name
- full_name
- address
- phone
- place

Primitive types:
- int (integer between 0 and 10)
- int:MIN,MAX (integer between MIN and MAX values)
- float (values default between 0 and 1)
- float:ROUNDING (number of decimal places to round float value)
- guid

## TODO
- [x] Phone number
    - [x] Mobile
    - [x] Landline
    - [x] Country codes
- [x] Random selection of supplied strings e.g. ${select:A,B,C}
- [x] Hierarchical placeholders split by `::` e.g. `${phone::mobile}`
- [] Option to relate first name and last name to the full name generated
    - [] Related placeholders by identifier e.g. `${<id>full_name} ${<id>first_name}` e.g. `${<person1>full_name} ${<person1>first_name}`
    - [] Each entity with an ID contains fields for each placeholder type that is lazy loaded as required and as each placeholder is populated.
    Placeholders are populated by reaching into the desired entity and getting the field for that type of exists, otherwise it is generated

## Related entity plan:
The current parsing system aims to parse an entire ${...} block into a placeholder type.
These placeholders contain information about itself as well as its formatting.
While this could be considered a different aspect of the output, the arguments are closely tied to the value of the placeholder synthesised.
For example an integer is given a min and max value.
This could be done Just In Time for generating the respective entity or document.

When an entity is parsed of the form `${<id>type::subtype:arg1,arg2}`
it should be mapped to an optional entity => Some()