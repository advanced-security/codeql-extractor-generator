private import codeql.{{ config.name }}.AST

query predicate astNodes(AstNode n) { any() }

query predicate literals(Literals l) { any() }
