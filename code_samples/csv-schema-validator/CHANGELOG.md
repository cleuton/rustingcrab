1) Version 0.1.0 - First version. Basic validations: 

- Range(min=, max=) # for floating 64
- Required # Any
- Custom = "\<function>\" # Any
- Regex = "\<regex>\" # String

2) Version 0.1.1

- Fixed problem with Option<String> for Custom and Regex validations. Now you can use Option or not. 
- Added Length(min=, max=) for Strings. 

2) Version 0.1.2

- Added 3 validations: `not_blank`, `one_of()`, and `not_in()`.
