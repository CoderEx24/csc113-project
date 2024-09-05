from itertools import chain
from functools import reduce

grammar = {
    'program': ["class_prod ';' program", "class_prod ';'"],

    'class_prod': ["'class' 'TYPE' '{' feature_list '}'",
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

nonterminals = list(grammar.keys())

terminals = []
for prods in grammar.values():
    for p in prods:
        for piece in p.split(' '):
            if piece.startswith("'"):
                terminals.append(piece)

terminals = set(terminals)

def first(symbol, __cache=[]):
    global grammar, nonterminals, terminals

    if symbol in terminals or symbol == '':
        if symbol not in __cache:
            __cache.append(symbol)
            yield symbol

    else:
        for production in grammar[symbol]:
            for prod_symbol in production.split(' '):
                if prod_symbol == symbol:
                    break

                symbol_first_list = list(first(prod_symbol, __cache))

                for i in symbol_first_list:
                    yield i

                if '' not in symbol_first_list:
                    break

def follow(symbol):
    global grammar, nonterminals, terminals

    if symbol == '':
        raise "Epsilon has no follow"

    if symbol == nonterminals[0]:
        yield '$'

    for (head, prods) in grammar.items():
        for prod in prods:
            grammar_symbols = prod.split(' ')

            if symbol not in grammar_symbols:
                continue

            for (idx, s) in enumerate(grammar_symbols):
                if s != symbol:
                    continue

                if idx < len(grammar_symbols) - 1:
                    for gs in grammar_symbols[idx + 1:]:
                        gs_first_list = list(first(gs))
                        for i in gs_first_list:
                            if i != '':
                                yield i

                        if '' not in gs_first_list:
                            break
                else:
                    if head == symbol:
                        continue

                    for i in follow(head):
                        if i != '':
                            yield i

def lr0_itemset_closure(itemset):
    global grammar, nonterminals, terminals

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

def lr0_itemset_goto(itemset, grammar_symbol):
    new_itemset = []
    for (head, production, dot) in itemset:
        pieces = production.split(' ')
        if dot >= len(pieces):
            break

        if pieces[dot] == grammar_symbol:
            new_itemset.append((head, production, dot + 1))

    return lr0_itemset_closure(new_itemset)

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

def generate_lr0_automaton():
    global grammar, nonterminals, terminals

    lr0_itemsets = [
        lr0_itemset_closure([('program_', 'program', 0)])
    ]

    lr0_gotos = {}

    for itemset in lr0_itemsets:
        i = lr0_itemsets.index(itemset)

        for symbol in chain(terminals, nonterminals):
            new_itemset = lr0_itemset_goto(itemset, symbol)

            if len(new_itemset) == 0:
                continue

            if new_itemset not in lr0_itemsets:
                #print(f"Symbl: {symbol} I_{i} = {list(print_itemset(itemset))} => I_{len(itemsets)} = {list(print_itemset(new_itemset))}")
                lr0_itemsets.append(new_itemset)

            lr0_gotos[i].append(lr0_itemsets.index(new_itemset))

    return lr0_itemsets, lr0_gotos


graphviz_str = "digraph LR0 { rankdir=LR; \n"

lr0_itemsets, lr0_gotos = generate_lr0_automaton()

for i, itemset in enumerate(lr0_itemsets):
    itemset_str = reduce(lambda p, a: f'{p}{a}\\n ', print_itemset(itemset), '')
    graphviz_str += f"I_{i} [shape=square, label=\"{itemset_str}\"]; \n"

for i in lr0_gotos:
    target_str = reduce(lambda p, a: f"{p}I_{a} ", lr0_gotos[i], '')
    graphviz_str += f"I_{i} -> {{ {target_str} }};\n"

graphviz_str += "\n}"

with open("graph.gv", 'w') as g:
    g.write(graphviz_str)


