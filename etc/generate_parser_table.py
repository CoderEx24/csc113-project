from itertools import chain
from functools import reduce

grammar = {
    'program': ["class_prod ';' program", "class_prod ';'"],

    'class_prod': ["'class' 'TYPE' '{' feature_list '}'",
              "'class' 'TYPE' 'inherits' 'TYPE' '{' feature_list '}'",
              "'class' 'TYPE' 'inherits' 'TYPE' '{' '}'"],

    'feature_list': ["feature ';' feature_list",
                     "feature ';'"],

    'feature': ["'ID' '(' formal_list ')' ':' 'TYPE' '{' expr '}'",
                "'ID' '(' ')' ':' 'TYPE' '{' expr '}'",
                "'ID' ':' 'TYPE' '<-' expr",
                "'ID' ':' 'TYPE'"],

    'formal_list': ["formal ',' formal_list",
                    "formal"],

    'formal': ["'ID' ':' 'TYPE'"],

    'expr_list': ["expr ',' expr_list", "expr"],

    'block_list': ["expr ';' block_list", "expr ';'"],

    'assign_list': ["'ID' ':' 'TYPE' '<-' expr ',' assign_list",
                    "'ID' ':' 'TYPE' ',' assign_list",
                    "'ID' ':' 'TYPE' '<-' expr",
                    "'ID' ':' 'TYPE'"],
    'case_list': ["'ID' ':' 'TYPE' '=>' expr ';' case_list",
                  "'ID' ':' 'TYPE' '=>' expr ';'"],

    'expr': ["'ID' '<-' expr",
             "expr '.' 'ID' '(' expr_list ')'",
             "expr '.' 'ID' '(' ')'",
             "expr '@' 'TYPE' '.' 'ID' '(' expr_list ')'",
             "expr '@' 'TYPE' '.' 'ID' '(' ')'",
             "'ID' '(' expr_list ')'",
             "'ID' '(' ')'",
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
             "expr '<=' expr",
             "expr '=' expr",
             "'not' expr",
             "'(' expr ')'",
             "'ID'",
             "'integer'",
             "'string'",
             "'true'",
             "'false'"],
}

nonterminals = list(grammar.keys())

terminals = []
for prods in grammar.values():
    for p in prods:
        for piece in p.split(' '):
            if piece.startswith("'"):
                terminals.append(piece)

terminals = set(terminals)

productions = []
for k in grammar:
    for prod in grammar[k]:
        productions.append((k, prod))

grammar = (grammar, nonterminals, terminals, productions)

def first(symbol, grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    if symbol in terminals or symbol == '':
        yield symbol

    else:
        for production in grammar[symbol]:
            for prod_symbol in production.split(' '):
                if prod_symbol == symbol:
                    break

                symbol_first_list = set(first(prod_symbol, grammar_))

                for i in symbol_first_list:
                    yield i

                if '' not in symbol_first_list:
                    break

def follow(symbol, grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    if symbol == '':
        raise "Epsilon has no follow"

    if symbol == nonterminals[0]:
        yield '$'

    for nonterminal in grammar:
        for production in grammar[nonterminal]:
            grammar_symbols = production.split(' ')

            if symbol not in grammar_symbols:
                continue

            for (idx, s) in enumerate(grammar_symbols):
                if s != symbol:
                    continue

                if idx < len(grammar_symbols) - 1:
                    for gs in grammar_symbols[idx + 1:]:
                        gs_first_list = set(first(gs, grammar_))
                        for i in gs_first_list:
                            if i != '':
                                yield i

                        if '' not in gs_first_list:
                            break
                else:
                    if nonterminal == symbol:
                        continue

                    for i in follow(nonterminal, grammar_):
                        if i != '':
                            yield i

def lr0_itemset_closure(itemset, grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    new_itemset = [*itemset]
    for (_, production, dot) in new_itemset:
        grammar_symbols = production.split(' ')
        if dot < len(grammar_symbols) and \
                grammar_symbols[dot] in nonterminals:

            symbol = grammar_symbols[dot]
            new_entries = [(symbol, prod, 0) for prod in grammar[symbol]]
            new_entries = set(new_entries) - set(new_itemset)
            new_itemset.extend(new_entries)

    return new_itemset

def lr0_itemset_goto(itemset, grammar_symbol, grammar_):
    new_itemset = []
    for (head, production, dot) in lr0_itemset_closure(itemset, grammar_):
        pieces = production.split(' ')
        if dot >= len(pieces):
            continue

        if pieces[dot] == grammar_symbol:
            new_itemset.append((head, production, dot + 1))

    return new_itemset

def print_itemset(itemset):
    for (head, production, dot) in itemset:
        item_str = f'{head} -> '
        pieces = production.split(' ')

        for p in pieces[:dot]:
            item_str += f'{p} '

        item_str += '* '

        for p in pieces[dot:]:
            item_str += f'{p} '

        yield item_str.strip()

def generate_lr0_automaton(grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    lr0_itemsets = [
        [(f'{nonterminals[0]}_', nonterminals[0], 0)]
    ]

    lr0_gotos = {}

    for itemset in lr0_itemsets:

        for symbol in chain(nonterminals, terminals, '$'):
            new_itemset = lr0_itemset_goto(itemset, symbol, grammar_)
            i = lr0_itemsets.index(itemset)

            if len(new_itemset) == 0:
                continue

            if new_itemset not in lr0_itemsets:
                lr0_itemsets.append(new_itemset)

            lr0_gotos[(i, symbol)] = lr0_itemsets.index(new_itemset)

    return lr0_itemsets, lr0_gotos

def generate_lr0_parsing_table(itemsets, gotos, grammar_):
    grammar, nonterminals, terminals, productions = grammar_

    table = {}
    nonterminal_gotos = {}

    for i in range(len(itemsets)):
        for s in terminals:
            table[(i, s)] = []
        table[(i, '$')] = []

        for s in nonterminals:
            nonterminal_gotos[(i, s)] = None

    table[(1, '$')] = ["acc"]

    for i, itemset in enumerate(itemsets):
        for (head, production, dot) in lr0_itemset_closure(itemset, grammar_):
            grammar_symbols = production.split(' ')
            symbol_after_dot = grammar_symbols[dot] if dot < len(grammar_symbols) else None

            if symbol_after_dot is None:
                for s in follow(head, grammar_):
                    table[(i, s)].append(f"r{productions.index((head, production))}")

            elif symbol_after_dot in terminals:
                j = gotos[(i, symbol_after_dot)]
                table[(i, symbol_after_dot)].append(f"s{j}")

            else:
                goto_state = lr0_itemset_goto(itemset, symbol_after_dot, grammar_)
                j = itemsets.index(goto_state)
                nonterminal_gotos[(i, symbol_after_dot)] = j

    for i in table:
        table[i] = sorted(set(table[i]))
    return table, nonterminal_gotos

def write_table(parsing_table, nonterminal_gotos, grammar_):
    grammar, nonterminals, _, productions = grammar_
    # {{{ Token names table
    token_name_table = {
        "$": 'Token::EndOfInput',
        "':'": 'Token::Colon',
        "','": 'Token::Comma',
        "';'": 'Token::SemiColon',
        "'{'": 'Token::LeftBrace',
        "'}'": 'Token::RightBrace',
        "'('": 'Token::LeftParen',
        "')'": 'Token::RightParen',
        "'['": 'Token::LeftBracket',
        "']'": 'Token::RightBracket',
        "'=>'": 'Token::FatArrow',
        "'.'": 'Token::Dot',
        "'@'": 'Token::At',
        "'~'": 'Token::Complement',
        "'+'": 'Token::MathOp(MathOp::Plus)',
        "'-'": 'Token::MathOp(MathOp::Minus)',
        "'*'": 'Token::MathOp(MathOp::Multiply)',
        "'/'": 'Token::MathOp(MathOp::Divide)',
        "'<-'": 'Token::Assignment',
        "'<='": 'Token::Relop(Relop::LE)',
        "'<'": 'Token::Relop(Relop::LT)',
        "'='": 'Token::Relop(Relop::EE)',
        "'ID'": 'Token::Id(_)',
        "'TYPE'": 'Token::Type(_)',
        "'integer'": 'Token::Integer(_)',
        "'string'": 'Token::StringLiteral(_)',
        "'class'": 'Token::Class',
        "'else'": 'Token::Else',
        "'false'": 'Token::False',
        "'fi'": 'Token::Fi',
        "'if'": 'Token::If',
        "'in'": 'Token::In',
        "'inherits'": 'Token::Inherits',
        "'isvoid'": 'Token::Isvoid',
        "'let'": 'Token::Let',
        "'loop'": 'Token::Loop',
        "'pool'": 'Token::Pool',
        "'then'": 'Token::Then',
        "'while'": 'Token::While',
        "'case'": 'Token::Case',
        "'esac'": 'Token::Esac',
        "'new'": 'Token::New',
        "'of'": 'Token::Of',
        "'not'": 'Token::Not',
        "'true'": 'Token::True',
    }
    # }}} 

    with open("actions.txt", "w") as f:
        for k in parsing_table:
            if len(parsing_table[k]) == 0:
                continue

            table_entry = f"({k[0]}, {token_name_table[k[1]]}) => "

            for entry in parsing_table[k]:
                if entry[0] == 's':
                    table_entry += f"Action::Shift({entry[1:]}), "
                elif entry[0] == 'r':
                    idx = int(entry[1:])
                    prod_len = productions[idx][1].count(' ') + 1
                    table_entry += f"Action::Reduce({idx}, {prod_len}), "
                elif entry == "acc":
                    table_entry += "Action::Accept, "

            f.write(table_entry)
            f.write('\n')

    with open("gotos.txt", 'w') as f:
        for k in filter(lambda k: nonterminal_gotos[k] is not None, nonterminal_gotos):
            f.write(f"({k[0]}, {nonterminals.index(k[1])}) => Ok({nonterminal_gotos[k]}),\n")

    with open("productions.txt", 'w') as f:
        for i, (k, p) in enumerate(productions):
                f.write(f"{i} => Ok(\"{k} -> {p}\"),\n")

def write_graph(lr0_itemsets, lr0_gotos):
    graphviz_str = """digraph LR0 {
        splines=ortho;
        randdir=LR;
    """

    for i, itemset in enumerate(lr0_itemsets):
        itemset_str = reduce(lambda p, a: f'{p}{a}\\n ', print_itemset(itemset), '')
        graphviz_str += f"\tI_{i} [shape=square, label=\"{i}\\n{itemset_str}\"]; \n"

    inverted_gotos = {}
    for i, sym in lr0_gotos:
        j = lr0_gotos[(i, sym)]
        if (j, sym) not in inverted_gotos:
            inverted_gotos[(j, sym)] = []

        inverted_gotos[(j, sym)].append(i)

    for i, sym in inverted_gotos:
        source_str = reduce(lambda acc, p: f"{acc}I_{p} ", inverted_gotos[(i, sym)], '')
        graphviz_str += f"\t{{ {source_str} }} -> I_{i} ;\n"

    graphviz_str += "\n}"

    with open("graph.gv", 'w') as g:
        g.write(graphviz_str)




if __name__ == '__main__':
    import argparse
    lr0_itemsets, lr0_gotos = generate_lr0_automaton(grammar)

    with open('lr0_automaton.bin', 'wb') as f:
        import pickle
        pickle.dump((lr0_itemsets, lr0_gotos), f)

    write_graph(lr0_itemsets, lr0_gotos)

    args_parser = argparse.ArgumentParser(
        prog="generate_parser_table",
        description="A script to generate the parser table"
    )

    args_parser.add_argument('--store-table', action="store", type=bool, default=True)
    args = args_parser.parse_args()

    if args.store_table:
        table, gotos = generate_lr0_parsing_table(lr0_itemsets, lr0_gotos, grammar)

        write_table(table, gotos, grammar)

    conflicting_states = filter(lambda combo: len(combo[1]) > 1, table.items())
    conflicting_states = reduce(lambda acc, combo: [*acc, combo[0][0]], conflicting_states, [])
    conflicting_states = sorted(set(conflicting_states))
    print(f"{conflicting_states = }\nn={len(conflicting_states)}")

