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
Referencing types and their subtypes is done with the syntax: 'type::subtype:arg1,arg2' e.g. 'name::first'

### Complex types:
- name
    - first
    - last
    - full
- location
    - address
    - place
    - street
- phone
    - mobile
    - landline
- dist (random distributions)
    - normal:MEAN,STANDARD_DEVIATION (normally distributed random variable with mean and standard deviation)

Primitive types:
- int:MIN,MAX (integer between MIN and MAX values)
- float:MIN,MAX (float value between MIN and MAX values)
- set:A,B,C,D (randomly selected element of the provided set e.g. B)
- guid