grammar Cool;

program: (class ';')*;

class: 'class' Id ('inherits' Id)? '{' feature* '}' ;

feature: Id '(' (formal (',' formal)* )? ')' ':' Id '{' expr* '}'
       | Id ':' Id ('<-' expr)?
       ;

formal: Id ':' Id ;

expr: Id '<-' Id
    | expr ('@' Id )* '.' Id '(' (expr (',' expr )*)? ')'
    | Id '(' (expr (',' expr)*)? ')'
    | 'if' expr 'then' expr 'else' expr 'fi'
    | 'while' expr 'loop' expr 'pool'
    | 'new' Id
    | 'isvoid' Id
    | expr '+' expr
    | expr '-' expr
    | expr '*' expr
    | expr '/' expr
    | expr '<' expr
    | expr '=' expr
    | expr '<=' expr
    | 'not' expr
    | '(' expr ')'
    | Id
    | Integer
    | String
    | 'true'
    | 'false'
    |
    ;

COMMENT: '(*' String* '*)' -> skip;
NEWLINE: [\n\r] -> skip;
Id: [A-Za-z_] [A-Za-z0-9_]*;
Integer: [0-9]+;
String: '"' [A-Za-z0-9]* '"';


