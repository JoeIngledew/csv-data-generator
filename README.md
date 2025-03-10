# CSV Data Generator

A simple tool to generate CSV data for testing.

## Usage

If running from within the project:

```sh
cargo run -- --input <input path> --count <row count> > <output path>
```

## Input format

The input must follow the expected structure:

```json
{
  "fields": [
    {
      "field_name": "my_field_name",
      "field_type": "<TYPE>"
    },
    ...
  ]
}
```

The currently available field types are as follows:

 - Name
 - String
 - Number
 - Date
 - Boolean
 - Uuid
 - PatternString (*)
 - RangeInteger (*)
 - CompanyBrand
 - Selection (*)

(*) The options marked with a star require some additional structure, eg.

```json
{
  "fields": [
    {
      "field_name": "my_pattern",
      "field_type": {
        "PatternString": "\\d{2}-[A-Z]{6}"
      }
    },
    {
      "field_name": "my_range",
      "field_type": {
        "RangeInteger": {
          "min": 0,
          "max": 500
        }
      }
    },
    {
      "field_name": "my_selection",
      "field_type": {
        "Selection": [
          "OPTION_1",
          "OPTION_2"
        ]
      }
    }
  ]
}
```

PatternString will produce a random string that satisfies the regular expression provided. Please not anchors will cause the program to crash.

RangeInteger will produce a random integer within the provided `min` and `max` (inclusive of both upper and lower bounds)

Selection will choose a random option from the provided array of strings.