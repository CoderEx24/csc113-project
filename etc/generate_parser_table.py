from itertools import chain
from functools import reduce

def read_grammar(grammar_filename):
    terminals = []
    nonterminals = []
    productions = []
    grammar = {}
    with open(grammar_filename, 'r') as f:
        for line in filter(lambda l: not l.isspace(), f.readlines()):
            head, production = line.split('->')
            head = head.strip()
            production = production.strip()

            if head not in grammar:
                grammar[head] = [production]

            grammar[head].append(production)
            productions.append((head, production))
            if head not in nonterminals:
                nonterminals.append(head)

            for t in filter(lambda p: p.startswith("'") and p not in terminals, production.split(' ')):
                terminals.append(t)

    return (grammar, nonterminals, terminals, productions)

def first(symbol, grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    if symbol in terminals or symbol == '':
        yield symbol

    else:
        possible_first_symbols = terminals.copy()

        for production in grammar[symbol]:
            for prod_symbol in production.split(' '):
                if prod_symbol == symbol:
                    break

                symbol_first_list = first(prod_symbol, grammar_)

                for i in filter(lambda i: i in possible_first_symbols, symbol_first_list):
                    possible_first_symbols.remove(i)
                    yield i

                if '' not in symbol_first_list:
                    break

def follow(symbol, grammar_):
    grammar, nonterminals, terminals, _ = grammar_

    if symbol == '':
        raise "Epsilon has no follow"

    if symbol == nonterminals[0]:
        yield '$'

    possible_follow_symbols = [*terminals, '$']

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
                        gs_first_list = first(gs, grammar_)
                        for i in filter(lambda ii: ii != '' and ii in possible_follow_symbols, gs_first_list):
                            possible_follow_symbols.remove(i)
                            yield i

                        if '' not in gs_first_list:
                            break
                else:
                    if nonterminal == symbol:
                        continue

                    for i in filter(lambda ii: ii in possible_follow_symbols, follow(nonterminal, grammar_)):
                        possible_follow_symbols.remove(i)
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

def write_table(parsing_table, nonterminal_gotos, grammar_, actions_filename, gotos_filename, productions_filename):
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

    with open(actions_filename, "w") as f:
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

    with open(gotos_filename, 'w') as f:
        for k in filter(lambda k: nonterminal_gotos[k] is not None, nonterminal_gotos):
            f.write(f"({k[0]}, {nonterminals.index(k[1])}) => Ok({nonterminal_gotos[k]}),\n")

    with open(productions_filename, 'w') as f:
        for i, (k, p) in enumerate(productions):
                f.write(f"{i} => Ok(\"{k} -> {p}\"),\n")

def write_graph(lr0_itemsets, lr0_gotos, graph_filename):
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

    with open(graph_filename, 'w') as g:
        g.write(graphviz_str)

if __name__ == '__main__':
    import argparse
    from os import path

    args_parser = argparse.ArgumentParser(
        prog="generate_parser_table",
        description="A script to generate the parser table"
    )

    args_parser.add_argument('grammar_filepath')
    args_parser.add_argument('--dont-store-table', action='store_true')
    args_parser.add_argument('--dont-store-automaton', action='store_true')
    args_parser.add_argument('--dont-store-graph', action='store_true')
    args_parser.add_argument('-O', '--output-directory', action='store', type=str, default='.')
    args_parser.add_argument('--automaton-filename', action='store', type=str, default='lr0_automaton.bin')
    args_parser.add_argument('--actions-filename', action='store', type=str, default='actions.txt')
    args_parser.add_argument('--gotos-filename', action='store', type=str, default='gotos.txt')
    args_parser.add_argument('--productions-filename', action='store', type=str, default='productions.txt')
    args_parser.add_argument('--graph-filename', action='store', type=str, default='graph.gv')
    args = args_parser.parse_args()

    output_dir = args.output_directory
    automaton_filename = path.join(output_dir, args.automaton_filename)
    actions_filename = path.join(output_dir, args.actions_filename)
    gotos_filename = path.join(output_dir, args.gotos_filename)
    productions_filename = path.join(output_dir, args.productions_filename)
    graph_filename = path.join(output_dir, args.graph_filename)

    grammar = read_grammar(args.grammar_filepath)
    lr0_itemsets, lr0_gotos = generate_lr0_automaton(grammar)

    if not args.dont_store_automaton:
        with open(automaton_filename, 'wb') as f:
            import pickle
            pickle.dump((lr0_itemsets, lr0_gotos), f)

    if not args.dont_store_graph:
        write_graph(lr0_itemsets, lr0_gotos, graph_filename)

    if not args.dont_store_table:
        table, gotos = generate_lr0_parsing_table(lr0_itemsets, lr0_gotos, grammar)

        write_table(table, gotos, grammar, actions_filename, gotos_filename, productions_filename)

    conflicting_states = filter(lambda combo: len(combo[1]) > 1, table.items())
    conflicting_states = reduce(lambda acc, combo: [*acc, combo[0][0]], conflicting_states, [])
    conflicting_states = sorted(set(conflicting_states))
    print(f"{conflicting_states = }\nn={len(conflicting_states)}")

