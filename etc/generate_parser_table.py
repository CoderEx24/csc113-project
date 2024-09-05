grammar = {
    'program': ["class ';' program", "class ';'"],

    'class': ["'class' 'TYPE' '{' feature_list '}'",
              "'class' 'TYPE' 'inherits' 'TYPE' '{' feature_list '}'"],

    'feature_list': ["feature ';' feature_list",
                     "feature ';'"],

    'feature': ["'ID' '(' formal_list ')' ':' 'TYPE' '{' expr '}'",
                "'ID' ':' 'TYPE' '<-' expr",
                "'ID' ':' 'TYPE'"],

    'formal_list': ["formal ',' formal_list",
                    "formal"],

    'formal': ["'ID' ':' 'TYPE'"],

    'expr_list': ["expr ',' expr_list", "expr"],

    'block_list': ["expr ';' block_list", "expr ';'"],
    'optional_assign': ["'<-' expr", ""],
    'assign_list': ["'ID' ':' 'TYPE' optional_assign ',' assign_list",
                    "'ID' ':' 'TYPE' optional_assign"],
    'case_list': ["'ID' ':' 'TYPE' '=>' expr ';' case_list",
                  "'ID' ':' 'TYPE' '=>' expr ';'"],

    'optional_cast': ["'@' 'TYPE'", ""],
    'expr': ["'ID' '<-' expr",
             "expr optional_cast '.' 'ID' '(' expr_list ')'",
             "'ID' '(' expr_list ')'",
             "'if' expr 'then' expr 'else' expr 'fi'",
             "'while' expr 'loop' expr 'pool'",
             "'{' block_list '}'",
             "'let' assign_list 'in' expr",
             "'case' expr 'of' case_list 'esac'",
             "'new' 'TYPE'",
             "'isvoid' expr",
             "expr '+' expr",
             "expr '-' expr",
             "expr '*' expr",
             "expr '/' expr",
             "'~' expr",
             "expr '<' expr",
             "expr '<-' expr",
             "expr '=' expr",
             "'not' expr",
             "'(' expr ')'",
             "'ID'",
             "'integer'",
             "'string'",
             "'true'",
             "'false'"],
}

nonterminals = grammar.keys()

terminals = []
for prods in grammar.values():
    for p in prods:
        for piece in p.split(' '):
            if piece.startswith("'"):
                terminals.append(piece[1:-1])

terminals = set(terminals)

def first(symbol):
    global grammar, nonterminals, terminals

    print(f"first({symbol})")
    if symbol in terminals or symbol == '':
        yield symbol

    else:
        for production in grammar[symbol]:
            for prod_symbol in production.split(' '):
                prod_symbol = prod_symbol \
                    if prod_symbol in nonterminals else prod_symbol[1:-1]

                if prod_symbol == symbol:
                    break

                symbol_first_list = list(first(prod_symbol))

                for i in symbol_first_list:
                    yield i

                print(symbol_first_list)
                if '' not in symbol_first_list:
                    break

def follow(symbol):
    global grammar, nonterminals, terminals

    if symbol == '':
        raise "Epsilon has no follow"

    if symbol == grammar.keys()[0]:
        yield '$'

    for prods in grammar.values():
        for prod in prods:
            print(prod)

print(set(first('expr')))
