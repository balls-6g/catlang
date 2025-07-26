mod string_syntaxerror;

use string_syntaxerror::StringSyntaxError;

enum ParseCodeError {
    StringSyntaxError(StringSyntaxError),
}
