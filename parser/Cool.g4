grammar Cool;

program: (class ';')*;

class: 'class' Id ('inherits' Id)? '{' (feature)* '}' ;

feature: Id '(' (formal (',' formal)* )? ')' 
            ':' Id '{' (expr)* '}' ';'
       | Id ':' Id ('<-' expr)? ';'
       ;

formal: Id ':' Id ;

expr: Id '<-' expr
    | expr ('@' Id )* '.' Id '(' (expr (',' expr )*)? ')'
    | Id '(' (expr (',' expr)*)? ')'
    | 'if' expr 'then' expr 'else' expr 'fi'
    | 'while' expr 'loop' expr 'pool'
    | '{' (expr ';')+ '}'
    | 'let' Id ':' Id ('<-' expr)? 
            ( ',' Id ':' Id ('<-' expr)? )* 'in' expr
    | 'case' expr 'of' (Id ':' Id '=>' expr ';')+ 'esac'
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
    ;

TEXT: ;
MULTILINE_COMMENT: '(*' TEXT '*)' -> skip;
COMMENT: '--' TEXT -> skip;
NEWLINE: [\n\r]+ -> skip;
SPACE: [ \t]+ -> skip;
Id: [A-Za-z_] [A-Za-z0-9_]*;
Integer: [0-9]+;
String: '"' [A-Za-z0-9 \\]* '"';


