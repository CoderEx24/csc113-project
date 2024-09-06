from itertools import chain
from functools import reduce

'''
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
'''

grammar = {
    'E': ["E '+' T", "E '-' T", "T"],
    'T': ["T '*' F", "T '/' F", "F"],
    'F': ["'id'", "'(' E ')'"],
}

nonterminals = list(grammar.keys())

terminals = []
for prods in grammar.values():
    for p in prods:
        for piece in p.split(' '):
            if piece.startswith("'"):
                terminals.append(piece)

terminals = set(terminals)

grammar = (grammar, nonterminals, terminals)

def first(symbol, grammar_):
    grammar, nonterminals, terminals = grammar_

    if symbol in terminals or symbol == '':
        yield symbol

    else:
        for production in grammar[symbol]:
            for prod_symbol in production.split(' '):
                if prod_symbol == symbol:
                    break

                symbol_first_list = list(first(prod_symbol))

                for i in symbol_first_list:
                    yield i

                if '' not in symbol_first_list:
                    break

def follow(symbol, grammar_):
    grammar, nonterminals, terminals = grammar_

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
                        gs_first_list = list(set(first(gs, grammar_)))
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
    grammar, nonterminals, terminals = grammar_

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
    for (head, production, dot) in itemset:
        pieces = production.split(' ')
        if dot >= len(pieces):
            continue

        if pieces[dot] == grammar_symbol:
            new_itemset.append((head, production, dot + 1))

    return lr0_itemset_closure(new_itemset, grammar_)

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
    grammar, nonterminals, terminals = grammar_

    lr0_itemsets = [
        lr0_itemset_closure([(f'{nonterminals[0]}_', nonterminals[0], 0)], grammar_)
    ]

    lr0_gotos = {}

    for itemset in lr0_itemsets:
        i = lr0_itemsets.index(itemset)

        for symbol in chain(nonterminals, terminals):
            new_itemset = lr0_itemset_goto(itemset, symbol, grammar_)

            if len(new_itemset) == 0:
                continue

            if new_itemset not in lr0_itemsets:
                lr0_itemsets.append(new_itemset)

            lr0_gotos[(i, symbol)] = lr0_itemsets.index(new_itemset)

    return lr0_itemsets, lr0_gotos

def generate_lr0_parsing_table(itemsets, gotos, grammar_):
    grammar, nonterminals, terminals = grammar_

    table = {(2, '$'): ["acc"]}
    productions = []

    for i in grammar.values():
        productions.extend(i)

    for i in range(len(itemsets)):
        for s in terminals:
            table[(i, s)] = []
        table[(i, '$')] = []

    for i, itemset in enumerate(itemsets):
        for (head, production, dot) in itemset:
            grammar_symbols = production.split(' ')
            symbol_after_dot = grammar_symbols[dot] if dot < len(grammar_symbols) else None

            if symbol_after_dot is None:
                for s in follow(head, grammar_):
                    table[(i, s)].append(f"r{productions.index(production)}")

            elif symbol_after_dot in terminals:
                j = gotos[(i, symbol_after_dot)]
                table[(i, symbol_after_dot)].append(f"s{j}")


    for i in table:
        table[i] = sorted(set(table[i]))
    return table

lr0_itemsets, lr0_gotos = generate_lr0_automaton(grammar)
productions = []

for i in grammar[0].values():
    productions.extend(i)

table = generate_lr0_parsing_table(lr0_itemsets, lr0_gotos, grammar)

for k in table:
    if len(table[k]) > 0:
        print(f"{k} -> {table[k]}")

for k in table:
    if len(table[k]) > 1:
        actions = list(table[k])
        actions_str = ''
        for action in actions:
            if action[0] == 'r':
                actions_str += f"reduce by {productions[int(action[1:])]}, "
            elif action[0] == 's':
                actions_str += f"shift {action[1:]}, "

        print(f"{k} -> {actions_str[:-2]}")

conflicting_states = filter(lambda combo: len(combo[1]) > 1, table.items())
conflicting_states = reduce(lambda acc, combo: [*acc, combo[0][0]], conflicting_states, [])
conflicting_states = sorted(set(conflicting_states))
print(f"{conflicting_states = }")

graphviz_str = "digraph LR0 { rankdir=LR; \n"

for i, itemset in enumerate(lr0_itemsets):
    itemset_str = reduce(lambda p, a: f'{p}{a}\\n ', print_itemset(itemset), '')
    graphviz_str += f"\tI_{i} [shape=square, label=\"{i}\\n{itemset_str}\"]; \n"

for i, sym in lr0_gotos:
    graphviz_str += f"\tI_{i} -> I_{lr0_gotos[(i, sym)]} [headlabel=\"{sym}\"];\n"

graphviz_str += "\n}"

with open("graph.gv", 'w') as g:
    g.write(graphviz_str)


